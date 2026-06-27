use automerge::ReadDoc;
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::de;

#[derive(Debug)]
pub struct Projection {
    pub(super) changes: Vec<automerge::ChangeHash>,

    pub(super) root_progress: super::node::Progress,
    pub(super) nodes: FxHashMap<automerge::ObjId, super::node::NodeData>,
    pub(super) parent: FxHashMap<automerge::ObjId, automerge::ObjId>,
    pub(super) children: FxHashMap<automerge::ObjId, Vec<automerge::ObjId>>,
}

impl Projection {
    pub(super) fn new(document: &automerge::Automerge) -> super::error::Result<Self> {
        let mut projection = Self {
            changes: document.get_heads(),

            root_progress: super::node::Progress::default(),
            nodes: FxHashMap::default(),
            parent: FxHashMap::default(),
            children: FxHashMap::default(),
        };
        projection.rebuild(document)?;
        Ok(projection)
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.parent.clear();
        self.children.clear();
        self.root_progress = super::node::Progress::default();
    }
}

impl Projection {
    fn node_name_mut(&mut self) -> Result<&mut String, super::error::TreeError> {
        Ok(&mut self
            .nodes
            .get_mut(&automerge::ObjId::Root)
            .ok_or(super::error::TreeError::MissingProperty)?
            .name)
    }
}

impl Projection {
    pub(super) fn change_node(
        &mut self,
        id: automerge::ObjId,
        document: &automerge::Automerge,
    ) -> super::error::Result<()> {
        let node = super::node::NodeData::from_doc(document, &id)?;
        self.nodes.entry(id).and_modify(|stored_node| {
            stored_node.name = node.name;
            stored_node.desc = node.desc;
        });
        Ok(())
    }

    pub(super) fn apply_patches(
        &mut self,
        document: &automerge::Automerge,
        patches: Vec<automerge::Patch>,
    ) {
        use automerge::patches::PatchAction;
        let mut paths_to_recompute = FxHashSet::default();

        for patch in patches {
            match patch.action {
                // Node properties changed (name, desc, totals)
                PatchAction::PutMap { .. } | PatchAction::Increment { .. } => {
                    paths_to_recompute.insert(patch.obj);
                }

                // The CHILDREN list structure changed
                PatchAction::Insert { .. }
                | PatchAction::PutSeq { .. }
                | PatchAction::DeleteSeq { .. } => {
                    if let Ok(mut parents) = document.parents(&patch.obj) {
                        if let Some(parent_info) = parents.next() {
                            let parent_id = parent_info.obj;

                            // 1. Fetch current live children from the document
                            let mut new_child_ids = Vec::new();
                            let list_len = document.length(&patch.obj);
                            for idx in 0..list_len {
                                if let Ok(Some((_, child_id))) = document.get(&patch.obj, idx) {
                                    new_child_ids.push(child_id.clone());
                                }
                            }

                            // 2. Identify and purge deleted nodes
                            if let Some(old_child_ids) = self.children.get(&parent_id).cloned() {
                                // <-- Add .cloned() here
                                for old_id in &old_child_ids {
                                    // <-- Iterate by reference over the cloned Vec
                                    if !new_child_ids.contains(old_id) {
                                        self.purge_recursive(old_id); // Now self is free to be borrowed mutably!
                                    }
                                }
                            }

                            // 3. Rebuild map relationships
                            for new_id in &new_child_ids {
                                self.parent.insert(new_id.clone(), parent_id.clone());
                            }
                            self.children.insert(parent_id.clone(), new_child_ids);

                            // 4. Mark parent so its progress updates based on new children
                            paths_to_recompute.insert(parent_id);
                        }
                    }
                }

                // Complete node removed (often handled via parent list, but acts as a fallback)
                PatchAction::DeleteMap { .. } => {
                    self.purge_recursive(&patch.obj);
                }
                _ => {}
            }
        }

        // Re-parse marked nodes and bubble up their progress calculations
        for id in paths_to_recompute {
            if id != automerge::ObjId::Root {
                if let Ok(mut node_data) = super::node::NodeData::from_doc(document, &id) {
                    // Retain existing progress until recalculated to prevent flicker
                    if let Some(existing) = self.nodes.get(&id) {
                        node_data.progress = existing.progress;
                    }
                    self.nodes.insert(id.clone(), node_data);
                }
            }

            // Walk up the tree to recalculate branch progress
            let mut current_id = id;
            loop {
                let progress = self.calculate_progress(document, &current_id);

                if current_id == automerge::ObjId::Root {
                    self.root_progress = progress;
                    break;
                } else if let Some(node) = self.nodes.get_mut(&current_id) {
                    node.progress = progress;
                }

                if let Some(parent_id) = self.parent.get(&current_id).cloned() {
                    current_id = parent_id;
                } else {
                    break;
                }
            }
        }

        // Finalize the state sync
        self.changes = document.get_heads();
    }
}

impl Projection {
    fn rebuild(&mut self, document: &automerge::Automerge) -> super::error::Result<()> {
        self.clear();
        self.build_recursive(document, &automerge::ObjId::Root)?;
        self.changes = document.get_heads();

        Ok(())
    }

    fn build_recursive(
        &mut self,
        document: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::error::Result<super::node::Progress> {
        let mut child_ids = Vec::new();

        if let Ok(Some((_, list_id))) = document.get(id, super::CHILDREN) {
            let list_len = document.length(&list_id);
            for idx in 0..list_len {
                if let Ok(Some((_, child_id))) = document.get(&list_id, idx) {
                    child_ids.push(child_id.clone());
                    self.parent.insert(child_id.clone(), id.clone());
                    self.build_recursive(document, &child_id)?;
                }
            }
        }

        self.children.insert(id.clone(), child_ids);

        let progress = self.calculate_progress(document, id);

        if id == &automerge::ObjId::Root {
            self.root_progress = progress;
            return Ok(progress);
        }

        if let Ok(mut node_data) = super::node::NodeData::from_doc(document, id) {
            node_data.progress = progress;
            self.nodes.insert(id.clone(), node_data);
        }

        Ok(progress)
    }
}

impl Projection {
    pub(super) fn update_node_name(&mut self, id: &automerge::ObjId, name: String) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.name = name;
        }
    }

    pub(super) fn update_node_desc(&mut self, id: &automerge::ObjId, desc: String) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.desc = desc;
        }
    }

    pub(super) fn update_node_total(&mut self, id: &automerge::ObjId, total: u32) {
        if let Some(node) = self.nodes.get_mut(id) {
            let mut progress = node.progress;
            progress.total = total;
            node.progress = progress;
        }
    }
}

impl Projection {
    fn calculate_progress(
        &self,
        document: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::node::Progress {
        let child_ids = self.children.get(id).cloned().unwrap_or_default();

        if child_ids.is_empty() {
            if id == &automerge::ObjId::Root {
                return super::node::Progress::default();
            }
            if let Ok(node) = super::node::NodeData::from_doc(document, id) {
                return node.progress;
            }
            super::node::Progress::default()
        } else {
            let total = (child_ids.len() as u32) * 100;
            let mut completed = 0;

            for cid in &child_ids {
                if let Some(child_node) = self.nodes.get(cid) {
                    completed += child_node.progress.procentage() as u32;
                }
            }
            super::node::Progress::new(completed, total)
        }
    }

    pub(super) fn purge_recursive(&mut self, id: &automerge::ObjId) {
        self.nodes.remove(id);
        self.parent.remove(id);
        if let Some(child_ids) = self.children.remove(id) {
            for child_id in child_ids {
                self.purge_recursive(&child_id);
            }
        }
    }
}

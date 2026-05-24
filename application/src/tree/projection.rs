use std::collections::HashMap;

use automerge::ReadDoc;

#[derive(Debug)]
pub struct Projection {
    pub(super) changes: Vec<automerge::ChangeHash>,

    pub(super) root_progress: super::node::Progress,
    pub(super) nodes: HashMap<automerge::ObjId, super::node::Node>,
    pub(super) parent: HashMap<automerge::ObjId, automerge::ObjId>,
    pub(super) children: HashMap<automerge::ObjId, Vec<automerge::ObjId>>,
}

impl Projection {
    pub(super) fn new(document: &automerge::Automerge) -> super::error::Result<Self> {
        let mut projection = Self {
            changes: document.get_heads(),
            nodes: HashMap::new(),
            parent: HashMap::new(),
            children: HashMap::new(),
            root_progress: super::node::Progress::default(),
        };
        projection.rebuild(document)?;

        Ok(projection)
    }
}

impl Projection {
    fn clear(&mut self) {
        self.nodes.clear();
        self.parent.clear();
        self.children.clear();
        self.root_progress = super::node::Progress::default();
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
            if let Ok(node) = super::node::Node::from_doc(document, id) {
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
}

impl Projection {
    pub(super) fn rebuild(&mut self, document: &automerge::Automerge) -> super::error::Result<()> {
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

        if let Ok(mut node_data) = super::node::Node::from_doc(document, id) {
            node_data.progress = progress;
            self.nodes.insert(id.clone(), node_data);
        }

        Ok(progress)
    }
}

impl Projection {
    pub(super) fn purge_recursive(&mut self, id: &automerge::ObjId) {
        self.nodes.remove(id);
        self.parent.remove(id);
        if let Some(child_ids) = self.children.remove(id) {
            for child_id in child_ids {
                self.purge_recursive(&child_id);
            }
        }
    }

    pub(super) fn update_up_from(
        &mut self,
        mut id: automerge::ObjId,
        parent_id: Option<automerge::ObjId>,
        document: &automerge::Automerge,
    ) {
        if let Some(parent_id) = parent_id {
            self.parent.insert(id.clone(), parent_id.clone());
            self.children.entry(parent_id).or_default().push(id.clone());
        }

        let progress = self.calculate_progress(document, &id);

        if id == automerge::ObjId::Root {
            self.root_progress = progress;
        } else if let Some(node) = self.nodes.get_mut(&id) {
            node.progress = progress;
        } else if let Ok(mut new_node) = super::node::Node::from_doc(document, &id) {
            new_node.progress = progress;
            self.nodes.insert(id.clone(), new_node);
        }

        let Some(mut current_id) = self.parent.get(&id).cloned() else {
            return;
        };

        loop {
            let branch_progress = self.calculate_progress(document, &current_id);

            if current_id == automerge::ObjId::Root {
                self.root_progress = branch_progress;
                break;
            } else if let Some(node) = self.nodes.get_mut(&current_id) {
                node.progress = branch_progress;
            }

            if let Some(next_parent_id) = self.parent.get(&current_id).cloned() {
                current_id = next_parent_id;
            } else {
                break;
            }
        }

        self.changes = document.get_heads();
    }
}

impl Projection {
    pub fn apply_patches(
        &mut self,
        document: &automerge::Automerge,
        patches: Vec<automerge::Patch>,
    ) {
        use automerge::patches::PatchAction;

        let mut paths_to_recompute = Vec::new();

        for patch in patches {
            let obj_id = patch.obj;

            match patch.action {
                PatchAction::PutMap { .. }
                | PatchAction::PutSeq { .. }
                | PatchAction::Increment { .. }
                | PatchAction::Insert { .. }
                | PatchAction::DeleteMap { .. }
                | PatchAction::DeleteSeq { .. } => {
                    paths_to_recompute.push(obj_id);
                }
                _ => {}
            }
        }

        paths_to_recompute.sort();
        paths_to_recompute.dedup();

        for id in paths_to_recompute {
            // Replace update_path with update_up_from
            self.update_up_from(id, None, document);
        }

        self.changes = document.get_heads();
    }
}

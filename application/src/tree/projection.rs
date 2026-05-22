use std::collections::HashMap;

use automerge::ReadDoc;

#[derive(Debug)]
pub struct Projection {
    pub(super) changes: Vec<automerge::ChangeHash>,

    pub(super) root_progress: super::node::Progress,
    pub(super) nodes: HashMap<automerge::ObjId, super::node::Node>,
    pub(super) children: HashMap<automerge::ObjId, Vec<automerge::ObjId>>,
}

impl Projection {
    pub(super) fn new(document: &automerge::Automerge) -> super::error::Result<Self> {
        let mut proj = Self {
            changes: document.get_heads(),
            nodes: HashMap::new(),
            children: HashMap::new(),
            root_progress: super::node::Progress::default(),
        };
        proj.rebuild(document)?;
        Ok(proj)
    }
}

impl Projection {
    fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.root_progress = super::node::Progress::default();
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
        let mut root_progress = super::node::Progress::default();
        let mut child_ids = Vec::new();

        if let Ok(Some((_, list_id))) = document.get(id, super::CHILDREN) {
            let list_len = document.length(&list_id);
            for idx in 0..list_len {
                if let Ok(Some((_, child_id))) = document.get(&list_id, idx) {
                    child_ids.push(child_id.clone());
                    root_progress += self.build_recursive(document, &child_id)?;
                }
            }
        }

        self.children.insert(id.clone(), child_ids.clone());

        if id == &automerge::ObjId::Root {
            self.root_progress = root_progress.clone();
            return Ok(root_progress);
        }

        if let Ok(mut node_data) = super::node::Node::from_doc(document, id) {
            if child_ids.is_empty() {
                root_progress = node_data.progress.clone();
            } else {
                node_data.progress = root_progress.clone();
            }
            self.nodes.insert(id.clone(), node_data);
        }

        Ok(root_progress)
    }
}
impl Projection {
    pub fn update_path(
        &mut self,
        document: &automerge::Automerge,
        mut current_id: automerge::ObjId,
    ) -> super::error::Result<()> {
        loop {
            let mut new_progress = super::node::Progress::default();
            let child_ids = self.children.get(&current_id).cloned().unwrap_or_default();

            if child_ids.is_empty() && current_id != automerge::ObjId::Root {
                if let Ok(doc_node) = super::node::Node::from_doc(document, &current_id) {
                    new_progress = doc_node.progress;
                }
            } else {
                for cid in &child_ids {
                    if let Some(child_node) = self.nodes.get(cid) {
                        new_progress += child_node.progress.clone();
                    }
                }
            }

            if current_id == automerge::ObjId::Root {
                self.root_progress = new_progress;
                break;
            } else if let Some(node) = self.nodes.get_mut(&current_id) {
                node.progress = new_progress;
            }

            let mut parents = document.parents(&current_id)?;
            if parents.next().is_some()
                && let Some(map_parent) = parents.next()
            {
                current_id = map_parent.obj;
                continue;
            }
            break;
        }

        self.changes = document.get_heads();
        Ok(())
    }

    pub fn apply_patches(
        &mut self,
        document: &automerge::Automerge,
        patches: Vec<automerge::Patch>,
    ) -> super::error::Result<()> {
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
            self.update_path(document, id)?;
        }

        self.changes = document.get_heads();

        Ok(())
    }
}

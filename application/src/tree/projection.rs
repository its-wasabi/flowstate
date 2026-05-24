use automerge::ReadDoc;
// Crucial: Swap out standard HashMap for FxHashMap
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
pub struct Projection {
    pub(super) changes: Vec<automerge::ChangeHash>,

    pub(super) root_progress: super::node::Progress,
    pub(super) nodes: FxHashMap<automerge::ObjId, super::node::Node>,
    pub(super) parent: FxHashMap<automerge::ObjId, automerge::ObjId>,
    pub(super) children: FxHashMap<automerge::ObjId, Vec<automerge::ObjId>>,

    // Cache invalidation flags for the UI loop
    dirty_nodes: FxHashSet<automerge::ObjId>,
    root_dirty: bool,
}

impl Projection {
    pub(super) fn new(document: &automerge::Automerge) -> super::error::Result<Self> {
        let mut projection = Self {
            changes: document.get_heads(),
            nodes: FxHashMap::default(),
            parent: FxHashMap::default(),
            children: FxHashMap::default(),
            dirty_nodes: FxHashSet::default(),
            root_progress: super::node::Progress::default(),
            root_dirty: false,
        };
        projection.rebuild(document)?;
        Ok(projection)
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.parent.clear();
        self.children.clear();
        self.dirty_nodes.clear();
        self.root_dirty = false;
        self.root_progress = super::node::Progress::default();
    }

    pub(super) fn rebuild(&mut self, document: &automerge::Automerge) -> super::error::Result<()> {
        self.clear();
        self.build_structure(document, &automerge::ObjId::Root)?;

        // Compute progress cleanly in one pass after structure is built
        self.root_dirty = true;
        self.recompute_dirty(document);
        self.changes = document.get_heads();

        Ok(())
    }

    /// Recursively builds the parent/child structure without eagerly computing progress
    fn build_structure(
        &mut self,
        document: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::error::Result<()> {
        let mut child_ids = Vec::new();

        if let Ok(Some((_, list_id))) = document.get(id, super::CHILDREN) {
            let list_len = document.length(&list_id);
            for idx in 0..list_len {
                if let Ok(Some((_, child_id))) = document.get(&list_id, idx) {
                    child_ids.push(child_id.clone());
                    self.parent.insert(child_id.clone(), id.clone());
                    self.build_structure(document, &child_id)?;
                }
            }
        }

        self.children.insert(id.clone(), child_ids);

        if id != &automerge::ObjId::Root {
            if let Ok(node_data) = super::node::Node::from_doc(document, id) {
                self.nodes.insert(id.clone(), node_data);
            }
        }

        Ok(())
    }
}

// --- The UI Loop Optimization ---

impl Projection {
    /// Marks a node and its ancestors as dirty.
    /// Stops propagating instantly if a parent is already dirty.
    fn mark_dirty_up(&mut self, mut id: automerge::ObjId) {
        loop {
            if id == automerge::ObjId::Root {
                self.root_dirty = true;
                break;
            }

            // If the node was ALREADY dirty, its parents are too. Break early.
            if !self.dirty_nodes.insert(id.clone()) {
                break;
            }

            if let Some(parent_id) = self.parent.get(&id) {
                id = parent_id.clone();
            } else {
                break;
            }
        }
    }

    /// Triggers a single DFS sweep to recalculate only the dirtied branches.
    fn recompute_dirty(&mut self, document: &automerge::Automerge) {
        if !self.root_dirty && self.dirty_nodes.is_empty() {
            return;
        }

        self.root_progress = self.recompute_dfs(document, &automerge::ObjId::Root);
        self.dirty_nodes.clear();
        self.root_dirty = false;
    }

    fn recompute_dfs(
        &mut self,
        document: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::node::Progress {
        // Pruning: If this branch isn't dirty, return the cached value immediately.
        if id != &automerge::ObjId::Root && !self.dirty_nodes.contains(id) {
            if let Some(node) = self.nodes.get(id) {
                return node.progress;
            }
        }

        let child_ids = self.children.get(id).cloned().unwrap_or_default();

        let progress = if child_ids.is_empty() {
            if id == &automerge::ObjId::Root {
                super::node::Progress::default()
            } else if let Ok(node) = super::node::Node::from_doc(document, id) {
                node.progress
            } else {
                super::node::Progress::default()
            }
        } else {
            let total = (child_ids.len() as u32) * 100;
            let mut completed = 0;

            for cid in &child_ids {
                completed += self.recompute_dfs(document, cid).procentage() as u32;
            }
            super::node::Progress::new(completed, total)
        };

        if id != &automerge::ObjId::Root {
            if let Some(node) = self.nodes.get_mut(id) {
                node.progress = progress;
            } else if let Ok(mut new_node) = super::node::Node::from_doc(document, id) {
                new_node.progress = progress;
                self.nodes.insert(id.clone(), new_node);
            }
        }

        progress
    }
}

// --- Updates and Patches ---

impl Projection {
    pub fn apply_patches(
        &mut self,
        document: &automerge::Automerge,
        patches: Vec<automerge::Patch>,
    ) {
        use automerge::patches::PatchAction;

        for patch in patches {
            match patch.action {
                PatchAction::PutMap { .. }
                | PatchAction::PutSeq { .. }
                | PatchAction::Increment { .. }
                | PatchAction::Insert { .. }
                | PatchAction::DeleteMap { .. }
                | PatchAction::DeleteSeq { .. } => {
                    // Mark branches dirty as we go, but DO NOT recompute yet
                    self.mark_dirty_up(patch.obj);
                }
                _ => {}
            }
        }

        // ONE single pass to recalculate only the affected tree branches
        self.recompute_dirty(document);
        self.changes = document.get_heads();
    }

    pub(super) fn purge_recursive(&mut self, id: &automerge::ObjId) {
        self.nodes.remove(id);
        self.parent.remove(id);
        self.dirty_nodes.remove(id);

        if let Some(child_ids) = self.children.remove(id) {
            for child_id in child_ids {
                self.purge_recursive(&child_id);
            }
        }
    }

    pub(super) fn update_node(
        &mut self,
        id: automerge::ObjId,
        document: &automerge::Automerge,
    ) -> super::error::Result<()> {
        let node = super::node::Node::from_doc(document, &id)?;
        self.nodes.insert(id.clone(), node);

        // Explicitly flag this node because modifying the total tasks
        // requires the parent node's progress to re-evaluate.
        self.mark_dirty_up(id);
        self.recompute_dirty(document);

        Ok(())
    }

    pub(super) fn update_up_from(
        &mut self,
        id: automerge::ObjId,
        parent_id: Option<automerge::ObjId>,
        document: &automerge::Automerge,
    ) {
        if let Some(pid) = parent_id {
            self.parent.insert(id.clone(), pid.clone());
            self.children.entry(pid).or_default().push(id.clone());
        }

        // Just flag it and fire the recomputation
        self.mark_dirty_up(id);
        self.recompute_dirty(document);
        self.changes = document.get_heads();
    }
}

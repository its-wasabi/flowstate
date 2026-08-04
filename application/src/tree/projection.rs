use automerge::{ReadDoc, patches::PatchAction};

#[derive(Debug)]
pub struct Projection {
    pub(super) root_progress: super::node::Progress,
    pub(super) nodes: rustc_hash::FxHashMap<automerge::ObjId, super::node::NodeData>,
    pub(super) parent: rustc_hash::FxHashMap<automerge::ObjId, automerge::ObjId>,
    pub(super) children: rustc_hash::FxHashMap<automerge::ObjId, Vec<automerge::ObjId>>,

    pub(super) changes: Vec<automerge::ChangeHash>,
}

impl Projection {
    pub(super) fn new(document: &automerge::Automerge) -> Self {
        let mut projection = Self {
            root_progress: super::node::Progress::default(),
            nodes: rustc_hash::FxHashMap::default(),
            parent: rustc_hash::FxHashMap::default(),
            children: rustc_hash::FxHashMap::default(),

            changes: document.get_heads(),
        };

        projection.rebuild(document);
        projection
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.parent.clear();
        self.children.clear();
        self.root_progress = super::node::Progress::default();
    }

    pub(super) fn rebuild(&mut self, document: &automerge::Automerge) {
        self.clear();
        self.build_subtree(document, &automerge::ObjId::Root);
        self.changes = document.get_heads();
    }

    pub fn update(&mut self, document: &automerge::Automerge) {
        let current_heads = document.get_heads();
        let patches = document.diff(&self.changes, &current_heads);
        if patches.is_empty() {
            return;
        }

        let mut dirty_nodes = rustc_hash::FxHashSet::default();
        let mut lists_to_sync = rustc_hash::FxHashSet::default();

        for patch in patches {
            let obj = &patch.obj;
            match &patch.action {
                PatchAction::PutMap { key: _, .. } => {
                    dirty_nodes.insert(obj.clone());
                }
                PatchAction::Increment { .. } => {
                    dirty_nodes.insert(obj.clone());
                }
                PatchAction::DeleteMap { key, .. } => {
                    if key == super::CHILDREN
                        && let Some(old_children) = self.children.remove(obj)
                    {
                        for child in old_children {
                            self.purge(&child);
                        }
                    }
                    dirty_nodes.insert(obj.clone());
                }
                PatchAction::Insert { .. }
                | PatchAction::PutSeq { .. }
                | PatchAction::DeleteSeq { .. }
                    if let Ok(mut parents) = document.parents(obj) =>
                {
                    if let Some(parent_info) = parents.next() {
                        lists_to_sync.insert((obj.clone(), parent_info.obj));
                    }
                }
                _ => {}
            }
        }

        // 2. Sync Modified Lists
        for (list_id, parent_id) in lists_to_sync {
            let mut new_child_ids = Vec::new();
            let list_len = document.length(&list_id);
            for idx in 0..list_len {
                if let Ok(Some((_, child_id))) = document.get(&list_id, idx) {
                    new_child_ids.push(child_id);
                }
            }

            let old_child_ids = self.children.get(&parent_id).cloned().unwrap_or_default();

            for old_id in &old_child_ids {
                if !new_child_ids.contains(old_id) {
                    self.purge(old_id);
                }
            }

            for new_id in &new_child_ids {
                if !self.parent.contains_key(new_id) {
                    self.parent.insert(new_id.clone(), parent_id.clone());
                    self.build_subtree(document, new_id);
                }
            }

            self.children.insert(parent_id.clone(), new_child_ids);
            dirty_nodes.insert(parent_id);
        }

        // 3. Bubble up progress recalculations iteratively
        let mut queue = std::collections::VecDeque::from_iter(dirty_nodes);
        let mut queued: rustc_hash::FxHashSet<automerge::ObjId> = queue.iter().cloned().collect();

        while let Some(id) = queue.pop_front() {
            queued.remove(&id);

            // Fetch node data (with a fallback for Root if it has no properties yet)
            let doc_node_opt = super::node::NodeData::from_doc(document, &id)
                .ok()
                .or_else(|| {
                    if id == automerge::ObjId::Root {
                        let mut fallback = super::node::NodeData::default();
                        fallback.name = "Project Root".into();
                        Some(fallback)
                    } else {
                        None
                    }
                });

            if let Some(mut doc_node) = doc_node_opt {
                if let Some(existing) = self.nodes.get(&id) {
                    doc_node.progress = existing.progress;
                }
                self.nodes.insert(id.clone(), doc_node);
            }

            let new_progress = self.calculate_progress(document, &id);

            let progress_changed = if id == automerge::ObjId::Root {
                let changed = self.root_progress != new_progress;
                self.root_progress = new_progress;
                // Keep the self.nodes cache in sync for the UI
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.progress = new_progress;
                }
                changed
            } else if let Some(node) = self.nodes.get_mut(&id) {
                let changed = node.progress != new_progress;
                node.progress = new_progress;
                changed
            } else {
                false
            };

            if progress_changed {
                if let Some(parent_id) = self.parent.get(&id) {
                    if !queued.contains(parent_id) {
                        queue.push_back(parent_id.clone());
                        queued.insert(parent_id.clone());
                    }
                }
            }
        }

        // 4. Record new sync point
        self.changes = document.get_heads();
    }

    fn categorize_patches(
        &self,
        document: &automerge::Automerge,
        patches: &[automerge::patches::Patch],
    ) -> (
        rustc_hash::FxHashSet<automerge::ObjId>,
        rustc_hash::FxHashSet<automerge::ObjId>,
        rustc_hash::FxHashSet<automerge::ObjId>,
    ) {
        let mut structural = rustc_hash::FxHashSet::default();
        let mut metadata = rustc_hash::FxHashSet::default();
        let mut progress = rustc_hash::FxHashSet::default();

        for patch in patches {
            let automerge::patches::Patch { obj, action, .. } = patch.clone();

            match action {
                PatchAction::PutMap { key, .. } => match key.as_str() {
                    super::NODE_NAME | super::NODE_DESC => {
                        metadata.insert(obj);
                    }
                    super::NODE_TASK_COMPLETED | super::NODE_TASK_TOTAL => {
                        progress.insert(obj);
                    }
                    super::CHILDREN => {
                        structural.insert(obj);
                    }
                    _ => {
                        progress.insert(obj);
                    }
                },

                PatchAction::DeleteMap { key, .. } => {
                    if key == super::CHILDREN {
                        structural.insert(obj);
                    }
                }
                PatchAction::Increment { .. } => {
                    progress.insert(obj);
                }

                PatchAction::Insert { .. }
                | PatchAction::PutSeq { .. }
                | PatchAction::DeleteSeq { .. } => {
                    if let Ok(mut parents) = document.parents(&obj) {
                        if let Some(parent_info) = parents.next() {
                            structural.insert(parent_info.obj);
                        }
                    }
                }
                _ => {}
            }
        }

        (structural, metadata, progress)
    }

    fn build_subtree(&mut self, document: &automerge::Automerge, start_id: &automerge::ObjId) {
        let mut stack = vec![start_id.clone()];
        let mut visit_order = Vec::new();

        while let Some(id) = stack.pop() {
            visit_order.push(id.clone());
            let mut child_ids = Vec::new();

            if let Ok(Some((_, list_id))) = document.get(&id, super::CHILDREN) {
                let list_len = document.length(&list_id);
                for idx in 0..list_len {
                    if let Ok(Some((_, child_id))) = document.get(&list_id, idx) {
                        child_ids.push(child_id.clone());
                        self.parent.insert(child_id.clone(), id.clone());
                        stack.push(child_id);
                    }
                }
            }
            self.children.insert(id.clone(), child_ids);
        }

        for id in visit_order.into_iter().rev() {
            let progress = self.calculate_progress(document, &id);

            if id == automerge::ObjId::Root {
                self.root_progress = progress;
                // Root requires fallback NodeData initially
                let mut node_data =
                    super::node::NodeData::from_doc(document, &id).unwrap_or_else(|_| {
                        let mut fallback = super::node::NodeData::default();
                        fallback.name = "Project Root".into();
                        fallback
                    });
                node_data.progress = progress;
                self.nodes.insert(id, node_data);
            } else if let Ok(mut node_data) = super::node::NodeData::from_doc(document, &id) {
                node_data.progress = progress;
                self.nodes.insert(id, node_data);
            }
        }
    }

    fn calculate_progress(
        &self,
        document: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::node::Progress {
        let child_ids = self.children.get(id).cloned().unwrap_or_default();

        if child_ids.is_empty() {
            // LEAF NODE: Reads exactly what the user set for this node.
            if id == &automerge::ObjId::Root {
                return super::node::Progress::default();
            }
            if let Ok(node) = super::node::NodeData::from_doc(document, id) {
                return node.progress;
            }
            super::node::Progress::default()
        } else {
            // INNER NODE: Calculated ONLY from direct children.
            // We use a high-precision integer scale to completely avoid floating-point loss.
            // 10,000 means 100.00%
            const SCALE: u64 = 10_000;

            // The max potential progress is treating each child as 1 whole unit
            let total = child_ids.len() as u64 * SCALE;
            let mut completed: u64 = 0;

            for cid in &child_ids {
                if let Some(child_node) = self.nodes.get(cid) {
                    let cp = &child_node.progress;
                    if cp.total > 0 {
                        // Integer math guarantees exact fractions.
                        // If child is maxed out (completed == total), this adds exactly SCALE.
                        completed += (cp.completed as u64 * SCALE) / cp.total as u64;
                    }
                }
            }

            // Since SCALE is 10_000, total fits safely in u32 for up to ~429,000 children.
            super::node::Progress::new(completed as u32, total as u32)
        }
    }

    /// Iteratively removes a node and all of its nested children from the projection cache.
    pub(super) fn purge(&mut self, start_id: &automerge::ObjId) {
        let mut stack = vec![start_id.clone()];

        while let Some(id) = stack.pop() {
            self.nodes.remove(&id);
            self.parent.remove(&id);
            if let Some(child_ids) = self.children.remove(&id) {
                stack.extend(child_ids);
            }
        }
    }
}

impl Projection {
    pub(super) fn get_progress(&self, id: &automerge::ObjId) -> Option<super::node::Progress> {
        if id == &automerge::ObjId::Root {
            Some(self.root_progress)
        } else {
            self.nodes.get(id).map(|node| node.progress)
        }
    }

    pub(super) fn get_node(&self, id: &automerge::ObjId) -> Option<super::node::NodeData> {
        self.nodes.get(id).cloned()
    }

    pub(super) fn get_parent(&self, id: &automerge::ObjId) -> Option<&automerge::ObjId> {
        self.parent.get(id)
    }
}

// TODO: Remove and use automerge::ObjType::Text - then edit with splice_text()
impl Projection {
    pub fn update_node_name(&mut self, id: &automerge::ObjId, name: String) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.name = name;
        }
    }

    pub fn update_node_desc(&mut self, id: &automerge::ObjId, desc: String) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.desc = desc;
        }
    }

    pub fn update_node_total(&mut self, id: &automerge::ObjId, total: u32) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.progress.total = total;
            if node.progress.completed < node.progress.total {
                node.progress.completed = total;
            }
        }
    }
}

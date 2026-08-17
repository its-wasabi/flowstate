use automerge::ReadDoc;

#[derive(Debug)]
pub struct Projection {
    last_heads: Vec<automerge::ChangeHash>,
    nodes_map_obj_id: automerge::ObjId,

    pub(super) node: rustc_hash::FxHashMap<uuid::Uuid, super::node::NodeData>,
    pub(super) progress: rustc_hash::FxHashMap<uuid::Uuid, super::progress::Progress>,

    pub(super) nodes_root: Vec<uuid::Uuid>,
    pub(super) parent: rustc_hash::FxHashMap<uuid::Uuid, uuid::Uuid>,
    pub(super) children: rustc_hash::FxHashMap<uuid::Uuid, Vec<uuid::Uuid>>,
}

impl Projection {
    pub(super) fn new(document: &automerge::Automerge) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, nodes_root_id) = document
            .get(automerge::ObjId::Root, super::NODES)?
            .ok_or(super::super::error::TreeError::MissingRoot)?;

        let mut projection = Self {
            last_heads: Vec::new(),
            nodes_map_obj_id: nodes_root_id,

            node: rustc_hash::FxHashMap::default(),
            progress: rustc_hash::FxHashMap::default(),

            nodes_root: Vec::new(),
            parent: rustc_hash::FxHashMap::default(),
            children: rustc_hash::FxHashMap::default(),
        };

        let nodes_map_range = document.map_range(&projection.nodes_map_obj_id, ..);
        for range_item in nodes_map_range {
            let uuid_str = range_item.key.as_ref();
            let uuid = uuid::Uuid::parse_str(uuid_str)?;
            let (_, node_id) = document
                .get(&projection.nodes_map_obj_id, uuid_str)?
                .ok_or(crate::store::error::TreeError::MissingProperty)?;
            let node_data = super::node::NodeData::from_id(document, &node_id)?;

            projection.progress.insert(uuid, node_data.progress);
            projection.node.insert(uuid, node_data);

            if let Some((parent_val, _)) = document.get(&node_id, super::NODE_PARENT)? {
                let parent_uuid_str = parent_val
                    .to_str()
                    .ok_or(crate::store::error::TreeError::InvalidValueType)?;
                let parent_uuid = uuid::Uuid::parse_str(parent_uuid_str)?;

                projection.parent.insert(uuid, parent_uuid);
                projection
                    .children
                    .entry(parent_uuid)
                    .or_default()
                    .push(uuid);
            } else {
                projection.nodes_root.push(uuid);
            }
        }

        projection.rebuild_progress();

        projection.last_heads = document.get_heads();

        Ok(projection)
    }

    pub(super) fn rebuild_progress(&mut self) {
        let mut queue = std::collections::VecDeque::with_capacity(self.node.len());
        let mut pending = rustc_hash::FxHashMap::default();

        for &node_uuid in self.node.keys() {
            let child_count = self.children.get(&node_uuid).map_or(0, Vec::len);
            if child_count == 0 {
                queue.push_back(node_uuid);
            } else {
                pending.insert(node_uuid, child_count);
            }
        }

        while let Some(node_uuid) = queue.pop_front() {
            if let Some(kids) = self.children.get(&node_uuid)
                && !kids.is_empty()
            {
                let child_progresses = kids.iter().filter_map(|child| self.progress.get(child));
                self.progress.insert(
                    node_uuid,
                    super::progress::Progress::from_many(child_progresses),
                );
            }

            if let Some(&parent_uuid) = self.parent.get(&node_uuid)
                && let Some(count) = pending.get_mut(&parent_uuid)
            {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(parent_uuid);
                }
            }
        }
    }
}

// TODO: Remove PatchEffect entirely cause RebuildAll variant is unused anyway
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PatchEffect {
    RebuildAll,
    Track(ChangeType),
}

// TODO: You should split Metadata into Name and Desc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ChangeType {
    NodeAdded(uuid::Uuid),
    NodeRemoved(uuid::Uuid),
    HierarchyChanged(uuid::Uuid),
    MetadataChanged(uuid::Uuid),
    ProgressChanged(uuid::Uuid),
}

impl Projection {
    pub fn update(
        &mut self,
        document: &automerge::Automerge,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let current_heads = document.get_heads();
        if self.last_heads == current_heads {
            return Ok(());
        }

        let patches = document.diff_obj(
            &self.nodes_map_obj_id,
            &self.last_heads,
            &current_heads,
            true,
        )?;

        let effects = patches
            .into_iter()
            .map(|patch| self.classify_patch(&patch))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<rustc_hash::FxHashSet<_>>();

        for effect in effects {
            if let Err(error) = self.apply_effect(document, effect) {
                // TODO: Think how to handle errors in the way to not break the projection and be able
                // to handle that and maybe fix or work with small error
                eprintln!("(REBUILDING) Error: {error}");
                *self = Self::new(document)?;
            }
        }

        self.last_heads = document.get_heads();

        Ok(())
    }

    fn classify_patch(
        &self,
        patch: &automerge::patches::Patch,
    ) -> Result<Option<PatchEffect>, Box<dyn std::error::Error>> {
        let extract_uuid = || -> Option<uuid::Uuid> {
            patch.path.get(1).and_then(|(_, prop)| {
                if let automerge::Prop::Map(uuid_str) = prop {
                    uuid::Uuid::parse_str(uuid_str).ok()
                } else {
                    None
                }
            })
        };

        match (&patch.obj, &patch.action) {
            (obj_id, automerge::PatchAction::PutMap { key, .. })
                if *obj_id == self.nodes_map_obj_id =>
            {
                Ok(Some(PatchEffect::Track(ChangeType::NodeAdded(
                    uuid::Uuid::parse_str(&key)?,
                ))))
            }

            (obj_id, automerge::PatchAction::DeleteMap { key, .. })
                if *obj_id == self.nodes_map_obj_id =>
            {
                Ok(Some(PatchEffect::Track(ChangeType::NodeRemoved(
                    uuid::Uuid::parse_str(&key)?,
                ))))
            }

            (_, automerge::PatchAction::PutMap { key, .. })
                if key == super::NODE_TASK_COMPLETED
                    || key == super::NODE_TASK_TOTAL
                    || key == super::NODE_NAME
                    || key == super::NODE_DESC
                    || key == super::NODE_PARENT =>
            {
                Ok(None)
            }

            (
                _,
                automerge::PatchAction::SpliceText { .. }
                | automerge::PatchAction::DeleteSeq { .. },
            ) => {
                if let Some(uuid) = extract_uuid() {
                    return Ok(Some(PatchEffect::Track(ChangeType::MetadataChanged(uuid))));
                }

                Ok(None)
            }

            (_, automerge::PatchAction::Increment { prop, .. })
                if prop.as_str() == Some(super::NODE_TASK_COMPLETED)
                    || prop.as_str() == Some(super::NODE_TASK_TOTAL) =>
            {
                let (_, property) = patch.path.get(1).unwrap();
                let automerge::Prop::Map(uuid_str) = property else {
                    panic!()
                };
                let uuid = uuid::Uuid::parse_str(uuid_str)?;

                Ok(Some(PatchEffect::Track(ChangeType::ProgressChanged(uuid))))
            }

            (x, y) => unimplemented!("{x:#?} - {y:#?}"),
        }
    }

    fn apply_effect(
        &mut self,
        document: &automerge::Automerge,
        effect: PatchEffect,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match effect {
            PatchEffect::RebuildAll => *self = Self::new(document)?,
            PatchEffect::Track(change_type) => self.apply_change(document, change_type)?,
        }

        Ok(())
    }

    fn apply_change(
        &mut self,
        document: &automerge::Automerge,
        change_type: ChangeType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match change_type {
            ChangeType::NodeAdded(uuid) => self.handle_node_added(document, uuid)?,
            ChangeType::NodeRemoved(uuid) => self.remove_node(uuid),
            ChangeType::HierarchyChanged(uuid) => self.handle_hierarchy_changed(document, uuid)?,
            ChangeType::MetadataChanged(uuid) => self.handle_metadata_changed(document, uuid)?,
            ChangeType::ProgressChanged(uuid) => self.handle_progress_changed(document, uuid)?,
        }

        Ok(())
    }

    /// Walks up from a specific node to the root, recalculating progress along the way.
    /// FIX: Added missing method that was causing E0599 errors.
    pub(super) fn propagate_progress_up(&mut self, mut current: uuid::Uuid) {
        loop {
            if let Some(kids) = self.children.get(&current) {
                if !kids.is_empty() {
                    let child_progresses = kids.iter().filter_map(|c| self.progress.get(c));
                    let calculated = super::progress::Progress::from_many(child_progresses);
                    self.progress.insert(current, calculated);

                    // ✓ FIX: Also update NodeData.progress
                    if let Some(node_data) = self.node.get_mut(&current) {
                        node_data.progress = calculated;
                    }
                }
            }

            match self.parent.get(&current).copied() {
                Some(p) => current = p,
                None => break,
            }
        }
    }

    /// Recalculates progress for multiple nodes simultaneously using topological sort.
    /// FIX: Added missing method that was causing E0599 errors.
    pub(super) fn propagate_progress_up_many<I>(&mut self, starts: I)
    where
        I: IntoIterator<Item = uuid::Uuid>,
    {
        let mut affected: rustc_hash::FxHashSet<uuid::Uuid> = rustc_hash::FxHashSet::default();

        for start in starts {
            if !self.node.contains_key(&start) {
                continue;
            }
            let mut current = start;
            loop {
                if !affected.insert(current) {
                    break;
                }
                match self.parent.get(&current).copied() {
                    Some(p) => current = p,
                    None => break,
                }
            }
        }

        if affected.is_empty() {
            return;
        }

        let mut pending_affected_children: rustc_hash::FxHashMap<uuid::Uuid, usize> =
            rustc_hash::FxHashMap::default();
        let mut ready: std::collections::VecDeque<uuid::Uuid> =
            std::collections::VecDeque::with_capacity(affected.len());

        for &uuid in &affected {
            let affected_child_count = self.children.get(&uuid).map_or(0, |kids| {
                kids.iter().filter(|c| affected.contains(c)).count()
            });
            if affected_child_count == 0 {
                ready.push_back(uuid);
            } else {
                pending_affected_children.insert(uuid, affected_child_count);
            }
        }

        while let Some(uuid) = ready.pop_front() {
            if let Some(kids) = self.children.get(&uuid) {
                if !kids.is_empty() {
                    let child_progresses = kids.iter().filter_map(|c| self.progress.get(c));
                    let calculated = super::progress::Progress::from_many(child_progresses);
                    self.progress.insert(uuid, calculated);

                    // ✓ FIX: Also update NodeData.progress
                    if let Some(node_data) = self.node.get_mut(&uuid) {
                        node_data.progress = calculated;
                    }
                }
            }

            let Some(parent_uuid) = self.parent.get(&uuid).copied() else {
                continue;
            };
            if !affected.contains(&parent_uuid) {
                continue;
            }

            if let Some(count) = pending_affected_children.get_mut(&parent_uuid) {
                *count -= 1;
                if *count == 0 {
                    ready.push_back(parent_uuid);
                }
            }
        }
    }

    fn handle_node_added(
        &mut self,
        document: &automerge::Automerge,
        uuid: uuid::Uuid,
    ) -> crate::store::error::Result<()> {
        let (_, node_id) = document
            .get(&self.nodes_map_obj_id, uuid.to_string())?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;

        let node_data = super::node::NodeData::from_id(document, &node_id)?;
        self.progress.insert(uuid, node_data.progress);
        self.node.insert(uuid, node_data);

        let mut dirty_nodes = vec![uuid];

        if let Some((parent_val, _)) = document.get(&node_id, super::NODE_PARENT)? {
            let parent_uuid_str = parent_val
                .to_str()
                .ok_or(crate::store::error::TreeError::InvalidValueType)?;
            let parent_uuid = uuid::Uuid::parse_str(parent_uuid_str)
                .map_err(|_| crate::store::error::TreeError::InvalidValueType)?;

            self.parent.insert(uuid, parent_uuid);
            self.children.entry(parent_uuid).or_default().push(uuid);
            dirty_nodes.push(parent_uuid); // Parent needs progress recalculation
        } else {
            self.nodes_root.push(uuid);
        }

        self.propagate_progress_up_many(dirty_nodes);
        Ok(())
    }

    fn remove_node(&mut self, uuid: uuid::Uuid) {
        self.node.remove(&uuid);
        self.progress.remove(&uuid);
        self.children.remove(&uuid);

        if let Some(&parent_uuid) = self.parent.get(&uuid) {
            if let Some(siblings) = self.children.get_mut(&parent_uuid) {
                siblings.retain(|&sibling_uuid| sibling_uuid != uuid);
            }

            self.propagate_progress_up(parent_uuid);
        } else {
            self.nodes_root.retain(|&root_uuid| root_uuid != uuid);
        }
    }

    fn handle_hierarchy_changed(
        &mut self,
        document: &automerge::Automerge,
        uuid: uuid::Uuid,
    ) -> crate::store::error::Result<()> {
        let old_parent = self.parent.remove(&uuid);
        let mut was_root = false;
        let mut dirty_nodes = Vec::new();

        // 1. Detach from old location
        if let Some(old_p) = old_parent {
            if let Some(siblings) = self.children.get_mut(&old_p) {
                siblings.retain(|&id| id != uuid);
            }
            dirty_nodes.push(old_p);
        } else {
            self.nodes_root.retain(|&id| id != uuid);
            was_root = true;
        }

        // 2. Attach to new location
        let (_, node_id) = document
            .get(&self.nodes_map_obj_id, uuid.to_string())?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;

        if let Some((parent_val, _)) = document.get(&node_id, super::NODE_PARENT)? {
            let parent_uuid_str = parent_val
                .to_str()
                .ok_or(crate::store::error::TreeError::InvalidValueType)?;
            let parent_uuid = uuid::Uuid::parse_str(parent_uuid_str)
                .map_err(|_| crate::store::error::TreeError::InvalidValueType)?;

            self.parent.insert(uuid, parent_uuid);
            self.children.entry(parent_uuid).or_default().push(uuid);
            dirty_nodes.push(parent_uuid);
        } else if !was_root {
            self.nodes_root.push(uuid); // It became a root
        }

        self.propagate_progress_up_many(dirty_nodes);
        Ok(())
    }

    fn handle_metadata_changed(
        &mut self,
        document: &automerge::Automerge,
        uuid: uuid::Uuid,
    ) -> crate::store::error::Result<()> {
        let (_, node_id) = document
            .get(&self.nodes_map_obj_id, uuid.to_string())?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;

        let node_data = super::node::NodeData::from_id(document, &node_id)?;

        if let Some(existing) = self.node.get_mut(&uuid) {
            existing.name = node_data.name;
            existing.desc = node_data.desc;
        }
        Ok(())
    }

    fn handle_progress_changed(
        &mut self,
        document: &automerge::Automerge,
        uuid: uuid::Uuid,
    ) -> crate::store::error::Result<()> {
        let (_, node_id) = document
            .get(&self.nodes_map_obj_id, uuid.to_string())?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;

        // Read new progress directly from Automerge
        let completed = document
            .get(&node_id, super::NODE_TASK_COMPLETED)?
            .and_then(|(v, _)| v.to_u64())
            .unwrap_or(0) as u32;

        let total = document
            .get(&node_id, super::NODE_TASK_TOTAL)?
            .and_then(|(v, _)| v.to_u64())
            .unwrap_or(0) as u32;

        let new_progress = super::progress::Progress::from_values(total, completed);
        self.progress.insert(uuid, new_progress);

        if let Some(node_data) = self.node.get_mut(&uuid) {
            node_data.progress = new_progress;
        }

        // Walk up the tree recalculating ancestors
        self.propagate_progress_up(uuid);
        Ok(())
    }
}

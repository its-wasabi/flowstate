pub mod analytics;
pub mod error;
pub mod node;
mod projection;

use automerge::{ReadDoc, transaction::Transactable};

/// List of all children of that object
pub const CHILDREN: &str = "l";
/// Name of that node
pub const NODE_NAME: &str = "n";
/// Description of that node
pub const NODE_DESC: &str = "d";
/// Total number of tasks for that node
pub const NODE_TASK_TOTAL: &str = "t";
/// Number of completed tasks for that node
pub const NODE_TASK_COMPLETED: &str = "c";

#[derive(Debug)]
pub struct Tree {
    pub document: automerge::Automerge,

    pub projection: projection::Projection,
    pub analytics: analytics::Analytics,
}

pub enum View {
    RootList {
        children: Vec<ChildEntry>,
    },

    InnerList {
        current_id: automerge::ObjId,
        current_node: node::NodeData,
        children: Vec<ChildEntry>,
    },

    Leaf {
        id: automerge::ObjId,
        node: node::NodeData,
    },
}

pub struct ChildEntry {
    pub id: automerge::ObjId,
    pub node: node::NodeData,
    pub is_leaf: bool,
}

impl Tree {
    pub fn new() -> error::Result<Self> {
        let mut document = automerge::Automerge::new();
        let mut tx = document.transaction();
        tx.put_object(automerge::ObjId::Root, CHILDREN, automerge::ObjType::List)?;
        tx.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        );

        let projection = projection::Projection::new(&document);
        let analytics = analytics::Analytics::new(&document);

        Ok(Self {
            document,

            projection,
            analytics,
        })
    }

    pub fn sync(&mut self) -> error::Result<()> {
        // TODO: Think about bringing up there common logic for projection and analytics
        self.projection.update(&self.document);
        self.analytics.update(&self.document);

        // TODO: Send over net

        Ok(())
    }

    // TODO: Move that into peer module
    pub(super) fn generate_sync_message(
        &self,
        local_sync_state: &mut automerge::sync::State,
    ) -> Option<Vec<u8>> {
        use automerge::sync::SyncDoc;
        self.document
            .generate_sync_message(local_sync_state)
            .map(automerge::sync::Message::encode)
    }

    // TODO: Move that into peer module
    pub(super) fn receive_sync_message(
        &mut self,
        local_sync_state: &mut automerge::sync::State,
        bytes: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use automerge::sync::SyncDoc;
        let msg = automerge::sync::Message::decode(bytes)?;
        self.document.receive_sync_message(local_sync_state, msg)?;
        Ok(())
    }
}

impl Tree {
    pub fn view(&self, id: &automerge::ObjId) -> error::Result<View> {
        if *id == automerge::ObjId::Root {
            return Ok(View::RootList {
                children: self.get_children(id),
            });
        }

        let current_node = self
            .projection
            .get_node(id)
            .ok_or(error::TreeError::MissingProperty)?;

        if self.has_children(id) {
            let children = self.get_children(id);
            Ok(View::InnerList {
                current_id: id.clone(),
                current_node,
                children,
            })
        } else {
            Ok(View::Leaf {
                id: id.clone(),
                node: current_node,
            })
        }
    }

    pub fn get_progress(&self, id: &automerge::ObjId) -> error::Result<node::Progress> {
        self.projection
            .get_progress(id)
            .ok_or(error::TreeError::MissingProperty)
    }

    pub fn get_parent(&self, id: &automerge::ObjId) -> error::Result<&automerge::ObjId> {
        self.projection
            .get_parent(id)
            .ok_or(error::TreeError::MissingProperty)
    }
}

impl Tree {
    fn get_children(&self, id: &automerge::ObjId) -> Vec<ChildEntry> {
        let child_ids = self
            .projection
            .children
            .get(id)
            .cloned()
            .unwrap_or_default();

        let mut childrens = Vec::with_capacity(child_ids.len());

        for child_id in child_ids {
            if let Some(node) = self.projection.nodes.get(&child_id).cloned() {
                let has_children = self.has_children(&child_id);
                childrens.push(ChildEntry {
                    id: child_id,
                    node,
                    is_leaf: !has_children,
                });
            }
        }

        childrens
    }

    fn has_children(&self, id: &automerge::ObjId) -> bool {
        self.projection
            .children
            .get(id)
            .is_some_and(|children| !children.is_empty())
    }
}

impl Tree {
    pub fn append_child(
        &mut self,
        parent_id: &automerge::ObjId,
        node: &node::NodeData,
    ) -> error::Result<automerge::ObjId> {
        let mut tx = self.document.transaction();
        let list_id = match tx.get(parent_id, CHILDREN)? {
            Some((_, list_id)) => list_id,
            None => tx.put_object(parent_id, CHILDREN, automerge::ObjType::List)?,
        };
        let list_len = tx.length(&list_id);
        let new_node_id = tx.insert_object(&list_id, list_len, automerge::ObjType::Map)?;
        node.apply_data(&mut tx, &new_node_id)?;
        tx.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        );
        self.sync()?;

        Ok(new_node_id)
    }

    pub fn delete(&mut self, id: &automerge::ObjId) -> error::Result<()> {
        let mut parents = self.document.parents(id)?;
        let parent_list = parents.next().ok_or(error::TreeError::MissingProperty)?;
        let parent_node = parents.next().ok_or(error::TreeError::MissingRoot)?;
        let _parent_id = parent_node.obj;

        let mut tx = self.document.transaction();
        tx.delete(&parent_list.obj, parent_list.prop)?;
        tx.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        );
        self.sync()?;

        Ok(())
    }

    pub fn change_node_name(&mut self, id: &automerge::ObjId, name: String) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(id, NODE_NAME, name)?;
        tx.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        );
        self.sync()?;

        Ok(())
    }

    pub fn change_node_desc(&mut self, id: &automerge::ObjId, desc: String) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(id, NODE_DESC, desc)?;
        tx.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        );
        self.sync()?;

        Ok(())
    }

    pub fn change_node_total(&mut self, id: &automerge::ObjId, total: u32) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(id, NODE_TASK_TOTAL, total)?;
        tx.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        );
        self.sync()?;

        Ok(())
    }

    pub fn change_node_completed(
        &mut self,
        id: &automerge::ObjId,
        delta: i64,
    ) -> error::Result<()> {
        use automerge::{ReadDoc, ScalarValue};

        let (total_val, _) = self
            .document
            .get(id, NODE_TASK_TOTAL)?
            .ok_or(error::TreeError::InvalidNodeType)?;
        let total = match total_val.into_scalar() {
            Ok(ScalarValue::Uint(u)) => {
                u32::try_from(u).map_err(|_| error::TreeError::InvalidValue)?
            }
            Ok(ScalarValue::Int(i)) => {
                u32::try_from(i).map_err(|_| error::TreeError::InvalidValue)?
            }
            _ => return Err(error::TreeError::InvalidNodeType),
        };

        let (completed_val, _) = self
            .document
            .get(id, NODE_TASK_COMPLETED)?
            .ok_or(error::TreeError::InvalidNodeType)?;
        let current_completed = match completed_val.into_scalar() {
            Ok(ScalarValue::Counter(counter)) => i64::from(counter),
            _ => return Err(error::TreeError::InvalidNodeType),
        };

        let safe_delta = {
            let safe_base = current_completed.clamp(0, i64::from(total));
            let safe_target = (safe_base + delta).clamp(0, i64::from(total));
            safe_target - current_completed
        };

        if safe_delta != 0 {
            let mut tx = self.document.transaction();
            tx.increment(id, NODE_TASK_COMPLETED, safe_delta)?;
            tx.commit_with(
                automerge::transaction::CommitOptions::default()
                    .with_time(chrono::Utc::now().timestamp_millis()),
            );
            self.sync()?;
        }

        Ok(())
    }
}

impl Default for Tree {
    fn default() -> Self {
        #[allow(clippy::expect_used)]
        Self::new().expect("failed to initialize root CHILDREN list on a fresh document")
    }
}

impl crate::io::storage::FromBytes for Tree {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let document = automerge::Automerge::load(bytes)?;
        let projection = projection::Projection::new(&document);
        let analytics = analytics::Analytics::new(&document);

        Ok(Self {
            document,

            projection,
            analytics,
        })
    }
}

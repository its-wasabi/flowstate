mod analytics;
pub mod node;
pub mod progress;
mod projection;

use super::ext::TransactionExt;
use automerge::{ReadDoc, transaction::Transactable};

pub use node::NodeData;
pub use progress::Progress;

/// List of all children of that object
pub const NODES: &str = "nodes";
/// Parent of the node
pub const NODE_PARENT: &str = "p";
/// Name of the node
pub const NODE_NAME: &str = "n";
/// Description of the node
pub const NODE_DESC: &str = "d";
/// Total number of tasks of node
pub const NODE_TASK_TOTAL: &str = "t";
/// Number of completed tasks of node
pub const NODE_TASK_COMPLETED: &str = "c";

#[derive(Debug, Clone)]
pub enum Command {
    AddNode {
        parent_uuid: Option<uuid::Uuid>,
        node_data: node::NodeData,
    },

    DelNode {
        uuid: uuid::Uuid,
    },

    MoveNode {
        uuid: uuid::Uuid,
        to_parent: uuid::Uuid,
    },

    SpliceNodeName {
        uuid: uuid::Uuid,
        index: usize,
        delete: usize,
        insert: String,
    },

    SpliceNodeDesc {
        uuid: uuid::Uuid,
        index: usize,
        delete: usize,
        insert: String,
    },

    UpdateNodeCompleted {
        uuid: uuid::Uuid,
        by: i64,
    },

    UpdateNodeTotal {
        uuid: uuid::Uuid,
        by: i64,
    },
}

#[derive(Debug)]
pub struct Tree {
    pub(super) projection: projection::Projection,
    pub(super) analytics: analytics::Analytics,
}

impl Tree {
    pub(super) fn new(
        document: &mut automerge::Automerge,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut transaction = document.transaction();

        // TODO: map that error into specific TreeInit error
        if transaction.get(automerge::ObjId::Root, NODES)?.is_none() {
            transaction.put_object(automerge::ObjId::Root, NODES, automerge::ObjType::Map)?;
        }

        transaction.commit_with_time();

        let projection = projection::Projection::new(&document)?;

        let analytics = analytics::Analytics::new(&document);

        Ok(Self {
            projection,
            analytics,
        })
    }

    pub(super) fn dispatch(
        &mut self,
        document: &mut automerge::Automerge,
        command: Command,
    ) -> super::error::Result<()> {
        match command {
            Command::AddNode {
                parent_uuid,
                node_data,
            } => Self::add_node(document, parent_uuid, &node_data),

            Command::DelNode { uuid } => self.remove_node(document, &uuid),

            Command::MoveNode { uuid, to_parent } => todo!(),

            Command::SpliceNodeName {
                uuid,
                index,
                delete,
                insert,
            } => todo!(),
            Command::SpliceNodeDesc {
                uuid,
                index,
                delete,
                insert,
            } => todo!(),

            Command::UpdateNodeCompleted { uuid, by } => Self::update_completed(document, uuid, by),

            Command::UpdateNodeTotal { uuid, by } => Self::update_total(document, uuid, by),
        }?;

        self.update(document)
    }

    fn update(&mut self, document: &automerge::Automerge) -> super::error::Result<()> {
        self.projection.update(document).unwrap();
        self.analytics.update(document)?;

        Ok(())
    }

    pub const fn get_root(&self) -> &[uuid::Uuid] {
        self.projection.nodes_root.as_slice()
    }

    pub fn get_parent_uuid(&self, uuid: &uuid::Uuid) -> super::error::Result<&uuid::Uuid> {
        self.projection
            .parent
            .get(uuid)
            .ok_or(super::error::TreeError::MissingProperty)
    }

    pub fn get_children_uuids(&self, uuid: &uuid::Uuid) -> super::error::Result<&[uuid::Uuid]> {
        self.projection
            .children
            .get(uuid)
            .map(Vec::as_slice)
            .ok_or(super::error::TreeError::MissingProperty)
    }

    pub fn get_node(&self, uuid: &uuid::Uuid) -> super::error::Result<&node::NodeData> {
        self.projection
            .node
            .get(uuid)
            .ok_or(super::error::TreeError::MissingProperty)
    }

    pub fn get_progress(&self, uuid: &uuid::Uuid) -> super::error::Result<&progress::Progress> {
        self.projection
            .progress
            .get(uuid)
            .ok_or(super::error::TreeError::MissingProperty)
    }

    pub fn has_children(&self, uuid: &uuid::Uuid) -> bool {
        self.projection
            .children
            .get(uuid)
            .is_some_and(|children| !children.is_empty())
    }

    fn add_node(
        document: &mut automerge::Automerge,
        parent_uuid: Option<uuid::Uuid>,
        node_data: &node::NodeData,
    ) -> super::error::Result<()> {
        let mut transaction = document.transaction();
        let (_, nodes_map_id) = transaction
            .get(automerge::ObjId::Root, NODES)?
            .ok_or(super::error::TreeError::MissingProperty)?;

        let new_node_uuid = uuid::Uuid::now_v7().to_string();
        let new_node_id =
            transaction.put_object(nodes_map_id, new_node_uuid, automerge::ObjType::Map)?;

        if let Some(parent) = parent_uuid {
            transaction.put(&new_node_id, NODE_PARENT, parent.to_string())?;
        }

        // TODO: handle that manually here
        node_data.apply_data(&mut transaction, &new_node_id)?;

        transaction.commit_with_time();

        Ok(())
    }

    fn remove_node(
        &self,
        document: &mut automerge::Automerge,
        uuid: &uuid::Uuid,
    ) -> super::error::Result<()> {
        if let Some(child_uuids) = self.projection.children.get(uuid) {
            for child_uuid in child_uuids {
                self.remove_node(document, child_uuid);
            }
        };

        let mut transaction = document.transaction();

        let (_, nodes_map_id) = transaction
            .get(automerge::ObjId::Root, NODES)?
            .ok_or(super::error::TreeError::MissingRoot)?;

        transaction.delete(nodes_map_id, uuid.to_string())?;

        transaction.commit_with_time();

        Ok(())
    }

    fn move_node(document: &automerge::Automerge, uuid: uuid::Uuid, parent: uuid::Uuid) {
        todo!()
    }

    pub fn splice_node_name(
        document: &automerge::Automerge,
        uuid: uuid::Uuid,
        index: usize,
        delete: usize,
        insert: &str,
    ) {
        splice_text(document);
    }

    fn splice_node_desc(
        document: &automerge::Automerge,
        uuid: uuid::Uuid,
        index: usize,
        delete: usize,
        insert: &str,
    ) {
        splice_text(document);
    }

    fn update_completed(
        document: &mut automerge::Automerge,
        uuid: uuid::Uuid,
        by: i64,
    ) -> super::error::Result<()> {
        let mut transaction = document.transaction();
        let (_, nodes_map_id) = transaction
            .get(automerge::ObjId::Root, NODES)?
            .ok_or(super::error::TreeError::MissingRoot)?;

        let (_, node_id) = transaction
            .get(&nodes_map_id, uuid.to_string())?
            .ok_or(super::error::TreeError::MissingProperty)?;

        let (total_val, _) = transaction
            .get(&node_id, NODE_TASK_TOTAL)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let total = progress::Total::from(
            total_val
                .to_i64()
                .ok_or(super::error::TreeError::InvalidValueType)?,
        );

        let (completed_val, _) = transaction
            .get(&node_id, NODE_TASK_COMPLETED)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let current_completed = progress::Completed::from_i64(
            completed_val
                .to_i64()
                .ok_or(super::error::TreeError::InvalidValueType)?,
            total,
        );

        let updated = current_completed.change_by(by, total);
        let delta = updated.value() as i64 - current_completed.value() as i64;

        if delta != 0 {
            transaction.increment(&node_id, NODE_TASK_COMPLETED, delta)?;
        }

        transaction.commit_with_time();
        Ok(())
    }

    fn update_total(
        document: &mut automerge::Automerge,
        uuid: uuid::Uuid,
        by: i64,
    ) -> super::error::Result<()> {
        let mut transaction = document.transaction();
        let (_, nodes_map_id) = transaction
            .get(automerge::ObjId::Root, NODES)?
            .ok_or(super::error::TreeError::MissingRoot)?;

        let (_, node_id) = transaction
            .get(&nodes_map_id, uuid.to_string())?
            .ok_or(super::error::TreeError::MissingProperty)?;

        let (total_val, _) = transaction
            .get(&node_id, NODE_TASK_TOTAL)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let current_total = progress::Total::from(
            total_val
                .to_i64()
                .ok_or(super::error::TreeError::InvalidValueType)?,
        );

        let new_total = current_total.change_by(by);
        transaction.put(&node_id, NODE_TASK_TOTAL, new_total.value())?;

        let (completed_val, _) = transaction
            .get(&node_id, NODE_TASK_COMPLETED)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let raw_completed = completed_val.to_i64().unwrap_or(0);

        let constrained_completed = progress::Completed::from_i64(raw_completed, new_total);
        let delta = constrained_completed.value() as i64 - raw_completed;

        // If the new Total caused Completed to shrink, delta will be negative
        if delta != 0 {
            transaction.increment(&node_id, NODE_TASK_COMPLETED, delta)?;
        }

        transaction.commit_with_time();
        Ok(())
    }
}

pub enum View<'a> {
    RootList {
        children: &'a [uuid::Uuid],
    },

    InnerList {
        current_uuid: uuid::Uuid,
        current_node: &'a node::NodeData,
        children: &'a [uuid::Uuid],
    },

    Leaf {
        current_uuid: uuid::Uuid,
        current_node: &'a node::NodeData,
    },
}

impl Tree {
    pub fn view(&self, uuid: Option<&uuid::Uuid>) -> super::error::Result<View<'_>> {
        let Some(uuid) = uuid else {
            return Ok(View::RootList {
                children: self.get_root(),
            });
        };

        let current_node = self.get_node(uuid)?;

        if self.has_children(uuid) {
            Ok(View::InnerList {
                current_uuid: *uuid,
                current_node,
                children: self.get_children_uuids(uuid)?,
            })
        } else {
            Ok(View::Leaf {
                current_uuid: *uuid,
                current_node,
            })
        }
    }
}

fn splice_text(document: &automerge::Automerge) {
    todo!()
}

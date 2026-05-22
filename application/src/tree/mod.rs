pub mod error;
pub mod node;
mod projection;
pub(crate) mod sync;

use crate::tree::error::TreeError;
use automerge::{ReadDoc, transaction::Transactable};
pub use node::Node;
use std::collections::HashMap;

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
    projection: projection::Projection,
}

impl crate::storage::FromBytes for Tree {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let document = automerge::Automerge::load(bytes)?;
        let cached_heads = document.get_heads();
        let projection = projection::Projection::new(&document)?;

        Ok(Self {
            document,
            projection,
        })
    }
}

impl Tree {
    pub fn new() -> error::Result<Self> {
        let mut document = automerge::Automerge::new();
        let mut tx = document.transaction();
        tx.put_object(automerge::ObjId::Root, CHILDREN, automerge::ObjType::List)?;
        tx.commit();

        let projection = projection::Projection::new(&document)?;

        Ok(Self {
            document,
            projection,
        })
    }
}

impl Default for Tree {
    fn default() -> Self {
        #[allow(clippy::expect_used)]
        Self::new().expect("failed to initialize root CHILDREN list on a fresh document")
    }
}

impl Tree {
    pub fn sync_check(&mut self) -> error::Result<()> {
        let current_heads = self.document.get_heads();
        if self.projection.changes != current_heads {
            let patches = self.document.diff(&self.projection.changes, &current_heads);
            self.projection.apply_patches(&self.document, patches)?;
        }

        Ok(())
    }
}

impl Tree {
    pub fn is_leaf(&self, id: &automerge::ObjId) -> error::Result<bool> {
        Ok(self
            .projection
            .children
            .get(id)
            .is_none_or(std::vec::Vec::is_empty))
    }
}

impl Tree {
    pub fn get_node(&self, id: &automerge::ObjId) -> error::Result<node::Node> {
        self.projection
            .nodes
            .get(id)
            .cloned()
            .ok_or(error::TreeError::MissingProperty)
    }

    pub fn get_progress(&self, id: &automerge::ObjId) -> error::Result<node::Progress> {
        if id == &automerge::ObjId::Root {
            return Ok(self.projection.root_progress.clone());
        }

        self.get_node(id).map(|n| n.progress)
    }

    pub fn get_children(
        &self,
        id: &automerge::ObjId,
    ) -> error::Result<Vec<(automerge::ObjId, node::Node)>> {
        let child_ids = self
            .projection
            .children
            .get(id)
            .cloned()
            .unwrap_or_default();
        let mut result = Vec::with_capacity(child_ids.len());

        for child_id in child_ids {
            if let Some(node) = self.projection.nodes.get(&child_id) {
                result.push((child_id, node.clone()));
            }
        }

        Ok(result)
    }

    pub fn get_parent(
        &self,
        id: &automerge::ObjId,
    ) -> error::Result<(automerge::ObjId, node::Node)> {
        let mut parents = self.document.parents(id)?;
        parents.next().ok_or(error::TreeError::MissingProperty)?;
        let parent = parents.next().ok_or(error::TreeError::MissingProperty)?;

        if parent.obj == automerge::ObjId::Root {
            return Err(error::TreeError::MissingRoot);
        }

        let data = self.get_node(&parent.obj)?;
        Ok((parent.obj, data))
    }
}

impl Tree {
    pub fn append_child(
        &mut self,
        parent_id: &automerge::ObjId,
        node: node::Node,
    ) -> error::Result<automerge::ObjId> {
        let mut tx = self.document.transaction();
        let list_id = match tx.get(parent_id, CHILDREN)? {
            Some((_, list_id)) => list_id,
            None => tx.put_object(parent_id, CHILDREN, automerge::ObjType::List)?,
        };
        let list_len = tx.length(&list_id);
        let new_node_id = tx.insert_object(&list_id, list_len, automerge::ObjType::Map)?;
        node.apply_data(&mut tx, &new_node_id)?;
        tx.commit();

        self.projection.nodes.insert(new_node_id.clone(), node);
        self.projection
            .children
            .insert(new_node_id.clone(), Vec::new());
        if let Some(siblings) = self.projection.children.get_mut(parent_id) {
            siblings.push(new_node_id.clone());
        }

        self.projection
            .update_path(&self.document, parent_id.clone())?;

        Ok(new_node_id)
    }

    pub fn remove(&mut self, id: &automerge::ObjId) -> error::Result<()> {
        let mut parents = self.document.parents(id)?;
        let parent_list = parents.next().ok_or(error::TreeError::MissingProperty)?;
        let parent_node = parents.next().ok_or(error::TreeError::MissingRoot)?;
        let parent_id = parent_node.obj;

        // Map out descendant cache removal
        let mut to_remove = vec![id.clone()];
        let mut idx = 0;
        while idx < to_remove.len() {
            let current = &to_remove[idx];
            if let Some(children) = self.projection.children.get(current) {
                to_remove.extend(children.iter().cloned());
            }
            idx += 1;
        }

        // Delete from Automerge
        let mut tx = self.document.transaction();
        tx.delete(&parent_list.obj, parent_list.prop)?;
        tx.commit();

        // Delete from cache
        for rm_id in to_remove {
            self.projection.nodes.remove(&rm_id);
            self.projection.children.remove(&rm_id);
        }
        if let Some(siblings) = self.projection.children.get_mut(&parent_id) {
            siblings.retain(|cid| cid != id);
        }

        // Recalculate progress upwards
        self.projection.update_path(&self.document, parent_id)?;

        Ok(())
    }

    pub fn change_progress_completed(
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
        let current = match completed_val.into_scalar() {
            Ok(ScalarValue::Counter(c)) => i64::try_from(c).unwrap_or(0),
            _ => return Err(error::TreeError::InvalidNodeType),
        };

        let next = current + delta;
        let clamped = next.max(0).min(total as i64);
        let diff = clamped - current;

        if diff == 0 {
            return Ok(());
        }

        // Edit Automerge
        let mut tx = self.document.transaction();
        tx.increment(id, NODE_TASK_COMPLETED, diff)?;
        tx.commit();

        // Recalculate progress upwards
        self.projection.update_path(&self.document, id.clone())?;

        Ok(())
    }
}

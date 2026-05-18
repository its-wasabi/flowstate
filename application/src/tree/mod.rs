use automerge::{ReadDoc, transaction::Transactable};

pub mod error;
pub(crate) mod sync;

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

#[derive(Debug, Clone)]
pub struct NodeData {
    pub name: String,
    pub desc: String,

    pub task_total: u64,
    pub task_completed: u64,
}

impl NodeData {
    fn apply_data(
        self,
        tx: &mut automerge::transaction::Transaction<'_>,
        node_id: &automerge::ObjId,
    ) -> error::Result<()> {
        use automerge::transaction::Transactable;
        tx.put(node_id, NODE_NAME, self.name)?;
        tx.put(node_id, NODE_DESC, self.desc)?;
        tx.put(node_id, NODE_TASK_TOTAL, self.task_total)?;
        tx.put(node_id, NODE_TASK_COMPLETED, self.task_completed)?;
        tx.put_object(node_id, CHILDREN, automerge::ObjType::List)?;

        Ok(())
    }
}

impl NodeData {
    pub fn from_doc(doc: &automerge::Automerge, id: &automerge::ObjId) -> error::Result<Self> {
        use automerge::{ReadDoc, ScalarValue};

        let (name_val, _) = doc
            .get(id, NODE_NAME)?
            .ok_or(error::TreeError::MissingProperty)?;
        let name = match name_val.into_scalar() {
            Ok(ScalarValue::Str(s)) => s.to_string(),
            _ => return Err(error::TreeError::MissingProperty),
        };

        let (desc_val, _) = doc
            .get(id, NODE_DESC)?
            .ok_or(error::TreeError::MissingProperty)?;
        let desc = match desc_val.into_scalar() {
            Ok(ScalarValue::Str(s)) => s.to_string(),
            _ => return Err(error::TreeError::MissingProperty),
        };

        let (total_val, _) = doc
            .get(id, NODE_TASK_TOTAL)?
            .ok_or(error::TreeError::MissingProperty)?;
        let task_total = match total_val.into_scalar() {
            Ok(ScalarValue::Uint(u)) => u,
            Ok(ScalarValue::Int(i)) => {
                u64::try_from(i).map_err(|_| error::TreeError::InvalidValue)?
            }
            _ => return Err(error::TreeError::MissingProperty),
        };

        let (completed_val, _) = doc
            .get(id, NODE_TASK_COMPLETED)?
            .ok_or(error::TreeError::MissingProperty)?;
        let task_completed = match completed_val.into_scalar() {
            Ok(ScalarValue::Uint(u)) => u,
            Ok(ScalarValue::Int(i)) => {
                u64::try_from(i).map_err(|_| error::TreeError::InvalidValue)?
            }
            _ => return Err(error::TreeError::MissingProperty),
        };

        Ok(Self {
            name,
            desc,
            task_total,
            task_completed,
        })
    }
}

#[derive(Debug)]
pub struct Tree {
    pub document: automerge::Automerge,
}

impl Default for Tree {
    fn default() -> Self {
        #[allow(clippy::expect_used)]
        Self::new().expect("failed to initialize root CHILDREN list on a fresh document")
    }
}

impl crate::storage::FromBytes for Tree {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            document: automerge::Automerge::load(bytes)?,
        })
    }
}

impl Tree {
    pub fn new() -> error::Result<Self> {
        let mut document = automerge::Automerge::new();
        let mut tx = document.transaction();
        tx.put_object(automerge::ObjId::Root, CHILDREN, automerge::ObjType::List)?;
        tx.commit();
        Ok(Self { document })
    }
}

impl Tree {
    pub fn get_node(&self, id: &automerge::ObjId) -> error::Result<NodeData> {
        NodeData::from_doc(&self.document, id)
    }
    pub fn get_children(
        &self,
        id: &automerge::ObjId,
    ) -> error::Result<Vec<(automerge::ObjId, NodeData)>> {
        let Some((_, list_id)) = self.document.get(id, CHILDREN)? else {
            return Ok(Vec::with_capacity(0));
        };
        let list_len = self.document.length(&list_id);
        let mut children = Vec::with_capacity(list_len);
        for i in 0..list_len {
            let (_, child_id) = self
                .document
                .get(&list_id, i)?
                .ok_or(error::TreeError::MissingProperty)?;
            let data = NodeData::from_doc(&self.document, &child_id)?;
            children.push((child_id, data));
        }

        Ok(children)
    }
    pub fn get_parent(&self, id: &automerge::ObjId) -> error::Result<(automerge::ObjId, NodeData)> {
        let mut parents = self.document.parents(id)?;
        // first parent is the list containing this node, skip it
        parents.next().ok_or(error::TreeError::MissingProperty)?;
        // second parent is the actual node map
        let parent = parents.next().ok_or(error::TreeError::MissingRoot)?;
        if parent.obj == automerge::ObjId::Root {
            return Err(error::TreeError::MissingRoot);
        }
        let data = NodeData::from_doc(&self.document, &parent.obj)?;
        Ok((parent.obj, data))
    }
}

impl Tree {
    pub fn append_child(
        &mut self,
        id: &automerge::ObjId,
        node: NodeData,
    ) -> error::Result<automerge::ObjId> {
        let mut tx = self.document.transaction();
        let list_id = match tx.get(id, CHILDREN)? {
            Some((_, list_id)) => list_id,
            None => tx.put_object(id, CHILDREN, automerge::ObjType::List)?,
        };
        let list_len = tx.length(&list_id);

        let new_node_id = tx.insert_object(&list_id, list_len, automerge::ObjType::Map)?;
        node.apply_data(&mut tx, &new_node_id)?;

        tx.commit();
        Ok(new_node_id)
    }
}

impl Tree {
    pub fn remove(&mut self, id: &automerge::ObjId) -> error::Result<()> {
        let mut tx = self.document.transaction();
        let mut parents = tx.parents(id)?;
        let parent = parents.next().ok_or(error::TreeError::MissingProperty)?;
        tx.delete(&parent.obj, parent.prop)?;

        tx.commit();
        Ok(())
    }
}

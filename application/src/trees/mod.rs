use automerge::{ReadDoc, transaction::Transactable};

pub mod error;
pub mod node;
pub(crate) mod sync;

/// Root of entire document
const DOC_ROOT: &str = "root";

/// List of all children of that node
const NODE_CHILDREN: &str = "l";
/// Name of that node
const NODE_NAME: &str = "n";
/// Description of that node
const NODE_DESC: &str = "d";
/// Total number of tasks for that node
const NODE_TASK_TOTAL: &str = "t";
/// Number of completed tasks for that node
const NODE_TASK_COMPLETED: &str = "c";

#[derive(Debug)]
pub struct Trees {
    pub(super) document: automerge::Automerge,
}

impl Default for Trees {
    fn default() -> Self {
        use automerge::transaction::Transactable;
        let mut document = automerge::Automerge::new();
        let mut tx = document.transaction();
        tx.put_object(automerge::ROOT, DOC_ROOT, automerge::ObjType::List);
        tx.commit();

        Self { document }
    }
}

impl crate::storage::FromBytes for Trees {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            document: automerge::Automerge::load(bytes)?,
        })
    }
}

impl Trees {
    pub fn root_list(&self) -> Result<automerge::ObjId, error::TreeError> {
        use automerge::ReadDoc;
        self.document
            .get(automerge::ROOT, DOC_ROOT)?
            .ok_or(error::TreeError::MissingRoot)
            .map(|(value, obj_id)| obj_id)
    }

    pub fn add_tree(
        &mut self,
        name: String,
        desc: String,
    ) -> Result<automerge::ObjId, error::TreeError> {
        let list = self.root_list()?;
        let mut tx = self.document.transaction();

        let list_len = tx.length(&list);
        let node_id = tx.put_object(
            &list,
            automerge::Prop::Seq(list_len),
            automerge::ObjType::Map,
        )?;

        tx.put(&node_id, NODE_NAME, name)?;
        tx.put(&node_id, NODE_DESC, desc)?;
        tx.put(&node_id, NODE_TASK_TOTAL, 1u32)?;
        tx.put(&node_id, NODE_TASK_COMPLETED, 0u32)?;

        tx.put_object(&node_id, NODE_CHILDREN, automerge::ObjType::List)?;
        tx.commit();

        Ok(node_id)
    }

    pub fn add_child_node(
        &mut self,
        parent_id: &automerge::ObjId,
        name: String,
        desc: String,
    ) -> Result<automerge::ObjId, error::TreeError> {
        let mut tx = self.document.transaction();

        let (_, list_id) = tx
            .get(parent_id, NODE_CHILDREN)?
            .ok_or(error::TreeError::MissingProperty)?;
        let list_len = tx.length(&list_id);

        let child_id = tx.put_object(
            &list_id,
            automerge::Prop::Seq(list_len),
            automerge::ObjType::Map,
        )?;

        tx.put(&child_id, NODE_NAME, name)?;
        tx.put(&child_id, NODE_DESC, desc)?;
        tx.put(&child_id, NODE_TASK_TOTAL, 1u32)?;
        tx.put(&child_id, NODE_TASK_COMPLETED, 0u32)?;
        tx.put_object(&child_id, NODE_CHILDREN, automerge::ObjType::List)?;

        tx.commit();
        Ok(child_id)
    }
}
impl Trees {
    pub fn set_node_name(&mut self, node_id: &automerge::ObjId, name: String) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(node_id, NODE_NAME, name)?;
        tx.commit();

        Ok(())
    }
    pub fn set_node_desc(&mut self, node_id: &automerge::ObjId, desc: String) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(node_id, NODE_DESC, desc)?;
        tx.commit();

        Ok(())
    }
    pub fn set_node_total(&mut self, node_id: &automerge::ObjId, total: u32) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(node_id, NODE_TASK_TOTAL, total)?;
        tx.commit();

        Ok(())
    }
    pub fn set_node_completed(
        &mut self,
        node_id: &automerge::ObjId,
        completed: u32,
    ) -> error::Result<()> {
        let mut tx = self.document.transaction();
        tx.put(node_id, NODE_TASK_COMPLETED, completed)?;
        tx.commit();

        Ok(())
    }
}

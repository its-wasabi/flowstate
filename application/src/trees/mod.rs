pub mod error;
pub mod node;
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

#[derive(Debug)]
pub struct Trees {
    pub document: automerge::Automerge,
}

impl Default for Trees {
    fn default() -> Self {
        use automerge::transaction::Transactable;
        let mut document = automerge::Automerge::new();
        let mut tx = document.transaction();
        tx.put_object(automerge::ObjId::Root, CHILDREN, automerge::ObjType::List)
            .unwrap();
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
    pub fn append_node_at(
        &mut self,
        parent_node_id: &automerge::ObjId,
        node_data: node::NodeData,
    ) -> error::Result<()> {
        use automerge::ReadDoc;
        use automerge::transaction::Transactable;
        let mut tx = self.document.transaction();

        let (_, parent_list_id) = tx
            .get(parent_node_id, CHILDREN)?
            .ok_or(error::TreeError::MissingProperty)?;
        let parent_list_len = tx.length(&parent_list_id);

        let new_node_id =
            tx.insert_object(&parent_list_id, parent_list_len, automerge::ObjType::Map)?;
        node_data.apply_data(&mut tx, &new_node_id)?;

        tx.commit();
        Ok(())
    }

    pub fn get_nodes_at(
        &self,
        parent_node_id: &automerge::ObjId,
    ) -> error::Result<Vec<automerge::ObjId>> {
        use automerge::ReadDoc;
        let (_, list_id) = self
            .document
            .get(parent_node_id, CHILDREN)?
            .ok_or(error::TreeError::MissingProperty)?;

        let list_len = self.document.length(&list_id);
        let mut nodes = Vec::with_capacity(list_len);
        for i in 0..list_len {
            let (child, child_id) = self
                .document
                .get(&list_id, i)?
                .ok_or(error::TreeError::MissingProperty)?;

            let data = child_id;
            nodes.push(data);
        }

        Ok(nodes)
    }

    pub fn get_node_data(&self, node_id: &automerge::ObjId) -> error::Result<node::NodeData> {
        node::NodeData::from_doc(&self.document, node_id)
    }

    pub fn get_node_progress(&self, node_id: &automerge::ObjId) -> error::Result<(u64, u64)> {
        use automerge::ReadDoc;

        let (_, list_id) = self
            .document
            .get(node_id, CHILDREN)?
            .ok_or(error::TreeError::MissingProperty)?;

        let list_len = self.document.length(&list_id);

        if list_len == 0 {
            let node = node::NodeData::from_doc(&self.document, node_id)?;
            Ok((node.task_total, node.task_completed))
        } else {
            let mut sum_total: u64 = 0;
            let mut sum_completed: u64 = 0;

            for i in 0..list_len {
                let (_, child_id) = self
                    .document
                    .get(&list_id, i)?
                    .ok_or(error::TreeError::MissingProperty)?;

                let (t, c) = self.get_node_progress(&child_id)?;
                sum_total += t;
                sum_completed += c;
            }

            Ok((sum_total, sum_completed))
        }
    }
}

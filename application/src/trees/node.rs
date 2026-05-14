#[derive(Debug)]
pub struct NodeData {
    pub name: String,
    pub desc: String,

    pub task_total: u64,
    pub task_completed: u64,
}

impl NodeData {
    pub(super) fn apply_data(
        self,
        tx: &mut automerge::transaction::Transaction<'_>,
        node_id: &automerge::ObjId,
    ) -> super::error::Result<()> {
        use automerge::transaction::Transactable;
        tx.put(node_id, super::NODE_NAME, self.name)?;
        tx.put(node_id, super::NODE_DESC, self.desc)?;
        tx.put(node_id, super::NODE_TASK_TOTAL, self.task_total)?;
        tx.put(node_id, super::NODE_TASK_COMPLETED, self.task_completed)?;
        tx.put_object(node_id, super::CHILDREN, automerge::ObjType::List)?;

        Ok(())
    }
    pub fn from_doc(
        doc: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::error::Result<Self> {
        use automerge::ReadDoc;
        use automerge::ScalarValue;

        let get_string = |key| -> super::error::Result<String> {
            let (val, _) = doc
                .get(id, key)?
                .ok_or(super::error::TreeError::MissingProperty)?;
            match val.into_scalar() {
                Ok(ScalarValue::Str(s)) => Ok(s.to_string()),
                _ => Err(super::error::TreeError::MissingProperty),
            }
        };

        let get_u64 = |key| -> super::error::Result<u64> {
            let (val, _) = doc
                .get(id, key)?
                .ok_or(super::error::TreeError::MissingProperty)?;
            match val.into_scalar() {
                Ok(ScalarValue::Uint(u)) => Ok(u),
                Ok(ScalarValue::Int(i)) => Ok(i as u64),
                _ => Err(super::error::TreeError::MissingProperty),
            }
        };

        Ok(Self {
            name: get_string(super::NODE_NAME)?,
            desc: get_string(super::NODE_DESC)?,
            task_total: get_u64(super::NODE_TASK_TOTAL)?,
            task_completed: get_u64(super::NODE_TASK_COMPLETED)?,
        })
    }
}

#[derive(Debug, Clone)]
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
}

impl NodeData {
    pub fn from_doc(
        doc: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> super::error::Result<Self> {
        use automerge::{ReadDoc, ScalarValue};

        let (name_val, _) = doc
            .get(id, super::NODE_NAME)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let name = match name_val.into_scalar() {
            Ok(ScalarValue::Str(s)) => s.to_string(),
            _ => return Err(super::error::TreeError::MissingProperty),
        };

        let (desc_val, _) = doc
            .get(id, super::NODE_DESC)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let desc = match desc_val.into_scalar() {
            Ok(ScalarValue::Str(s)) => s.to_string(),
            _ => return Err(super::error::TreeError::MissingProperty),
        };

        let (total_val, _) = doc
            .get(id, super::NODE_TASK_TOTAL)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let task_total = match total_val.into_scalar() {
            Ok(ScalarValue::Uint(u)) => u,
            Ok(ScalarValue::Int(i)) => {
                u64::try_from(i).map_err(|_| super::error::TreeError::InvalidValue)?
            }
            _ => return Err(super::error::TreeError::MissingProperty),
        };

        let (completed_val, _) = doc
            .get(id, super::NODE_TASK_COMPLETED)?
            .ok_or(super::error::TreeError::MissingProperty)?;
        let task_completed = match completed_val.into_scalar() {
            Ok(ScalarValue::Uint(u)) => u,
            Ok(ScalarValue::Int(i)) => {
                u64::try_from(i).map_err(|_| super::error::TreeError::InvalidValue)?
            }
            _ => return Err(super::error::TreeError::MissingProperty),
        };

        Ok(Self {
            name,
            desc,
            task_total,
            task_completed,
        })
    }
}

use automerge::{ReadDoc, transaction::Transactable};

#[derive(Debug, Clone)]
pub struct NodeData {
    pub name: String,
    pub desc: String,

    pub progress: super::progress::Progress,
}

impl Default for NodeData {
    fn default() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            progress: super::progress::Progress::from_values(10, 0),
        }
    }
}

impl NodeData {
    pub(super) fn apply_data(
        &self,
        transaction: &mut automerge::transaction::Transaction<'_>,
        node_id: &automerge::ObjId,
    ) -> super::super::error::Result<()> {
        transaction.put(node_id, super::NODE_NAME, self.name.clone())?;
        transaction.put(node_id, super::NODE_DESC, self.desc.clone())?;

        transaction.put(node_id, super::NODE_TASK_TOTAL, self.progress.total())?;
        transaction.put(
            node_id,
            super::NODE_TASK_COMPLETED,
            automerge::ScalarValue::counter(i64::from(self.progress.completed())),
        )?;

        Ok(())
    }
}

impl NodeData {
    pub(super) fn from_id(
        document: &automerge::Automerge,
        id: &automerge::ObjId,
    ) -> crate::store::error::Result<Self> {
        let (name, _) = document
            .get(id, super::NODE_NAME)?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;
        let name = name.to_string();
        let (desc, _) = document
            .get(id, super::NODE_DESC)?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;
        let desc = desc.to_string();

        let (completed, _) = document
            .get(id, super::NODE_TASK_COMPLETED)?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;
        let completed: u32 = completed
            .to_u64()
            .ok_or(crate::store::error::TreeError::InvalidValueType)?
            .min(u32::MAX as u64) as u32;

        let (total, _) = document
            .get(id, super::NODE_TASK_TOTAL)?
            .ok_or(crate::store::error::TreeError::MissingProperty)?;
        let total: u32 = total
            .to_u64()
            .ok_or(crate::store::error::TreeError::InvalidValueType)?
            .min(u32::MAX as u64) as u32;

        let progress = super::progress::Progress::from_values(completed, total);

        Ok(Self {
            name,
            desc,
            progress,
        })
    }
}

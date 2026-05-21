use std::{
    iter::Product,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone)]
pub struct Progress {
    pub total: u64,
    pub completed: u64,
}

impl Progress {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn progress(&self) -> f32 {
        self.completed as f32 / self.total as f32
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            total: 0,
            completed: 0,
        }
    }
}

impl AddAssign for Progress {
    fn add_assign(&mut self, rhs: Self) {
        self.total = self.total + rhs.total;
        self.completed = self.completed + rhs.completed;
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub desc: String,

    pub progress: Progress,
}

impl Node {
    pub(super) fn apply_data(
        self,
        tx: &mut automerge::transaction::Transaction<'_>,
        node_id: &automerge::ObjId,
    ) -> super::error::Result<()> {
        use automerge::transaction::Transactable;
        tx.put(node_id, super::NODE_NAME, self.name)?;
        tx.put(node_id, super::NODE_DESC, self.desc)?;
        tx.put(node_id, super::NODE_TASK_TOTAL, self.progress.total)?;
        tx.put(node_id, super::NODE_TASK_COMPLETED, self.progress.completed)?;
        tx.put_object(node_id, super::CHILDREN, automerge::ObjType::List)?;

        Ok(())
    }
}

impl Node {
    pub(super) fn from_doc(
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
            progress: Progress {
                total: task_total,
                completed: task_completed,
            },
        })
    }
}

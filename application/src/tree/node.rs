#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    total: u32,
    completed: u32,
}

impl Progress {
    /// # Panics
    /// Panics if completed is bigger than total
    #[must_use]
    pub const fn new(completed: u32, total: u32) -> Self {
        assert!(
            completed <= total,
            "Completed should never be bigger than total"
        );

        Self { total, completed }
    }

    #[must_use]
    pub const fn total(&self) -> u32 {
        self.total
    }

    #[must_use]
    pub const fn completed(&self) -> u32 {
        self.completed
    }

    #[must_use]
    #[inline]
    pub const fn procentage(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }

        let pct = (self.completed as u64 * 100) / self.total as u64;

        #[allow(clippy::cast_precision_loss)]
        {
            pct as f32
        }
    }
}

impl std::ops::AddAssign for Progress {
    fn add_assign(&mut self, rhs: Self) {
        self.total = self.total + rhs.total;
        self.completed = self.completed + rhs.completed;
    }
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.procentage())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub desc: String,

    pub progress: Progress,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            progress: Progress::new(0, 10),
        }
    }
}

impl Node {
    pub(super) fn apply_data(
        &self,
        tx: &mut automerge::transaction::Transaction<'_>,
        node_id: &automerge::ObjId,
    ) -> super::error::Result<()> {
        use automerge::transaction::Transactable;
        tx.put(node_id, super::NODE_NAME, self.name.clone())?;
        tx.put(node_id, super::NODE_DESC, self.desc.clone())?;

        tx.put(node_id, super::NODE_TASK_TOTAL, self.progress.total)?;
        tx.put(
            node_id,
            super::NODE_TASK_COMPLETED,
            automerge::ScalarValue::counter(i64::from(self.progress.completed)),
        )?;

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
            Ok(ScalarValue::Uint(u)) => {
                u32::try_from(u).map_err(|_| super::error::TreeError::InvalidValue)?
            }
            Ok(ScalarValue::Int(i)) => {
                u32::try_from(i).map_err(|_| super::error::TreeError::InvalidValue)?
            }
            _ => return Err(super::error::TreeError::InvalidNodeType),
        };

        let (completed_val, _) = doc
            .get(id, super::NODE_TASK_COMPLETED)?
            .ok_or(super::error::TreeError::MissingProperty)?;

        let task_completed = match completed_val.into_scalar() {
            Ok(automerge::ScalarValue::Counter(c)) => {
                let i = i64::try_from(c).unwrap_or(0);
                u32::try_from(i).map_err(|_| super::error::TreeError::InvalidValue)?
            }
            _ => return Err(super::error::TreeError::InvalidNodeType),
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

#![allow(dead_code)]

pub enum TaskKind {
    Todo { done: bool },

    Progress { completed: u64, total: u64 },
}
pub struct Task {
    kind: TaskKind,
}

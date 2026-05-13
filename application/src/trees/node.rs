pub struct Node {
    name: String,
    desc: String,

    task_total: u32,
    task_completed: u32,

    children: Vec<automerge::ObjId>,
}

impl Node {
    pub fn read(doc: &impl automerge::ReadDoc, id: &automerge::ObjId) -> Option<Node> {
        None
    }
}

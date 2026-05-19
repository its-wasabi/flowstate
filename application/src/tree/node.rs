pub struct NodeEdit {
    data: super::NodeData,
}

impl NodeEdit {
    fn from_node(node_data: &super::NodeData) -> Self {
        Self {
            data: node_data.clone(),
        }
    }
}

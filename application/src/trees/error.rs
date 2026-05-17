pub type Result<T> = std::result::Result<T, TreeError>;

#[derive(Debug)]
pub enum TreeError {
    MissingRoot,
    MissingProperty,
    // TODO: Remove
    InvalidNodeType,
    InvalidValue,
    Automerge(automerge::AutomergeError),
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for TreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Automerge(automerge_error) => Some(automerge_error),
            _ => None,
        }
    }
}

impl From<automerge::AutomergeError> for TreeError {
    fn from(value: automerge::AutomergeError) -> Self {
        Self::Automerge(value)
    }
}

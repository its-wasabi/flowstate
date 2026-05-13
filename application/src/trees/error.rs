pub type Result<T> = std::result::Result<T, TreeError>;

pub enum TreeError {
    MissingRoot,
    MissingProperty,
    Automerge(automerge::AutomergeError),
}

impl From<automerge::AutomergeError> for TreeError {
    fn from(value: automerge::AutomergeError) -> Self {
        Self::Automerge(value)
    }
}

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
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRoot => {
                write!(fmt, "missing root node in document")
            }
            Self::MissingProperty => {
                write!(fmt, "missing required property")
            }
            Self::InvalidNodeType => {
                write!(fmt, "invalid node type")
            }
            Self::InvalidValue => {
                write!(fmt, "invalid value")
            }
            Self::Automerge(err) => {
                write!(fmt, "automerge error: {err}")
            }
        }
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

mod error;
mod ext;
pub mod tree;

#[derive(Debug, Clone)]
pub enum Command {
    Tree(tree::Command),
}

#[derive(Debug)]
pub struct Store {
    pub tree: tree::Tree,

    pub(super) document: automerge::Automerge,
}

impl Store {
    pub fn dispatch(&mut self, command: Command) -> error::Result<()> {
        match command {
            Command::Tree(command) => self.tree.dispatch(&mut self.document, command)?,
        }

        Ok(())
    }
}

impl crate::io::storage::FromBytes for Store {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut document = automerge::Automerge::new();
        let tree = tree::Tree::new(&mut document)?;
        Ok(Self { tree, document })
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut document = automerge::Automerge::load(bytes)?;
        let tree = tree::Tree::new(&mut document)?;
        Ok(Self { tree, document })
    }
}

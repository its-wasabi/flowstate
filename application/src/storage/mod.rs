pub mod config;
pub mod data;
pub mod paths;

#[derive(Debug)]
pub struct Storage {
    paths: paths::Paths,
    document: automerge::AutoCommit,
}

impl Storage {
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(parent) = self.paths.app_data_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let bytes = self.document.save();
        std::fs::write(&self.paths.app_data_file, bytes)
    }

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let paths = paths::Paths::new().ok_or("")?;

        let document = match std::fs::read(&paths.app_data_file) {
            Ok(bytes) => automerge::AutoCommit::load(&bytes)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => automerge::AutoCommit::new(),
            Err(err) => return Err(Box::new(err)),
        };

        Ok(Self { paths, document })
    }
}

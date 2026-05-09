pub mod paths;
pub mod storable;

#[derive(Debug)]
pub struct Storage {
    paths: paths::Paths,
}

pub enum StorageType {
    Config,
    Data,
}

/// This block is responsible for disk io
impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let paths = paths::Paths::new().ok_or("Failed to resolve application paths")?;
        Self::ensure_parents_exists(&paths)?;
        Ok(Self { paths })
    }

    pub fn ensure_parents_exists(paths: &paths::Paths) -> std::io::Result<()> {
        std::fs::create_dir_all(&paths.app_data_dir)?;
        std::fs::create_dir_all(&paths.app_config_dir)?;

        Ok(())
    }
}

impl Storage {
    pub fn load_or_default<T>(&self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: storable::Storable + Default,
    {
        let path = match T::storage_type() {
            StorageType::Config => self.paths.app_config_dir.join(T::file_name()),
            StorageType::Data => self.paths.app_data_dir.join(T::file_name()),
        };

        let data = match std::fs::read(path) {
            Ok(bytes) => T::from_bytes(&bytes)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => T::default(),
            Err(err) => return Err(Box::new(err)),
        };

        Ok(data)
    }

    pub fn store<T>(&self, data: &mut T) -> std::io::Result<()>
    where
        T: storable::Storable,
    {
        let path = match T::storage_type() {
            StorageType::Config => self.paths.app_config_dir.join(T::file_name()),
            StorageType::Data => self.paths.app_data_dir.join(T::file_name()),
        };

        let bytes = data.as_bytes();
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

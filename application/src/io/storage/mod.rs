use std::sync::{Arc, Mutex};

pub mod paths;

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>;
}

impl FromBytes for automerge::AutoCommit {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::load(bytes)?)
    }
}

impl FromBytes for crate::config::Config {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    paths: Arc<paths::Paths>,
    runtime: tokio::runtime::Handle,
    pending_tasks: Arc<Mutex<Vec<tokio::task::JoinHandle<std::io::Result<()>>>>>,
}

impl Storage {
    pub fn new(runtime: tokio::runtime::Handle) -> Result<Self, Box<dyn std::error::Error>> {
        let paths = paths::Paths::new().ok_or("Failed to resolve application paths")?;
        std::fs::create_dir_all(&paths.app_data_dir)?;
        std::fs::create_dir_all(&paths.app_config_dir)?;

        let paths = Arc::new(paths);
        let pending = Arc::default();

        Ok(Self {
            paths,
            runtime,
            pending_tasks: pending,
        })
    }
}

impl Storage {
    pub fn load_or_default<T: FromBytes + Default>(
        &self,
        path_suffix: &str,
        storage_kind: paths::StorageKind,
    ) -> Result<T, Box<dyn std::error::Error>> {
        Ok(
            match std::fs::read(self.paths.resolve_path(path_suffix, storage_kind)) {
                Ok(bytes) => T::from_bytes(&bytes)?,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => T::default(),
                Err(err) => return Err(Box::new(err)),
            },
        )
    }

    pub fn save(&self, suffix: &'static str, kind: paths::StorageKind, bytes: Vec<u8>) {
        let path = self.paths.resolve_path(suffix, kind);
        let task_handle = self
            .runtime
            .spawn(async move { tokio::fs::write(path, bytes).await });

        // TODO: If data is poisoned should I still keep it. Does it even run or something
        let mut pending = self
            .pending_tasks
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        pending.retain(|th| !th.is_finished());

        pending.push(task_handle);
    }

    pub fn flush(&self) {
        let handles: Vec<_> = self
            .pending_tasks
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .drain(..)
            .collect();

        self.runtime.block_on(async {
            for handle in handles {
                if let Err(err) = handle.await {
                    eprintln!("storage: save task panicked: {err}");
                }
            }
        });
    }
}

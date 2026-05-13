// NOTE: If storage.load_or_create() fails to load exiling document it will load it from sever and
// then overwrite broken file
// NOTE: Storage or Network doesn't talk with each other but instead App uses them as subsystem to
// manage document and other stuff by itself
// NOTE: You never send whole document even on startup only last known state and receive changes
// since then

#![allow(unused)]
#![allow(clippy::missing_errors_doc)]

pub mod analytics;
pub mod config;
pub mod error;
pub mod peer;
pub mod storage;
pub mod trees;

pub const APP_NAME: &str = "flowstate";

const DOCUMENT_SAVE_PATH: &str = "data.bin";
const CONFIG_SAVE_PATH: &str = "config.json";

#[derive(Debug)]
pub struct Core {
    runtime: tokio::runtime::Runtime,
    storage: storage::Storage,
    config: config::Config,
    sync: peer::Peer,
    tree: trees::Trees,
}

impl Core {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let storage = storage::Storage::new(runtime.handle().clone())?;

        let config: config::Config =
            storage.load_or_default(CONFIG_SAVE_PATH, storage::paths::StorageKind::Config)?;
        let mut tree: trees::Trees =
            storage.load_or_default(DOCUMENT_SAVE_PATH, storage::paths::StorageKind::Data)?;

        let sync = peer::Peer::new(&mut tree.document, config.server_socket)?;

        Ok(Self {
            runtime,
            storage,
            config,
            sync,
            tree,
        })
    }

    pub fn save(&mut self) -> serde_json::Result<()> {
        self.storage.save(
            DOCUMENT_SAVE_PATH,
            storage::paths::StorageKind::Data,
            self.tree.document.save(),
        );

        self.storage.save(
            CONFIG_SAVE_PATH,
            storage::paths::StorageKind::Config,
            self.config.as_bytes()?,
        );

        Ok(())
    }

    pub fn sync(&mut self) {
        self.sync.sync(&mut self.tree.document);
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        self.storage.flush();
    }
}

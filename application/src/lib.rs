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
pub mod tree;

pub const APP_NAME: &str = "flowstate";

const DOCUMENT_SAVE_PATH: &str = "data.bin";
const CONFIG_SAVE_PATH: &str = "config.json";

#[derive(Debug)]
pub struct Core {
    runtime: tokio::runtime::Runtime,
    storage: storage::Storage,

    pub config: config::Config,
    pub tree: tree::Tree,

    sync: Option<peer::Peer>,
}

impl Core {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let storage = storage::Storage::new(runtime.handle().clone())?;

        let config: config::Config =
            storage.load_or_default(CONFIG_SAVE_PATH, storage::paths::StorageKind::Config)?;
        let mut tree: tree::Tree =
            storage.load_or_default(DOCUMENT_SAVE_PATH, storage::paths::StorageKind::Data)?;

        let sync = if let Some(server_socket) = config.server_socket {
            Some(peer::Peer::new(&mut tree.document, server_socket)?)
        } else {
            None
        };

        Ok(Self {
            runtime,
            storage,

            config,
            tree,

            sync,
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
}

impl Drop for Core {
    fn drop(&mut self) {
        self.save();
        self.storage.flush();
    }
}

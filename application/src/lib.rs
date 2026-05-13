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
pub mod network;
pub mod storage;

pub const APP_NAME: &str = "flowstate";

const DOCUMENT_SAVE_PATH: &str = "document.bin";
const CONFIG_SAVE_PATH: &str = "config.json";

#[derive(Debug)]
pub struct Core {
    storage: storage::Storage,
    network: Option<network::Network>,

    config: config::Config,
    document: automerge::AutoCommit,
    runtime: tokio::runtime::Runtime,
}

impl Core {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let storage = storage::Storage::new(runtime.handle().clone())?;

        let config: config::Config =
            storage.load_or_default("config.json", storage::paths::StorageKind::Config)?;
        let mut document: automerge::AutoCommit =
            storage.load_or_default("document.bin", storage::paths::StorageKind::Data)?;

        let network = if let Some(server_socket) = config.server_socket {
            Some(network::Network::new(&mut document, server_socket)?)
        } else {
            None
        };

        Ok(Self {
            storage,
            network,
            config,
            document,
            runtime,
        })
    }

    pub fn save(&mut self) -> serde_json::Result<()> {
        self.storage.save(
            DOCUMENT_SAVE_PATH,
            storage::paths::StorageKind::Data,
            self.document.save(),
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
        self.storage.flush();
    }
}

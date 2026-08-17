// NOTE: If storage.load_or_create() fails to load exiling document it will load it from sever and
// then overwrite broken file
// NOTE: You never send whole document even on startup only last known state and receive changes
// since then

mod config;
mod error;
mod io;
mod peer;
pub mod store;

pub const APP_NAME: &str = "flowstate";
pub const APP_VERSION: (u32, u32, u32) = utils::crate_version!();

const DOCUMENT_SAVE_FILE_PATH: &str = "data.bin";
const CONFIG_SAVE_FILE_PATH: &str = "config.json";

#[derive(Debug)]
pub struct Core {
    runtime: tokio::runtime::Runtime,
    storage: io::storage::Storage,
    network: io::network::Network,

    peer: Option<peer::Peer>,

    pub config: config::Config,
    pub store: store::Store,
}

impl Core {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let storage = io::storage::Storage::new(runtime.handle().clone())?;
        let network = io::network::Network::new();

        let config: config::Config = storage.load_or_create(
            CONFIG_SAVE_FILE_PATH,
            io::storage::paths::StorageKind::Config,
        )?;

        let store = storage.load_or_create(
            DOCUMENT_SAVE_FILE_PATH,
            io::storage::paths::StorageKind::Data,
        )?;

        let peer = if let Some(server_socket) = config.server_socket {
            Some(peer::Peer::new(server_socket)?)
        } else {
            None
        };

        Ok(Self {
            runtime,
            storage,
            network,

            peer,

            config,
            store,
        })
    }

    // TODO: Rethink the sync approach maybe make it store internal the entire peer thing only
    // external part would be generic network API
    pub fn dispatch_store(
        &mut self,
        command: store::Command,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.store.dispatch(command)?;

        if let Some(peer) = &mut self.peer {
            let sync_message = peer.generate_sync_message(&mut self.store.document);
            if let Some(sync_message) = sync_message {
                println!("TODO: sync here");
            }
        };

        Ok(())
    }

    pub fn save(&mut self) -> serde_json::Result<()> {
        self.storage.save(
            DOCUMENT_SAVE_FILE_PATH,
            io::storage::paths::StorageKind::Data,
            self.store.document.save(),
        );

        self.storage.save(
            CONFIG_SAVE_FILE_PATH,
            io::storage::paths::StorageKind::Config,
            self.config.as_bytes()?,
        );

        Ok(())
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        if self.save().is_err() {
            eprintln!("FAILED (critical): TO SAVE APP DATA ON SHUTDOWN!!!")
        };

        self.storage.flush();
    }
}

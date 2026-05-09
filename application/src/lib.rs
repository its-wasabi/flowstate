// NOTE: If storage.load_or_create() fails to load exiling document it will load it from sever and
// then overwrite broken file
// NOTE: Storage or Network doesn't talk with each other but instead App uses them as subsystem to
// manage document and other stuff by itself
// NOTE: You never send whole document even on startup only last known state and receive changes
// since then

#![allow(unused)]
#![allow(clippy::missing_errors_doc)]

pub mod network;
pub mod storage;

pub const APP_NAME: &str = "flowstate";

#[derive(Debug)]
pub struct App {
    pub storage: storage::Storage,
    pub network: network::Network,

    pub document: automerge::AutoCommit,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage = storage::Storage::new()?;
        let mut document = storage.load_or_default()?;

        let server_socket = std::net::SocketAddr::new([127, 0, 0, 1].into(), 8080);
        let network = network::Network::new(&mut document, server_socket)?;

        Ok(Self {
            storage,
            network,
            document,
        })
    }
}

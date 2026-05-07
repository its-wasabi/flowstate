// NOTE: If storage.load_or_create() fails to load exiling document it will load it from sever and
// then overwrite broken file
// NOTE: Storage or Network doesn't talk with each other but instead App uses them as subsystem to
// manage document and other stuff by itself
// NOTE: You never send whole document even on startup only last known state and receive changes
// since then

#![allow(unused)]
#![allow(clippy::missing_errors_doc)]

pub mod storage;

pub const APP_NAME: &str = "flowstate";

#[derive(Debug)]
pub struct App {
    storage: storage::Storage,
    document: Document,
}

#[derive(Debug, Default)]
pub struct Document(std::cell::RefCell<automerge::AutoCommit>);

impl storage::Storable for Document {
    fn file_name() -> &'static str {
        "document.bin"
    }

    fn storage_type() -> storage::StorageType {
        storage::StorageType::Data
    }

    fn into_bytes(&self) -> Vec<u8> {
        self.0.borrow_mut().save()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(std::cell::RefCell::new(automerge::AutoCommit::load(
            bytes,
        )?)))
    }
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage = storage::Storage::new()?;
        let document = storage.load_or_create()?;
        Ok(Self { storage, document })
    }
}

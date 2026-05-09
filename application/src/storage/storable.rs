pub trait Storable: Sized {
    fn storage_type() -> super::StorageType;
    fn file_name() -> &'static str;
    fn as_bytes(&mut self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>;
}

impl Storable for automerge::AutoCommit {
    fn file_name() -> &'static str {
        "document.bin"
    }

    fn storage_type() -> super::StorageType {
        super::StorageType::Data
    }

    fn as_bytes(&mut self) -> Vec<u8> {
        self.save()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::load(bytes)?)
    }
}

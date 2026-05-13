#[derive(Debug, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Config {
    pub(super) server_socket: Option<std::net::SocketAddr>,
    // TODO: Create some field that can be managed by ui Frontend that manages how things are
    // displayed and colors
}

impl Config {
    pub fn as_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&self)
    }
}

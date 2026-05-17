#[derive(Debug, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Config {
    pub(super) server_socket: Option<std::net::SocketAddr>,
}

impl Config {
    pub fn as_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self)
    }
}

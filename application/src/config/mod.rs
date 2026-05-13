#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Config {
    pub(super) server_socket: std::net::SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_socket: std::net::SocketAddr::new(std::net::IpAddr::from([127, 0, 0, 1]), 8080),
        }
    }
}

impl Config {
    pub fn as_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self)
    }
}

pub struct Config {
    pub address: std::net::SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: std::net::SocketAddr::new(std::net::IpAddr::from([127, 0, 0, 0]), 3000),
        }
    }
}

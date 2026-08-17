#[derive(Debug, Default)]
pub struct Network {}

impl Network {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn send(&self, socket: std::net::SocketAddr, bytes: &[u8]) {}
}

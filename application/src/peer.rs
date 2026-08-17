#[derive(Debug)]
pub struct Peer {
    local_peer_state: automerge::sync::State,
    pub server_uri: tokio_tungstenite::tungstenite::http::Uri,

    outgoing: tokio::sync::mpsc::Sender<Vec<u8>>,
    incoming: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

impl Peer {
    pub fn new(socket_addr: std::net::SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        let local_peer_state = automerge::sync::State::new();
        let server_uri = tokio_tungstenite::tungstenite::http::Uri::builder()
            .scheme("ws")
            .authority(socket_addr.to_string())
            .path_and_query("/")
            .build()?;

        let (_to_net_tx, to_net_rx) = tokio::sync::mpsc::channel(32);
        let (from_net_tx, _from_net_rx) = tokio::sync::mpsc::channel(32);

        Ok(Self {
            local_peer_state,
            server_uri,
            incoming: to_net_rx,
            outgoing: from_net_tx,
        })
    }

    pub fn sync(
        &mut self,
        _document: &mut automerge::Automerge,
        patches: &[automerge::patches::Patch],
    ) {
    }

    pub(crate) fn generate_sync_message(
        &mut self,
        document: &mut automerge::Automerge,
    ) -> Option<Vec<u8>> {
        use automerge::sync::SyncDoc;
        document
            .generate_sync_message(&mut self.local_peer_state)
            .map(automerge::sync::Message::encode)
    }

    pub(crate) fn receive_sync_message(
        &mut self,
        document: &mut automerge::Automerge,
        bytes: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use automerge::sync::SyncDoc;
        let msg = automerge::sync::Message::decode(bytes)?;
        document.receive_sync_message(&mut self.local_peer_state, msg)?;
        Ok(())
    }
}

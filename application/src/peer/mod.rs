#[derive(Debug)]
pub struct Peer {
    local_peer_state: automerge::sync::State,
    server_uri: tokio_tungstenite::tungstenite::http::Uri,

    outgoing: tokio::sync::mpsc::Sender<Vec<u8>>,
    incoming: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

impl Peer {
    pub fn new(
        document: &mut automerge::Automerge,
        socket_addr: std::net::SocketAddr,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut local_peer_state = automerge::sync::State::new();
        let server_uri = tokio_tungstenite::tungstenite::http::Uri::builder()
            .scheme("ws")
            .authority(socket_addr.to_string())
            .path_and_query("/")
            .build()?;

        let (to_net_tx, to_net_rx) = tokio::sync::mpsc::channel(32);
        let (from_net_tx, from_net_rx) = tokio::sync::mpsc::channel(32);

        Ok(Self {
            local_peer_state,
            server_uri,
            incoming: to_net_rx,
            outgoing: from_net_tx,
        })
    }

    pub fn sync(&mut self, document: &mut automerge::Automerge) {}

    // async fn network_task(
    //     &self,
    //     outgoing: &mut tokio::sync::mpsc::Sender<Vec<u8>>,
    //     incoming: tokio::sync::mpsc::Receiver<Vec<u8>>,
    // ) {
    //     let (web_socket, _) = tokio_tungstenite::connect_async(&self.server_uri)
    //         .await
    //         .unwrap();
    //
    //     use futures_util::StreamExt;
    //     let (mut write, mut read) = web_socket.split();
    //
    //     loop {
    //         tokio::select! {
    //             Some(msg) = outgoing.recv() => {
    //                 write.send(Message::Binary(msg.into())).await.unwrap()
    //             },
    //
    //             Some(msg) = read.next() => {
    //                 if let Ok(tokio_tungstenite::tungstenite::Message::Binary(bytes)) = msg {
    //                     incoming.send(bytes.into()).await.unwrap();
    //                 }
    //             }
    //         }
    //     }
    // }
}

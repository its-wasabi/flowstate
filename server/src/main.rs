struct State {
    document: automerge::Automerge,
    sync_event_tx: tokio::sync::broadcast::Sender<()>,
}

impl State {
    fn new() -> Self {
        Self {
            // TODO: Move io module from application into utils and use here load_or_create method
            document: automerge::Automerge::default(),
            sync_event_tx: tokio::sync::broadcast::Sender::new(1),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(clippy::expect_used)]
    let socket: std::net::SocketAddr = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -- <IP:PORT>")
        .parse()?;

    let listener = tokio::net::TcpListener::bind(&socket).await?;
    println!("\x1b[38;5;2m[SERVER]\x1b[0m Listening on: {socket}");

    let state = std::sync::Arc::new(tokio::sync::RwLock::new(State::new()));

    while let Ok((tcp_stream, remote_socket_addr)) = listener.accept().await {
        println!("\x1b[38;5;2m[SERVER]\x1b[0m Connection request from: {remote_socket_addr}");

        if let Err(error) = handle_connection(std::sync::Arc::clone(&state), tcp_stream).await {
            eprintln!("\x1b[38;5;1m[SERVER]\x1b[0m {error}");
        }
    }

    Ok(())
}

async fn handle_connection(
    state: std::sync::Arc<tokio::sync::RwLock<State>>,
    tcp_stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    use futures_util::StreamExt;
    let (mut ws_tx, mut ws_rx) = tokio_tungstenite::accept_async(tcp_stream).await?.split();
    let mut sync_event_rx = state.read().await.sync_event_tx.subscribe();

    let mut sync_state = automerge::sync::State::default();
    send_sync_message(&state, &mut sync_state, &mut ws_tx).await?;

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws_rx.next() => {
                match msg {
                    tokio_tungstenite::tungstenite::Message::Binary(sync_msg) => handle_binary_message(&state,&mut sync_state, &sync_msg, &mut ws_tx).await?,
                    tokio_tungstenite::tungstenite::Message::Text(_) => todo!("Handle future features"),
                    tokio_tungstenite::tungstenite::Message::Close(close_frame) => break,
                    _ => {},
                }
            },

            Ok(()) = sync_event_rx.recv() => {
                send_sync_message(&state, &mut sync_state, &mut ws_tx).await?;
            }
        }
    }

    Ok(())
}

async fn handle_binary_message(
    state: &std::sync::Arc<tokio::sync::RwLock<State>>,
    sync_state: &mut automerge::sync::State,
    sync_msg: &tokio_tungstenite::tungstenite::Bytes,
    ws_tx: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        tokio_tungstenite::tungstenite::Message,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(sync_msg) = automerge::sync::Message::decode(sync_msg) {
        recv_sync_message(state, sync_state, sync_msg).await?;
        send_sync_event(state).await?;
        // TODO: Shouldn't you do that optionally here only if something actually changed
        send_sync_message(state, sync_state, ws_tx).await?;
    }

    Ok(())
}

async fn generate_sync_message(
    state: &std::sync::Arc<tokio::sync::RwLock<State>>,
    sync_state: &mut automerge::sync::State,
) -> Option<automerge::sync::Message> {
    use automerge::sync::SyncDoc;
    let state_guard = state.read().await;
    state_guard.document.generate_sync_message(sync_state)
}

async fn send_sync_message(
    state: &std::sync::Arc<tokio::sync::RwLock<State>>,
    sync_state: &mut automerge::sync::State,
    ws_tx: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        tokio_tungstenite::tungstenite::Message,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    use futures_util::SinkExt;
    if let Some(msg) = generate_sync_message(state, sync_state).await {
        let payload = tokio_tungstenite::tungstenite::Message::Binary(msg.encode().into());
        ws_tx.send(payload).await?;
    }

    Ok(())
}

async fn recv_sync_message(
    state: &std::sync::Arc<tokio::sync::RwLock<State>>,
    sync_state: &mut automerge::sync::State,
    sync_msg: automerge::sync::Message,
) -> Result<(), automerge::AutomergeError> {
    use automerge::sync::SyncDoc;
    let mut state_guard = state.write().await;
    state_guard
        .document
        .receive_sync_message(sync_state, sync_msg)
}

async fn send_sync_event(
    state: &std::sync::Arc<tokio::sync::RwLock<State>>,
) -> Result<usize, tokio::sync::broadcast::error::SendError<()>> {
    let state_guard = state.read().await;
    state_guard.sync_event_tx.send(())
}

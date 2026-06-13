// #[tokio::main]
// async fn main2() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
//     let (tx, _) = tokio::sync::broadcast::channel::<String>(256);
//
//     loop {
//         let (stream, addr) = listener.accept().await?;
//         let tx = tx.clone();
//         let mut rx = tx.subscribe();
//
//         tokio::spawn(async move {
//             let Ok(ws) = tokio_tungstenite::accept_async(stream).await else {
//                 eprintln!("{addr} Handshake failed");
//                 return;
//             };
//
//             use futures_util::StreamExt;
//             let (mut sink, mut source) = ws.split();
//
//             let write = tokio::spawn(async move {
//                 while let Ok(msg) = rx.recv().await {
//                     use futures_util::SinkExt;
//                     if sink
//                         .send(tokio_tungstenite::tungstenite::Message::text(msg))
//                         .await
//                         .is_err()
//                     {
//                         break;
//                     }
//                 }
//             });
//
//             while let Some(Ok(msg)) = source.next().await {
//                 if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
//                     let _ = tx.send(text.to_string());
//                 }
//             }
//             write.abort();
//             eprintln!("{addr} disconnected");
//         });
//     }
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let mut tasks = tokio::task::JoinSet::new();

    loop {
        tokio::select! {
            Ok((stream, addr)) = listener.accept() => {
                use futures_util::StreamExt;
                tasks.spawn(async move {
                    let Ok(ws) = tokio_tungstenite::accept_async(stream).await else {
                        eprintln!("Handshake failed {addr}");
                        return;
                    };

                    let (mut sink, mut source) = ws.split();

                    while let Some(Ok(msg)) = source.next().await {
                        if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {

                            use futures_util::SinkExt;
                            let _ = sink.send(
                                tokio_tungstenite::tungstenite::Message::text(text.to_string())
                            ).await;
                        }
                    }
                    eprintln!("{addr} disconnected");
                });
            }
            _ = tokio::signal::ctrl_c() => {
                eprintln!("shutting down, draining connections");
                break;
            }
        }
    }

    // Wait for all active connections to finish
    while tasks.join_next().await.is_some() {}
    Ok(())
}

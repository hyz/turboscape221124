use std::env;

use futures_util::{future, pin_mut, SinkExt, StreamExt};
use reqwest::StatusCode;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{client_async, connect_async, WebSocketStream};

pub(crate) async fn integration_test() {
    let addr = dotenv::var("URL_WS_SRV").unwrap(); //format!("ws://{addr}/websocket");
    tracing::debug!("URL_WS_SRV {}", addr);

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let (socket, _response) = tokio_tungstenite::connect_async(addr).await.unwrap();
    tracing::debug!("connect_async:response: {:?}", _response);

    let (mut sink, stream) = socket.split();

    sink.send(Message::Text("(test)".to_string()))
        .await
        .unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    // tokio::spawn(read_stdin(stdin_tx));

    let stdin_to_ws = stdin_rx.map(Ok).forward(sink);
    let ws_to_stdout = {
        stream.for_each(|message| async {
            if let Ok(Message::Text(text)) = message {
                tracing::debug!("integration_test rcpt: {}", text);
                tokio::io::stdout()
                    .write_all(text.as_bytes())
                    .await
                    .unwrap();
                tokio::io::stdout().write(b"\n").await.unwrap();
                _ = tokio::io::stdout().flush().await;
            }
            // let data = message.unwrap();//.into_data();
            // tokio::io::stdout().write_all(&data).await.unwrap();
            // tokio::io::stdout().write(b"\n").await.unwrap();
            // _ = tokio::io::stdout().flush().await;
        })
    };

    futures::pin_mut!(stdin_to_ws, ws_to_stdout);
    futures::future::select(stdin_to_ws, ws_to_stdout).await;

    tracing::debug!("integration_test ___ **End**");

    // sink.send(tokio_tungstenite::tungstenite::Message::Text(
    //     "integration_test".to_owned(),
    // ))
    // .await
    // .unwrap();

    // let msg = match stream.next().await.unwrap().unwrap() {
    //     tokio_tungstenite::tungstenite::Message::Text(msg) => msg,
    //     other => panic!("expected a text message but got {other:?}"),
    //     // Message::Text(_) => todo!(),
    //     // Message::Binary(_) => todo!(),
    //     // Message::Ping(_) => todo!(),
    //     // Message::Pong(_) => todo!(),
    //     // Message::Close(_) => todo!(),
    //     // Message::Frame(_) => todo!(),
    // };

    // // assert_eq!(msg, "You said: foo");
}
// async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
//     let mut stdin = tokio::io::stdin();
//     loop {
//         let mut buf = String::new(); //vec![0; 1024];
//         let _n = match stdin.read_to_string(&mut buf).await {
//             Err(_) | Ok(0) => break,
//             Ok(n) => n,
//         };
//         // buf.truncate(n);
//         tx.unbounded_send(Message::Text(buf.trim().to_string()))
//             .unwrap();
//     }
// }

pub async fn ws() {
    dotenv::dotenv().ok();
    let server_url = dotenv::var("SERVER_URL").unwrap();

    let url = url::Url::parse(&server_url).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}

pub async fn test_axum_websocket() -> Result<(), Box<dyn std::error::Error>> {
    let (mut stream, response) = {
        let tcp = tokio::net::TcpStream::connect("u8080.de:80")
            .await
            .expect("Failed to connect");
        let url = url::Url::parse("ws://u8080.de/sock-chat").unwrap();

        client_async(url, tcp)
            .await
            .expect("Client failed to connect")
    };
    assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);

    stream
        .send(Message::Text(
            r#"{"jsonrpc":"2.0","method":"shout","params":["foo"],"id":2}"#.into(),
        ))
        .await?;

    let res = stream.next().await.unwrap().unwrap();
    match res {
        Message::Text(text) => {
            dbg!(Message::Text(text));
            // assert_eq!(text, r#"{"jsonrpc":"2.0","id":2,"result":"FOO"}"#);
        }
        _ => panic!("Received unexepcted message {:?}", res),
    }

    let (client, _on_close) = tungstenite_client(stream);
    let res = client.send("add".to_string()).await?;
    // let res: f32 = serde_json::from_value(res).unwrap();
    // assert_eq!(res, 3.5);
    // let res: String = serde_json::from_value(client.send_request("shout", Some(["foo"])).await?)?;
    // assert_eq!(res.as_str(), "FOO");
    Ok(())
}

// use crate::{OutReceiver, RpcClient, RpcServer, RpcSession};
// use futures_util::{SinkExt, StreamExt};
// use tokio::{
//     io::{AsyncRead, AsyncWrite},
//     sync::oneshot,
// };
// use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

fn tungstenite_client<S>(
    stream: WebSocketStream<S>,
    // service: R,
) -> (
    async_channel::Sender<String>,
    futures_channel::oneshot::Receiver<String>,
)
where
    // R: RpcServer,
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    // let (client, out_rx) = RpcClient::new();
    let (tx_out, out_rx) = async_channel::bounded(10);

    // let session = RpcSession::new(client.clone(), service);
    let (tx, rx) = futures_channel::oneshot::channel();
    tokio::spawn(async move {
        let res = handle_tungstenite(stream, out_rx).await;
        let _ = tx.send("".to_string());
    });
    (tx_out, rx)
}

pub async fn handle_tungstenite<S>(
    mut stream: WebSocketStream<S>,
    mut out_rx: async_channel::Receiver<String>,
    // session: RpcSession<R>,
) -> Result<(), Box<dyn std::error::Error>>
where
    // R: RpcServer,
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    loop {
        tokio::select! {
            message = out_rx.next() => {
                    // let message = serde_json::to_string(&message)?;
                    stream.send(Message::Text(message.unwrap())).await?;
            }
            message = stream.next() => {
                match message {
                    Some(Ok(Message::Text(message))) => {
                        // session.handle_incoming(&message).await;
                    },
                    Some(Ok(Message::Binary(_))) => {
                        return Err("Binary messages are not supported.".into())
                    }
                    Some(Ok(_)) => {}
                    Some(Err(err)) => {
                        return Err(err.into())
                    }
                    None => break,
                }
            }
        }
    }
    Ok(())
}

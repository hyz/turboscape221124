use chrono::offset::Local;
use futures::stream::{SplitSink, SplitStream};
use std::env;
use std::fmt::Debug;
use tauri::{AppHandle, EventLoopMessage, Manager, Wry};
use tokio::sync::broadcast;

use futures_util::{future, pin_mut, SinkExt, StreamExt};
use reqwest::{StatusCode, Url};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{client_async, connect_async, WebSocketStream};

// let addr = dotenv::var("URL_WS_SRV").unwrap(); //format!("ws://{addr}/websocket");
pub(crate) async fn ws_chat_test(
    ws_url: Url,
    sub_rx: broadcast::Receiver<String>,
    _app: AppHandle<Wry>, //fresh: futures_channel::mpsc::UnboundedSender<Fresh>,
) {
    let url = ws_url.as_ref();
    #[cfg(debug_assertions)]
    if let Some(_win) = _app.get_window("main") {
        _win.emit("jedi", Some("console.log('jedi emit-from ws_chat_test')"))
            .unwrap();
        _app.run_on_main_thread(|| {}).unwrap();
    }
    // let ws_url = match url::Url::parse(addr.as_ref()) {
    //     Err(err) => {
    //         tracing::error!("Url::parse {}: {:?}", addr.as_ref(), err);
    //         return;
    //     }
    //     Ok(url) => url,
    // };

    let today = Local::now().format("%Y%h%d.%H %FT%T");
    tracing::debug!("{today} {} ", url);

    let (mut sink, stream) = match tokio_tungstenite::connect_async(url).await {
        Err(err) => {
            tracing::error!("connect_async {}: {:?}", url, err);
            return;
        }
        Ok((socket, _response)) => {
            assert_eq!(_response.status(), StatusCode::SWITCHING_PROTOCOLS);
            tracing::debug!("connect_async:response: {:?}", _response);
            socket.split()
        }
    };

    let name = Local::now().format("___%y%h%d.%H%M");
    sink.send(Message::Text(name.to_string())).await.unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    // tokio::spawn(read_stdin(stdin_tx));

    let stdin_to_ws = stdin_rx.map(Ok).forward(sink);
    let ws_to_stdout = stream.for_each(|message| async {
        //     tokio::io::stdout().write_all(text.as_bytes()).await.unwrap();
        //     tokio::io::stdout().write(b"\n").await.unwrap();
        //     tokio::io::stdout().flush().await.unwrap();
        match message {
            Err(errmsg) => {
                tracing::error!("rcpt Err: {errmsg:?}");
            }
            Ok(Message::Text(text)) => {
                tracing::debug!("rcpt Message::Text: {text}");
            }
            Ok(message) => {
                tracing::debug!("rcpt Message::: {message:?}");
            }
        }
    });

    futures::pin_mut!(stdin_to_ws, ws_to_stdout);
    futures::future::select(stdin_to_ws, ws_to_stdout).await;

    tracing::debug!("ws_chat_test ___ **End**");
    if let Some(_win) = _app.get_window("main") {
        let url = ws_url; //url.to_string();
        #[cfg(debug_assertions)]
        tauri::async_runtime::spawn(async move {
            use crate::{Trigger, EV_TRIGGER};
            let today = Local::now().format("%y%h%d.%H%M %FT%T");
            tracing::debug!("{today} {} {}", _win.label(), url);

            _win.emit("jedi", Some("console.log('___')")).unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let data = Trigger::WebsocketChat { url };
            _win.trigger(EV_TRIGGER, serde_json::to_string(&data).ok());
        });
    }
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

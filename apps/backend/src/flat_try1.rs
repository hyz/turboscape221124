//! Manual implementation of `FromRequest` that wraps another extractor
//!
//! + Powerful API: Implementing `FromRequest` grants access to `RequestParts`
//!   and `async/await`. This means that you can create more powerful rejections
//! - Boilerplate: Requires creating a new extractor for every custom rejection
//! - Complexity: Manually implementing `FromRequest` results on more complex code
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, MatchedPath},
    http::Request,
    http::StatusCode,
    response::IntoResponse,
    Json, RequestPartsExt,
};
use serde_json::{json, Value};

use crate::flatbuf::{construct, Flatbuf};
use crate::protocols::query_generated::query;

pub async fn handler(Json(value): Json<Value>) -> impl IntoResponse {
    Json(dbg!(value));
}

pub async fn first_try(fbuf: Flatbuf<query::Query<'_>>) -> impl IntoResponse {
    let query = construct(&fbuf);
    dbg!(query);

    let (bytes, head) = {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

        let js = r#"console.log"#;
        let arg = "greet";

        let script = Some(builder.create_string(js));
        let input = Some(builder.create_vector(arg.as_bytes()));

        let evals = query::Evals::create(&mut builder, &query::EvalsArgs { script, input });

        builder.finish(evals, None);
        builder.collapse() // builder.finished_data()
    };

    bytes[head..].to_vec()
    // json!(dbg!(""));
}

// async fn request_json_get(url: hyper::Uri) -> Result<GetResult> {
//     let client = Client::new();

//     // Fetch the url...
//     let res = client.get(url).await?;

//     // asynchronously aggregate the chunks of the body
//     let body = hyper::body::aggregate(res).await?;

//     // try to parse as json with serde_json
//     let get_result: GetResult = serde_json::from_reader(body.reader())?;

//     Ok(get_result)
// }

// async fn request_json_post(uri: &str) -> Result<(), Box<dyn std::error::Error>> {
//     use hyper::body::Buf;
//     use hyper::{body::Body, client, Method, Request};
//     use serde::{Deserialize, Serialize};

//     let client = hyper::http::Client::new();

//     let address = Address {
//         street: "10 Downing Street".to_owned(),
//         city: "London".to_owned(),
//     };

//     // Serialize it to a JSON string.
//     let json_body = serde_json::to_vec(&address)?;

//     let req = Request::builder()
//         .method(Method::POST)
//         .uri(uri)
//         .header("X-Custom-Foo", "bar")
//         .body(Body::from(json_body))?;

//     // Fetch the url...
//     let res = client.request(req).await?;

//     // asynchronously aggregate the chunks of the body
//     let body = hyper::body::aggregate(res).await?;

//     // try to parse as json with serde_json
//     let get_result: GetResult = serde_json::from_reader(body.reader())?;

//     Ok(get_result)
// }

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
// use hyper::Request;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;

    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let mut frames = vec![];

    while let Some(next) = res.frame().await {
        let frame = next?;
        frames.push(frame);
    }
    for frame in frames {
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(&chunk).await?;
        }
    }

    println!("\n\nDone!");

    Ok(())
}

// flatbuffers/tests/rust_usage_test/bin/monster_example.rs
// flatbuffers/samples/sample_binary.rs

//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-customize-extractor-error
//! ```

mod custom_extractor;
mod derive_from_request;
mod with_rejection;

mod flatbuf;
mod protocols;

mod flat_try1;

use axum::{routing::post, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "fbuf_error=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with some routes
    let app = Router::new()
        .route("/with-rejection", post(with_rejection::handler))
        .route("/custom-extractor", post(custom_extractor::handler))
        .route("/derive-from-request", post(derive_from_request::handler))
        .route("/v1/an", post(flat_try1::first_try));

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

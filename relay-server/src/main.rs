mod config;
mod crypto;
mod network;
mod relay;
mod api;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;
use utils::logger::setup_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger();
    dotenv::dotenv().ok();

    let config = config::Config::from_env()?;
    tracing::info!("Starting relay server: {}", config.relay_addr);

    // TODO: Initialize database connection pool
    // TODO: Initialize Redis connection
    // TODO: Start gRPC internal service on port 9000

    let app = Router::new()
        .route("/health", get(api::handlers::health))
        .route("/ws", get(network::websocket::ws_handler))
        .route("/api/keys/upload", post(api::handlers::upload_keys))
        .route("/api/messages/queue", post(api::handlers::queue_message))
        .with_state(());

    let addr: SocketAddr = config.relay_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

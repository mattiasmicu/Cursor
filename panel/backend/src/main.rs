use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let app = Router::new().route("/health", get(|| async { "ok" }));
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!("panel listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}

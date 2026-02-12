use axum::{
    routing::{get, get_service},
    Router,
};
use std::{env, net::SocketAddr, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let asset_dir = env::var("ASSET_DIR").unwrap_or_else(|_| "flutter_app/build/web".to_string());
    let asset_dir = PathBuf::from(asset_dir);
    let index_html = asset_dir.join("index.html");

    let static_service = get_service(
        ServeDir::new(&asset_dir).not_found_service(ServeFile::new(index_html)),
    );

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest_service("/", static_service);

    let addr: SocketAddr = env::var("ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
        .parse()
        .expect("Invalid ADDR value");

    tracing::info!("Serving Flutter web build from {:?} on {}", asset_dir, addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}

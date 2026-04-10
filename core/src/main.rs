mod error;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("claw_proxy_core=info".parse().unwrap()))
        .init();

    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yaml"));

    let config = claw_proxy_core::config::load(&config_path)
        .expect("Failed to load config");

    let providers = claw_proxy_core::build_providers(&config);
    let router = Arc::new(claw_proxy_core::router::Router::new(
        providers,
        config.routing.strategy.clone(),
    ));

    // Proxy server state
    let proxy_state = claw_proxy_core::proxy::AppState { router: Arc::clone(&router) };
    let proxy_app = claw_proxy_core::proxy::create_router(proxy_state);
    let proxy_addr = format!("127.0.0.1:{}", config.server.proxy_port);

    tracing::info!("Proxy listening on http://{}", proxy_addr);

    let listener = tokio::net::TcpListener::bind(&proxy_addr).await.unwrap();
    axum::serve(listener, proxy_app).await.unwrap();
}
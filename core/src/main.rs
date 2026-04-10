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

    let (log_tx, _) = broadcast::channel(256);

    // Proxy server
    let proxy_state = claw_proxy_core::proxy::AppState { router: Arc::clone(&router) };
    let proxy_app = claw_proxy_core::proxy::create_router(proxy_state);
    let proxy_addr = format!("127.0.0.1:{}", config.server.proxy_port);

    // Admin server
    let admin_state = claw_proxy_core::admin::AdminState {
        router: Arc::clone(&router),
        log_tx,
        start_time: std::time::Instant::now(),
        proxy_port: config.server.proxy_port,
        admin_port: config.server.admin_port,
    };
    let admin_app = claw_proxy_core::admin::create_router(admin_state);
    let admin_addr = format!("127.0.0.1:{}", config.server.admin_port);

    tracing::info!("Proxy  → http://{}", proxy_addr);
    tracing::info!("Admin  → http://{}", admin_addr);

    let proxy_listener = tokio::net::TcpListener::bind(&proxy_addr).await.unwrap();
    let admin_listener = tokio::net::TcpListener::bind(&admin_addr).await.unwrap();

    tokio::join!(
        axum::serve(proxy_listener, proxy_app),
        axum::serve(admin_listener, admin_app),
    );
}

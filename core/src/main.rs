use std::{io, path::PathBuf, sync::Arc};

use claw_proxy_core::config::ProviderType;
use claw_proxy_core::error::{AppError, Result};
use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("claw_proxy_core=info".parse().unwrap()))
        .init();

    if let Err(error) = run().await {
        tracing::error!(reason = %sanitize_startup_error(&error), "claw-proxy startup failed");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yaml"));

    let config = claw_proxy_core::config::load(&config_path)
        .map_err(|_| AppError::Config("configuration could not be loaded".to_string()))?;

    let providers = claw_proxy_core::build_providers(&config);
    let router = Arc::new(claw_proxy_core::router::Router::new(
        providers,
        config.routing.strategy.clone(),
    ));
    let anthropic_alias_model = config
        .providers
        .iter()
        .find_map(|provider| {
            (provider.provider_type == ProviderType::OpenAI)
                .then(|| provider.models.first().cloned())
                .flatten()
        })
        .unwrap_or_default();

    let (log_tx, _) = broadcast::channel(256);

    // Proxy server
    let proxy_state = claw_proxy_core::proxy::AppState {
        router: Arc::clone(&router),
        anthropic_alias_model,
    };
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

    let proxy_listener = tokio::net::TcpListener::bind(&proxy_addr)
        .await
        .map_err(|_| io_error("proxy listener could not be started"))?;
    let admin_listener = tokio::net::TcpListener::bind(&admin_addr)
        .await
        .map_err(|_| io_error("admin listener could not be started"))?;

    tokio::try_join!(
        axum::serve(proxy_listener, proxy_app),
        axum::serve(admin_listener, admin_app),
    )
    .map_err(|_| io_error("server exited unexpectedly"))?;

    Ok(())
}

fn io_error(message: &'static str) -> AppError {
    AppError::Io(io::Error::new(io::ErrorKind::Other, message))
}

fn sanitize_startup_error(error: &AppError) -> &'static str {
    match error {
        AppError::Config(_) => "configuration could not be loaded",
        AppError::Io(_) => "local listeners could not be started",
        AppError::Provider(_) => "provider initialization failed",
        AppError::NoProviders => "no providers were configured for startup",
        AppError::Normalize(_) => "startup failed during request normalization setup",
    }
}

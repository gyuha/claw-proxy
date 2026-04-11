use std::fmt;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::models::proxy_settings::{ProxySettings, ProxySettingsValidationError};

pub struct ProxyRuntimeSupervisor {
    inner: Mutex<ProxyRuntimeTaskState>,
}

struct ProxyRuntimeTaskState {
    cancellation_token: Option<CancellationToken>,
    task: Option<JoinHandle<()>>,
}

impl ProxyRuntimeSupervisor {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(ProxyRuntimeTaskState {
                cancellation_token: None,
                task: None,
            }),
        }
    }

    pub async fn start(&self, settings: ProxySettings) -> Result<(), ProxyRuntimeError> {
        let normalized = settings.normalized()?;
        let _health_url = proxy_health_url(&normalized)?;
        let listener = TcpListener::bind(proxy_bind_address(&normalized))
            .await
            .map_err(ProxyRuntimeError::Bind)?;
        let health_route = proxy_health_route(&normalized);
        let app = Router::new().route(health_route.as_str(), get(proxy_health_handler));
        let cancellation_token = CancellationToken::new();
        let shutdown_token = cancellation_token.clone();
        let server = axum::serve(listener, app).with_graceful_shutdown(async move {
            shutdown_token.cancelled().await;
        });

        let task = tokio::spawn(async move {
            let _ = server.await;
        });

        let mut guard = self.inner.lock().await;
        guard.cancellation_token = Some(cancellation_token);
        guard.task = Some(task);

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), ProxyRuntimeError> {
        let (cancellation_token, task) = {
            let mut guard = self.inner.lock().await;
            (guard.cancellation_token.take(), guard.task.take())
        };

        if let Some(token) = cancellation_token {
            token.cancel();
        }

        if let Some(task) = task {
            task.await.map_err(ProxyRuntimeError::Join)?;
        }

        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        let guard = self.inner.lock().await;

        guard
            .task
            .as_ref()
            .is_some_and(|task| !task.is_finished())
    }
}

impl Default for ProxyRuntimeSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ProxyRuntimeError {
    Settings(ProxySettingsValidationError),
    Bind(std::io::Error),
    Join(tokio::task::JoinError),
    Url(url::ParseError),
}

impl fmt::Display for ProxyRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Settings(error) => error.fmt(f),
            Self::Bind(error) => write!(f, "failed to bind proxy runtime: {error}"),
            Self::Join(error) => write!(f, "failed to join proxy runtime task: {error}"),
            Self::Url(error) => write!(f, "failed to compose proxy health url: {error}"),
        }
    }
}

impl std::error::Error for ProxyRuntimeError {}

impl From<ProxySettingsValidationError> for ProxyRuntimeError {
    fn from(value: ProxySettingsValidationError) -> Self {
        Self::Settings(value)
    }
}

impl From<url::ParseError> for ProxyRuntimeError {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}

fn proxy_bind_address(settings: &ProxySettings) -> String {
    match settings.listen_host.as_str() {
        "::1" => format!("[::1]:{}", settings.listen_port),
        _ => format!("{}:{}", settings.listen_host, settings.listen_port),
    }
}

fn proxy_health_route(settings: &ProxySettings) -> String {
    let base_endpoint = settings.base_endpoint.trim_end_matches('/');
    if base_endpoint.is_empty() {
        "/health".to_string()
    } else {
        format!("{base_endpoint}/health")
    }
}

fn proxy_health_url(settings: &ProxySettings) -> Result<Url, ProxyRuntimeError> {
    let mut url = Url::parse(&settings.effective_base_url()?)?;
    let health_route = proxy_health_route(settings);
    url.set_path(&health_route);

    Ok(url)
}

async fn proxy_health_handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
    }))
}

#[cfg(test)]
mod tests {
    use super::{proxy_health_route, proxy_health_url};
    use crate::models::proxy_settings::ProxySettings;

    #[test]
    fn proxy_runtime_health_route_extends_base_endpoint() {
        let settings = ProxySettings::default();

        assert_eq!(proxy_health_route(&settings), "/v1/health");
        assert_eq!(
            proxy_health_url(&settings)
                .expect("health url composes from default settings")
                .as_str(),
            "http://127.0.0.1:8787/v1/health"
        );
    }
}

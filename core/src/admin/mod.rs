use axum::{Router, routing::{get, post, delete}};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::router::Router as ProxyRouter;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub mod handlers;
pub mod ws;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
    pub model: String,
    pub provider: String,
    pub source_format: String,
    pub status_code: u16,
    pub latency_ms: u64,
}

#[derive(Clone)]
pub struct AdminState {
    pub router: Arc<ProxyRouter>,
    pub log_tx: broadcast::Sender<LogEntry>,
    pub start_time: std::time::Instant,
}

pub fn create_router(state: AdminState) -> Router {
    Router::new()
        .route("/status", get(handlers::status))
        .route("/providers", get(handlers::list_providers))
        .route("/stats", get(handlers::stats))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
}
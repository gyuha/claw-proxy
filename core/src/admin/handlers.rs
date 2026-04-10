use axum::{extract::State, Json};
use serde::Serialize;
use super::AdminState;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub uptime_secs: u64,
    pub proxy_port: u16,
    pub admin_port: u16,
}

pub async fn status(State(state): State<AdminState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
        uptime_secs: state.start_time.elapsed().as_secs(),
        proxy_port: 47380,
        admin_port: 47381,
    })
}

#[derive(Serialize)]
pub struct ProviderInfo {
    pub name: String,
    pub available: bool,
}

pub async fn list_providers(State(state): State<AdminState>) -> Json<Vec<ProviderInfo>> {
    let providers = state.router.providers().iter().map(|p| ProviderInfo {
        name: p.name().to_string(),
        available: p.is_available(),
    }).collect();
    Json(providers)
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub provider_count: usize,
}

pub async fn stats(State(state): State<AdminState>) -> Json<StatsResponse> {
    Json(StatsResponse {
        provider_count: state.router.providers().len(),
    })
}
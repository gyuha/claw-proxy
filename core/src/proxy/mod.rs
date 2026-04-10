use axum::{
    Router,
    routing::post,
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::router::Router as ProxyRouter;
use crate::normalizer::{
    InternalRequest,
    from_openai::OpenAIRequest,
    from_anthropic::AnthropicRequest,
    to_openai, to_anthropic,
};
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub router: Arc<ProxyRouter>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/chat/completions", post(handle_openai))
        .route("/v1/messages", post(handle_anthropic))
        .with_state(state)
}

async fn handle_openai(
    State(state): State<AppState>,
    Json(req): Json<OpenAIRequest>,
) -> impl IntoResponse {
    let internal = match InternalRequest::from_openai(req) {
        Ok(internal) => internal,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": error.to_string()})),
            )
                .into_response();
        }
    };
    match dispatch(state, internal).await {
        Ok(resp) => {
            let openai_resp = to_openai::convert(resp);
            (StatusCode::OK, Json(openai_resp)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    }
}

async fn handle_anthropic(
    State(state): State<AppState>,
    Json(req): Json<AnthropicRequest>,
) -> impl IntoResponse {
    let internal = match InternalRequest::from_anthropic(req) {
        Ok(internal) => internal,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": error.to_string()})),
            )
                .into_response();
        }
    };
    match dispatch(state, internal).await {
        Ok(resp) => {
            let anthropic_resp = to_anthropic::convert(resp);
            (StatusCode::OK, Json(anthropic_resp)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    }
}

async fn dispatch(
    state: AppState,
    request: InternalRequest,
) -> Result<crate::normalizer::InternalResponse, AppError> {
    let provider = state.router.next_provider()?;
    provider.chat_completion(request).await
}

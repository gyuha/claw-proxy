use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use crate::error::AppError;
use crate::normalizer::{
    from_anthropic::AnthropicRequest,
    from_openai::OpenAIRequest,
    to_anthropic, to_openai, InternalRequest,
};
use crate::router::Router as ProxyRouter;

#[derive(Clone)]
pub struct AppState {
    pub router: Arc<ProxyRouter>,
    pub anthropic_alias_model: String,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/chat/completions", post(handle_openai))
        .route("/v1/messages", post(handle_anthropic))
        .route("/v1/messages/count_tokens", post(handle_anthropic_count_tokens))
        .with_state(state)
}

async fn handle_openai(
    State(state): State<AppState>,
    Json(req): Json<OpenAIRequest>,
) -> impl IntoResponse {
    let internal = match InternalRequest::from_openai(req) {
        Ok(internal) => internal,
        Err(error) => return openai_bad_request(normalize_message(&error)),
    };
    match dispatch(state, internal).await {
        Ok(resp) => {
            let openai_resp = to_openai::convert(resp);
            (StatusCode::OK, Json(openai_resp)).into_response()
        }
        Err(AppError::Normalize(message)) => openai_bad_request(message),
        Err(error) => internal_error(error),
    }
}

async fn handle_anthropic(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AnthropicRequest>,
) -> impl IntoResponse {
    if let Err(error) = validate_anthropic_headers(&headers) {
        return anthropic_bad_request(normalize_message(&error));
    }

    let internal = match InternalRequest::from_anthropic(req) {
        Ok(internal) => internal,
        Err(error) => return anthropic_bad_request(normalize_message(&error)),
    };
    let internal = match alias_anthropic_model(&state, internal) {
        Ok(internal) => internal,
        Err(error) => return anthropic_bad_request(normalize_message(&error)),
    };

    match dispatch(state, internal).await {
        Ok(resp) => {
            let anthropic_resp = to_anthropic::convert(resp);
            (StatusCode::OK, Json(anthropic_resp)).into_response()
        }
        Err(AppError::Normalize(message)) => anthropic_bad_request(message),
        Err(error) => internal_error(error),
    }
}

async fn handle_anthropic_count_tokens(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AnthropicRequest>,
) -> impl IntoResponse {
    if let Err(error) = validate_anthropic_headers(&headers) {
        return anthropic_bad_request(normalize_message(&error));
    }

    let _anthropic_beta = headers
        .get("anthropic-beta")
        .and_then(|value| value.to_str().ok());

    let internal = match InternalRequest::from_anthropic(req) {
        Ok(internal) => internal,
        Err(error) => return anthropic_bad_request(normalize_message(&error)),
    };

    (StatusCode::OK, Json(to_anthropic::count(&internal))).into_response()
}

async fn dispatch(
    state: AppState,
    request: InternalRequest,
) -> Result<crate::normalizer::InternalResponse, AppError> {
    let provider = state.router.next_provider()?;
    provider.chat_completion(request).await
}

fn validate_anthropic_headers(headers: &HeaderMap) -> Result<(), AppError> {
    let anthropic_version = headers
        .get("anthropic-version")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty());

    let _anthropic_beta = headers
        .get("anthropic-beta")
        .and_then(|value| value.to_str().ok());

    if anthropic_version.is_none() {
        return Err(AppError::Normalize(
            "missing anthropic-version header".to_string(),
        ));
    }

    Ok(())
}

fn alias_anthropic_model(
    state: &AppState,
    mut request: InternalRequest,
) -> Result<InternalRequest, AppError> {
    if is_claude_facing_model(&request.model) {
        if state.anthropic_alias_model.is_empty() {
            return Err(AppError::Normalize(
                "no OpenAI model configured for Anthropic aliasing".to_string(),
            ));
        }
        request.model = state.anthropic_alias_model.clone();
    }

    Ok(request)
}

fn is_claude_facing_model(model: &str) -> bool {
    let lowercase = model.to_ascii_lowercase();
    lowercase.starts_with("claude-")
        || lowercase.contains("sonnet")
        || lowercase.contains("haiku")
        || lowercase.contains("opus")
}

fn normalize_message(error: &AppError) -> String {
    match error {
        AppError::Normalize(message)
        | AppError::Config(message)
        | AppError::Provider(message) => message.clone(),
        AppError::NoProviders => "no available providers".to_string(),
        AppError::Io(_) => "request could not be processed".to_string(),
    }
}

fn openai_bad_request(message: String) -> axum::response::Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "error": {
                "message": message,
                "type": "invalid_request_error"
            }
        })),
    )
        .into_response()
}

fn anthropic_bad_request(message: String) -> axum::response::Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "type": "error",
            "error": {
                "type": "invalid_request_error",
                "message": message
            }
        })),
    )
        .into_response()
}

fn internal_error(error: AppError) -> axum::response::Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": normalize_message(&error)})),
    )
        .into_response()
}

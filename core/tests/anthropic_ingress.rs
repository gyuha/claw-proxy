use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use claw_proxy_core::{
    config::RoutingStrategy,
    error::AppError,
    normalizer::{InternalRequest, InternalResponse},
    providers::Provider,
    proxy::{create_router, AppState},
    router::Router as ProxyRouter,
};
use tower::ServiceExt;

#[derive(Clone, Default)]
struct Capture {
    requests: Arc<Mutex<Vec<InternalRequest>>>,
}

struct MockProvider {
    capture: Capture,
}

#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        "mock-anthropic"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn supports_model(&self, _model: &str) -> bool {
        true
    }

    async fn chat_completion(
        &self,
        request: InternalRequest,
    ) -> Result<InternalResponse, AppError> {
        self.capture
            .requests
            .lock()
            .expect("capture")
            .push(request.clone());

        Ok(InternalResponse {
            id: "msg_test".to_string(),
            model: request.model,
            content: "anthropic ok".to_string(),
            input_tokens: 5,
            output_tokens: 7,
        })
    }
}

#[tokio::test]
async fn anthropic_messages_roundtrip() {
    let capture = Capture::default();
    let router = Arc::new(ProxyRouter::new(
        vec![Box::new(MockProvider {
            capture: capture.clone(),
        })],
        RoutingStrategy::RoundRobin,
    ));
    let app = create_router(AppState {
        router,
        anthropic_alias_model: "gpt-4o".to_string(),
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/messages")
                .header("content-type", "application/json")
                .header("anthropic-version", "2023-06-01")
                .body(Body::from(
                    serde_json::json!({
                        "model": "claude-3-5-sonnet-latest",
                        "max_tokens": 128,
                        "messages": [{"role": "user", "content": "hello anthropic"}]
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let payload: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(payload["type"], "message");
    assert_eq!(payload["content"][0]["text"], "anthropic ok");

    let recorded = capture.requests.lock().expect("capture");
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].model, "gpt-4o");
}

#[tokio::test]
async fn anthropic_gateway_contract() {
    let capture = Capture::default();
    let router = Arc::new(ProxyRouter::new(
        vec![Box::new(MockProvider {
            capture: capture.clone(),
        })],
        RoutingStrategy::RoundRobin,
    ));
    let app = create_router(AppState {
        router,
        anthropic_alias_model: "gpt-4o".to_string(),
    });

    let messages_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/messages")
                .header("content-type", "application/json")
                .header("anthropic-version", "2023-06-01")
                .header("anthropic-beta", "tools-2024-04-04")
                .body(Body::from(
                    serde_json::json!({
                        "model": "claude-3-5-sonnet-latest",
                        "max_tokens": 128,
                        "messages": [{"role": "user", "content": "count these words"}]
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("messages response");

    assert_eq!(messages_response.status(), StatusCode::OK);

    let count_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/messages/count_tokens")
                .header("content-type", "application/json")
                .header("anthropic-version", "2023-06-01")
                .header("anthropic-beta", "tools-2024-04-04")
                .body(Body::from(
                    serde_json::json!({
                        "model": "claude-3-5-sonnet-latest",
                        "max_tokens": 128,
                        "messages": [{"role": "user", "content": "count these words"}]
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("count response");

    assert_eq!(count_response.status(), StatusCode::OK);
    let count_body = to_bytes(count_response.into_body(), usize::MAX)
        .await
        .expect("body");
    let count_payload: serde_json::Value =
        serde_json::from_slice(&count_body).expect("json");
    assert_eq!(count_payload["input_tokens"], 3);

    let missing_header_response = create_router(AppState {
        router: Arc::new(ProxyRouter::new(
            vec![Box::new(MockProvider { capture })],
            RoutingStrategy::RoundRobin,
        )),
        anthropic_alias_model: "gpt-4o".to_string(),
    })
    .oneshot(
        Request::builder()
            .method("POST")
            .uri("/v1/messages")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "model": "claude-3-5-sonnet-latest",
                    "max_tokens": 128,
                    "messages": [{"role": "user", "content": "missing header"}]
                })
                .to_string(),
            ))
            .expect("request"),
    )
    .await
    .expect("missing header response");

    assert_eq!(missing_header_response.status(), StatusCode::BAD_REQUEST);
    let missing_header_body = to_bytes(missing_header_response.into_body(), usize::MAX)
        .await
        .expect("body");
    let missing_header_payload: serde_json::Value =
        serde_json::from_slice(&missing_header_body).expect("json");
    assert_eq!(missing_header_payload["type"], "error");
    assert_eq!(
        missing_header_payload["error"]["type"],
        "invalid_request_error"
    );
}

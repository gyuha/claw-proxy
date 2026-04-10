use std::sync::Arc;

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

struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        "mock-openai"
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
        Ok(InternalResponse {
            id: "chatcmpl-test".to_string(),
            model: request.model,
            content: request
                .messages
                .last()
                .map(|message| format!("echo: {}", message.content))
                .unwrap_or_else(|| "echo: ".to_string()),
            input_tokens: 3,
            output_tokens: 2,
        })
    }
}

#[tokio::test]
async fn openai_chat_completion_roundtrip() {
    let router = Arc::new(ProxyRouter::new(
        vec![Box::new(MockProvider)],
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
                .uri("/v1/chat/completions")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "model": "gpt-4o",
                        "messages": [
                            {"role": "user", "content": "Hello from OpenAI"}
                        ],
                        "stream": false
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

    assert_eq!(payload["object"], "chat.completion");
    assert_eq!(payload["choices"][0]["message"]["content"], "echo: Hello from OpenAI");
    assert_eq!(payload["choices"][0]["message"]["role"], "assistant");
}

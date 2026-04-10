use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use claw_proxy_core::{
    error::AppError,
    normalizer::{ApiFormat, InternalRequest, Message, Role},
    providers::{openai::OpenAIProvider, Provider},
};
use tokio::{net::TcpListener, sync::oneshot};

#[derive(Clone)]
struct MockOpenAIState {
    request_count: Arc<AtomicUsize>,
}

#[tokio::test]
async fn openai_provider_mock_upstream() {
    let state = MockOpenAIState {
        request_count: Arc::new(AtomicUsize::new(0)),
    };
    let app = Router::new()
        .route("/v1/chat/completions", post(mock_openai_chat))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock upstream");
    let base_url = format!(
        "http://{}",
        listener.local_addr().expect("mock upstream address")
    );
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("serve mock upstream");
    });

    let provider = OpenAIProvider::new_with_base_url(
        "mock-openai".to_string(),
        "sk-test".to_string(),
        vec!["gpt-4o-mini".to_string()],
        base_url,
    );

    let success = provider
        .chat_completion(canonical_request())
        .await
        .expect("successful completion");
    assert_eq!(success.id, "chatcmpl_mock_success");
    assert_eq!(success.model, "gpt-4o-mini-upstream");
    assert_eq!(success.content, "mock upstream reply");
    assert_eq!(success.input_tokens, 11);
    assert_eq!(success.output_tokens, 7);

    let failure = provider
        .chat_completion(canonical_request())
        .await
        .expect_err("expected sanitized provider failure");
    match failure {
        AppError::Provider(message) => {
            assert!(message.contains("502"));
            assert!(!message.contains("raw upstream body"));
            assert!(!message.contains("sk-live-secret"));
        }
        other => panic!("unexpected error: {other:?}"),
    }

    let _ = shutdown_tx.send(());
    server.await.expect("mock upstream shutdown");
}

fn canonical_request() -> InternalRequest {
    InternalRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![Message {
            role: Role::User,
            content: "hello mock upstream".to_string(),
        }],
        stream: false,
        max_tokens: Some(32),
        temperature: Some(0.2),
        source_format: ApiFormat::OpenAI,
    }
}

async fn mock_openai_chat(
    State(state): State<MockOpenAIState>,
) -> Response {
    let request_index = state.request_count.fetch_add(1, Ordering::SeqCst);

    if request_index == 0 {
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": "chatcmpl_mock_success",
                "model": "gpt-4o-mini-upstream",
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "mock upstream reply"
                        }
                    }
                ],
                "usage": {
                    "prompt_tokens": 11,
                    "completion_tokens": 7
                }
            })),
        )
            .into_response();
    }

    (
        StatusCode::BAD_GATEWAY,
        "raw upstream body with sk-live-secret should not escape",
    )
        .into_response()
}

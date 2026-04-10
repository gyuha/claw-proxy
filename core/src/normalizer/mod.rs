// core/src/normalizer/mod.rs
use serde::{Deserialize, Serialize};
use crate::error::AppError;

pub mod from_openai;
pub mod from_anthropic;
pub mod to_openai;
pub mod to_anthropic;

pub use from_openai::OpenAIRequest;
pub use from_anthropic::AnthropicRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiFormat {
    OpenAI,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct InternalRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub source_format: ApiFormat,
}

impl InternalRequest {
    pub fn from_openai(req: OpenAIRequest) -> std::result::Result<Self, AppError> {
        from_openai::convert(req)
    }

    pub fn from_anthropic(req: AnthropicRequest) -> std::result::Result<Self, AppError> {
        from_anthropic::convert(req)
    }
}

#[derive(Debug, Clone)]
pub struct InternalResponse {
    pub id: String,
    pub model: String,
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_openai_request() {
        let json = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": "Hello"}],
            "stream": false
        });
        let req: OpenAIRequest = serde_json::from_value(json).unwrap();
        let internal = InternalRequest::from_openai(req).unwrap();
        assert_eq!(internal.model, "gpt-4o");
        assert_eq!(internal.messages.len(), 1);
        assert!(matches!(internal.source_format, ApiFormat::OpenAI));
    }

    #[test]
    fn test_from_anthropic_request() {
        let json = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": "Hello"}]
        });
        let req: AnthropicRequest = serde_json::from_value(json).unwrap();
        let internal = InternalRequest::from_anthropic(req).unwrap();
        assert_eq!(internal.model, "claude-3-5-sonnet-20241022");
        assert!(matches!(internal.source_format, ApiFormat::Anthropic));
    }

    #[test]
    fn rejects_openai_stream_true() {
        let request = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": true
        });
        let req: OpenAIRequest = serde_json::from_value(request).unwrap();
        let error = InternalRequest::from_openai(req).unwrap_err();
        assert!(matches!(error, AppError::Normalize(message) if message.contains("unsupported stream")));
    }

    #[test]
    fn rejects_openai_developer_role() {
        let request = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{"role": "developer", "content": "do not compile"}],
            "stream": false
        });
        let req: OpenAIRequest = serde_json::from_value(request).unwrap();
        let error = InternalRequest::from_openai(req).unwrap_err();
        assert!(matches!(error, AppError::Normalize(message) if message.contains("unsupported OpenAI role")));
    }

    #[test]
    fn rejects_openai_content_array() {
        let request = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{
                "role": "user",
                "content": [{"type": "text", "text": "content array"}]
            }],
            "stream": false
        });
        let req: OpenAIRequest = serde_json::from_value(request).unwrap();
        let error = InternalRequest::from_openai(req).unwrap_err();
        assert!(matches!(error, AppError::Normalize(message) if message.contains("content array")));
    }

    #[test]
    fn rejects_anthropic_non_text_content_blocks() {
        let request = serde_json::json!({
            "model": "claude-3-5-sonnet-latest",
            "max_tokens": 128,
            "messages": [{
                "role": "user",
                "content": [{"type": "image", "source": {"type": "base64"}}]
            }]
        });
        let req: AnthropicRequest = serde_json::from_value(request).unwrap();
        let error = InternalRequest::from_anthropic(req).unwrap_err();
        assert!(matches!(error, AppError::Normalize(message) if message.contains("non-text")));
    }

    #[test]
    fn rejects_tool_fields() {
        let request = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": "use a tool"}],
            "tools": [{"type": "function", "function": {"name": "tool"}}],
            "tool_choice": "auto"
        });
        let req: OpenAIRequest = serde_json::from_value(request).unwrap();
        let error = InternalRequest::from_openai(req).unwrap_err();
        assert!(matches!(error, AppError::Normalize(message) if message.contains("tool")));
    }
}

// core/src/normalizer/to_anthropic.rs
use serde::Serialize;
use super::{InternalRequest, InternalResponse};

#[derive(Debug, Serialize)]
pub struct AnthropicResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: String,
    pub model: String,
    pub content: Vec<AnthropicContent>,
    pub usage: AnthropicUsage,
    pub stop_reason: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct CountTokensResponse {
    pub input_tokens: u32,
}

pub fn convert(resp: InternalResponse) -> AnthropicResponse {
    AnthropicResponse {
        id: resp.id.clone(),
        response_type: "message".to_string(),
        role: "assistant".to_string(),
        model: resp.model.clone(),
        content: vec![AnthropicContent {
            content_type: "text".to_string(),
            text: resp.content.clone(),
        }],
        usage: AnthropicUsage {
            input_tokens: resp.input_tokens,
            output_tokens: resp.output_tokens,
        },
        stop_reason: "end_turn".to_string(),
    }
}

pub fn count(request: &InternalRequest) -> CountTokensResponse {
    let normalized_text = request
        .messages
        .iter()
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    CountTokensResponse {
        input_tokens: normalized_text.split_whitespace().count() as u32,
    }
}

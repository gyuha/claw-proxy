// core/src/normalizer/from_openai.rs
use serde::{Deserialize, Serialize};
use super::{InternalRequest, Message, Role, ApiFormat};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    #[serde(default)]
    pub stream: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

pub fn convert(req: OpenAIRequest) -> InternalRequest {
    let messages = req.messages.into_iter().map(|m| Message {
        role: match m.role.as_str() {
            "system" => Role::System,
            "assistant" => Role::Assistant,
            _ => Role::User,
        },
        content: m.content,
    }).collect();

    InternalRequest {
        model: req.model,
        messages,
        stream: req.stream,
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        source_format: ApiFormat::OpenAI,
    }
}

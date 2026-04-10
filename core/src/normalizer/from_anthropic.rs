// core/src/normalizer/from_anthropic.rs
use serde::{Deserialize, Serialize};
use super::{InternalRequest, Message, Role, ApiFormat};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicRequest {
    pub model: String,
    pub messages: Vec<AnthropicMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    #[serde(default)]
    pub stream: bool,
    pub system: Option<String>,
}

pub fn convert(req: AnthropicRequest) -> InternalRequest {
    let mut messages: Vec<Message> = Vec::new();

    if let Some(system) = req.system {
        messages.push(Message { role: Role::System, content: system });
    }

    for m in req.messages {
        messages.push(Message {
            role: match m.role.as_str() {
                "assistant" => Role::Assistant,
                _ => Role::User,
            },
            content: m.content,
        });
    }

    InternalRequest {
        model: req.model,
        messages,
        stream: req.stream,
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        source_format: ApiFormat::Anthropic,
    }
}

// core/src/normalizer/from_anthropic.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::AppError;
use super::{ApiFormat, InternalRequest, Message, Role};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicContent {
    Text(String),
    Blocks(Vec<AnthropicContentBlock>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: AnthropicContent,
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
    #[serde(default)]
    pub tools: Option<Value>,
    #[serde(default)]
    pub tool_choice: Option<Value>,
}

pub fn convert(req: AnthropicRequest) -> std::result::Result<InternalRequest, AppError> {
    if req.stream {
        return Err(AppError::Normalize("unsupported stream request".to_string()));
    }
    if req.tools.is_some() || req.tool_choice.is_some() {
        return Err(AppError::Normalize(
            "unsupported tool-bearing payload fields".to_string(),
        ));
    }

    let mut messages: Vec<Message> = Vec::new();

    if let Some(system) = req.system {
        messages.push(Message {
            role: Role::System,
            content: system,
        });
    }

    for message in req.messages {
        let role = match message.role.as_str() {
            "user" => Role::User,
            "assistant" => Role::Assistant,
            other => {
                return Err(AppError::Normalize(format!(
                    "unsupported Anthropic role: {other}"
                )))
            }
        };

        let content = match message.content {
            AnthropicContent::Text(text) => text,
            AnthropicContent::Blocks(blocks) => flatten_text_blocks(blocks)?,
        };

        messages.push(Message {
            role,
            content,
        });
    }

    Ok(InternalRequest {
        model: req.model,
        messages,
        stream: req.stream,
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        source_format: ApiFormat::Anthropic,
    })
}

fn flatten_text_blocks(
    blocks: Vec<AnthropicContentBlock>,
) -> std::result::Result<String, AppError> {
    let mut text_segments = Vec::with_capacity(blocks.len());

    for block in blocks {
        if block.block_type != "text" {
            return Err(AppError::Normalize(format!(
                "unsupported Anthropic non-text content block: {}",
                block.block_type
            )));
        }

        let text = block.text.ok_or_else(|| {
            AppError::Normalize("unsupported Anthropic text block without text".to_string())
        })?;
        text_segments.push(text);
    }

    Ok(text_segments.join("\n"))
}

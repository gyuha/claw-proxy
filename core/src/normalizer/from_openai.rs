// core/src/normalizer/from_openai.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::AppError;
use super::{ApiFormat, InternalRequest, Message, Role};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OpenAIContent {
    Text(String),
    Parts(Vec<Value>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: OpenAIContent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    #[serde(default)]
    pub stream: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    #[serde(default)]
    pub tools: Option<Value>,
    #[serde(default)]
    pub tool_choice: Option<Value>,
    #[serde(default)]
    pub function_call: Option<Value>,
}

pub fn convert(req: OpenAIRequest) -> std::result::Result<InternalRequest, AppError> {
    if req.stream {
        return Err(AppError::Normalize("unsupported stream request".to_string()));
    }
    if req.tools.is_some() || req.tool_choice.is_some() || req.function_call.is_some() {
        return Err(AppError::Normalize(
            "unsupported tool-bearing payload fields".to_string(),
        ));
    }

    let messages = req
        .messages
        .into_iter()
        .map(|message| {
            let role = match message.role.as_str() {
                "system" => Role::System,
                "user" => Role::User,
                "assistant" => Role::Assistant,
                other => {
                    return Err(AppError::Normalize(format!(
                        "unsupported OpenAI role: {other}"
                    )))
                }
            };

            let content = match message.content {
                OpenAIContent::Text(text) => text,
                OpenAIContent::Parts(_) => {
                    return Err(AppError::Normalize(
                        "unsupported OpenAI content array".to_string(),
                    ))
                }
            };

            Ok(Message { role, content })
        })
        .collect::<std::result::Result<Vec<_>, AppError>>()?;

    Ok(InternalRequest {
        model: req.model,
        messages,
        stream: req.stream,
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        source_format: ApiFormat::OpenAI,
    })
}

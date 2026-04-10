// core/src/normalizer/to_openai.rs
use serde::Serialize;
use super::InternalResponse;

#[derive(Debug, Serialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub choices: Vec<OpenAIChoice>,
    pub usage: OpenAIUsage,
}

#[derive(Debug, Serialize)]
pub struct OpenAIChoice {
    pub index: u32,
    pub message: OpenAIChoiceMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
pub struct OpenAIChoiceMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

pub fn convert(resp: InternalResponse) -> OpenAIResponse {
    OpenAIResponse {
        id: resp.id.clone(),
        object: "chat.completion".to_string(),
        model: resp.model.clone(),
        choices: vec![OpenAIChoice {
            index: 0,
            message: OpenAIChoiceMessage {
                role: "assistant".to_string(),
                content: resp.content.clone(),
            },
            finish_reason: "stop".to_string(),
        }],
        usage: OpenAIUsage {
            prompt_tokens: resp.input_tokens,
            completion_tokens: resp.output_tokens,
            total_tokens: resp.input_tokens + resp.output_tokens,
        },
    }
}

use async_trait::async_trait;
use crate::normalizer::{InternalRequest, InternalResponse, Role};
use crate::normalizer::from_openai::{OpenAIRequest, OpenAIMessage};
use crate::error::AppError;
use super::Provider;
use uuid::Uuid;

pub struct OpenAIProvider {
    name: String,
    api_key: String,
    models: Vec<String>,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(name: String, api_key: String, models: Vec<String>) -> Self {
        Self {
            name,
            api_key,
            models,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn supports_model(&self, model: &str) -> bool {
        self.models.iter().any(|m| m == model)
    }

    async fn chat_completion(
        &self,
        request: InternalRequest,
    ) -> Result<InternalResponse, AppError> {
        let messages: Vec<OpenAIMessage> = request.messages.iter().map(|m| {
            OpenAIMessage {
                role: match m.role {
                    Role::System => "system".to_string(),
                    Role::User => "user".to_string(),
                    Role::Assistant => "assistant".to_string(),
                },
                content: m.content.clone(),
            }
        }).collect();

        let body = OpenAIRequest {
            model: request.model.clone(),
            messages,
            stream: false,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let resp = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::Provider(format!("OpenAI error {}: {}", status, text)));
        }

        let json: serde_json::Value = resp.json().await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(InternalResponse {
            id: json["id"].as_str().unwrap_or(&Uuid::new_v4().to_string()).to_string(),
            model: request.model,
            content,
            input_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
        })
    }
}

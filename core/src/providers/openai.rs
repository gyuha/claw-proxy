use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppError;
use crate::normalizer::from_openai::{OpenAIContent, OpenAIMessage, OpenAIRequest};
use crate::normalizer::{InternalRequest, InternalResponse, Role};

use super::Provider;

const OPENAI_BASE_URL: &str = "https://api.openai.com";

pub struct OpenAIProvider {
    name: String,
    api_key: String,
    models: Vec<String>,
    base_url: String,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(name: String, api_key: String, models: Vec<String>) -> Self {
        Self::new_with_base_url(name, api_key, models, OPENAI_BASE_URL.to_string())
    }

    pub fn new_with_base_url(
        name: String,
        api_key: String,
        models: Vec<String>,
        base_url: String,
    ) -> Self {
        Self {
            name,
            api_key,
            models,
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    fn completion_url(&self) -> String {
        format!("{}/v1/chat/completions", self.base_url)
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
                content: OpenAIContent::Text(m.content.clone()),
            }
        }).collect();

        let body = OpenAIRequest {
            model: request.model.clone(),
            messages,
            stream: false,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            tools: None,
            tool_choice: None,
            function_call: None,
        };

        let resp = self.client
            .post(self.completion_url())
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|error| AppError::Provider(format!("OpenAI request failed: {error}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let _ = resp.bytes().await;
            return Err(AppError::Provider(format!("OpenAI upstream error {}", status.as_u16())));
        }

        let json: serde_json::Value = resp.json().await
            .map_err(|error| AppError::Provider(format!("OpenAI response decode failed: {error}")))?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(InternalResponse {
            id: json["id"].as_str().unwrap_or(&Uuid::new_v4().to_string()).to_string(),
            model: json["model"]
                .as_str()
                .unwrap_or(request.model.as_str())
                .to_string(),
            content,
            input_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
        })
    }
}

use async_trait::async_trait;
use crate::normalizer::{InternalRequest, InternalResponse};
use crate::error::AppError;
use super::Provider;

pub struct ClaudeProvider {
    name: String,
}

impl ClaudeProvider {
    pub fn new(name: String, _api_key: String, _models: Vec<String>) -> Self {
        Self { name }
    }
}

#[async_trait]
impl Provider for ClaudeProvider {
    fn name(&self) -> &str { &self.name }
    fn is_available(&self) -> bool { false } // 스텁
    fn supports_model(&self, _model: &str) -> bool { false }
    async fn chat_completion(&self, _request: InternalRequest) -> Result<InternalResponse, AppError> {
        Err(AppError::Provider("Claude provider not yet implemented".to_string()))
    }
}

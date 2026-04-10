use async_trait::async_trait;
use crate::normalizer::{InternalRequest, InternalResponse};
use crate::error::AppError;

pub mod openai;
pub mod claude;
pub mod gemini;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn supports_model(&self, model: &str) -> bool;
    async fn chat_completion(
        &self,
        request: InternalRequest,
    ) -> Result<InternalResponse, AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_name() {
        let provider = openai::OpenAIProvider::new(
            "test".to_string(),
            "sk-test".to_string(),
            vec!["gpt-4o".to_string()],
        );
        assert_eq!(provider.name(), "test");
        assert!(provider.is_available());
    }

    #[test]
    fn test_supports_model() {
        let provider = openai::OpenAIProvider::new(
            "test".to_string(),
            "sk-test".to_string(),
            vec!["gpt-4o".to_string(), "gpt-4o-mini".to_string()],
        );
        assert!(provider.supports_model("gpt-4o"));
        assert!(!provider.supports_model("claude-3-5-sonnet"));
    }
}

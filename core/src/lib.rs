pub mod config;
pub mod error;
pub mod normalizer;
pub mod providers;
pub mod router;
pub mod proxy;
pub mod admin;

use config::{Config, ProviderType};
use providers::{Provider, openai::OpenAIProvider, claude::ClaudeProvider, gemini::GeminiProvider};

pub fn build_providers(config: &Config) -> Vec<Box<dyn Provider>> {
    config.providers.iter().map(|p| -> Box<dyn Provider> {
        match p.provider_type {
            ProviderType::OpenAI => Box::new(OpenAIProvider::new(
                p.name.clone(), p.api_key.clone(), p.models.clone(),
            )),
            ProviderType::Claude => Box::new(ClaudeProvider::new(
                p.name.clone(), p.api_key.clone(), p.models.clone(),
            )),
            ProviderType::Gemini => Box::new(GeminiProvider::new(
                p.name.clone(), p.api_key.clone(), p.models.clone(),
            )),
        }
    }).collect()
}
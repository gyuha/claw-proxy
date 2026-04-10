// core/src/config/mod.rs
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::{AppError, Result};

pub mod watcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub routing: RoutingConfig,
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub proxy_port: u16,
    pub admin_port: u16,
    pub ws_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            proxy_port: 47380,
            admin_port: 47381,
            ws_port: 47382,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    pub strategy: RoutingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RoutingStrategy {
    RoundRobin,
    Failover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    pub api_key: String,
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    OpenAI,
    Claude,
    Gemini,
}

pub fn load(path: impl AsRef<Path>) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AppError::Config(e.to_string()))?;
    serde_yaml::from_str(&content)
        .map_err(|e| AppError::Config(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let yaml = r#"
server:
  proxy_port: 47380
  admin_port: 47381
  ws_port: 47382
routing:
  strategy: round_robin
providers:
  - name: test-openai
    type: openai
    api_key: sk-test
    models:
      - gpt-4o
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.server.proxy_port, 47380);
        assert_eq!(config.providers.len(), 1);
        assert_eq!(config.providers[0].name, "test-openai");
        assert!(matches!(config.routing.strategy, RoutingStrategy::RoundRobin));
    }
}

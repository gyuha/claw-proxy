# Claw Proxy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 여러 AI 프로바이더를 단일 로컬 프록시로 통합하는 크로스 플랫폼 메뉴바 앱을 구축한다.

**Architecture:** Rust 코어 바이너리(프록시 엔진)를 Tauri 앱이 사이드카로 번들하여, 메뉴바 트레이 아이콘 클릭 시 사이드바 팝오버 창을 표시한다. UI는 HTTP REST(:47381)와 WebSocket(:47382)으로 코어와 통신하며, 프록시 엔드포인트(:47380)는 OpenAI 및 Anthropic 형식 모두 수신한다.

**Tech Stack:** Rust (tokio, axum, reqwest, serde_yaml, notify, async-trait), Tauri 2.x, React 18, TypeScript, TailwindCSS, shadcn/ui, Zustand

---

## 파일 구조 맵

```
claw-proxy/
├── Cargo.toml                              # 워크스페이스 루트
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                         # 서버 진입점
│       ├── error.rs                        # 공통 에러 타입
│       ├── config/
│       │   ├── mod.rs                      # Config 구조체, 로더
│       │   └── watcher.rs                  # hot reload (notify)
│       ├── normalizer/
│       │   ├── mod.rs                      # InternalRequest/Response, ApiFormat
│       │   ├── from_openai.rs              # OpenAI → Internal 변환
│       │   ├── from_anthropic.rs           # Anthropic → Internal 변환
│       │   ├── to_openai.rs                # Internal → OpenAI 응답 변환
│       │   └── to_anthropic.rs             # Internal → Anthropic 응답 변환
│       ├── providers/
│       │   ├── mod.rs                      # Provider 트레이트
│       │   ├── openai.rs                   # OpenAI 구현체
│       │   ├── claude.rs                   # Claude 스텁
│       │   └── gemini.rs                   # Gemini 스텁
│       ├── router/
│       │   ├── mod.rs                      # Router, RoutingStrategy
│       │   ├── round_robin.rs              # RoundRobin 구현
│       │   └── failover.rs                 # Failover 구현
│       ├── proxy/
│       │   └── mod.rs                      # /v1/chat/completions, /v1/messages 핸들러
│       └── admin/
│           ├── mod.rs                      # 관리 API 라우터
│           ├── handlers.rs                 # REST 핸들러
│           └── ws.rs                       # WebSocket 로그 스트리밍
├── apps/desktop/
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   ├── tauri.conf.json                 # sidecar, tray, positioner 설정
│   │   ├── capabilities/default.json       # Tauri 권한
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       └── tray.rs                     # 트레이 아이콘, 컨텍스트 메뉴
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx                         # 사이드바 레이아웃
│   │   ├── pages/
│   │   │   ├── Dashboard.tsx
│   │   │   ├── Providers.tsx
│   │   │   ├── Logs.tsx
│   │   │   └── Settings.tsx
│   │   ├── store/
│   │   │   ├── server.ts                   # serverStore (Zustand)
│   │   │   ├── providers.ts                # providerStore
│   │   │   ├── logs.ts                     # logStore
│   │   │   └── config.ts                   # configStore
│   │   └── api/
│   │       ├── client.ts                   # REST fetch 헬퍼
│   │       └── ws.ts                       # WebSocket 연결 관리
│   ├── package.json
│   ├── tsconfig.json
│   └── vite.config.ts
├── packages/shared-types/
│   ├── package.json
│   └── src/
│       └── index.ts                        # 공유 TS 타입
└── configs/
    └── config.example.yaml
```

---

## Phase 1 — 코어 프록시 서버

### Task 1: 워크스페이스 초기화

**Files:**
- Create: `Cargo.toml`
- Create: `core/Cargo.toml`
- Create: `core/src/main.rs`
- Create: `core/src/error.rs`

- [ ] **Step 1: 워크스페이스 Cargo.toml 작성**

```toml
# Cargo.toml
[workspace]
members = ["core", "apps/desktop/src-tauri"]
resolver = "2"
```

- [ ] **Step 2: core/Cargo.toml 작성**

```toml
[package]
name = "claw-proxy-core"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "claw-proxy"
path = "src/main.rs"

[lib]
name = "claw_proxy_core"
path = "src/lib.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = { version = "0.7", features = ["ws"] }
reqwest = { version = "0.12", features = ["json", "stream"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
notify = "6"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
tokio-tungstenite = "0.21"
futures = "0.3"
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio-test = "0.4"
```

- [ ] **Step 3: error.rs 작성**

```rust
// core/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("Router error: no available providers")]
    NoProviders,
    #[error("Normalize error: {0}")]
    Normalize(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

- [ ] **Step 4: 최소 main.rs 작성 (컴파일 확인용)**

```rust
// core/src/main.rs
mod error;

#[tokio::main]
async fn main() {
    println!("claw-proxy starting...");
}
```

- [ ] **Step 5: lib.rs 작성**

```rust
// core/src/lib.rs
pub mod config;
pub mod error;
pub mod normalizer;
pub mod providers;
pub mod router;
pub mod proxy;
pub mod admin;
```

- [ ] **Step 6: 컴파일 확인**

```bash
cargo build -p claw-proxy-core
```

Expected: `Finished` (경고는 무시 가능)

- [ ] **Step 7: 커밋**

```bash
git add Cargo.toml core/
git commit -m "chore: initialize Rust workspace and core crate"
```

---

### Task 2: Config 로더

**Files:**
- Create: `core/src/config/mod.rs`
- Create: `configs/config.example.yaml`

- [ ] **Step 1: 테스트 작성**

```rust
// core/src/config/mod.rs 상단에 테스트 모듈 포함하여 작성
// 먼저 테스트만 작성 (구현 전)
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
```

- [ ] **Step 2: 테스트 실행 (실패 확인)**

```bash
cargo test -p claw-proxy-core config
```

Expected: FAIL — `config` 모듈 없음

- [ ] **Step 3: Config 구현**

```rust
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
```

- [ ] **Step 4: watcher.rs 스텁 작성 (컴파일용)**

```rust
// core/src/config/watcher.rs
// hot reload는 Task 8에서 구현
```

- [ ] **Step 5: 테스트 실행 (통과 확인)**

```bash
cargo test -p claw-proxy-core config
```

Expected: PASS

- [ ] **Step 6: config.example.yaml 작성**

```yaml
# configs/config.example.yaml
server:
  proxy_port: 47380
  admin_port: 47381
  ws_port: 47382

routing:
  strategy: round_robin  # round_robin | failover

providers:
  - name: openai-account-1
    type: openai
    api_key: sk-YOUR_KEY_HERE
    models:
      - gpt-4o
      - gpt-4o-mini

  - name: openai-account-2
    type: openai
    api_key: sk-YOUR_KEY_HERE
    models:
      - gpt-4o
```

- [ ] **Step 7: 커밋**

```bash
git add core/src/config/ configs/
git commit -m "feat(core): add config loader with YAML parsing"
```

---

### Task 3: 정규화 레이어 (Normalizer)

**Files:**
- Create: `core/src/normalizer/mod.rs`
- Create: `core/src/normalizer/from_openai.rs`
- Create: `core/src/normalizer/from_anthropic.rs`
- Create: `core/src/normalizer/to_openai.rs`
- Create: `core/src/normalizer/to_anthropic.rs`

- [ ] **Step 1: 테스트 작성**

```rust
// core/src/normalizer/mod.rs 하단 테스트 모듈 (먼저 작성)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_openai_request() {
        let json = serde_json::json!({
            "model": "gpt-4o",
            "messages": [
                {"role": "user", "content": "Hello"}
            ],
            "stream": false
        });
        let req: OpenAIRequest = serde_json::from_value(json).unwrap();
        let internal = InternalRequest::from_openai(req);
        assert_eq!(internal.model, "gpt-4o");
        assert_eq!(internal.messages.len(), 1);
        assert!(matches!(internal.source_format, ApiFormat::OpenAI));
    }

    #[test]
    fn test_from_anthropic_request() {
        let json = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 1024,
            "messages": [
                {"role": "user", "content": "Hello"}
            ]
        });
        let req: AnthropicRequest = serde_json::from_value(json).unwrap();
        let internal = InternalRequest::from_anthropic(req);
        assert_eq!(internal.model, "claude-3-5-sonnet-20241022");
        assert!(matches!(internal.source_format, ApiFormat::Anthropic));
    }
}
```

- [ ] **Step 2: 테스트 실행 (실패 확인)**

```bash
cargo test -p claw-proxy-core normalizer
```

Expected: FAIL

- [ ] **Step 3: mod.rs — 공통 타입 구현**

```rust
// core/src/normalizer/mod.rs
use serde::{Deserialize, Serialize};

pub mod from_openai;
pub mod from_anthropic;
pub mod to_openai;
pub mod to_anthropic;

pub use from_openai::OpenAIRequest;
pub use from_anthropic::AnthropicRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiFormat {
    OpenAI,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct InternalRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub source_format: ApiFormat,
}

impl InternalRequest {
    pub fn from_openai(req: OpenAIRequest) -> Self {
        from_openai::convert(req)
    }

    pub fn from_anthropic(req: AnthropicRequest) -> Self {
        from_anthropic::convert(req)
    }
}

#[derive(Debug, Clone)]
pub struct InternalResponse {
    pub id: String,
    pub model: String,
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_openai_request() {
        let json = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": "Hello"}],
            "stream": false
        });
        let req: OpenAIRequest = serde_json::from_value(json).unwrap();
        let internal = InternalRequest::from_openai(req);
        assert_eq!(internal.model, "gpt-4o");
        assert_eq!(internal.messages.len(), 1);
        assert!(matches!(internal.source_format, ApiFormat::OpenAI));
    }

    #[test]
    fn test_from_anthropic_request() {
        let json = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": "Hello"}]
        });
        let req: AnthropicRequest = serde_json::from_value(json).unwrap();
        let internal = InternalRequest::from_anthropic(req);
        assert_eq!(internal.model, "claude-3-5-sonnet-20241022");
        assert!(matches!(internal.source_format, ApiFormat::Anthropic));
    }
}
```

- [ ] **Step 4: from_openai.rs 구현**

```rust
// core/src/normalizer/from_openai.rs
use serde::{Deserialize, Serialize};
use super::{InternalRequest, Message, Role, ApiFormat};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    #[serde(default)]
    pub stream: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

pub fn convert(req: OpenAIRequest) -> InternalRequest {
    let messages = req.messages.into_iter().map(|m| Message {
        role: match m.role.as_str() {
            "system" => Role::System,
            "assistant" => Role::Assistant,
            _ => Role::User,
        },
        content: m.content,
    }).collect();

    InternalRequest {
        model: req.model,
        messages,
        stream: req.stream,
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        source_format: ApiFormat::OpenAI,
    }
}
```

- [ ] **Step 5: from_anthropic.rs 구현**

```rust
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
```

- [ ] **Step 6: to_openai.rs 구현**

```rust
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
```

- [ ] **Step 7: to_anthropic.rs 구현**

```rust
// core/src/normalizer/to_anthropic.rs
use serde::Serialize;
use super::InternalResponse;

#[derive(Debug, Serialize)]
pub struct AnthropicResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: String,
    pub model: String,
    pub content: Vec<AnthropicContent>,
    pub usage: AnthropicUsage,
    pub stop_reason: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

pub fn convert(resp: InternalResponse) -> AnthropicResponse {
    AnthropicResponse {
        id: resp.id.clone(),
        response_type: "message".to_string(),
        role: "assistant".to_string(),
        model: resp.model.clone(),
        content: vec![AnthropicContent {
            content_type: "text".to_string(),
            text: resp.content.clone(),
        }],
        usage: AnthropicUsage {
            input_tokens: resp.input_tokens,
            output_tokens: resp.output_tokens,
        },
        stop_reason: "end_turn".to_string(),
    }
}
```

- [ ] **Step 8: 테스트 통과 확인**

```bash
cargo test -p claw-proxy-core normalizer
```

Expected: PASS

- [ ] **Step 9: 커밋**

```bash
git add core/src/normalizer/
git commit -m "feat(core): add request/response normalizer (OpenAI <-> Anthropic)"
```

---

### Task 4: Provider 트레이트 + OpenAI 구현체

**Files:**
- Create: `core/src/providers/mod.rs`
- Create: `core/src/providers/openai.rs`
- Create: `core/src/providers/claude.rs`
- Create: `core/src/providers/gemini.rs`

- [ ] **Step 1: 테스트 작성**

```rust
// core/src/providers/mod.rs 하단 테스트 (먼저 작성)
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
```

- [ ] **Step 2: 테스트 실행 (실패 확인)**

```bash
cargo test -p claw-proxy-core providers
```

Expected: FAIL

- [ ] **Step 3: mod.rs — Provider 트레이트 구현**

```rust
// core/src/providers/mod.rs
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
```

- [ ] **Step 4: openai.rs 구현**

```rust
// core/src/providers/openai.rs
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
```

- [ ] **Step 5: claude.rs 스텁 작성**

```rust
// core/src/providers/claude.rs
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
```

- [ ] **Step 6: gemini.rs 스텁 작성**

```rust
// core/src/providers/gemini.rs
use async_trait::async_trait;
use crate::normalizer::{InternalRequest, InternalResponse};
use crate::error::AppError;
use super::Provider;

pub struct GeminiProvider {
    name: String,
}

impl GeminiProvider {
    pub fn new(name: String, _api_key: String, _models: Vec<String>) -> Self {
        Self { name }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str { &self.name }
    fn is_available(&self) -> bool { false } // 스텁
    fn supports_model(&self, _model: &str) -> bool { false }
    async fn chat_completion(&self, _request: InternalRequest) -> Result<InternalResponse, AppError> {
        Err(AppError::Provider("Gemini provider not yet implemented".to_string()))
    }
}
```

- [ ] **Step 7: 테스트 통과 확인**

```bash
cargo test -p claw-proxy-core providers
```

Expected: PASS

- [ ] **Step 8: 커밋**

```bash
git add core/src/providers/
git commit -m "feat(core): add Provider trait, OpenAI impl, Claude/Gemini stubs"
```

---

### Task 5: 라우터 (RoundRobin + Failover)

**Files:**
- Create: `core/src/router/mod.rs`
- Create: `core/src/router/round_robin.rs`
- Create: `core/src/router/failover.rs`

- [ ] **Step 1: 테스트 작성**

```rust
// core/src/router/mod.rs 하단 테스트 (먼저 작성)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::Provider;
    use crate::normalizer::{InternalRequest, InternalResponse, ApiFormat};
    use crate::error::AppError;
    use async_trait::async_trait;

    struct MockProvider {
        name: String,
        available: bool,
    }

    #[async_trait]
    impl Provider for MockProvider {
        fn name(&self) -> &str { &self.name }
        fn is_available(&self) -> bool { self.available }
        fn supports_model(&self, _: &str) -> bool { true }
        async fn chat_completion(&self, _: InternalRequest) -> Result<InternalResponse, AppError> {
            Ok(InternalResponse {
                id: "test".to_string(),
                model: "test".to_string(),
                content: format!("response from {}", self.name),
                input_tokens: 10,
                output_tokens: 5,
            })
        }
    }

    #[test]
    fn test_round_robin_cycles() {
        let providers: Vec<Box<dyn Provider>> = vec![
            Box::new(MockProvider { name: "p1".to_string(), available: true }),
            Box::new(MockProvider { name: "p2".to_string(), available: true }),
        ];
        let router = Router::new(providers, RoutingStrategy::RoundRobin);
        assert_eq!(router.next_provider().unwrap().name(), "p1");
        assert_eq!(router.next_provider().unwrap().name(), "p2");
        assert_eq!(router.next_provider().unwrap().name(), "p1");
    }
}
```

- [ ] **Step 2: 테스트 실행 (실패 확인)**

```bash
cargo test -p claw-proxy-core router
```

Expected: FAIL

- [ ] **Step 3: mod.rs 구현**

```rust
// core/src/router/mod.rs
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crate::providers::Provider;
use crate::config::RoutingStrategy;
use crate::error::AppError;

pub mod round_robin;
pub mod failover;

pub struct Router {
    providers: Vec<Arc<Box<dyn Provider>>>,
    strategy: RoutingStrategy,
    counter: AtomicUsize,
}

impl Router {
    pub fn new(providers: Vec<Box<dyn Provider>>, strategy: RoutingStrategy) -> Self {
        Self {
            providers: providers.into_iter().map(Arc::new).collect(),
            strategy,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn next_provider(&self) -> Result<Arc<Box<dyn Provider>>, AppError> {
        match self.strategy {
            RoutingStrategy::RoundRobin => round_robin::next(&self.providers, &self.counter),
            RoutingStrategy::Failover => failover::next(&self.providers),
        }
    }

    pub fn providers(&self) -> &[Arc<Box<dyn Provider>>] {
        &self.providers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normalizer::{InternalRequest, InternalResponse, ApiFormat};
    use async_trait::async_trait;

    struct MockProvider { name: String, available: bool }

    #[async_trait]
    impl Provider for MockProvider {
        fn name(&self) -> &str { &self.name }
        fn is_available(&self) -> bool { self.available }
        fn supports_model(&self, _: &str) -> bool { true }
        async fn chat_completion(&self, _: InternalRequest) -> Result<InternalResponse, AppError> {
            Ok(InternalResponse {
                id: "test".to_string(),
                model: "test".to_string(),
                content: format!("response from {}", self.name),
                input_tokens: 10,
                output_tokens: 5,
            })
        }
    }

    #[test]
    fn test_round_robin_cycles() {
        let providers: Vec<Box<dyn Provider>> = vec![
            Box::new(MockProvider { name: "p1".to_string(), available: true }),
            Box::new(MockProvider { name: "p2".to_string(), available: true }),
        ];
        let router = Router::new(providers, RoutingStrategy::RoundRobin);
        assert_eq!(router.next_provider().unwrap().name(), "p1");
        assert_eq!(router.next_provider().unwrap().name(), "p2");
        assert_eq!(router.next_provider().unwrap().name(), "p1");
    }
}
```

- [ ] **Step 4: round_robin.rs 구현**

```rust
// core/src/router/round_robin.rs
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::providers::Provider;
use crate::error::AppError;

pub fn next(
    providers: &[Arc<Box<dyn Provider>>],
    counter: &AtomicUsize,
) -> Result<Arc<Box<dyn Provider>>, AppError> {
    let available: Vec<_> = providers.iter().filter(|p| p.is_available()).collect();
    if available.is_empty() {
        return Err(AppError::NoProviders);
    }
    let idx = counter.fetch_add(1, Ordering::Relaxed) % available.len();
    Ok(Arc::clone(available[idx]))
}
```

- [ ] **Step 5: failover.rs 구현**

```rust
// core/src/router/failover.rs
use std::sync::Arc;
use crate::providers::Provider;
use crate::error::AppError;

pub fn next(providers: &[Arc<Box<dyn Provider>>]) -> Result<Arc<Box<dyn Provider>>, AppError> {
    providers.iter()
        .find(|p| p.is_available())
        .map(Arc::clone)
        .ok_or(AppError::NoProviders)
}
```

- [ ] **Step 6: 테스트 통과 확인**

```bash
cargo test -p claw-proxy-core router
```

Expected: PASS

- [ ] **Step 7: 커밋**

```bash
git add core/src/router/
git commit -m "feat(core): add round-robin and failover router"
```

---

### Task 6: 프록시 HTTP 서버 (axum)

**Files:**
- Create: `core/src/proxy/mod.rs`

- [ ] **Step 1: proxy/mod.rs 구현**

```rust
// core/src/proxy/mod.rs
use axum::{
    Router,
    routing::post,
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
    response::IntoResponse,
};
use std::sync::Arc;
use crate::router::Router as ProxyRouter;
use crate::normalizer::{
    InternalRequest,
    from_openai::OpenAIRequest,
    from_anthropic::AnthropicRequest,
    to_openai, to_anthropic,
    ApiFormat,
};
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub router: Arc<ProxyRouter>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/chat/completions", post(handle_openai))
        .route("/v1/messages", post(handle_anthropic))
        .with_state(state)
}

async fn handle_openai(
    State(state): State<AppState>,
    Json(req): Json<OpenAIRequest>,
) -> impl IntoResponse {
    let internal = InternalRequest::from_openai(req);
    match dispatch(state, internal).await {
        Ok(resp) => {
            let openai_resp = to_openai::convert(resp);
            (StatusCode::OK, Json(openai_resp)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    }
}

async fn handle_anthropic(
    State(state): State<AppState>,
    Json(req): Json<AnthropicRequest>,
) -> impl IntoResponse {
    let internal = InternalRequest::from_anthropic(req);
    match dispatch(state, internal).await {
        Ok(resp) => {
            let anthropic_resp = to_anthropic::convert(resp);
            (StatusCode::OK, Json(anthropic_resp)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    }
}

async fn dispatch(
    state: AppState,
    request: InternalRequest,
) -> Result<crate::normalizer::InternalResponse, AppError> {
    let provider = state.router.next_provider()?;
    provider.chat_completion(request).await
}
```

- [ ] **Step 2: main.rs 업데이트 (서버 실제 실행)**

```rust
// core/src/main.rs
mod error;

use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("claw_proxy_core=info".parse().unwrap()))
        .init();

    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yaml"));

    let config = claw_proxy_core::config::load(&config_path)
        .expect("Failed to load config");

    let providers = claw_proxy_core::build_providers(&config);
    let router = Arc::new(claw_proxy_core::router::Router::new(
        providers,
        config.routing.strategy.clone(),
    ));

    let proxy_state = claw_proxy_core::proxy::AppState { router };
    let proxy_app = claw_proxy_core::proxy::create_router(proxy_state);

    let proxy_addr = format!("127.0.0.1:{}", config.server.proxy_port);
    tracing::info!("Proxy listening on http://{}", proxy_addr);

    let listener = tokio::net::TcpListener::bind(&proxy_addr).await.unwrap();
    axum::serve(listener, proxy_app).await.unwrap();
}
```

- [ ] **Step 3: lib.rs에 build_providers 헬퍼 추가**

```rust
// core/src/lib.rs
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
```

- [ ] **Step 4: 빌드 확인**

```bash
cargo build -p claw-proxy-core
```

Expected: `Finished`

- [ ] **Step 5: 커밋**

```bash
git add core/src/proxy/ core/src/main.rs core/src/lib.rs
git commit -m "feat(core): add axum proxy server with OpenAI and Anthropic endpoints"
```

---

## Phase 2 — 관리 API

### Task 7: 관리 REST API + WebSocket 로그

**Files:**
- Create: `core/src/admin/mod.rs`
- Create: `core/src/admin/handlers.rs`
- Create: `core/src/admin/ws.rs`

- [ ] **Step 1: 공유 상태 타입 정의 (admin/mod.rs)**

```rust
// core/src/admin/mod.rs
use axum::{Router, routing::{get, post, delete}};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::router::Router as ProxyRouter;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub mod handlers;
pub mod ws;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
    pub model: String,
    pub provider: String,
    pub source_format: String,
    pub status_code: u16,
    pub latency_ms: u64,
}

#[derive(Clone)]
pub struct AdminState {
    pub router: Arc<ProxyRouter>,
    pub log_tx: broadcast::Sender<LogEntry>,
    pub start_time: std::time::Instant,
}

pub fn create_router(state: AdminState) -> Router {
    Router::new()
        .route("/status", get(handlers::status))
        .route("/providers", get(handlers::list_providers))
        .route("/stats", get(handlers::stats))
        .with_state(state)
}
```

- [ ] **Step 2: handlers.rs 구현**

```rust
// core/src/admin/handlers.rs
use axum::{extract::State, Json};
use serde::Serialize;
use super::AdminState;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub uptime_secs: u64,
    pub proxy_port: u16,
    pub admin_port: u16,
}

pub async fn status(State(state): State<AdminState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
        uptime_secs: state.start_time.elapsed().as_secs(),
        proxy_port: 47380,
        admin_port: 47381,
    })
}

#[derive(Serialize)]
pub struct ProviderInfo {
    pub name: String,
    pub available: bool,
}

pub async fn list_providers(State(state): State<AdminState>) -> Json<Vec<ProviderInfo>> {
    let providers = state.router.providers().iter().map(|p| ProviderInfo {
        name: p.name().to_string(),
        available: p.is_available(),
    }).collect();
    Json(providers)
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub provider_count: usize,
}

pub async fn stats(State(state): State<AdminState>) -> Json<StatsResponse> {
    Json(StatsResponse {
        provider_count: state.router.providers().len(),
    })
}
```

- [ ] **Step 3: ws.rs 구현 (WebSocket 로그 스트리밍)**

```rust
// core/src/admin/ws.rs
use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use super::AdminState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AdminState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AdminState) {
    let mut rx = state.log_tx.subscribe();
    let (mut sender, _receiver) = socket.split();

    while let Ok(entry) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&entry) {
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    }
}
```

- [ ] **Step 4: admin router에 ws 엔드포인트 추가**

`core/src/admin/mod.rs`의 `create_router` 함수를 다음으로 교체:

```rust
pub fn create_router(state: AdminState) -> Router {
    Router::new()
        .route("/status", get(handlers::status))
        .route("/providers", get(handlers::list_providers))
        .route("/stats", get(handlers::stats))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
}
```

- [ ] **Step 5: main.rs 업데이트 (관리 서버 추가)**

`core/src/main.rs`를 다음으로 교체:

```rust
// core/src/main.rs
mod error;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("claw_proxy_core=info".parse().unwrap()))
        .init();

    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yaml"));

    let config = claw_proxy_core::config::load(&config_path)
        .expect("Failed to load config");

    let providers = claw_proxy_core::build_providers(&config);
    let router = Arc::new(claw_proxy_core::router::Router::new(
        providers,
        config.routing.strategy.clone(),
    ));

    let (log_tx, _) = broadcast::channel(256);

    // Proxy server
    let proxy_state = claw_proxy_core::proxy::AppState { router: Arc::clone(&router) };
    let proxy_app = claw_proxy_core::proxy::create_router(proxy_state);
    let proxy_addr = format!("127.0.0.1:{}", config.server.proxy_port);

    // Admin server
    let admin_state = claw_proxy_core::admin::AdminState {
        router: Arc::clone(&router),
        log_tx,
        start_time: std::time::Instant::now(),
    };
    let admin_app = claw_proxy_core::admin::create_router(admin_state);
    let admin_addr = format!("127.0.0.1:{}", config.server.admin_port);

    tracing::info!("Proxy  → http://{}", proxy_addr);
    tracing::info!("Admin  → http://{}", admin_addr);

    let proxy_listener = tokio::net::TcpListener::bind(&proxy_addr).await.unwrap();
    let admin_listener = tokio::net::TcpListener::bind(&admin_addr).await.unwrap();

    tokio::join!(
        axum::serve(proxy_listener, proxy_app),
        axum::serve(admin_listener, admin_app),
    );
}
```

- [ ] **Step 6: 빌드 + 수동 테스트**

```bash
cargo build -p claw-proxy-core
# config.yaml 복사 후 실행
cp configs/config.example.yaml config.yaml
./target/debug/claw-proxy config.yaml
# 별도 터미널에서:
curl http://localhost:47381/status
```

Expected: `{"status":"running","uptime_secs":...}`

- [ ] **Step 7: 커밋**

```bash
git add core/src/admin/ core/src/main.rs
git commit -m "feat(core): add admin REST API and WebSocket log streaming"
```

---

### Task 8: Config Hot Reload

**Files:**
- Modify: `core/src/config/watcher.rs`

- [ ] **Step 1: watcher.rs 구현**

```rust
// core/src/config/watcher.rs
use notify::{Watcher, RecursiveMode, recommended_watcher, Event};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::watch;
use super::{Config, load};

pub fn start(path: PathBuf, tx: watch::Sender<Config>) {
    let path_clone = path.clone();
    std::thread::spawn(move || {
        let (notify_tx, notify_rx) = mpsc::channel();
        let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = notify_tx.send(event);
            }
        }).expect("Failed to create watcher");

        watcher.watch(&path_clone, RecursiveMode::NonRecursive)
            .expect("Failed to watch config file");

        loop {
            if notify_rx.recv_timeout(Duration::from_secs(1)).is_ok() {
                // debounce: 100ms 대기 후 추가 이벤트 소진
                std::thread::sleep(Duration::from_millis(100));
                while notify_rx.try_recv().is_ok() {}

                match load(&path_clone) {
                    Ok(new_config) => {
                        tracing::info!("Config reloaded from {:?}", path_clone);
                        let _ = tx.send(new_config);
                    }
                    Err(e) => tracing::error!("Config reload failed: {}", e),
                }
            }
        }
    });
}
```

- [ ] **Step 2: 빌드 확인**

```bash
cargo build -p claw-proxy-core
```

Expected: `Finished`

- [ ] **Step 3: 커밋**

```bash
git add core/src/config/watcher.rs
git commit -m "feat(core): add config hot reload with notify"
```

---

## Phase 3 — Tauri 메뉴바 앱

### Task 9: Tauri 앱 스캐폴딩

**Files:**
- Create: `apps/desktop/package.json`
- Create: `apps/desktop/vite.config.ts`
- Create: `apps/desktop/tsconfig.json`
- Create: `apps/desktop/index.html`
- Create: `apps/desktop/src-tauri/Cargo.toml`
- Create: `apps/desktop/src-tauri/tauri.conf.json`
- Create: `apps/desktop/src-tauri/src/main.rs`
- Create: `apps/desktop/src-tauri/src/lib.rs`

- [ ] **Step 1: package.json 작성**

```json
{
  "name": "claw-proxy-desktop",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri": "tauri"
  },
  "dependencies": {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-shell": "^2",
    "react": "^18",
    "react-dom": "^18",
    "zustand": "^4"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "@vitejs/plugin-react": "^4",
    "autoprefixer": "^10",
    "postcss": "^8",
    "tailwindcss": "^3",
    "typescript": "^5",
    "vite": "^5"
  }
}
```

- [ ] **Step 2: vite.config.ts 작성**

```typescript
// apps/desktop/vite.config.ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "chrome105",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
```

- [ ] **Step 3: src-tauri/Cargo.toml 작성**

```toml
[package]
name = "claw-proxy-desktop"
version = "0.1.0"
edition = "2021"

[lib]
name = "claw_proxy_desktop_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
tauri-plugin-shell = "2"
tauri-plugin-positioner = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

- [ ] **Step 4: tauri.conf.json 작성**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Claw Proxy",
  "version": "0.1.0",
  "identifier": "io.clawproxy.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420"
  },
  "app": {
    "withGlobalTauri": true,
    "trayIcon": {
      "iconPath": "icons/tray-icon.png",
      "iconAsTemplate": true
    },
    "windows": [
      {
        "label": "main",
        "title": "Claw Proxy",
        "width": 400,
        "height": 600,
        "resizable": false,
        "decorations": false,
        "visible": false,
        "alwaysOnTop": true,
        "skipTaskbar": true
      }
    ]
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "externalBin": ["binaries/claw-proxy"]
  }
}
```

- [ ] **Step 5: src-tauri/src/lib.rs 작성 (tray 설정)**

```rust
// apps/desktop/src-tauri/src/lib.rs
use tauri::{
    Manager, Runtime,
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    menu::{Menu, MenuItem},
};
use tauri_plugin_positioner::{WindowExt, Position};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.move_window(Position::TrayCenter);
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .on_menu_event(|app, event| {
                    if event.id == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: src-tauri/src/main.rs 작성**

```rust
// apps/desktop/src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    claw_proxy_desktop_lib::run();
}
```

- [ ] **Step 7: 커밋**

```bash
git add apps/desktop/
git commit -m "feat(desktop): scaffold Tauri menubar app with tray icon"
```

---

### Task 10: React UI — 공유 타입 + 기본 레이아웃

**Files:**
- Create: `packages/shared-types/src/index.ts`
- Create: `apps/desktop/src/main.tsx`
- Create: `apps/desktop/src/App.tsx`
- Create: `apps/desktop/src/api/client.ts`
- Create: `apps/desktop/src/api/ws.ts`

- [ ] **Step 1: 공유 타입 작성**

```typescript
// packages/shared-types/src/index.ts
export interface ServerStatus {
  status: 'running' | 'stopped';
  uptime_secs: number;
  proxy_port: number;
  admin_port: number;
}

export interface ProviderInfo {
  name: string;
  available: boolean;
}

export interface LogEntry {
  timestamp: string;
  request_id: string;
  model: string;
  provider: string;
  source_format: 'openai' | 'anthropic';
  status_code: number;
  latency_ms: number;
}
```

- [ ] **Step 2: API 클라이언트 작성**

```typescript
// apps/desktop/src/api/client.ts
const ADMIN_BASE = 'http://localhost:47381';

export async function getStatus() {
  const res = await fetch(`${ADMIN_BASE}/status`);
  return res.json();
}

export async function getProviders() {
  const res = await fetch(`${ADMIN_BASE}/providers`);
  return res.json();
}
```

- [ ] **Step 3: WebSocket 클라이언트 작성**

```typescript
// apps/desktop/src/api/ws.ts
import type { LogEntry } from '../../../packages/shared-types/src';

const WS_URL = 'ws://localhost:47381/ws';

export function connectLogStream(onEntry: (entry: LogEntry) => void): () => void {
  const ws = new WebSocket(WS_URL);

  ws.onmessage = (event) => {
    try {
      const entry: LogEntry = JSON.parse(event.data);
      onEntry(entry);
    } catch {}
  };

  return () => ws.close();
}
```

- [ ] **Step 4: App.tsx — 사이드바 레이아웃**

```tsx
// apps/desktop/src/App.tsx
import { useState } from 'react';
import Dashboard from './pages/Dashboard';
import Providers from './pages/Providers';
import Logs from './pages/Logs';
import Settings from './pages/Settings';

type Page = 'dashboard' | 'providers' | 'logs' | 'settings';

export default function App() {
  const [page, setPage] = useState<Page>('dashboard');

  const nav = [
    { id: 'dashboard' as Page, label: '대시보드' },
    { id: 'providers' as Page, label: '프로바이더' },
    { id: 'logs' as Page, label: '로그' },
    { id: 'settings' as Page, label: '설정' },
  ];

  return (
    <div className="flex flex-col h-screen bg-zinc-900 text-white text-sm">
      {/* 상태 헤더 */}
      <div className="flex items-center justify-between px-4 py-2 bg-zinc-800 border-b border-zinc-700">
        <span className="font-medium">🦀 Claw Proxy</span>
        <span className="text-xs text-green-400">● 실행 중 :47380</span>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* 사이드 네비게이션 */}
        <nav className="w-24 bg-zinc-800 border-r border-zinc-700 py-2">
          {nav.map(item => (
            <button
              key={item.id}
              onClick={() => setPage(item.id)}
              className={`w-full px-2 py-3 text-xs text-center hover:bg-zinc-700 transition-colors ${
                page === item.id ? 'bg-zinc-700 text-white' : 'text-zinc-400'
              }`}
            >
              {item.label}
            </button>
          ))}
        </nav>

        {/* 메인 콘텐츠 */}
        <main className="flex-1 overflow-y-auto p-4">
          {page === 'dashboard' && <Dashboard />}
          {page === 'providers' && <Providers />}
          {page === 'logs' && <Logs />}
          {page === 'settings' && <Settings />}
        </main>
      </div>
    </div>
  );
}
```

- [ ] **Step 5: 각 페이지 스텁 작성**

```tsx
// apps/desktop/src/pages/Dashboard.tsx
export default function Dashboard() {
  return <div className="text-zinc-300">대시보드</div>;
}
```

```tsx
// apps/desktop/src/pages/Providers.tsx
export default function Providers() {
  return <div className="text-zinc-300">프로바이더</div>;
}
```

```tsx
// apps/desktop/src/pages/Logs.tsx
export default function Logs() {
  return <div className="text-zinc-300">로그</div>;
}
```

```tsx
// apps/desktop/src/pages/Settings.tsx
export default function Settings() {
  return <div className="text-zinc-300">설정</div>;
}
```

- [ ] **Step 6: main.tsx 작성**

```tsx
// apps/desktop/src/main.tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

- [ ] **Step 7: index.css (Tailwind 기본)**

```css
/* apps/desktop/src/index.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

* { box-sizing: border-box; }
body { margin: 0; overflow: hidden; }
```

- [ ] **Step 8: 커밋**

```bash
git add packages/ apps/desktop/src/
git commit -m "feat(ui): add sidebar layout with page stubs and API client"
```

---

### Task 11: UI — Zustand 스토어 + 대시보드 페이지

**Files:**
- Create: `apps/desktop/src/store/server.ts`
- Create: `apps/desktop/src/store/providers.ts`
- Create: `apps/desktop/src/store/logs.ts`
- Modify: `apps/desktop/src/pages/Dashboard.tsx`
- Modify: `apps/desktop/src/pages/Providers.tsx`
- Modify: `apps/desktop/src/pages/Logs.tsx`

- [ ] **Step 1: server store 작성**

```typescript
// apps/desktop/src/store/server.ts
import { create } from 'zustand';
import { getStatus } from '../api/client';
import type { ServerStatus } from '../../../packages/shared-types/src';

interface ServerStore {
  status: ServerStatus | null;
  loading: boolean;
  fetch: () => Promise<void>;
}

export const useServerStore = create<ServerStore>((set) => ({
  status: null,
  loading: false,
  fetch: async () => {
    set({ loading: true });
    try {
      const status = await getStatus();
      set({ status, loading: false });
    } catch {
      set({ loading: false });
    }
  },
}));
```

- [ ] **Step 2: providers store 작성**

```typescript
// apps/desktop/src/store/providers.ts
import { create } from 'zustand';
import { getProviders } from '../api/client';
import type { ProviderInfo } from '../../../packages/shared-types/src';

interface ProviderStore {
  providers: ProviderInfo[];
  fetch: () => Promise<void>;
}

export const useProviderStore = create<ProviderStore>((set) => ({
  providers: [],
  fetch: async () => {
    try {
      const providers = await getProviders();
      set({ providers });
    } catch {}
  },
}));
```

- [ ] **Step 3: logs store 작성**

```typescript
// apps/desktop/src/store/logs.ts
import { create } from 'zustand';
import type { LogEntry } from '../../../packages/shared-types/src';

const MAX_LOGS = 500;

interface LogStore {
  logs: LogEntry[];
  addLog: (entry: LogEntry) => void;
  clear: () => void;
}

export const useLogStore = create<LogStore>((set) => ({
  logs: [],
  addLog: (entry) => set((state) => ({
    logs: [entry, ...state.logs].slice(0, MAX_LOGS),
  })),
  clear: () => set({ logs: [] }),
}));
```

- [ ] **Step 4: Dashboard.tsx 구현**

```tsx
// apps/desktop/src/pages/Dashboard.tsx
import { useEffect } from 'react';
import { useServerStore } from '../store/server';
import { useProviderStore } from '../store/providers';

export default function Dashboard() {
  const { status, fetch: fetchStatus } = useServerStore();
  const { providers, fetch: fetchProviders } = useProviderStore();

  useEffect(() => {
    fetchStatus();
    fetchProviders();
    const interval = setInterval(fetchStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const activeCount = providers.filter(p => p.available).length;

  return (
    <div className="space-y-4">
      <div className="bg-zinc-800 rounded-lg p-4">
        <div className="flex items-center justify-between">
          <span className="text-zinc-300">서버 상태</span>
          {status ? (
            <span className="text-green-400 text-xs">● 실행 중</span>
          ) : (
            <span className="text-red-400 text-xs">● 연결 중...</span>
          )}
        </div>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="bg-zinc-800 rounded-lg p-3">
          <div className="text-zinc-500 text-xs">활성 프로바이더</div>
          <div className="text-2xl font-bold mt-1">{activeCount}</div>
        </div>
        <div className="bg-zinc-800 rounded-lg p-3">
          <div className="text-zinc-500 text-xs">업타임</div>
          <div className="text-2xl font-bold mt-1">
            {status ? `${Math.floor(status.uptime_secs / 60)}m` : '--'}
          </div>
        </div>
      </div>
    </div>
  );
}
```

- [ ] **Step 5: Providers.tsx 구현**

```tsx
// apps/desktop/src/pages/Providers.tsx
import { useEffect } from 'react';
import { useProviderStore } from '../store/providers';

export default function Providers() {
  const { providers, fetch } = useProviderStore();

  useEffect(() => { fetch(); }, []);

  return (
    <div className="space-y-2">
      <h2 className="text-xs font-medium text-zinc-400 uppercase tracking-wider">프로바이더</h2>
      {providers.length === 0 && (
        <p className="text-zinc-500 text-xs">등록된 프로바이더가 없습니다.</p>
      )}
      {providers.map(p => (
        <div key={p.name} className="bg-zinc-800 rounded-lg p-3 flex items-center justify-between">
          <span className="text-zinc-200 text-xs">{p.name}</span>
          <span className={`text-xs ${p.available ? 'text-green-400' : 'text-red-400'}`}>
            {p.available ? '● 활성' : '● 비활성'}
          </span>
        </div>
      ))}
    </div>
  );
}
```

- [ ] **Step 6: Logs.tsx 구현**

```tsx
// apps/desktop/src/pages/Logs.tsx
import { useEffect } from 'react';
import { useLogStore } from '../store/logs';
import { connectLogStream } from '../api/ws';

export default function Logs() {
  const { logs, addLog, clear } = useLogStore();

  useEffect(() => {
    const disconnect = connectLogStream(addLog);
    return disconnect;
  }, []);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <h2 className="text-xs font-medium text-zinc-400 uppercase tracking-wider">요청 로그</h2>
        <button onClick={clear} className="text-xs text-zinc-500 hover:text-zinc-300">지우기</button>
      </div>
      <div className="space-y-1">
        {logs.length === 0 && (
          <p className="text-zinc-500 text-xs">요청이 없습니다.</p>
        )}
        {logs.map(log => (
          <div key={log.request_id} className="bg-zinc-800 rounded p-2 text-xs">
            <div className="flex justify-between text-zinc-400">
              <span>{log.model}</span>
              <span className={log.status_code < 400 ? 'text-green-400' : 'text-red-400'}>
                {log.status_code}
              </span>
            </div>
            <div className="flex justify-between text-zinc-500 mt-1">
              <span>{log.provider}</span>
              <span>{log.latency_ms}ms</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
```

- [ ] **Step 7: 커밋**

```bash
git add apps/desktop/src/store/ apps/desktop/src/pages/
git commit -m "feat(ui): implement dashboard, providers, logs pages with Zustand stores"
```

---

## Phase 4 — 통합 + 마무리

### Task 12: 코어 바이너리 빌드 + Tauri 사이드카 연동

**Files:**
- Modify: `apps/desktop/src-tauri/tauri.conf.json`
- Modify: `apps/desktop/src-tauri/src/lib.rs`

- [ ] **Step 1: 코어 바이너리 빌드 스크립트 작성**

```bash
# apps/desktop/build-core.sh
#!/bin/bash
cargo build --release -p claw-proxy-core
mkdir -p src-tauri/binaries
cp ../../target/release/claw-proxy src-tauri/binaries/claw-proxy-$(rustc -Vv | grep host | cut -d' ' -f2)
```

```bash
chmod +x apps/desktop/build-core.sh
```

- [ ] **Step 2: Tauri lib.rs에 사이드카 시작 추가**

`apps/desktop/src-tauri/src/lib.rs`의 `.setup` 클로저 안에 다음 추가 (tray 설정 코드 앞):

```rust
// 코어 바이너리 시작
use tauri_plugin_shell::ShellExt;
let config_path = dirs::config_dir()
    .unwrap_or_default()
    .join("claw-proxy")
    .join("config.yaml");

let sidecar = app.shell().sidecar("claw-proxy").unwrap();
let (_rx, _child) = sidecar
    .args([config_path.to_string_lossy().as_ref()])
    .spawn()
    .expect("Failed to start claw-proxy core");
```

- [ ] **Step 3: 통합 빌드 테스트**

```bash
cd apps/desktop
./build-core.sh
npm run tauri dev
```

Expected: 메뉴바에 🦀 아이콘 표시, 클릭 시 팝오버 창 표시

- [ ] **Step 4: 커밋**

```bash
git add apps/desktop/build-core.sh apps/desktop/src-tauri/src/lib.rs
git commit -m "feat(desktop): integrate core sidecar startup with Tauri"
```

---

### Task 13: 최종 확인 + README

**Files:**
- Create: `README.md`

- [ ] **Step 1: 엔드투엔드 테스트 (코어만)**

```bash
# 터미널 1 — 코어 실행
cp configs/config.example.yaml config.yaml
# config.yaml에 실제 API 키 입력 후:
./target/release/claw-proxy config.yaml

# 터미널 2 — OpenAI 형식 요청
curl http://localhost:47380/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"gpt-4o","messages":[{"role":"user","content":"hi"}]}'

# 터미널 3 — Anthropic 형식 요청 (Claude Code 연동 검증)
export ANTHROPIC_BASE_URL=http://localhost:47380
curl http://localhost:47380/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: dummy" \
  -d '{"model":"claude-3-5-sonnet-20241022","max_tokens":100,"messages":[{"role":"user","content":"hi"}]}'
```

Expected: 두 요청 모두 각 형식에 맞는 응답 반환

- [ ] **Step 2: 전체 테스트 실행**

```bash
cargo test --workspace
```

Expected: 모든 테스트 PASS

- [ ] **Step 3: README 작성**

```markdown
# Claw Proxy

여러 AI 프로바이더를 단일 로컬 프록시로 통합하는 크로스 플랫폼 메뉴바 앱.

## 빠른 시작

### 코어만 실행 (CLI)

\`\`\`bash
cp configs/config.example.yaml config.yaml
# config.yaml에 API 키 입력
cargo run -p claw-proxy-core -- config.yaml
\`\`\`

### Claude Code 연동

\`\`\`bash
export ANTHROPIC_BASE_URL=http://localhost:47380
claude  # 기존 명령 그대로 사용
\`\`\`

### 데스크탑 앱 실행

\`\`\`bash
cd apps/desktop
./build-core.sh
npm install
npm run tauri dev
\`\`\`

## 포트

| 포트  | 역할 |
|-------|------|
| 47380 | 프록시 (OpenAI + Anthropic 형식) |
| 47381 | 관리 API |
| 47382 | WebSocket 로그 |
```

- [ ] **Step 4: 최종 커밋**

```bash
git add README.md
git commit -m "docs: add README with quick start guide"
```

# Claw Proxy — 설계 문서

**작성일**: 2026-04-10  
**상태**: 승인됨

---

## 1. 프로젝트 개요

Claw Proxy는 여러 AI 프로바이더(OpenAI, Claude, Gemini 등)를 단일 로컬 프록시 서버로 통합하는 크로스 플랫폼 개발자 도구다. OpenAI 호환 API와 Anthropic 호환 API를 모두 노출하여, Claude Code를 포함한 다양한 AI 도구들이 멀티 프로바이더/멀티 계정 라우팅을 투명하게 활용할 수 있게 한다.

**참고**: [vibeproxy](https://github.com/automazeio/vibeproxy)를 기반으로, 크로스 플랫폼 및 범용 API 형식 지원을 추가한 구현.

---

## 2. 핵심 요구사항

- 로컬 프록시 서버로 실행 (CLI-first)
- OpenAI 호환 엔드포인트 (`/v1/chat/completions`) 노출
- Anthropic 호환 엔드포인트 (`/v1/messages`) 노출 (Claude Code 지원)
- 멀티 프로바이더 아키텍처
- 멀티 계정 라우팅 (round-robin, failover)
- macOS + Windows 크로스 플랫폼
- Tauri 데스크탑 UI
- 코어 엔진과 UI 완전 분리

---

## 3. 프로젝트 구조

```
claw-proxy/
├── core/                        # Rust 크레이트 (lib + binary)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs              # 서버 진입점 (직접 실행 가능)
│       ├── proxy/               # axum HTTP 서버, 요청 핸들러
│       ├── router/              # 라우팅 로직 (round-robin, failover)
│       ├── providers/           # Provider 트레이트 + 구현체
│       │   ├── mod.rs           # Provider 트레이트 정의
│       │   ├── openai.rs        # OpenAI 구현체
│       │   ├── claude.rs        # Claude 스텁
│       │   └── gemini.rs        # Gemini 스텁
│       ├── normalizer/          # 요청/응답 형식 감지 및 변환
│       ├── config/              # YAML 로더 + hot reload
│       ├── admin/               # 관리 API (REST + WebSocket)
│       └── error.rs
├── apps/desktop/
│   ├── src-tauri/               # Tauri 백엔드 (코어 사이드카 제어)
│   └── src/                     # React UI
│       ├── pages/               # Dashboard, Providers, Logs, Settings
│       ├── components/
│       ├── store/               # Zustand 스토어
│       └── api/                 # 코어 HTTP/WebSocket 클라이언트
├── packages/shared-types/       # 공유 TS 타입
└── configs/
    └── config.example.yaml
```

---

## 4. 포트 배치

| 포트    | 역할                                          |
|---------|-----------------------------------------------|
| `47380` | 프록시 엔드포인트 (`/v1/chat/completions`, `/v1/messages`) |
| `47381` | 관리 API (REST — 상태 조회, 설정, 프로바이더 관리) |
| `47382` | WebSocket (실시간 요청 로그 스트리밍)          |

모든 포트는 `config.yaml`에서 변경 가능.

---

## 5. 아키텍처 — 데이터 흐름

```
클라이언트 요청 (OpenAI 형식 또는 Anthropic 형식)
    ↓ :47380
[Format Detector]
    ├── /v1/chat/completions  → OpenAI 형식
    └── /v1/messages          → Anthropic 형식
    ↓
[Normalizer] → InternalRequest (내부 통일 형식)
    ↓
[Router] → 라우팅 전략에 따라 Provider 선택
    ↓
[Provider Adapter] → 각 외부 API 형식으로 변환 후 전송
    ↓
[Response Normalizer] → 호출자 형식에 맞게 역변환
    ↓
클라이언트 응답 (요청한 형식 그대로 반환)

Tauri UI ←→ :47381 (REST 제어)
Tauri UI  ←  :47382 (WebSocket 로그 수신)
```

---

## 6. 코어 엔진 설계

### 6.1 내부 통일 형식

```rust
pub struct InternalRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub source_format: ApiFormat,
}

pub enum ApiFormat {
    OpenAI,
    Anthropic,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

pub enum Role { System, User, Assistant }
```

### 6.2 Provider 트레이트

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn chat_completion(
        &self,
        request: InternalRequest,
    ) -> Result<InternalResponse, ProviderError>;
}
```

모든 프로바이더는 이 트레이트를 구현. 라우터는 `Box<dyn Provider>`만 다룸.

### 6.3 프로바이더 구현 범위 (MVP)

| 프로바이더 | 상태         |
|------------|--------------|
| OpenAI     | 완전 구현    |
| Claude     | 스텁 (인터페이스만) |
| Gemini     | 스텁 (인터페이스만) |

### 6.4 라우팅 전략

```rust
pub enum RoutingStrategy {
    RoundRobin,
    Failover,
}
```

- **RoundRobin**: 계정 목록을 순서대로 순환
- **Failover**: 첫 번째 계정 실패 시 다음으로 자동 전환, 에러 카운트 추적

### 6.5 Config 구조 (YAML)

```yaml
server:
  proxy_port: 47380
  admin_port: 47381
  ws_port: 47382

routing:
  strategy: round_robin  # round_robin | failover

providers:
  - name: openai-account-1
    type: openai
    api_key: sk-...
    models: ["gpt-4o", "gpt-4o-mini"]

  - name: openai-account-2
    type: openai
    api_key: sk-...
    models: ["gpt-4o"]
```

### 6.6 Hot Reload

`notify` 크레이트로 `config.yaml` 변경 감지 → 재시작 없이 프로바이더/라우팅 설정 자동 갱신.

---

## 7. 관리 API

### REST (`:47381`)

| 메서드 | 경로                  | 설명                    |
|--------|-----------------------|-------------------------|
| GET    | `/status`             | 서버 상태, 업타임, 포트  |
| GET    | `/providers`          | 프로바이더 목록 및 상태  |
| POST   | `/providers`          | 프로바이더 추가          |
| DELETE | `/providers/:name`    | 프로바이더 삭제          |
| GET    | `/config`             | 현재 설정 조회           |
| PUT    | `/config`             | 설정 업데이트 (hot reload 트리거) |
| GET    | `/stats`             | 요청 통계                |

### WebSocket (`:47382`)

```json
{
  "timestamp": "2026-04-10T12:00:00Z",
  "request_id": "uuid",
  "model": "gpt-4o",
  "provider": "openai-account-1",
  "source_format": "anthropic",
  "status_code": 200,
  "latency_ms": 342
}
```

---

## 8. Tauri UI 설계

### 8.1 레이아웃

```
┌─────────────────────────────────────────┐
│ [●] Claw Proxy  포트: 47380  [시작/중지] │  ← 상단 상태 바
├──────┬──────────────────────────────────┤
│      │                                  │
│ 대시 │   메인 콘텐츠 영역               │
│ 보드 │                                  │
│      │                                  │
│ 프로 │                                  │
│ 바이 │                                  │
│ 더   │                                  │
│      │                                  │
│ 로그 │                                  │
│      │                                  │
│ 설정 │                                  │
└──────┴──────────────────────────────────┘
```

### 8.2 페이지 구성

**대시보드**
- 서버 상태 (실행 중 / 중지됨)
- 시작/중지 버튼
- 요약 카드: 활성 프로바이더 수, 총 요청 수, 평균 응답시간

**프로바이더 관리**
- 등록된 프로바이더/계정 목록
- 각 계정 상태 (활성 / 오류 / 비활성)
- 계정 추가/삭제/수정 → config.yaml에 반영

**실시간 로그**
- WebSocket(`:47382`)으로 스트리밍 수신
- 요청별: 타임스탬프, 모델, 프로바이더, 상태코드, 응답시간
- 필터: 프로바이더별, 상태별

**설정**
- 포트 변경
- 라우팅 전략 선택
- config.yaml 직접 편집

### 8.3 Zustand 스토어 구조

```typescript
serverStore    // 서버 상태, 시작/중지 액션
providerStore  // 프로바이더 목록, CRUD 액션
logStore       // 요청 로그 (최근 500건 유지)
configStore    // 설정 값
```

### 8.4 Tauri ↔ 코어 통신

- Tauri `sidecar`로 코어 바이너리 번들 및 프로세스 관리
- UI → 코어: `fetch`로 `:47381` REST API 호출
- UI ← 코어: `WebSocket`으로 `:47382` 로그 수신

---

## 9. Claude Code 연동

```bash
export ANTHROPIC_BASE_URL=http://localhost:47380
# 이후 claude 명령어 그대로 사용 — Claw Proxy를 통해 라우팅됨
```

Claw Proxy는 `/v1/messages` (Anthropic 형식) 요청을 수신하여 내부 통일 형식으로 변환 후 설정된 프로바이더로 전달.

---

## 10. 기술 스택

### 코어 (Rust)
- `tokio` — 비동기 런타임
- `axum` — HTTP 서버
- `reqwest` — HTTP 클라이언트
- `serde` / `serde_json` / `serde_yaml` — 직렬화
- `tracing` — 로깅
- `tower` — 미들웨어
- `notify` — 파일 변경 감지 (hot reload)
- `async-trait` — 비동기 트레이트

### 데스크탑 (TypeScript/React)
- Tauri 2.x
- React 18
- TypeScript
- TailwindCSS
- shadcn/ui
- Zustand

---

## 11. 개발 단계

| 단계 | 내용 |
|------|------|
| Phase 1 | 코어 프록시 서버 + OpenAI 어댑터 + 요청 정규화 레이어 |
| Phase 2 | 멀티 계정 라우팅 (round-robin, failover) + 관리 API |
| Phase 3 | Tauri UI 통합 (대시보드, 로그, 프로바이더 관리) |
| Phase 4 | Claude / Gemini 어댑터 구현 |

---

## 12. 제약사항

- 코어는 UI 없이 독립 실행 가능해야 함
- 라우터에 프로바이더별 로직 금지 (어댑터 패턴 엄수)
- 모든 프로바이더는 공통 `Provider` 트레이트 구현
- UI 레이어에 비즈니스 로직 금지
- API 키는 MVP에서 평문 YAML 저장 (이후 keyring으로 강화 예정)

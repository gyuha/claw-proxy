# Phase 01: Core Proxy Foundation - Research

**Researched:** 2026-04-10 [VERIFIED: research session timestamp]  
**Domain:** Rust local AI proxy core with OpenAI- and Anthropic-compatible ingress [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md]  
**Confidence:** HIGH [VERIFIED: codebase review + runtime checks + official protocol docs]

## User Constraints

No `*-CONTEXT.md` exists for this phase, so the planner should treat `PROJECT.md`, `ROADMAP.md`, `REQUIREMENTS.md`, `AGENTS.md`, and the current scaffold as the locked source of truth for Phase 1. [VERIFIED: phase init output; .planning/PROJECT.md; .planning/ROADMAP.md; .planning/REQUIREMENTS.md; AGENTS.md]

- Phase 1 is limited to a runnable standalone proxy, dual-format ingress, one internal request model, and a working OpenAI adapter. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md]
- The core must be independently runnable from YAML without depending on the desktop app. [VERIFIED: .planning/PROJECT.md; AGENTS.md; .planning/REQUIREMENTS.md]
- `/v1/chat/completions` and `/v1/messages` are both first-class entrypoints in MVP. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]
- Provider-specific logic must stay out of the router and live in adapters behind the shared `Provider` trait. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]
- Plaintext YAML secrets are accepted for MVP and should not be “fixed” by expanding scope in Phase 1. [VERIFIED: AGENTS.md; .planning/PROJECT.md; .planning/REQUIREMENTS.md]
- OpenAI is the only fully implemented upstream path required in Phase 1; Claude and Gemini adapters are explicitly later-phase work. [VERIFIED: .planning/ROADMAP.md; .planning/PROJECT.md]

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROXY-01 | Developer can run the proxy as a standalone local process by starting the Rust core with a YAML config file. [VERIFIED: .planning/REQUIREMENTS.md] | Bootstrap already loads YAML and starts proxy/admin listeners, but startup error handling and configured-port reporting need cleanup. [VERIFIED: core/src/main.rs; core/src/config/mod.rs; core/src/admin/handlers.rs] |
| PROXY-02 | Developer can send OpenAI-compatible requests to `/v1/chat/completions` and receive an OpenAI-compatible response from the proxy. [VERIFIED: .planning/REQUIREMENTS.md] | Route exists and the OpenAI adapter reaches the upstream API, but the current shape only supports a text-only subset and does not preserve newer chat-completions features such as developer role, multipart content, tools, or streaming. [VERIFIED: core/src/proxy/mod.rs; core/src/providers/openai.rs; core/src/normalizer/from_openai.rs; core/src/normalizer/to_openai.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] |
| PROXY-03 | Developer can send Anthropic-compatible requests to `/v1/messages` and receive an Anthropic-compatible response from the proxy. [VERIFIED: .planning/REQUIREMENTS.md] | Route exists, but the current Anthropic normalizer only accepts string content and the response path emits a narrow text-only message shape. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/from_anthropic.rs; core/src/normalizer/to_anthropic.rs] [CITED: https://platform.claude.com/docs/en/api/overview] |
| PROXY-04 | Developer can point Claude Code at the proxy with `ANTHROPIC_BASE_URL=http://localhost:<proxy_port>` and keep using the normal Claude workflow. [VERIFIED: .planning/REQUIREMENTS.md] | Claude Code gateway docs require Anthropic Messages format with `/v1/messages`, `/v1/messages/count_tokens`, and forwarding `anthropic-beta` plus `anthropic-version`; because Phase 1 only has an OpenAI upstream adapter, the proxy also needs server-side Anthropic-model aliasing so Claude Code does not require `ANTHROPIC_MODEL` overrides. The scaffold currently exposes only `/v1/messages` and does not alias Anthropic model names. [VERIFIED: core/src/proxy/mod.rs; core/src/providers/openai.rs] [CITED: https://code.claude.com/docs/en/llm-gateway] |
| PROXY-05 | Proxy request handling converts supported caller formats into one internal request model before routing to providers. [VERIFIED: .planning/REQUIREMENTS.md] | The scaffold already fans both ingress routes into `InternalRequest`, but the internal model is too narrow for full current wire compatibility because it reduces message content to plain strings. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/mod.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] [CITED: https://platform.claude.com/docs/en/api/overview] |
| PROV-01 | Operator can use a fully implemented OpenAI adapter in MVP for real upstream requests. [VERIFIED: .planning/REQUIREMENTS.md] | The adapter performs real HTTP requests to OpenAI today, but it hardcodes the upstream base URL, parses responses through ad-hoc JSON indexing, and is not yet shaped for deterministic integration testing. [VERIFIED: core/src/providers/openai.rs] |
</phase_requirements>

## Summary

The current scaffold is a viable starting point, not a finished MVP. `cargo check` and `cargo test -p claw-proxy-core` both pass, the binary boots from `configs/config.example.yaml`, and the admin status endpoint responds at runtime. [VERIFIED: `cargo check -p claw-proxy-core`; `cargo test -p claw-proxy-core`; `cargo run -p claw-proxy-core -- configs/config.example.yaml`; `curl http://127.0.0.1:47381/status`] The gap is compatibility depth: the scaffold proves only a text-only, non-streaming happy path plus direct OpenAI forwarding. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/mod.rs; core/src/providers/openai.rs]

The two biggest planning risks are Anthropic protocol coverage and Claude Code gateway compatibility. Anthropic’s current API requires `x-api-key`, `anthropic-version`, and JSON requests to `POST /v1/messages`, while Claude Code’s gateway contract additionally expects `/v1/messages/count_tokens` and forwarding of `anthropic-beta` and `anthropic-version`. Because Phase 1 routes those Anthropic-format requests into an OpenAI-only upstream adapter, normal Claude Code usage also requires server-side Anthropic-model aliasing rather than client-side model pinning. [CITED: https://platform.claude.com/docs/en/api/overview] [CITED: https://platform.claude.com/docs/en/api/versioning] [CITED: https://code.claude.com/docs/en/llm-gateway] The scaffold does not expose `count_tokens`, does not model headers, does not alias Anthropic model names, and currently collapses both OpenAI and Anthropic message content into `String`. [VERIFIED: core/src/proxy/mod.rs; core/src/providers/openai.rs; core/src/normalizer/mod.rs; core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs]

**Primary recommendation:** Plan Phase 1 as a strict text-only, non-streaming MVP that preserves the current dual-route architecture, adds the missing Claude Code gateway surfaces (`/v1/messages/count_tokens` and Anthropic header handling), resolves Anthropic-facing model IDs to configured OpenAI model IDs on the server, makes the OpenAI adapter injectable for tests, and explicitly rejects unsupported multipart/tool payloads with 4xx errors instead of silently coercing them. [VERIFIED: core/src/proxy/mod.rs; core/src/providers/openai.rs; core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] [CITED: https://code.claude.com/docs/en/llm-gateway] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create]

## Project Constraints (from CLAUDE.md)

No `CLAUDE.md` exists at the workspace root or project root, so there are no additional repo-specific directives beyond the GSD/project artifacts already listed above. [VERIFIED: filesystem check for `/Users/gyuha/workspace/claw-proxy-worktrees/feat-gsd-01/CLAUDE.md`; filesystem check for `/Users/gyuha/workspace/claw-proxy/CLAUDE.md`]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | 2021 edition [VERIFIED: core/Cargo.toml] | Core language/runtime target for the proxy. [VERIFIED: core/Cargo.toml] | The approved architecture is Rust-first for the proxy core. [VERIFIED: AGENTS.md; .planning/PROJECT.md] |
| tokio | `1` [VERIFIED: core/Cargo.toml] | Async runtime for HTTP serving and provider I/O. [VERIFIED: core/Cargo.toml] | The scaffold already uses Tokio for `main`, listeners, and channels. [VERIFIED: core/src/main.rs; core/Cargo.toml] |
| axum | `0.7` [VERIFIED: core/Cargo.toml] | Local proxy/admin HTTP server and WebSocket upgrade handling. [VERIFIED: core/Cargo.toml; core/src/proxy/mod.rs; core/src/admin/mod.rs] | Axum is already wired for both proxy and admin routers; replacing it would add no Phase 1 value. [VERIFIED: core/src/proxy/mod.rs; core/src/admin/mod.rs] |
| reqwest | `0.12` [VERIFIED: core/Cargo.toml] | Outbound HTTP client for upstream provider calls. [VERIFIED: core/Cargo.toml; core/src/providers/openai.rs] | The OpenAI adapter already depends on Reqwest and Phase 1 only needs one real upstream path. [VERIFIED: core/src/providers/openai.rs] |
| serde / serde_json / serde_yaml | `1` / `1` / `0.9` [VERIFIED: core/Cargo.toml] | Typed JSON ingress/egress and YAML config loading. [VERIFIED: core/Cargo.toml; core/src/config/mod.rs; core/src/normalizer/*] | Typed structs are the right boundary for strict compatibility tests and config safety. [VERIFIED: core/src/config/mod.rs; core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| tracing / tracing-subscriber | `0.1` / `0.3` [VERIFIED: core/Cargo.toml] | Runtime logging and filterable startup diagnostics. [VERIFIED: core/Cargo.toml; core/src/main.rs] | Use for startup, request lifecycle, and upstream failures, but avoid logging raw secrets or full provider error payloads. [VERIFIED: core/src/main.rs; runtime curl against proxy] |
| thiserror | `1` [VERIFIED: core/Cargo.toml] | Shared application error enum. [VERIFIED: core/Cargo.toml; core/src/error.rs] | Use for typed error boundaries instead of ad-hoc strings across handlers and adapters. [VERIFIED: core/src/error.rs; core/src/proxy/mod.rs] |
| uuid | `1` [VERIFIED: core/Cargo.toml] | Fallback request/response IDs. [VERIFIED: core/Cargo.toml; core/src/providers/openai.rs] | Use for synthetic IDs only when the upstream response is missing one. [VERIFIED: core/src/providers/openai.rs] |
| tokio broadcast + axum ws | built into current stack [VERIFIED: core/Cargo.toml; core/src/admin/ws.rs] | Live log fanout on the admin side. [VERIFIED: core/src/admin/mod.rs; core/src/admin/ws.rs] | Phase 1 does not depend on WebSocket logging, but the scaffold already has the shape. [VERIFIED: .planning/ROADMAP.md; core/src/admin/mod.rs] |
| notify | `6` [VERIFIED: core/Cargo.toml] | Config hot reload scaffold. [VERIFIED: core/Cargo.toml; core/src/config/watcher.rs] | Keep it out of Phase 1 execution scope; hot reload is Phase 2 work. [VERIFIED: .planning/ROADMAP.md; core/src/config/watcher.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Preserving the current chat-completions entrypoint | OpenAI Responses API [ASSUMED] | Not aligned with the locked requirement to support `/v1/chat/completions` in Phase 1. [VERIFIED: .planning/REQUIREMENTS.md] |
| Typed ingress structs | `serde_json::Value` everywhere [VERIFIED: current OpenAI adapter already uses ad-hoc JSON on response parse] | Dynamic JSON would make compatibility regressions harder to detect and weaken input validation. [VERIFIED: core/src/providers/openai.rs; core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] |
| Injected upstream base URL in provider constructors | Hardcoded `https://api.openai.com` [VERIFIED: core/src/providers/openai.rs] | Hardcoding simplifies production setup but makes deterministic integration tests and local mock upstreams much harder. [VERIFIED: core/src/providers/openai.rs] |

**Build/test command:**  
```bash
cargo test -p claw-proxy-core
```

**Version verification:** The stack above is intentionally based on the repo-pinned versions in `core/Cargo.toml` because the project constraints already approve Axum/Tokio/Reqwest for this codebase and Phase 1 does not require a dependency refresh. [VERIFIED: AGENTS.md; .planning/PROJECT.md; core/Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```text
core/src/
├── config/          # YAML loading and future watcher plumbing
├── proxy/           # Public compatibility routes
├── normalizer/      # Inbound/outbound wire-format conversion
├── providers/       # Upstream-specific adapters behind Provider
├── router/          # Provider selection only
└── admin/           # Status/providers/stats/ws side-channel

core/tests/
├── proxy_boot.rs    # standalone startup + configured ports
├── openai_ingress.rs
├── anthropic_ingress.rs
└── openai_provider.rs
```

The existing module split is already the right Phase 1 shape; planning should preserve it and add `core/tests/` for end-to-end coverage. [VERIFIED: core/src/*; no `core/tests/` directory found]

### Pattern 1: Canonical Internal Request/Response Boundary

**What:** Both public routes should normalize into one canonical request model, and provider adapters should return one canonical response model before caller-format re-encoding. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/mod.rs; .planning/REQUIREMENTS.md]

**When to use:** For every proxy request path in Phase 1, including `/v1/chat/completions`, `/v1/messages`, and `/v1/messages/count_tokens`. [VERIFIED: .planning/REQUIREMENTS.md] [CITED: https://code.claude.com/docs/en/llm-gateway]

**Example:**
```rust
// Source rationale: current proxy router + normalizer boundary
// Verified against core/src/proxy/mod.rs and core/src/normalizer/mod.rs
let internal = match api_format {
    ApiFormat::OpenAI => InternalRequest::from_openai(openai_req),
    ApiFormat::Anthropic => InternalRequest::from_anthropic(anthropic_req),
};

let provider = router.next_provider_for(&internal.model)?;
let internal_response = provider.chat_completion(internal).await?;
```

The only recommended change is widening the canonical model enough to represent supported text blocks and unsupported-shape rejections without leaking provider-specific structs into the router. [VERIFIED: core/src/normalizer/mod.rs; core/src/router/mod.rs] [ASSUMED]

### Pattern 2: Caller-Format-Preserving Egress

**What:** Re-encode the provider result back into the same caller-facing protocol that came in. [VERIFIED: core/src/proxy/mod.rs; .planning/REQUIREMENTS.md]

**When to use:** Always; Phase 1 success explicitly depends on OpenAI callers receiving OpenAI-compatible responses and Anthropic callers receiving Anthropic-compatible responses. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md]

**Example:**
```rust
// Source rationale: current route fan-in/fan-out shape
match internal.source_format {
    ApiFormat::OpenAI => Json(to_openai::convert(internal_response)),
    ApiFormat::Anthropic => Json(to_anthropic::convert(internal_response)),
}
```

Current code already follows this shape, but the response structs are narrow and should reject or explicitly defer unsupported tool/multipart cases rather than pretending to be fully protocol-compatible. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/to_openai.rs; core/src/normalizer/to_anthropic.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create]

### Pattern 3: Provider Adapter as the Only Upstream-Specific Boundary

**What:** Keep upstream URLs, auth headers, request encoding, and response parsing inside provider adapters only. [VERIFIED: core/src/providers/mod.rs; core/src/providers/openai.rs; .planning/REQUIREMENTS.md]

**When to use:** For OpenAI in Phase 1 and for Claude/Gemini later. [VERIFIED: .planning/ROADMAP.md; .planning/PROJECT.md]

**Example:**
```rust
// Source rationale: current Provider trait, with recommended constructor changes
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
```

The OpenAI implementation should keep this trait shape but gain constructor-time injection for `base_url` and a typed response decoder so the adapter is testable without hitting the public internet. [VERIFIED: core/src/providers/mod.rs; core/src/providers/openai.rs] [ASSUMED]

### Anti-Patterns to Avoid

- **Silent content coercion:** Mapping unsupported OpenAI multipart content or Anthropic content blocks into plain strings will create false compatibility and hard-to-debug client failures. [VERIFIED: core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] [CITED: https://platform.claude.com/docs/en/api/overview]
- **Hardcoded runtime metadata:** `admin/status` currently returns fixed ports instead of configured values, which is acceptable as scaffold evidence but not as Phase 1 truth. [VERIFIED: core/src/admin/handlers.rs; core/src/config/mod.rs]
- **Router-level provider knowledge:** Selecting providers without considering the internal request model is fine for Phase 1 only if model mapping is handled before routing or locked out of scope. [VERIFIED: core/src/router/mod.rs; core/src/providers/mod.rs] [ASSUMED]
- **Hardcoded upstream URL in the adapter:** This blocks local mock-upstream integration tests. [VERIFIED: core/src/providers/openai.rs]
- **Returning raw upstream error text directly to callers:** The current proxy emits provider error strings straight into 500 responses, which risks leaking details and makes 4xx/5xx boundaries inaccurate. [VERIFIED: core/src/proxy/mod.rs; runtime curl against `/v1/chat/completions`] 

## Gaps Between Scaffold and Phase 1 Success

| Area | Current State | Gap to Close |
|------|---------------|--------------|
| Standalone boot | Binary loads YAML and binds proxy/admin listeners successfully. [VERIFIED: core/src/main.rs; core/src/config/mod.rs; runtime `cargo run`] | Startup should return structured errors instead of `expect`/`unwrap`, and admin status should report configured ports. [VERIFIED: core/src/main.rs; core/src/admin/handlers.rs] |
| OpenAI ingress | `/v1/chat/completions` exists and forwards to OpenAI. [VERIFIED: core/src/proxy/mod.rs; core/src/providers/openai.rs] | Current request/response structs only cover text-only chat-completions and ignore modern shapes like developer role semantics, content arrays, tools, and streaming. [VERIFIED: core/src/normalizer/from_openai.rs; core/src/normalizer/to_openai.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] |
| Anthropic ingress | `/v1/messages` exists and can pass through the OpenAI adapter when the request is shaped as simple text. [VERIFIED: core/src/proxy/mod.rs; runtime curl against `/v1/messages`] | Phase 1 still lacks `/v1/messages/count_tokens`, Anthropic header handling, content-block support, and explicit Claude Code compatibility tests. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/from_anthropic.rs] [CITED: https://code.claude.com/docs/en/llm-gateway] [CITED: https://platform.claude.com/docs/en/api/getting-started] |
| Internal normalization | There is already one `InternalRequest` and one `InternalResponse`. [VERIFIED: core/src/normalizer/mod.rs] | The internal model cannot represent structured content or tool metadata, so unsupported requests must either be rejected clearly or Phase 1 must widen the model. [VERIFIED: core/src/normalizer/mod.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] [CITED: https://platform.claude.com/docs/en/api/overview] |
| OpenAI adapter | Real upstream HTTP path exists. [VERIFIED: core/src/providers/openai.rs; runtime curl against proxy] | Adapter needs typed parsing, injectable base URL, and tests that use a mock upstream instead of a real API key. [VERIFIED: core/src/providers/openai.rs] |
| Claude Code support | `ANTHROPIC_BASE_URL` is the documented gateway entrypoint. [CITED: https://code.claude.com/docs/en/llm-gateway] | The gateway contract is not met until `/v1/messages/count_tokens` exists, required headers are preserved, and Anthropic-facing model IDs are translated server-side onto configured OpenAI models. [VERIFIED: core/src/proxy/mod.rs; core/src/providers/openai.rs] [CITED: https://code.claude.com/docs/en/llm-gateway] |

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| HTTP serving | Custom Hyper routing from scratch [ASSUMED] | Axum routers and extractors already in the scaffold. [VERIFIED: core/src/proxy/mod.rs; core/src/admin/mod.rs; core/Cargo.toml] | The current app already proves the route split; replacing the server framework adds Phase 1 risk without value. [VERIFIED: core/src/proxy/mod.rs; core/src/admin/mod.rs] |
| YAML parsing | Manual config parsing [ASSUMED] | `serde_yaml` with typed config structs. [VERIFIED: core/src/config/mod.rs; core/Cargo.toml] | Current typed config is sufficient for Phase 1 bootstrapping and future watcher reuse. [VERIFIED: core/src/config/mod.rs; core/src/config/watcher.rs] |
| Claude Code integration | Custom client patching, local Claude Code forks, or required client-side model pinning [ASSUMED] | Documented gateway entrypoint via `ANTHROPIC_BASE_URL`, with server-side Anthropic-model aliasing inside the proxy so extra model env overrides are optional rather than required. [CITED: https://code.claude.com/docs/en/llm-gateway] [CITED: https://code.claude.com/docs/en/model-config] | Phase 1 should adapt the proxy to the documented client contract, not force users to reconfigure Claude Code beyond `ANTHROPIC_BASE_URL`. [VERIFIED: .planning/REQUIREMENTS.md] |
| Provider abstraction | Router-side `match provider_type` logic on every request [ASSUMED] | The shared `Provider` trait plus adapter implementations. [VERIFIED: core/src/providers/mod.rs; .planning/REQUIREMENTS.md] | This is an explicit architectural constraint and the key extensibility requirement for later phases. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md] |
| Most automated verification | Live calls to the real OpenAI API [ASSUMED] | Local mock-upstream integration tests with one optional manual smoke test against a real key. [VERIFIED: core/src/providers/openai.rs; no integration tests found] | Live upstream tests are slow, credential-dependent, and make regressions harder to isolate. [VERIFIED: runtime curl requires real API key; no credential in example config] |

**Key insight:** The highest-value Phase 1 work is not new architecture; it is tightening protocol boundaries so the existing architecture stops pretending that “string in, string out” equals OpenAI/Anthropic compatibility. [VERIFIED: core/src/normalizer/mod.rs; core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] [CITED: https://code.claude.com/docs/en/llm-gateway]

## Common Pitfalls

### Pitfall 1: Treating Anthropic Messages as Just `{ role, content: string }`

**What goes wrong:** Claude/Anthropic requests that rely on content blocks, token counting, or Claude Code gateway behaviors fail even though `/v1/messages` exists. [VERIFIED: core/src/proxy/mod.rs; core/src/normalizer/from_anthropic.rs] [CITED: https://code.claude.com/docs/en/llm-gateway] [CITED: https://platform.claude.com/docs/en/api/getting-started]

**Why it happens:** The scaffold models Anthropic message content as `String` and does not expose `/v1/messages/count_tokens`. [VERIFIED: core/src/normalizer/from_anthropic.rs; core/src/proxy/mod.rs]

**How to avoid:** Decide the supported Phase 1 Anthropic subset explicitly, implement `count_tokens`, preserve required headers, and return 4xx for unsupported request shapes. [CITED: https://code.claude.com/docs/en/llm-gateway] [ASSUMED]

**Warning signs:** Claude Code can connect to the proxy but fails on startup, model selection, or first real session calls. [CITED: https://code.claude.com/docs/en/llm-gateway] [ASSUMED]

### Pitfall 2: Silently Rewriting Unknown OpenAI Roles to `user`

**What goes wrong:** Semantics drift when OpenAI requests use roles or message shapes outside the current three-role text-only subset. [VERIFIED: core/src/normalizer/from_openai.rs] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create]

**Why it happens:** The current normalizer maps any non-`system` and non-`assistant` role to `user`. [VERIFIED: core/src/normalizer/from_openai.rs]

**How to avoid:** Preserve explicitly supported roles and reject unknown roles until the internal model is widened deliberately. [VERIFIED: core/src/normalizer/from_openai.rs] [ASSUMED]

**Warning signs:** Requests with developer instructions or tool calls behave inconsistently across callers. [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] [ASSUMED]

### Pitfall 3: Thinking “Runnable” Means “Phase-Correct”

**What goes wrong:** Planning underestimates remaining work because the scaffold boots and the unit tests pass. [VERIFIED: `cargo test -p claw-proxy-core`; runtime `cargo run`] 

**Why it happens:** Current tests cover config parsing, simple normalization, and provider/router stubs, but not end-to-end proxy compatibility. [VERIFIED: codebase grep for tests; no integration tests found]

**How to avoid:** Treat runtime boot as proof of scaffolding only; Phase 1 exit criteria need mock-upstream integration tests on both public routes. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md] [ASSUMED]

**Warning signs:** A change passes `cargo test` but breaks `curl` compatibility on `/v1/messages` or `/v1/chat/completions`. [VERIFIED: current test inventory; runtime curl against proxy] [ASSUMED]

### Pitfall 4: Hardcoding URLs and Ports in the Wrong Layers

**What goes wrong:** Admin APIs lie about runtime configuration and provider tests require public internet access. [VERIFIED: core/src/admin/handlers.rs; core/src/providers/openai.rs]

**Why it happens:** `admin/status` returns fixed ports and the OpenAI adapter hardcodes `https://api.openai.com/v1/chat/completions`. [VERIFIED: core/src/admin/handlers.rs; core/src/providers/openai.rs]

**How to avoid:** Carry configured ports into admin state and make provider base URLs constructor-configurable. [VERIFIED: core/src/admin/handlers.rs; core/src/providers/openai.rs] [ASSUMED]

**Warning signs:** Tests need real credentials or `status` disagrees with `config.yaml`. [VERIFIED: core/src/admin/handlers.rs; configs/config.example.yaml] [ASSUMED]

## Code Examples

Verified patterns from project sources and official docs:

### Dual ingress fan-in

```rust
// Source: core/src/proxy/mod.rs
Router::new()
    .route("/v1/chat/completions", post(handle_openai))
    .route("/v1/messages", post(handle_anthropic))
```

This is the correct Phase 1 public surface and should stay intact while compatibility depth is filled in behind it. [VERIFIED: core/src/proxy/mod.rs; .planning/REQUIREMENTS.md]

### Claude Code gateway entrypoint

```bash
# Source: https://code.claude.com/docs/en/llm-gateway
export ANTHROPIC_BASE_URL=http://localhost:47380
```

Claude Code officially supports a gateway-style base URL, so Phase 1 should meet that contract rather than invent a separate local integration story. [CITED: https://code.claude.com/docs/en/llm-gateway]

### Current OpenAI chat-completions shape includes more than plain text

```json
// Source: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create
{
  "messages": [
    { "role": "developer", "content": "You are a helpful assistant." },
    { "role": "user", "content": "Hello!" }
  ]
}
```

The planner should not treat OpenAI chat-completions as a permanently `user/system/assistant + string` protocol. [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Anthropic gateway compatibility assumed only `POST /v1/messages` [ASSUMED] | Claude Code gateway docs now require Anthropic Messages format with `/v1/messages`, `/v1/messages/count_tokens`, header forwarding for `anthropic-beta` plus `anthropic-version`, and Phase 1 must pair that with server-side Anthropic-model aliasing because the only upstream adapter is OpenAI. [CITED: https://code.claude.com/docs/en/llm-gateway] | Verified against current docs on 2026-04-10. [VERIFIED: research session] | Phase 1 planning must include `count_tokens`, header preservation, and server-side model aliasing to satisfy PROXY-04 as written. [VERIFIED: .planning/REQUIREMENTS.md] |
| OpenAI chat-completions treated as string-only messages [ASSUMED] | Current OpenAI docs show developer-role messages, multipart content arrays, streaming chunks, and tool calls in chat completions. [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] | Verified against current docs on 2026-04-10. [VERIFIED: research session] | Phase 1 must either support a documented subset or reject unsupported requests predictably. [ASSUMED] |

**Deprecated/outdated:**

- Treating successful boot plus unit tests as proof of API compatibility is outdated for this phase because the scaffold is already bootable while still missing key protocol surfaces. [VERIFIED: runtime `cargo run`; `cargo test -p claw-proxy-core`; core/src/proxy/mod.rs] [CITED: https://code.claude.com/docs/en/llm-gateway]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | A strict text-only, non-streaming subset is sufficient for Phase 1 if unsupported shapes return clear 4xx errors. [ASSUMED] | Summary; Architecture Patterns; Common Pitfalls | Claude Code or early callers may require tool use, streaming, or multipart content sooner than planned. |
| A2 | At least one configured OpenAI model is available to serve as the Anthropic alias target during Phase 1. [ASSUMED] | Open Questions; Validation Architecture | If an operator configures no OpenAI models, Anthropic-format requests will need to fail fast with a clear 4xx configuration error. |
| A3 | Injecting `base_url` into the OpenAI provider is the minimal testability refactor and does not require changing the `Provider` trait itself. [ASSUMED] | Architecture Patterns; Validation Architecture | If later adapters need richer metadata or streaming-specific hooks, the trait may need broader changes earlier. |

## Open Questions (RESOLVED)

1. **Minimum Anthropic subset for Phase 1** [VERIFIED: .planning/REQUIREMENTS.md]  
   Decision: Phase 1 supports text-only Anthropic workflows through `POST /v1/messages` and `POST /v1/messages/count_tokens`, with `anthropic-version` required and `anthropic-beta` preserved when present. Tool-use blocks, non-text content blocks, attachments, and streaming are out of scope for this phase and must return clear 4xx errors instead of being coerced. [RESOLVED: 2026-04-10 during plan-check revision]

2. **Anthropic model names over an OpenAI-only upstream adapter** [VERIFIED: core/src/providers/openai.rs; .planning/ROADMAP.md]  
   Decision: Phase 1 includes server-side model aliasing for the Anthropic-compatible ingress so Claude Code can work with only `ANTHROPIC_BASE_URL` changed. Anthropic-facing model IDs or aliases accepted at `/v1/messages` must be translated to configured OpenAI model IDs before provider routing, with the supported alias table documented and covered by tests. This keeps the MVP aligned with the design goal of transparent local routing. [RESOLVED: 2026-04-10 during plan-check revision]

3. **Streaming support in Phase 1** [VERIFIED: core/src/normalizer/mod.rs; core/src/providers/openai.rs]  
   Decision: `stream: true` is out of scope for Phase 1. Both OpenAI- and Anthropic-compatible ingress paths must reject streaming requests explicitly with 4xx responses and matching tests, rather than silently forcing `stream: false`. [RESOLVED: 2026-04-10 during plan-check revision]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Cargo | Build/test the Rust core [VERIFIED: project stack] | ✓ [VERIFIED: `cargo --version`] | `1.93.1` [VERIFIED: `cargo --version`] | — |
| Rust compiler | Compile the proxy binary [VERIFIED: project stack] | ✓ [VERIFIED: `rustc --version`] | `1.93.1` [VERIFIED: `rustc --version`] | — |
| curl | Manual smoke tests for local proxy endpoints [VERIFIED: README.md; research runtime checks] | ✓ [VERIFIED: `curl --version`] | `8.7.1` [VERIFIED: `curl --version`] | Use Rust integration tests only. [ASSUMED] |
| OpenAI API key | Live upstream verification of PROV-01 [VERIFIED: core/src/providers/openai.rs] | ✗ in repo/example config [VERIFIED: configs/config.example.yaml; runtime 401 from example key] | — | Use a local mock upstream for automated coverage. [ASSUMED] |
| Node/npm | Not required for Phase 1 execution, but available in the environment. [VERIFIED: scope; `node --version`; `npm --version`] | ✓ [VERIFIED: `node --version`; `npm --version`] | `v25.9.0` / `11.12.1` [VERIFIED: `node --version`; `npm --version`] | — |

**Missing dependencies with no fallback:**

- None for planning or automated verification. [VERIFIED: environment audit]

**Missing dependencies with fallback:**

- A real OpenAI API key is missing from the checked-in example config, but automated phase coverage can use a mock upstream and reserve one manual smoke test for a human-supplied key. [VERIFIED: configs/config.example.yaml; runtime 401 from example key] [ASSUMED]

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in unit/integration test harness via `cargo test` [VERIFIED: workspace layout; `cargo test -p claw-proxy-core`] |
| Config file | none [VERIFIED: repository file scan] |
| Quick run command | `cargo test -p claw-proxy-core` [VERIFIED: current passing command] |
| Full suite command | `cargo test --workspace` [VERIFIED: README.md; workspace `Cargo.toml`] |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROXY-01 | Boot with YAML config and serve proxy/admin on configured local ports. [VERIFIED: .planning/REQUIREMENTS.md] | integration/smoke | `cargo test -p claw-proxy-core boot_smoke_starts_servers -- --exact` [ASSUMED] | ❌ Wave 0 |
| PROXY-02 | OpenAI request to `/v1/chat/completions` returns OpenAI-compatible JSON against a mock upstream. [VERIFIED: .planning/REQUIREMENTS.md] | integration | `cargo test -p claw-proxy-core openai_chat_completion_roundtrip -- --exact` [ASSUMED] | ❌ Wave 0 |
| PROXY-03 | Anthropic request to `/v1/messages` returns Anthropic-compatible JSON against the same mock upstream. [VERIFIED: .planning/REQUIREMENTS.md] | integration | `cargo test -p claw-proxy-core anthropic_messages_roundtrip -- --exact` [ASSUMED] | ❌ Wave 0 |
| PROXY-04 | Claude Code gateway surfaces exist, preserve required Anthropic headers/count-token path, and alias Anthropic-facing model IDs server-side onto configured OpenAI models. [VERIFIED: .planning/REQUIREMENTS.md] [CITED: https://code.claude.com/docs/en/llm-gateway] | integration + one manual smoke | `cargo test -p claw-proxy-core anthropic_gateway_contract -- --exact` [ASSUMED] | ❌ Wave 0 |
| PROXY-05 | Both ingress formats normalize into the same internal request model. [VERIFIED: .planning/REQUIREMENTS.md] | unit | `cargo test -p claw-proxy-core normalizer::tests -- --nocapture` [VERIFIED: current unit tests exist] | ✅ |
| PROV-01 | OpenAI adapter sends the correct upstream request and maps success/error responses. [VERIFIED: .planning/REQUIREMENTS.md] | integration | `cargo test -p claw-proxy-core openai_provider_mock_upstream -- --exact` [ASSUMED] | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** `cargo test -p claw-proxy-core` [VERIFIED: current passing command]
- **Per wave merge:** `cargo test --workspace` [VERIFIED: README.md; workspace `Cargo.toml`]
- **Phase gate:** Full suite green plus one manual `curl` smoke for both public proxy routes and one manual Claude Code gateway smoke using only `ANTHROPIC_BASE_URL`, because PROXY-04 is in-scope literally. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md] [ASSUMED]

### Wave 0 Gaps

- [ ] `core/tests/proxy_boot.rs` — server boot + configured-port smoke for PROXY-01. [ASSUMED]
- [ ] `core/tests/openai_ingress.rs` — `/v1/chat/completions` round-trip fixture tests for PROXY-02. [ASSUMED]
- [ ] `core/tests/anthropic_ingress.rs` — `/v1/messages` plus `/v1/messages/count_tokens` gateway tests for PROXY-03 and PROXY-04. [ASSUMED]
- [ ] `core/tests/openai_provider.rs` — mock-upstream adapter coverage for PROV-01. [ASSUMED]
- [ ] Shared mock-upstream helper — deterministic Axum test server or equivalent fixture harness. [ASSUMED]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no [VERIFIED: no inbound auth requirement in Phase 1 requirements] | none in scope for local MVP. [VERIFIED: .planning/REQUIREMENTS.md] |
| V3 Session Management | no [VERIFIED: no user/session model in Phase 1 requirements] | none in scope for local MVP. [VERIFIED: .planning/REQUIREMENTS.md] |
| V4 Access Control | no [VERIFIED: Phase 1 does not define authz or multi-user access control] | keep admin/proxy local-only binding for MVP. [VERIFIED: core/src/main.rs; .planning/PROJECT.md] |
| V5 Input Validation | yes [VERIFIED: public JSON proxy endpoints are phase-critical] | Use typed `serde` request structs plus explicit 4xx rejection for unsupported roles/content/header combinations. [VERIFIED: core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] [ASSUMED] |
| V6 Cryptography | no for new crypto work [VERIFIED: Phase 1 accepts plaintext YAML secrets] | Never hand-roll crypto; keep secrets out of logs and defer secure storage to later scope. [VERIFIED: AGENTS.md; .planning/PROJECT.md] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Upstream error text reflected directly to callers | Information Disclosure | Map upstream failures to sanitized proxy errors and log structured details separately. [VERIFIED: core/src/proxy/mod.rs; runtime 500 response body] [ASSUMED] |
| Unsupported role/content coercion changes request meaning | Tampering | Reject unsupported payload variants with 400 instead of defaulting to `user` or flattening blocks into strings. [VERIFIED: core/src/normalizer/from_openai.rs; core/src/normalizer/from_anthropic.rs] [ASSUMED] |
| Missing Anthropic header forwarding breaks gateway behaviors | Denial of Service | Preserve `anthropic-version` and `anthropic-beta` across Anthropic-compatible ingress paths. [CITED: https://code.claude.com/docs/en/llm-gateway] [ASSUMED] |
| Plaintext provider secrets in YAML and logs | Information Disclosure | Keep YAML local-only in MVP, avoid echoing secrets in status/errors, and document later hardening as out of scope. [VERIFIED: AGENTS.md; .planning/PROJECT.md; configs/config.example.yaml] |

## Sources

### Primary (HIGH confidence)

- Local codebase review: `core/src/main.rs`, `core/src/proxy/mod.rs`, `core/src/normalizer/*`, `core/src/providers/*`, `core/src/config/*`, `core/src/admin/*`, `core/Cargo.toml`, `configs/config.example.yaml`. [VERIFIED: codebase reads in this session]
- OpenAI Chat Completions API reference: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create [CITED: official documentation]
- Claude API overview: https://platform.claude.com/docs/en/api/overview [CITED: official documentation]
- Claude API versioning: https://platform.claude.com/docs/en/api/versioning [CITED: official documentation]
- Claude API getting started / token counting references: https://platform.claude.com/docs/en/api/getting-started [CITED: official documentation]
- Claude Code LLM gateway docs: https://code.claude.com/docs/en/llm-gateway [CITED: official documentation]
- Claude Code model configuration docs: https://code.claude.com/docs/en/model-config [CITED: official documentation]

### Secondary (MEDIUM confidence)

- None. [VERIFIED: source audit]

### Tertiary (LOW confidence)

- None. [VERIFIED: source audit]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - the approved project stack and repo-pinned dependencies are explicit. [VERIFIED: AGENTS.md; .planning/PROJECT.md; core/Cargo.toml]
- Architecture: HIGH - the current scaffold, roadmap, and requirements all align on the same module boundaries and provider abstraction. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md; core/src/*]
- Pitfalls: HIGH - the biggest risks are directly observable in current code and explicitly called out by official OpenAI/Anthropic/Claude Code docs. [VERIFIED: core/src/*] [CITED: https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create] [CITED: https://code.claude.com/docs/en/llm-gateway]

**Research date:** 2026-04-10 [VERIFIED: research session timestamp]  
**Valid until:** 2026-05-10 for codebase findings; re-check official protocol docs sooner if Phase 1 planning slips, because API gateway expectations and model-config behavior can drift. [VERIFIED: codebase findings are local] [ASSUMED]

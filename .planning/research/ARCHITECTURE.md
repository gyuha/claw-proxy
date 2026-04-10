# Architecture Research

**Domain:** Cross-platform desktop local AI proxy / gateway for coding tools
**Researched:** 2026-04-11
**Confidence:** MEDIUM-HIGH

## Standard Architecture

### System Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                    Desktop UI (React/TS)                   │
├─────────────────────────────────────────────────────────────┤
│ Settings │ Provider Accounts │ Routing Rules │ Diagnostics │
├─────────────────────────────────────────────────────────────┤
│                Host Layer (Tauri / Rust IPC)               │
├─────────────────────────────────────────────────────────────┤
│ Runtime Control │ Config Validation │ Secret Access │ Logs │
├─────────────────────────────────────────────────────────────┤
│                 Proxy Runtime / Adapter Layer              │
├─────────────────────────────────────────────────────────────┤
│ Request Router │ Provider Adapters │ Account Pool Manager  │
├─────────────────────────────────────────────────────────────┤
│         Store / Stronghold / OS Networking / Files         │
└─────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| Desktop UI | Present settings, account health, routing controls, and diagnostics | React views backed by typed IPC calls |
| Host layer | Own lifecycle commands, validation, permissions, and OS integration | Tauri commands in Rust |
| Proxy runtime | Accept local requests and dispatch them to provider adapters | Embedded core or supervised sidecar process |
| Provider adapter layer | Normalize provider-specific auth, request shaping, and error handling | Adapter interface with per-provider modules |
| Account pool manager | Track multiple accounts, eligibility, backoff, and fallback order | Shared routing service with health-aware selection |
| Persistence layer | Separate non-secret settings from secret material | Store plugin for config, Stronghold for secrets |

## Recommended Project Structure

```text
src/
├── app/                 # Desktop shell, routes, layout, app bootstrap
├── features/            # UI features by domain (providers, routing, diagnostics)
├── components/          # Reusable presentation components
├── lib/                 # Shared TS utilities, validation schemas, generated clients
└── styles/              # Design tokens and global styles

src-tauri/
├── src/
│   ├── commands/        # IPC commands exposed to the UI
│   ├── runtime/         # Proxy lifecycle, health checks, sidecar/embed orchestration
│   ├── providers/       # Provider adapter contracts and implementations
│   ├── routing/         # Account selection, failover, rule evaluation
│   ├── storage/         # Stronghold/store orchestration and migration logic
│   └── diagnostics/     # Event logging, status snapshots, recovery helpers
└── capabilities/        # Tauri capabilities and permissions
```

### Structure Rationale

- **UI and host are separated by explicit IPC contracts:** This keeps the frontend from reaching into proxy/runtime internals directly.
- **Provider and routing logic live in Rust-side modules:** The trust-sensitive, stateful parts of the system remain close to lifecycle and secret handling.
- **Diagnostics is its own subsystem:** Logging and recovery UX should not be bolted on after the fact.

## Architectural Patterns

### Pattern 1: Adapter Boundary for Providers

**What:** Define a single provider contract for auth state, model discovery, request translation, and error normalization.  
**When to use:** Immediately; this is foundational to multi-provider support.  
**Trade-offs:** Slightly more upfront design work, but prevents provider-specific logic from leaking across the app.

### Pattern 2: Host-Owned Runtime State

**What:** Treat the host/runtime layer as the source of truth for proxy status, provider health, and current routing config.  
**When to use:** For all lifecycle and diagnostics features.  
**Trade-offs:** More IPC/event plumbing, but avoids UI-only state drift and "looks running but isn't" bugs.

### Pattern 3: Split Secret and Non-Secret Persistence

**What:** Store credentials in strong secret storage and everything else in a separate config store.  
**When to use:** Always.  
**Trade-offs:** Slightly more storage plumbing, but safer migrations and clearer audit boundaries.

## Data Flow

### Request Flow

```text
External AI Tool
    ↓
Local Proxy Endpoint
    ↓
Routing Engine -> Account Pool -> Provider Adapter
    ↓
Provider API
    ↓
Normalized Response
    ↓
External AI Tool
```

### State Management

```text
Persistent Config + Secrets
    ↓
Host Runtime Snapshot
    ↓
UI Query / Event Subscription
    ↓
User Actions -> IPC Commands -> Host Validation -> Persist + Apply
```

### Key Data Flows

1. **Config apply flow:** User edits settings -> UI validates shape -> host re-validates -> runtime applies changes -> new status snapshot emitted
2. **Account failure flow:** Request fails -> adapter normalizes reason -> account pool marks backoff -> fallback account/provider chosen -> diagnostics updated
3. **Onboarding flow:** User connects provider -> host persists secret -> adapter verifies session -> UI receives health state and next-step instructions

## Anti-Patterns

### Anti-Pattern 1: UI-Centric Proxy State

**What people do:** Treat the frontend as the main owner of runtime truth.  
**Why it's wrong:** Runtime can crash, misconfigure, or rotate accounts without the UI knowing.  
**Do this instead:** Keep runtime truth in the host layer and stream snapshots/events to the UI.

### Anti-Pattern 2: Provider Logic Scattered Across Screens

**What people do:** Put auth rules, rate-limit handling, and model mapping in UI feature code.  
**Why it's wrong:** Every new provider multiplies hidden coupling.  
**Do this instead:** Centralize provider contracts and expose stable read/write commands to the UI.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| LLM provider APIs | Provider adapters behind a common contract | Normalize auth, model mapping, and error handling |
| Coding tools (Claude Code, Codex) | Local compatibility endpoint plus generated setup instructions | Keep first-class support narrow in v1 |
| OS services | Tauri plugins and platform APIs | Required for startup, storage, networking, updates, and packaging |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| UI ↔ host commands | Typed IPC | Validate payloads both sides |
| Host ↔ runtime | Direct module calls or supervised sidecar protocol | Keep restart and health semantics explicit |
| Routing ↔ providers | Internal trait/interface boundary | Enables adding providers without rewriting routing |

## Build Order Implications

1. Embedded runtime spine and typed IPC boundary first
2. Thin desktop shell and status surface second
3. Local proxy ingress, lifecycle, and health plumbing third
4. Secret storage and one provider adapter end to end fourth
5. Routing and account pooling fifth
6. Diagnostics, onboarding, and packaging hardening after core flows exist

## Sources

- VibeProxy README — validated local desktop proxy control-plane shape
- Tauri architecture and plugin docs — validated desktop-host + plugin capabilities
- Feature research in `.planning/research/FEATURES.md` — used to prioritize architecture boundaries

---
*Architecture research for: cross-platform desktop local AI proxy / gateway*
*Researched: 2026-04-11*

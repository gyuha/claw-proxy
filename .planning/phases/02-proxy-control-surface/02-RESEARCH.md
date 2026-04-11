# Phase 2: Proxy Control Surface - Research

**Researched:** 2026-04-11
**Domain:** Embedded local proxy runtime orchestration for a Tauri 2 desktop app
**Confidence:** HIGH

<user_constraints>
## User Constraints

No phase-specific `*-CONTEXT.md` exists for Phase 2, so the active constraints come from `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, existing Phase 1 artifacts, `AGENTS.md`, and `DESIGN.md` [VERIFIED: gsd-tools init] [VERIFIED: repo grep].
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROX-01 | User can start and stop the local proxy runtime from the UI | Supervised Rust-owned server task, typed start/stop commands, and lifecycle snapshot updates [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: .planning/ROADMAP.md] [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] [CITED: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html] |
| PROX-02 | User can configure the proxy listen host, port, and base endpoint from the UI | Host-side settings schema, validation, and renderer form draft pattern with typed command submission [VERIFIED: .planning/REQUIREMENTS.md] [CITED: https://v2.tauri.app/develop/calling-rust/] [CITED: https://docs.rs/url/latest/url/struct.Url.html] |
| PROX-03 | User can apply configuration changes without manually editing config files | SQLite-backed host persistence plus transactional apply/restart flow from commands instead of renderer file writes [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: .planning/ROADMAP.md] [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://docs.rs/rusqlite_migration/latest/rusqlite_migration/] |
| PROX-04 | User can see whether the local proxy is healthy, stopped, or misconfigured | Runtime snapshot evolution, low-volume lifecycle events, and UI-side event subscription with typed snapshot refresh [VERIFIED: .planning/REQUIREMENTS.md] [CITED: https://v2.tauri.app/develop/calling-frontend/] [VERIFIED: repo grep] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Keep app wiring in `src/app/`, reusable UI in `src/components/`, feature logic in `src/features/`, shared IPC helpers in `src/lib/`, styles in `src/styles/`, Rust commands in `src-tauri/src/commands/`, and Rust state/models in `src-tauri/src/runtime/` plus `src-tauri/src/models/` [VERIFIED: AGENTS.md].
- Preserve the Phase 1 frontend/backend boundary: Rust owns canonical payloads in snake_case and TypeScript normalizes them to camelCase before components consume them [VERIFIED: AGENTS.md] [VERIFIED: repo grep].
- Use `DESIGN.md` as the UI source of truth; Phase 2 UI should extend the existing warm Notion-inspired shell instead of introducing a conflicting visual system [VERIFIED: AGENTS.md] [VERIFIED: DESIGN.md].
- Use Vitest with Testing Library for frontend behavior near the feature code, and focused Rust unit tests near the Rust module under test [VERIFIED: AGENTS.md].
- Review any Tauri capability expansion carefully because plugin or command exposure changes desktop permissions and attack surface [VERIFIED: AGENTS.md].

## Summary

Phase 2 should keep the Phase 1 boundary intact: the Rust host remains the only writer for proxy lifecycle, settings, persistence, and health, while the React shell stays on typed command adapters plus event-driven refreshes [VERIFIED: repo grep] [CITED: https://v2.tauri.app/develop/state-management/] [CITED: https://v2.tauri.app/develop/calling-frontend/]. The current codebase only exposes a synthetic `RuntimeSnapshot` over `get_runtime_snapshot`, `initialize_runtime_state`, and `ping_runtime`, so this phase needs to add real supervision, persisted settings, and clearer status semantics without moving authority into the renderer [VERIFIED: repo grep].

The most stable Phase 2 stack is an in-process Rust HTTP server using `axum` on `tokio`, supervised with `CancellationToken`, with non-secret settings persisted in host-owned SQLite via `rusqlite` and schema migrations handled by `rusqlite_migration` [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] [CITED: https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html] [CITED: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html] [CITED: https://docs.rs/rusqlite_migration/latest/rusqlite_migration/] [VERIFIED: crates.io]. That aligns with the roadmap’s SQLite requirement for non-secret state and avoids exposing the database to the frontend through Tauri guest plugins [VERIFIED: .planning/ROADMAP.md] [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/].

**Primary recommendation:** Build Phase 2 around a supervised Rust-owned `axum` server, host-side SQLite settings persistence, and low-volume Tauri lifecycle events that invalidate and refresh the canonical runtime snapshot [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] [CITED: https://v2.tauri.app/develop/calling-frontend/] [VERIFIED: repo grep].

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `tauri` | `2.10.3` (published 2026-03-04) [VERIFIED: crates.io] | Desktop host, managed state, commands, events, and capability-aware IPC | Phase 1 already uses Tauri-managed state and command handlers, and Tauri v2 explicitly documents managed state, commands, events, and channels for this boundary [VERIFIED: repo grep] [CITED: https://v2.tauri.app/develop/state-management/] [CITED: https://v2.tauri.app/develop/calling-rust/] [CITED: https://v2.tauri.app/develop/calling-frontend/] |
| `axum` | `0.8.8` (published 2025-12-20) [VERIFIED: crates.io] | Embedded local HTTP ingress and routing for the proxy runtime | `axum::serve` supports `with_graceful_shutdown`, and `Router` supports typed shared state, which matches Phase 2 lifecycle control and Phase 4 growth into real compatibility routes [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] [CITED: https://docs.rs/axum/latest/axum/struct.Router.html] |
| `tokio` | `1.51.1` (published 2026-04-08) [VERIFIED: crates.io] | Async TCP listener, timing, and task execution for the proxy runtime | `TcpListener::bind` and `local_addr` give the exact listener control Phase 2 needs, and Tauri itself already depends on `tokio` [CITED: https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html] [CITED: https://docs.rs/crate/tauri/latest] |
| `rusqlite` | `0.39.0` (published 2026-03-15) [VERIFIED: crates.io] | Host-owned SQLite persistence for non-secret proxy settings | The roadmap explicitly calls for SQLite-backed non-secret persistence, and host-side `rusqlite` preserves runtime ownership better than renderer-facing storage plugins [VERIFIED: .planning/ROADMAP.md] [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `tokio-util` | `0.7.18` (published 2026-01-04) [VERIFIED: crates.io] | `CancellationToken` for orderly stop/restart of the proxy task | Use for start/stop/apply flows where a running listener must shut down cooperatively instead of relying on detached task aborts [CITED: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html] |
| `rusqlite_migration` | `2.5.0` (published 2026-03-21) [VERIFIED: crates.io] | Lightweight schema migrations for the settings database | Use from the first schema so Phase 2 does not seed manual migration debt; the crate documents `Migrations::to_latest()` and built-in validation [CITED: https://docs.rs/rusqlite_migration/latest/rusqlite_migration/] |
| `url` | `2.5.8` (published 2026-01-05) [VERIFIED: crates.io] | Absolute URL parsing and safe endpoint joining | Use when deriving or validating the effective local base URL from host, port, and endpoint values; avoid string concatenation for URL composition [CITED: https://docs.rs/url/latest/url/struct.Url.html] |
| `@tauri-apps/api` | `2.10.1` (published 2026-02-03) [VERIFIED: npm registry] | Frontend `invoke` and event subscription APIs | Use in feature-level adapters and listeners, not directly in components; Phase 1 already follows this pattern [VERIFIED: repo grep] [CITED: https://v2.tauri.app/develop/calling-rust/] [CITED: https://v2.tauri.app/develop/calling-frontend/] |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `axum` | raw `hyper` [ASSUMED] | Lower-level control is possible, but Phase 2 mainly needs fast routing plus graceful shutdown, so `axum` reduces boilerplate and preserves a clean path into Phase 4 routes [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] |
| host-side `rusqlite` | `tauri-plugin-sql` `2.4.0` (published 2026-04-04) [VERIFIED: npm registry] | The SQL plugin is designed for frontend communication with SQL databases and requires capability grants for guest-side access, which conflicts with the repo’s Rust-owned runtime boundary [CITED: https://v2.tauri.app/plugin/sql/] [VERIFIED: repo grep] |
| SQLite + migrations | `tauri-plugin-store` `2.4.2` (published 2026-01-08) [VERIFIED: npm registry] | Store is a file-backed key-value mechanism, not SQLite, and does not match the roadmap’s persistence requirement for this phase [CITED: https://v2.tauri.app/plugin/store/] [VERIFIED: .planning/ROADMAP.md] |
| Tauri events for all updates | Tauri channels | Channels are the better choice for high-throughput streams, but lifecycle/status changes are small and infrequent, so Phase 2 can use events and reserve channels for later log/streaming surfaces [CITED: https://v2.tauri.app/develop/calling-frontend/] |

**Installation:**
```bash
cargo add axum
cargo add tokio --features net,sync,time,rt-multi-thread
cargo add tokio-util
cargo add rusqlite
cargo add rusqlite_migration
cargo add url
```

## Architecture Patterns

### Recommended Project Structure
```text
src/
├── components/proxy/              # Lifecycle buttons, settings form, health/status cards [VERIFIED: AGENTS.md]
├── features/proxy/                # Proxy form state, subscriptions, adapters, tests [VERIFIED: AGENTS.md]
├── features/runtime/              # Shared runtime snapshot models and existing shell hook [VERIFIED: repo grep]
└── lib/ipc/                       # Typed Tauri command + event wrappers [VERIFIED: AGENTS.md]

src-tauri/src/
├── commands/proxy.rs              # start/stop/apply/settings commands [VERIFIED: AGENTS.md]
├── models/proxy_settings.rs       # persisted settings contract [VERIFIED: AGENTS.md]
├── models/runtime_snapshot.rs     # expanded status and health snapshot [VERIFIED: repo grep]
├── runtime/proxy_runtime.rs       # supervised server task and health transitions [RECOMMENDATION]
├── runtime/persistence.rs         # rusqlite connection, migrations, load/save [RECOMMENDATION]
└── events/runtime.rs              # low-volume invalidation event names [VERIFIED: repo grep]
```

### Pattern 1: Supervised In-Process Proxy Runtime
**What:** Keep a host-owned supervisor that validates config, binds the listener, spawns the server task, tracks the bound address, and records lifecycle failures in the runtime snapshot [CITED: https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html] [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] [CITED: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html].
**When to use:** Use for `start_proxy_runtime`, `stop_proxy_runtime`, and config apply flows that need clear healthy/stopped/misconfigured outcomes [VERIFIED: .planning/ROADMAP.md].
**Example:**
```rust
// Source: https://docs.rs/axum/latest/axum/serve/struct.Serve.html
// Source: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

let listener = TcpListener::bind(bind_addr).await?;
let bound_addr = listener.local_addr()?;
let cancel = CancellationToken::new();
let shutdown = cancel.clone();

let app = Router::new()
  .route("/health", get(health_handler))
  .with_state(shared_state);

let server = axum::serve(listener, app)
  .with_graceful_shutdown(async move {
    shutdown.cancelled().await;
  });

let task = tokio::spawn(async move {
  if let Err(error) = server.await {
    report_runtime_error(bound_addr, error);
  }
});
```

### Pattern 2: Validate, Persist, Then Apply
**What:** Treat settings edits as a host command pipeline: normalize input, validate host/port/base endpoint, persist in SQLite in one transaction, then update live runtime state and emit an invalidation event [CITED: https://v2.tauri.app/develop/calling-rust/] [CITED: https://docs.rs/rusqlite_migration/latest/rusqlite_migration/] [VERIFIED: .planning/ROADMAP.md].
**When to use:** Use for the proxy settings form and any “Apply” action from the desktop UI [VERIFIED: .planning/ROADMAP.md].
**Example:**
```rust
// Source: https://v2.tauri.app/develop/calling-rust/
#[tauri::command]
async fn apply_proxy_settings(
  app: tauri::AppHandle,
  state: tauri::State<'_, AppRuntimeState>,
  input: ProxySettingsInput,
) -> Result<RuntimeSnapshot, ProxyCommandError> {
  let settings = ProxySettings::try_from(input)?;
  state.persistence().save_settings(&settings)?;
  state.apply_settings(app, settings).await
}
```

### Pattern 3: Evented Invalidation, Typed Snapshot Refresh
**What:** Use Tauri events only as low-volume “state changed” notifications, then refetch the authoritative snapshot through the existing typed command adapter instead of treating event payloads as the primary contract [CITED: https://v2.tauri.app/develop/calling-frontend/] [VERIFIED: repo grep].
**When to use:** Use for health/lifecycle/status changes that the UI should react to immediately [VERIFIED: .planning/ROADMAP.md].
**Example:**
```ts
// Source: https://v2.tauri.app/develop/calling-frontend/
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('runtime://snapshot-updated', async () => {
  const nextSnapshot = await getRuntimeSnapshot();
  setSnapshot(nextSnapshot);
});

// Always dispose on component unmount.
return () => {
  void unlisten();
};
```

### Anti-Patterns to Avoid
- **Renderer-owned proxy truth:** Do not let React write DB rows or derive runtime health locally; Phase 1 already established Rust-owned state as the authority [VERIFIED: repo grep].
- **Naive stop-then-start apply flow:** If a config update stops the running listener before candidate settings are known good, the user can end up unexpectedly down; use validation plus rollback-safe apply sequencing [RECOMMENDATION].
- **Events as a typed data bus:** Tauri documents that events have no strong type support and are not designed for high-throughput traffic; use them for invalidation, not for logs or heavy payloads [CITED: https://v2.tauri.app/develop/calling-frontend/].
- **`Arc<Mutex<T>>` inside Tauri state by default:** Tauri already manages shared ownership for `State`; only add extra synchronization where the inner resource truly needs it [CITED: https://v2.tauri.app/develop/state-management/].

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Local HTTP proxy ingress | Custom `TcpListener` accept loop plus manual route dispatch | `axum` + `tokio::net::TcpListener` | The stack already gives route composition, shared state, bind inspection, and graceful shutdown hooks with less lifecycle risk [CITED: https://docs.rs/axum/latest/axum/serve/struct.Serve.html] [CITED: https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html] |
| Stop/restart signaling | Shared booleans or detached task aborts | `tokio_util::sync::CancellationToken` | The token API is built for cross-task cancellation and wakeup semantics, which maps directly to stop/apply flows [CITED: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html] |
| SQLite schema evolution | Ad hoc `CREATE TABLE IF NOT EXISTS` calls scattered across commands | `rusqlite_migration` | The crate gives explicit ordered migrations, atomic `to_latest`, and migration validation from the start [CITED: https://docs.rs/rusqlite_migration/latest/rusqlite_migration/] |
| URL/base endpoint composition | String concatenation for `host + ":" + port + endpoint` | `url::Url` | `Url::parse` and `join` prevent malformed absolute URL assembly and path-prefix mistakes [CITED: https://docs.rs/url/latest/url/struct.Url.html] |
| Renderer-side settings persistence | JSON files or guest-side SQL/store plugins as the source of truth | Host-owned `rusqlite` behind Tauri commands | The SQL plugin is explicitly frontend-facing, and Store is a file-backed key-value layer rather than SQLite; neither matches the roadmap boundary as cleanly [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/] [VERIFIED: .planning/ROADMAP.md] |

**Key insight:** Phase 2 is mostly about authority and failure handling, not raw CRUD; the wrong abstractions here will either leak host control into the renderer or make safe apply/restart logic much harder in later phases [VERIFIED: repo grep] [VERIFIED: .planning/ROADMAP.md].

## Common Pitfalls

### Pitfall 1: Keeping Generic `ready/error` Statuses Instead of Proxy-Specific Health
**What goes wrong:** The UI cannot distinguish “stopped by user,” “config invalid,” and “listener running” if Phase 2 keeps only the Phase 1 placeholder statuses [VERIFIED: repo grep].
**Why it happens:** Phase 1’s snapshot was intentionally synthetic and only needed to prove the shell boundary [VERIFIED: .planning/phases/01-runtime-foundation/01-02-SUMMARY.md] [VERIFIED: .planning/phases/01-runtime-foundation/01-03-SUMMARY.md].
**How to avoid:** Expand `RuntimeSnapshot` to capture proxy lifecycle state, last bind error, effective local base URL, and last successful health timestamp [RECOMMENDATION].
**Warning signs:** The UI has to infer health from error strings or loading flags instead of a first-class status enum [RECOMMENDATION].

### Pitfall 2: Treating Tauri Events as the Canonical State Contract
**What goes wrong:** Event payloads drift, listeners leak, and the renderer starts trusting untyped JSON strings instead of the typed command surface [CITED: https://v2.tauri.app/develop/calling-frontend/].
**Why it happens:** Tauri events are convenient, but the docs explicitly note that events have no strong type support and are not designed for high-throughput scenarios [CITED: https://v2.tauri.app/develop/calling-frontend/].
**How to avoid:** Emit small invalidation signals, refresh through `get_runtime_snapshot`, and always dispose listeners on unmount [CITED: https://v2.tauri.app/develop/calling-frontend/] [VERIFIED: repo grep].
**Warning signs:** Components call `listen()` directly with no cleanup or hold business-critical fields only in event payload types [CITED: https://v2.tauri.app/develop/calling-frontend/] [VERIFIED: repo grep].

### Pitfall 3: Exposing Persistence Plugins to the Renderer Because They Are Fast to Wire
**What goes wrong:** Runtime settings become writable from the guest side, capability surface expands, and the Phase 1 host-owned truth model gets undermined [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/] [VERIFIED: repo grep].
**Why it happens:** Tauri’s SQL and Store plugins are convenient and well-documented for webview use, but Phase 2 wants host-owned SQLite persistence, not renderer-side storage primitives [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/] [VERIFIED: .planning/ROADMAP.md].
**How to avoid:** Keep SQLite access private to Rust and expose only typed settings commands plus normalized snapshots to the renderer [RECOMMENDATION].
**Warning signs:** New capability entries like `sql:allow-execute` or `store:default` appear just to support the settings screen [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/] [VERIFIED: repo grep].

### Pitfall 4: Holding the Wrong Kind of Lock Across Await Points
**What goes wrong:** Lifecycle commands deadlock or serialize too much work when a synchronous lock guards an async IO resource for the whole command duration [CITED: https://v2.tauri.app/develop/state-management/].
**Why it happens:** Tauri state examples often start with standard `Mutex`/`RwLock`, but the docs call out async mutexes specifically for shared mutable access to IO resources such as database connections [CITED: https://v2.tauri.app/develop/state-management/].
**How to avoid:** Keep synchronous locks around short metadata updates only; isolate long-running proxy task handles and database work so guards are not held across awaits [CITED: https://v2.tauri.app/develop/state-management/] [RECOMMENDATION].
**Warning signs:** A command acquires a lock, awaits bind/start/stop work, and only then releases the guard [RECOMMENDATION].

## Code Examples

Verified patterns from official sources:

### Graceful Runtime Shutdown
```rust
// Source: https://docs.rs/axum/latest/axum/serve/struct.Serve.html
// Source: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html
let cancel = CancellationToken::new();
let shutdown = cancel.clone();

axum::serve(listener, router)
  .with_graceful_shutdown(async move {
    shutdown.cancelled().await;
  })
  .await?;
```

### Listener Bind and Effective Address Capture
```rust
// Source: https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html
let listener = TcpListener::bind(bind_addr).await?;
let effective_addr = listener.local_addr()?;
```

### UI Event Subscription with Cleanup
```ts
// Source: https://v2.tauri.app/develop/calling-frontend/
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('runtime://snapshot-updated', handleUpdate);
unlisten();
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Tauri 1 / pre-v2 migration patterns | Tauri 2 commands, managed state, events, channels, and capabilities [CITED: https://v2.tauri.app/develop/calling-rust/] [CITED: https://v2.tauri.app/develop/calling-frontend/] | Tauri 2.0.0 GA on 2024-10-02 [VERIFIED: npm registry] [VERIFIED: crates.io] | Phase 2 should stay on typed commands plus minimal event invalidation instead of inventing a parallel bridge [VERIFIED: repo grep] |
| File-backed guest-side key/value settings | Host-owned SQLite settings with explicit migrations [CITED: https://v2.tauri.app/plugin/store/] [CITED: https://docs.rs/rusqlite_migration/latest/rusqlite_migration/] | Current roadmap requires SQLite for this phase [VERIFIED: .planning/ROADMAP.md] | Better durability and future routing/provider joins without giving the renderer raw persistence authority [VERIFIED: .planning/ROADMAP.md] |
| Raw `invoke()` use inside components | Feature-level adapters and hooks over typed IPC helpers [VERIFIED: .planning/phases/01-runtime-foundation/01-02-SUMMARY.md] [VERIFIED: .planning/phases/01-runtime-foundation/01-03-SUMMARY.md] | Established in Phase 1 on 2026-04-11 [VERIFIED: repo grep] | Phase 2 UI should extend `src/features/*` adapters rather than calling Tauri APIs from components [VERIFIED: AGENTS.md] [VERIFIED: repo grep] |

**Deprecated/outdated:**
- Using Tauri events for high-throughput or strongly typed streaming is outdated for this problem; Tauri documents channels as the optimized mechanism for streaming data [CITED: https://v2.tauri.app/develop/calling-frontend/].
- Treating Tauri Store as the default persistence answer for Phase 2 is outdated relative to the roadmap because Store is a file-backed key-value layer, not SQLite [CITED: https://v2.tauri.app/plugin/store/] [VERIFIED: .planning/ROADMAP.md].

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | “Base endpoint” means a path prefix or canonical local base URL component rather than a remote upstream URL [ASSUMED] | Standard Stack / Open Questions | Validation rules and UI wording could be wrong, causing rework in settings and status presentation |
| A2 | Phase 2 may need to permit non-loopback host values such as `0.0.0.0` rather than restricting to loopback-only [ASSUMED] | Open Questions / Security Domain | Access-control and safe-default behavior would change materially |
| A3 | Introducing `rusqlite_migration` in Phase 2 is preferable to postponing migrations until a later schema change [ASSUMED] | Standard Stack / Don’t Hand-Roll | If the team wants zero extra dependency now, the plan would over-specify migration work |
| A4 | `axum` is the better fit than raw `hyper` for Phase 2 because routing and graceful shutdown matter more than lower-level server control [ASSUMED] | Alternatives Considered | If the team needs lower-level transport control immediately, the planned server abstraction would be too opinionated |
| A5 | A path-prefix interpretation of “base endpoint” keeps Phase 2 narrower and aligns with later compatibility routing [ASSUMED] | Open Questions | UI copy, validation rules, and route structure could all need revision |
| A6 | Loopback-first defaults with explicit user intent for broader exposure are the right safe default for the host field [ASSUMED] | Open Questions / Security Domain | The shipped UX and access-control posture could conflict with product intent |
| A7 | “Apply safely” may need rollback-safe reconfiguration semantics while the runtime is already running, not just pre-persistence validation [ASSUMED] | Open Questions | Plan scope could under-specify lifecycle safety and lead to visible downtime behavior |

## Open Questions

1. **What exactly is the “base endpoint” field?**
   - What we know: The roadmap and requirements call it out separately from host and port [VERIFIED: .planning/ROADMAP.md] [VERIFIED: .planning/REQUIREMENTS.md].
   - What’s unclear: Whether it should be a path prefix like `/v1`, a full absolute local URL, or a user-facing alias shown in onboarding copy [ASSUMED].
   - Recommendation: Lock this before planning UI and validation details; a path-prefix interpretation keeps Phase 2 narrower and aligns better with later compatibility routing [ASSUMED].

2. **Should v1 allow non-loopback bind hosts?**
   - What we know: Users must be able to configure the listen host from the UI [VERIFIED: .planning/REQUIREMENTS.md].
   - What’s unclear: Whether that means any valid host/IP or only safe local defaults unless the user explicitly opts into LAN exposure [ASSUMED].
   - Recommendation: Default to `127.0.0.1`, show clear warning copy for non-loopback values, and treat unrestricted exposure as an explicit user action if the team wants it in scope [ASSUMED].

3. **Does Phase 2 need rollback-safe apply while the runtime is already running?**
   - What we know: Users must apply config changes safely from the app [VERIFIED: .planning/ROADMAP.md].
   - What’s unclear: Whether “safe” means “validated before persistence” only, or “running proxy never stays down after a failed reconfigure” [ASSUMED].
   - Recommendation: Plan for rollback-safe apply if time allows; otherwise make “stop, edit, apply, start” explicit in the first implementation and keep the old config until the candidate config proves valid [ASSUMED].

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Frontend builds and Tauri CLI wrapper | ✓ [VERIFIED: local command] | `v24.13.0` [VERIFIED: local command] | — |
| npm | Existing repo scripts and package management | ✓ [VERIFIED: local command] | `11.12.1` [VERIFIED: local command] | `pnpm` is also installed if needed [VERIFIED: local command] |
| `cargo` | Rust host build and tests | ✓ [VERIFIED: local command] | `1.93.1` [VERIFIED: local command] | — |
| `rustc` | Rust crate compilation | ✓ [VERIFIED: local command] | `1.93.1` [VERIFIED: local command] | — |
| `pnpm` | Existing lockfile/tooling compatibility from Phase 1 artifacts | ✓ [VERIFIED: local command] | `10.28.2` [VERIFIED: local command] | `npm` scripts remain supported and documented in `AGENTS.md` [VERIFIED: AGENTS.md] |
| `sqlite3` CLI | Manual DB inspection and debugging only | ✓ [VERIFIED: local command] | `3.51.0` [VERIFIED: local command] | Not required for app runtime because Rust will talk to SQLite directly [RECOMMENDATION] |

**Missing dependencies with no fallback:**
- None [VERIFIED: local command].

**Missing dependencies with fallback:**
- None [VERIFIED: local command].

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `Vitest 3.2.4` for frontend behavior and Rust’s built-in `cargo test` harness for host modules [VERIFIED: package.json] [VERIFIED: local command] |
| Config file | `vitest.config.ts` for frontend; no dedicated Rust config file detected [VERIFIED: repo grep] |
| Quick run command | `npm test -- src/features/runtime/__tests__/runtime-status.test.tsx` for focused UI checks, plus targeted `cargo test --manifest-path src-tauri/Cargo.toml <module_name>` for Rust modules [VERIFIED: package.json] [VERIFIED: repo grep] |
| Full suite command | `npm test && cargo test --manifest-path src-tauri/Cargo.toml` [VERIFIED: AGENTS.md] |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROX-01 | Starting and stopping the proxy updates lifecycle state and supervision correctly | Rust unit/integration | `cargo test --manifest-path src-tauri/Cargo.toml proxy_runtime_supervision` | ❌ Wave 0 [VERIFIED: repo grep] |
| PROX-02 | Host, port, and base endpoint validation accepts safe values and rejects bad ones | Rust unit | `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_validation` | ❌ Wave 0 [VERIFIED: repo grep] |
| PROX-03 | Applying settings persists them in SQLite and updates live runtime behavior without manual file edits | Rust integration | `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_apply` | ❌ Wave 0 [VERIFIED: repo grep] |
| PROX-04 | The UI renders healthy, stopped, and misconfigured states from host lifecycle changes | Frontend integration | `npm test -- src/features/proxy/__tests__/proxy-runtime-status.test.tsx` | ❌ Wave 0 [VERIFIED: repo grep] |

### Sampling Rate
- **Per task commit:** `cargo test --manifest-path src-tauri/Cargo.toml <focused_module>` or `npm test -- <focused_test_file>` [RECOMMENDATION].
- **Per wave merge:** `npm test && cargo test --manifest-path src-tauri/Cargo.toml` [VERIFIED: AGENTS.md].
- **Phase gate:** Full frontend and Rust suites green before `/gsd-verify-work` [VERIFIED: .planning/config.json].

### Wave 0 Gaps
- [ ] `src-tauri/src/runtime/proxy_runtime.rs` tests for start/stop/cancel/restart sequencing [RECOMMENDATION].
- [ ] `src-tauri/src/runtime/persistence.rs` tests using a temp SQLite database and migration bootstrap [RECOMMENDATION].
- [ ] `src-tauri/src/models/proxy_settings.rs` tests for host/port/base endpoint validation and serialization [RECOMMENDATION].
- [ ] `src/features/proxy/__tests__/proxy-controls.test.tsx` for form edit/apply/disabled-state behavior [RECOMMENDATION].
- [ ] `src/features/proxy/__tests__/proxy-runtime-status.test.tsx` for healthy/stopped/misconfigured rendering and event refresh behavior [RECOMMENDATION].

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no [RECOMMENDATION] | Not central in Phase 2; provider auth lands later in Phase 3 [VERIFIED: .planning/ROADMAP.md] |
| V3 Session Management | no [RECOMMENDATION] | No web user-session concept is introduced by this phase [VERIFIED: .planning/ROADMAP.md] |
| V4 Access Control | yes [RECOMMENDATION] | Keep the proxy loopback-first, review Tauri capabilities carefully, and do not expose renderer DB access casually [VERIFIED: AGENTS.md] [CITED: https://v2.tauri.app/plugin/sql/] |
| V5 Input Validation | yes [RECOMMENDATION] | Validate host, port, and endpoint inputs in Rust before persistence or restart [VERIFIED: .planning/REQUIREMENTS.md] [CITED: https://v2.tauri.app/develop/calling-rust/] |
| V6 Cryptography | no [RECOMMENDATION] | No new secret storage or cryptographic primitive is required in this phase; that work belongs to provider credential handling later [VERIFIED: .planning/ROADMAP.md] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Binding the proxy to a non-loopback interface unintentionally | Information Disclosure / Elevation of Privilege | Default to loopback values, validate host input, and require explicit user intent before broader exposure [ASSUMED] |
| Malformed host/port/base endpoint input producing a broken or misleading local URL | Tampering | Validate and normalize on the Rust side, then emit a distinct `misconfigured` status instead of trying to infer from UI errors [RECOMMENDATION] |
| Stop/apply race leaves the runtime task alive or the UI stale | Denial of Service | Supervise the listener task, use cancellation tokens, and emit snapshot invalidation after each lifecycle transition [CITED: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html] [CITED: https://v2.tauri.app/develop/calling-frontend/] |
| Renderer gains direct database write access for convenience | Tampering | Keep SQLite behind Rust commands only and avoid adding SQL/store guest permissions unless later phases truly need them [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/] |

## Sources

### Primary (HIGH confidence)
- `.planning/REQUIREMENTS.md` - Phase requirement definitions and traceability [VERIFIED: repo grep]
- `.planning/ROADMAP.md` - Phase goal, plans, and success criteria [VERIFIED: repo grep]
- `.planning/phases/01-runtime-foundation/01-01-SUMMARY.md` - Foundation architecture intent [VERIFIED: repo grep]
- `.planning/phases/01-runtime-foundation/01-02-SUMMARY.md` - Rust-owned state and typed IPC boundary [VERIFIED: repo grep]
- `.planning/phases/01-runtime-foundation/01-03-SUMMARY.md` - Runtime-driven shell and current UI/testing pattern [VERIFIED: repo grep]
- `AGENTS.md` - Repo structure, testing, UI, and capability constraints [VERIFIED: repo grep]
- `DESIGN.md` - Existing visual system to preserve for Phase 2 UI [VERIFIED: repo grep]
- Tauri State Management - https://v2.tauri.app/develop/state-management/ [CITED]
- Tauri Calling Rust from the Frontend - https://v2.tauri.app/develop/calling-rust/ [CITED]
- Tauri Calling the Frontend from Rust - https://v2.tauri.app/develop/calling-frontend/ [CITED]
- Tauri SQL plugin - https://v2.tauri.app/plugin/sql/ [CITED]
- Tauri Store plugin - https://v2.tauri.app/plugin/store/ [CITED]
- `axum::serve::Serve` docs - https://docs.rs/axum/latest/axum/serve/struct.Serve.html [CITED]
- `axum::Router` docs - https://docs.rs/axum/latest/axum/struct.Router.html [CITED]
- `tokio::net::TcpListener` docs - https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html [CITED]
- `tokio_util::sync::CancellationToken` docs - https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html [CITED]
- `url::Url` docs - https://docs.rs/url/latest/url/struct.Url.html [CITED]
- `rusqlite_migration` docs - https://docs.rs/rusqlite_migration/latest/rusqlite_migration/ [CITED]
- npm registry package metadata for `@tauri-apps/api`, `@tauri-apps/cli`, `react`, `vite`, `vitest`, `typescript`, `@tauri-apps/plugin-sql`, and `@tauri-apps/plugin-store` [VERIFIED: npm registry]
- crates.io package metadata for `tauri`, `axum`, `tokio`, `tokio-util`, `rusqlite`, `rusqlite_migration`, and `url` [VERIFIED: crates.io]
- Local environment probes for `node`, `npm`, `cargo`, `rustc`, `pnpm`, and `sqlite3` [VERIFIED: local command]

### Secondary (MEDIUM confidence)
- None.

### Tertiary (LOW confidence)
- None.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - The recommended stack is grounded in current official docs, npm/crates version checks, and the existing Phase 1 boundary [VERIFIED: npm registry] [VERIFIED: crates.io] [VERIFIED: repo grep].
- Architecture: HIGH - The architecture follows both the repo’s established runtime-owned pattern and Tauri’s documented state/command/event model [VERIFIED: repo grep] [CITED: https://v2.tauri.app/develop/state-management/] [CITED: https://v2.tauri.app/develop/calling-rust/].
- Pitfalls: HIGH - The main pitfalls are directly evidenced by the current repo shape plus explicit Tauri plugin/event guidance [VERIFIED: repo grep] [CITED: https://v2.tauri.app/develop/calling-frontend/] [CITED: https://v2.tauri.app/plugin/sql/] [CITED: https://v2.tauri.app/plugin/store/].

**Research date:** 2026-04-11
**Valid until:** 2026-05-11 for planning decisions, but verify package versions again before implementation starts because Tauri, Vite, Vitest, and React are still moving quickly [VERIFIED: npm registry] [VERIFIED: crates.io].

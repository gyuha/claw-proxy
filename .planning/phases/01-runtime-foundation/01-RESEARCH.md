# Phase 1 Research: Runtime Foundation

**Phase:** 1
**Name:** Runtime Foundation
**Researched:** 2026-04-11
**Confidence:** HIGH

## User Constraints

No `CONTEXT.md` exists for this phase yet.

Use these project-level constraints as fixed inputs:

- Cross-platform desktop delivery on macOS, Windows, and Linux
- Tauri 2 desktop host with an embedded Rust runtime
- React + TypeScript UI with a thin control-plane shell
- Secrets must not be stored in SQLite, JSON, or general app-state stores
- Phase 1 is for runtime foundation and typed boundaries, not full provider/routing implementation

## Project Constraints

- [VERIFIED: `.planning/PROJECT.md`] Runtime state must be owned by the local application, not by UI-only state.
- [VERIFIED: `.planning/PROJECT.md`] Secret and non-secret persistence must be split cleanly.
- [VERIFIED: `.planning/ROADMAP.md`] Phase 1 must satisfy `DESK-02` and establish the embedded runtime spine, typed host/UI boundary, and minimal shell.
- [VERIFIED: `.planning/research/SUMMARY.md`] The leading architecture direction is Tauri 2 + embedded Rust runtime + React/TypeScript UI.

## Phase Boundary

This phase should deliver the minimum credible desktop shell and runtime spine that later phases can build on.

It should include:
- Tauri workspace and application bootstrap
- Embedded Rust runtime skeleton
- Typed command/event boundary between UI and runtime
- Canonical runtime snapshot model for status, health, and future settings areas
- Minimal UI shell that can render runtime status and structured navigation

It should explicitly NOT include:
- Real provider authentication flows
- Multi-account routing logic
- Full diagnostics history
- Secret-vault integration beyond interface boundaries if no credentials are stored yet

## Standard Stack

- **Desktop shell:** Tauri 2 [VERIFIED: `.planning/research/STACK.md`]
- **Runtime core:** Rust with Tokio-driven async runtime [VERIFIED: stack research synthesis]
- **UI:** React 19 + TypeScript + Vite [VERIFIED: `.planning/research/STACK.md`]
- **Styling:** Tailwind CSS 4 or equivalent token-driven styling layer, but keep component surface minimal in this phase [VERIFIED: stack researcher output]
- **Runtime ingress (next phase ready):** Axum/Hyper-style HTTP boundary should be anticipated now, but Phase 1 only needs interfaces and host wiring, not full ingress behavior [ASSUMED]
- **Persistence in this phase:** introduce the boundary now; actual SQLite-backed policy persistence can begin in Phase 2 when runtime settings become writable [INFERRED from roadmap]

## Recommended Project Structure

```text
src/
├── app/
│   ├── App.tsx
│   ├── bootstrap.tsx
│   └── routes/
├── components/
│   ├── shell/
│   └── status/
├── features/
│   └── runtime/
│       ├── api.ts
│       ├── models.ts
│       └── state.ts
├── lib/
│   ├── ipc/
│   └── validation/
└── styles/

src-tauri/
├── src/
│   ├── commands/
│   ├── runtime/
│   ├── models/
│   └── events/
└── capabilities/
```

Why this shape:
- Keep runtime models and commands explicit from the start so later provider/routing work plugs into stable boundaries.
- Keep the UI thin and feature-oriented, with one runtime feature slice instead of prematurely modeling future provider/routing screens.

## Architecture Patterns

### 1. Runtime-Owned Snapshot Pattern

Use a canonical Rust-side snapshot struct as the source of truth for:
- app status
- runtime status
- active configuration summary
- placeholder counts for providers/accounts
- last health check timestamp

The UI should render this snapshot and mutate state only through typed commands.

Why:
- Avoids the Phase 2 pitfall of UI/runtime divergence
- Makes diagnostics and replay easier later

### 2. Typed IPC Boundary Pattern

Define a small set of command and event contracts in Phase 1:
- `get_runtime_snapshot`
- `initialize_runtime_state`
- `ping_runtime`
- `subscribe_runtime_events`

Do not expose raw internal structs directly to the UI. Create shared serialized models or mirrored TS types generated from stable contracts.

### 3. Thin Shell, Thick Runtime Direction

Even though the UI is visible in Phase 1, the phase should not be UI-led.

The UI should only provide:
- window layout
- app navigation skeleton
- runtime status card/panel
- placeholder sections for future settings areas

The runtime should own:
- initialization
- state transitions
- health status semantics
- future-ready module boundaries

## Early Persistence Strategy

Phase 1 should create persistence boundaries, not the full persistence feature set.

Implement now:
- runtime config schema definitions
- storage interfaces/traits for metadata and secrets
- migration/version placeholder for future config evolution

Defer to later phases:
- real SQLite storage of runtime settings until settings are editable
- OS keychain/keyring writes until provider credentials actually exist

Rationale:
- This phase is about architecture correctness and future safety, not storing empty data just to prove persistence exists.

## Don't Hand-Roll

- Do not hand-roll a custom desktop event bus when Tauri events/commands are sufficient.
- Do not hand-roll secret storage or token encryption logic in Phase 1.
- Do not invent a separate internal API protocol between UI and runtime beyond typed commands/events.
- Do not over-model provider/routing entities before those phases exist; placeholders and interfaces are enough.

## Common Pitfalls

- **UI-first planning:** building a polished shell without runtime ownership creates rework in Phase 2. [VERIFIED: `.planning/research/PITFALLS.md`]
- **Premature persistence:** introducing real storage writes before config semantics are settled increases migration churn. [INFERRED]
- **Future leakage:** pulling provider/account/routing logic into Phase 1 weakens the boundary and bloats the initial plan. [VERIFIED: `.planning/research/FEATURES.md` narrow-v1 guidance]
- **Untyped IPC:** ad hoc payloads between React and Rust will become a maintenance trap by Phase 3. [ASSUMED but strongly supported by architecture direction]

## Specific Planning Guidance

The plan set for this phase should likely stay at 3 plans:

1. **Workspace + runtime bootstrap**
   - Create the Tauri 2 project skeleton, Rust runtime entry, React shell bootstrap, and initial build/test commands
2. **State schema + IPC contracts**
   - Define runtime snapshot models, commands, events, validation boundaries, and the serialization strategy
3. **Minimal control surface**
   - Build the desktop shell and runtime-status UI that consumes the typed snapshot and leaves room for future sections

Wave guidance:
- Plans 01 and 02 are closely coupled and should probably remain sequential
- Plan 03 can begin after runtime snapshot and command contracts exist

## Security Guidance for Phase 1

Even without provider auth, the plan should already enforce:
- runtime state as main-process authority
- no renderer-accessible secret placeholders
- redacted diagnostics conventions
- explicit storage interfaces separating secret and non-secret data

This phase should also include a small threat-model section in each PLAN.md, because later phases depend on these boundaries.

## Code Examples

### Runtime snapshot boundary

```rust
#[derive(Clone, Debug, serde::Serialize)]
pub struct RuntimeSnapshot {
    pub app_ready: bool,
    pub runtime_status: RuntimeStatus,
    pub active_profile: Option<String>,
    pub provider_slots: u32,
    pub account_slots: u32,
    pub last_health_check: Option<String>,
}
```

### UI-facing command surface

```rust
#[tauri::command]
async fn get_runtime_snapshot(state: tauri::State<'_, AppState>) -> Result<RuntimeSnapshot, String> {
    state.runtime.snapshot().await.map_err(|err| err.to_string())
}
```

### UI runtime feature contract

```ts
export type RuntimeSnapshot = {
  appReady: boolean
  runtimeStatus: 'starting' | 'ready' | 'stopped' | 'error'
  activeProfile: string | null
  providerSlots: number
  accountSlots: number
  lastHealthCheck: string | null
}
```

## Validation Architecture

Use a two-layer validation strategy in Phase 1:

### Layer 1: Fast developer feedback

- `pnpm vitest run` for TypeScript model/adapter smoke tests
- `cargo test` for Rust runtime model and command tests
- `cargo check` for host/runtime compile validation

### Layer 2: App-shell verification

- `pnpm tauri build --debug` or equivalent build smoke check
- one minimal app bootstrap verification that proves the shell can render runtime status without provider features

### Wave 0 requirements

If the generated Tauri template does not already provide test infrastructure, Wave 0 of the first execution plan should install:
- Vitest for TS-side boundary tests
- one Rust unit-test module for runtime snapshot/state transitions

### What must be verifiable by the end of Phase 1

- Tauri app boots successfully
- Rust runtime module compiles and exposes typed commands
- UI can request and render a runtime snapshot
- No command payloads rely on ad hoc untyped maps

## Sources

- [VERIFIED: `.planning/PROJECT.md`] product constraints and architecture commitments
- [VERIFIED: `.planning/ROADMAP.md`] Phase 1 goal, requirements, and success criteria
- [VERIFIED: `.planning/research/STACK.md`] Tauri 2 / Rust / React baseline
- [VERIFIED: `.planning/research/ARCHITECTURE.md`] runtime-owned architecture and build-order guidance
- [VERIFIED: `.planning/research/PITFALLS.md`] security and phase-ordering pitfalls
- [VERIFIED: `.planning/research/SUMMARY.md`] consolidated roadmap implications

## RESEARCH COMPLETE

Phase 1 planning should optimize for a stable runtime spine and typed boundaries, not feature breadth. The plan set should be small, sequential where boundaries are still forming, and explicit about verification of runtime ownership.

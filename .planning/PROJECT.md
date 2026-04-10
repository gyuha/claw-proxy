# Claw Proxy

## What This Is

Claw Proxy is a cross-platform developer tool that unifies multiple AI providers behind a single local proxy server. It exposes both OpenAI-compatible and Anthropic-compatible APIs so tools like Claude Code can route through multiple providers and accounts without changing their normal workflow.

The product is CLI-first, with a Tauri menu bar or system tray companion app for visibility and control. The core engine must remain independently runnable so the desktop UI is an optional control surface, not a dependency.

## Core Value

Developers can point their existing AI tooling at one local endpoint and transparently get reliable multi-provider, multi-account routing without changing how they work.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Run a standalone local proxy that accepts both OpenAI and Anthropic request formats.
- [ ] Route requests across multiple provider accounts using round-robin or failover strategies.
- [ ] Expose a management surface for status, provider configuration, config edits, and live request visibility.
- [ ] Deliver a macOS and Windows tray app that controls the core process without embedding business logic in the UI.
- [ ] Keep the provider layer extensible so new providers can be added behind a shared trait and adapter boundary.

### Out of Scope

- Linux desktop packaging — the approved MVP only commits to macOS and Windows.
- Cloud-hosted or multi-tenant proxy deployment — the initial product is explicitly local-first and developer-owned.
- Secret storage beyond plaintext YAML — keyring integration is deferred until after MVP.
- Provider-specific routing rules inside the router — adapter boundaries are a core architectural constraint and must stay intact.

## Context

Claw Proxy is based on the shape of vibeproxy, but expands it into a cross-platform product with dual API compatibility and a separate desktop control surface. The approved design fixes the public entrypoints at `:47380` for proxy traffic, `:47381` for REST admin APIs, and `:47382` for WebSocket log streaming, while allowing those ports to be overridden in `config.yaml`.

The current repository already contains an early scaffold for the Rust core, config loading, normalizers, router strategies, admin handlers, and a Tauri/React desktop shell. That scaffold is useful context, but it should be treated as an unverified starting point rather than proof that MVP behavior is already complete end-to-end.

The desktop experience is intentionally a tray-first popover rather than a conventional windowed app. The UI should focus on dashboard, providers, logs, and settings, while orchestration, routing, normalization, and provider logic stay in the Rust core.

## Constraints

- **Tech stack**: Rust core with Axum/Tokio/Reqwest plus Tauri 2.x, React 18, TypeScript, TailwindCSS, shadcn/ui, and Zustand — the approved architecture is already chosen.
- **Architecture**: Core engine and UI must be fully separated — the proxy must run without the desktop app, and the UI must not own business logic.
- **Compatibility**: MVP must work on macOS and Windows — tray behavior and sidecar control need to respect both environments.
- **API surface**: `/v1/chat/completions` and `/v1/messages` must both be first-class entrypoints — Claude Code support depends on the Anthropic-compatible path.
- **Security**: API keys live in plaintext YAML for MVP — acceptable for now, but this is a known limitation to isolate and later replace.
- **Extensibility**: The router cannot contain provider-specific behavior — all provider variance belongs in adapters implementing the shared trait.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Dual API compatibility from day one | Existing AI tools already expect OpenAI or Anthropic wire formats, so compatibility reduces adoption friction | — Pending |
| CLI-first local proxy as the primary product shape | Local-first keeps setup simple, preserves developer control, and matches Claude Code integration goals | — Pending |
| Separate proxy, admin REST, and WebSocket ports | Keeps request serving, control-plane actions, and live logging cleanly separated | — Pending |
| Tauri tray popover UI instead of a full desktop shell | The product should feel lightweight and always available, not like a heavy dashboard app | — Pending |
| Shared `Provider` trait with adapter pattern | New providers must fit behind one abstraction so routing remains provider-agnostic | — Pending |
| OpenAI first, Claude/Gemini completed in later phase | This sequence de-risks the MVP by proving the architecture on one full adapter before expanding | — Pending |
| Plain YAML configuration with hot reload in MVP | Fastest path to an operable product; stronger secret handling can follow after the basic workflow works | ⚠ Revisit |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-10 after initialization*

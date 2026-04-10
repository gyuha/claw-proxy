# Claw Proxy

## What This Is

Claw Proxy is a cross-platform desktop application that gives developers a local control plane for running and configuring an LLM proxy through a polished GUI. Inspired by VibeProxy's local proxy and account-bridging workflow, it expands the concept beyond a macOS menu bar utility into a full desktop product for managing proxy settings, multiple LLM services, and multiple accounts per provider from one place.

## Core Value

Developers can connect and control all of their AI provider accounts through one reliable local desktop proxy without juggling fragile config files or raw API keys across tools.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] User can install and run a local desktop proxy app on macOS, Windows, and Linux
- [ ] User can configure proxy runtime settings, ports, endpoints, and startup behavior through the UI
- [ ] User can connect multiple LLM providers and manage multiple accounts per provider securely
- [ ] User can choose how requests are routed across providers, models, and accounts with failover support
- [ ] User can inspect connection health, request activity, and configuration problems without leaving the app
- [ ] User can onboard common AI coding tools with clear local endpoint and setup instructions

### Out of Scope

- Hosted team control plane — v1 is local-first and single-user to keep security and shipping scope manageable
- Mobile companion app — not needed for the core desktop proxy workflow
- General-purpose chat playground — the product focus is proxy control for external AI tools, not becoming another chat client
- Provider-specific billing analytics — useful later, but not required to validate the core routing and account-management value

## Context

The reference product, VibeProxy, proves demand for a GUI-managed local proxy that bridges existing AI subscriptions into coding tools. Its strongest patterns are one-click server lifecycle management, local authentication flows, provider connection status, and multi-account support with automatic distribution and failover.

Claw Proxy should keep that local-first utility but be intentionally broader in platform support and product surface. Instead of a menu bar-only macOS app, the target is a cross-platform desktop experience with richer settings, clearer routing controls, better diagnostics, and a more extensible provider/account model.

There is also an existing visual direction in [DESIGN.md] centered on a calm, Notion-inspired interface system. That should inform the desktop UI tone, but product architecture and delivery speed matter more than pixel-perfect marketing polish for the initial milestone.

## Constraints

- **Compatibility**: Must support macOS, Windows, and Linux desktop environments — cross-platform delivery is part of the product thesis
- **Architecture**: Must run as a local-first app with an embedded proxy/runtime — core value depends on controlling traffic on the user's machine
- **Security**: Provider credentials and session tokens must be stored in the OS credential vault or equivalent secure storage — this is a trust-sensitive product handling paid AI accounts
- **Extensibility**: Provider integration model must support multiple services and multiple accounts per service — the app cannot be hard-wired to one vendor
- **UX**: Proxy configuration and recovery flows must be understandable from the UI alone — users should not need to hand-edit config files for normal operations
- **Persistence**: Secret and non-secret state must be split cleanly — metadata/policy can live in SQLite, but secrets must not

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Build Claw Proxy as a cross-platform desktop app instead of a platform-specific utility | The requested product direction is broader than VibeProxy and needs Windows/Linux support | — Pending |
| Treat the app as a local control plane for external AI tools, not as a first-party chat client | Keeps scope aligned with the proxy/routing problem users are actually trying to solve | — Pending |
| Make multi-provider and multi-account support a first-class v1 requirement | This is one of the clearest user-visible differentiators versus hand-managed configs | — Pending |
| Favor local secure storage and explicit routing policies over opaque background magic | Users need trust and control when paid AI subscriptions are involved | — Pending |
| Default to Tauri 2 with an embedded Rust runtime and React/TypeScript UI | Best fit for a cross-platform local-first desktop control plane with strong runtime ownership | — Pending |
| Use SQLite for non-secret metadata/policy and OS keychain/keyring storage for secrets | Separates operational data from credential risk and aligns with desktop trust expectations | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check - still the right priority?
3. Audit Out of Scope - reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-11 after initialization*

# Project Research Summary

**Project:** Claw Proxy
**Domain:** Cross-platform desktop local AI proxy / gateway for coding tools
**Researched:** 2026-04-11
**Confidence:** MEDIUM-HIGH

## Executive Summary

Claw Proxy fits a narrow but real product category: a local desktop control plane that keeps AI coding tools pointed at one dependable endpoint while the app manages providers, accounts, and routing behind the scenes. The strongest signal from VibeProxy and adjacent proxy projects is that users value operational simplicity more than feature breadth.

The recommended approach is to build Claw Proxy as a Tauri 2 cross-platform desktop app with an embedded Rust runtime, React/TypeScript UI, SQLite for non-secret metadata/policy, OS keychain/keyring storage for secrets, and an adapter-based provider layer. The roadmap should prioritize runtime ownership, secure account handling, and clear diagnostics before broader ecosystem support.

## Key Findings

### Recommended Stack

Tauri 2.x is the best default shell for Claw Proxy because its desktop plugin ecosystem directly supports startup behavior, persistent config, secret storage, and signed updates across macOS, Windows, and Linux. Pairing it with a Rust host layer and React/TypeScript UI gives a strong split between trust-sensitive runtime control and fast product iteration.

**Core technologies:**
- Tauri 2.x: desktop shell and native capability access
- Embedded Rust runtime: proxy ingress, routing, diagnostics, and lifecycle ownership
- React 19 + TypeScript 5 + Vite 8 + Tailwind 4: UI delivery for a settings-heavy control plane

### Expected Features

**Must have (table stakes):**
- Embedded proxy lifecycle management
- Secure provider/account storage
- Multi-provider support with multiple accounts per provider
- Simple routing and failover
- Basic diagnostics and activity view
- Claude Code and Codex onboarding guidance

**Should have (competitive):**
- Browser-assisted account bridging for one high-value provider
- Per-tool setup wizard and config generation

**Defer (v2+):**
- Team/admin SaaS features
- Broad client matrix
- Heavy analytics and billing views
- Plugin marketplace or scripting

### Architecture Approach

Use a host-owned runtime architecture: the UI issues commands and renders snapshots, but the embedded Rust runtime owns lifecycle, health, routing state, and provider/account truth. Provider integrations should sit behind a common adapter contract so Claw Proxy can stay narrow in v1 without painting itself into a corner.

**Major components:**
1. Desktop UI - settings, diagnostics, onboarding, and status surfaces
2. Embedded Rust runtime - lifecycle control, validation, SQLite/keyring orchestration, and event emission
3. Provider/routing subsystem - adapters, account pool management, and compatibility endpoint behavior

### Critical Pitfalls

1. **Plain-text secret drift** - separate secret storage from config from the start
2. **Runtime/UI state divergence** - make the embedded runtime the source of truth
3. **Provider sprawl in v1** - support a narrow provider/client matrix first
4. **Over-clever routing too early** - ship deterministic fallback before advanced policies
5. **Packaging left too late** - treat distribution as part of the product, not final polish

## Implications for Roadmap

### Phase 1: Runtime Foundation
**Rationale:** Runtime ownership and typed boundaries must exist before feature-heavy UI work.  
**Delivers:** Embedded runtime spine, typed IPC boundary, minimal shell.  
**Avoids:** Ad hoc UI/provider coupling.

### Phase 2: Proxy Control Surface
**Rationale:** The app's core promise is local runtime control.  
**Delivers:** Start/stop/apply-config flows, local ingress, health snapshots.  
**Avoids:** Runtime/UI drift.

### Phase 3: Provider Security Gate
**Rationale:** Secure account management should land before advanced routing.  
**Delivers:** Provider adapters, OS-vault secret handling, account CRUD.  
**Avoids:** Plain-text secret drift and provider-specific UI sprawl.

### Phase 4: Routing Compatibility Gate
**Rationale:** Routing only makes sense once provider/account truth exists.  
**Delivers:** Default routing, fallback, rotation, compatibility endpoint.  
**Avoids:** Overbuilding routing before reliability.

### Phase 5: Diagnostics & Onboarding
**Rationale:** Real flows must exist before good diagnostics and onboarding can be shaped.  
**Delivers:** Health views, activity, recovery, Claude Code/Codex setup help.

### Phase 6: Distribution & Hardening
**Rationale:** Packaging and trust requirements need a dedicated finish phase.  
**Delivers:** Installers, startup behavior, updates, release verification.

### Phase Ordering Rationale

- Secret handling must precede broad account and routing support
- Runtime truth must exist before diagnostics can be trusted
- Packaging concerns are visible throughout, but should not block core proxy validation

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Backed by current Tauri desktop plugin docs and the product's local-first constraints |
| Features | MEDIUM-HIGH | Strong pattern agreement across reference products, but v1 provider/client breadth remains a product call |
| Architecture | MEDIUM-HIGH | The host-owned runtime pattern is a strong fit, though exact embed-vs-sidecar details remain implementation-specific |
| Pitfalls | MEDIUM | Derived from category behavior and desktop product constraints rather than one canonical post-mortem source |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- Exact v1 provider set should be locked during Phase 1 planning
- Confirm the strongest differentiator for v1: account bridging versus guided client onboarding
- Validate Linux secure-storage and packaging behavior early in implementation

## Sources

### Primary
- VibeProxy GitHub repository: https://github.com/automazeio/vibeproxy
- Tauri Updater docs: https://v2.tauri.app/plugin/updater/
- Tauri Autostart docs: https://v2.tauri.app/plugin/autostart/
- Stack and architecture researcher outputs in `.planning/research/STACK.md` and `.planning/research/ARCHITECTURE.md`

### Secondary
- `.planning/research/FEATURES.md`

---
*Research completed: 2026-04-11*
*Ready for roadmap: yes*

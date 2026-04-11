# Roadmap: Claw Proxy

## Overview

Claw Proxy should be built as a Tauri 2 desktop app with an embedded Rust runtime that owns proxy state, routing, secrets, and diagnostics. The roadmap therefore starts with the runtime spine and typed control-plane boundary, then layers on proxy lifecycle, secure provider/account flows, compatibility/routing, diagnostics, and finally distribution hardening.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Runtime Foundation** - Establish the embedded Rust runtime, typed control-plane boundary, and minimal shell
- [x] **Phase 2: Proxy Control Surface** - Run, configure, and monitor the local proxy lifecycle from the desktop app
- [ ] **Phase 3: Provider Security Gate** - Add secure provider/account onboarding and local credential storage
- [ ] **Phase 4: Routing Compatibility Gate** - Deliver multi-account routing, failover, and tool-facing compatibility endpoints
- [ ] **Phase 5: Diagnostics & Onboarding** - Make failures understandable and onboarding smooth with logs, status, and guided setup
- [ ] **Phase 6: Distribution & Hardening** - Package, verify, and ship a trustworthy cross-platform desktop release

## Phase Details

### Phase 1: Runtime Foundation
**Goal**: Create the embedded runtime spine, typed host/UI boundary, and minimal desktop shell for Claw Proxy
**Depends on**: Nothing (first phase)
**Requirements**: DESK-02
**Success Criteria** (what must be TRUE):
  1. User can open Claw Proxy and navigate a minimal but structured desktop control surface
  2. The embedded runtime owns canonical state for providers, accounts, runtime settings, and health
  3. UI and runtime communicate through typed commands/events rather than direct coupling
**Plans**: 3 plans

Plans:
- [x] 01-01: Set up the Tauri 2 workspace, embedded Rust runtime, and thin desktop shell
- [x] 01-02: Define configuration schema, runtime-owned state snapshots, and IPC boundaries
- [x] 01-03: Implement the minimal control surface for runtime status and future settings areas

### Phase 2: Proxy Control Surface
**Goal**: Embed or orchestrate the local proxy runtime so users can manage it entirely from the desktop UI
**Depends on**: Phase 1
**Requirements**: PROX-01, PROX-02, PROX-03, PROX-04
**Success Criteria** (what must be TRUE):
  1. User can start and stop the local proxy runtime from the app
  2. User can edit proxy host, port, and endpoint settings and apply them safely
  3. App shows clear runtime status when the proxy is healthy, stopped, or misconfigured
**Plans**: 3 plans

Plans:
- [x] 02-01-PLAN.md — Define supervised proxy contracts and health foundation
- [x] 02-02-PLAN.md — Persist proxy settings and expose safe lifecycle commands
- [x] 02-03-PLAN.md — Build the proxy control surface and live status UI

### Phase 3: Provider Security Gate
**Goal**: Let users connect providers and manage credentials/accounts securely on their machine
**Depends on**: Phase 2
**Requirements**: PROV-01, PROV-02, PROV-03
**Success Criteria** (what must be TRUE):
  1. User can add at least one supported provider through a guided connection flow
  2. Credentials or tokens are stored in OS-backed secure storage instead of plain-text files or general app state stores
  3. User can enable or disable providers and see their current connection state
**Plans**: 3 plans

Plans:
- [ ] 03-01: Define provider adapter contracts and implement initial provider integrations
- [ ] 03-02: Add OS-vault credential storage and provider/account CRUD flows
- [ ] 03-03: Synchronize provider enablement state and connection status into the desktop UI

### Phase 4: Routing Compatibility Gate
**Goal**: Turn Claw Proxy into a real control plane with multi-account routing and external tool compatibility
**Depends on**: Phase 3
**Requirements**: PROV-04, ROUT-01, ROUT-02, ROUT-03
**Success Criteria** (what must be TRUE):
  1. User can select default provider/model behavior for outgoing requests
  2. User can configure account rotation or failover for provider outages and rate limits without opaque behavior
  3. External AI tools can send requests through a supported compatibility surface exposed by the local proxy
**Plans**: 3 plans

Plans:
- [ ] 04-01: Implement request routing policy engine, canonical request schema, and default model/provider mapping
- [ ] 04-02: Add multi-account rotation, failover, and rate-limit handling behavior with explicit health/backoff rules
- [ ] 04-03: Expose and verify a tool-compatible local API surface for Claude Code and Codex-style clients

### Phase 5: Diagnostics & Onboarding
**Goal**: Make the app debuggable and self-service when configuration, authentication, or client onboarding breaks
**Depends on**: Phase 4
**Requirements**: CLNT-01, CLNT-02, DIAG-01, DIAG-02, DIAG-03
**Success Criteria** (what must be TRUE):
  1. User can see current provider/account health from a single dashboard
  2. User can inspect recent activity and actionable error messages without opening logs manually
  3. User gets guided repair steps for common auth, runtime, or config failures
  4. User can follow app-generated setup instructions for Claude Code and Codex
**Plans**: 3 plans

Plans:
- [ ] 05-01: Build provider/account health dashboard and live status surfaces from runtime telemetry
- [ ] 05-02: Add recent activity log, error presentation, and debugging affordances
- [ ] 05-03: Implement guided recovery flows and client setup instructions for Claude Code and Codex

### Phase 6: Distribution & Hardening
**Goal**: Ship Claw Proxy as a reliable cross-platform desktop product with production-minded packaging and safety
**Depends on**: Phase 5
**Requirements**: DESK-01, DESK-03
**Success Criteria** (what must be TRUE):
  1. Users on supported desktop platforms can install and run signed or otherwise trusted builds
  2. App startup behavior, updates, and runtime packaging are predictable across platforms with migration and rollback safety
  3. Release verification covers proxy behavior, provider flows, and desktop lifecycle expectations
**Plans**: 3 plans

Plans:
- [ ] 06-01: Set up cross-platform packaging, installers, and release artifacts
- [ ] 06-02: Add login-start behavior, signed update strategy, migration versioning, and platform-specific trust requirements
- [ ] 06-03: Execute release hardening, QA verification, and ship-readiness checks

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Runtime Foundation | 3/3 | Human verification | - |
| 2. Proxy Control Surface | 3/3 | Complete | 2026-04-11 |
| 3. Provider Security Gate | 0/3 | Not started | - |
| 4. Routing Compatibility Gate | 0/3 | Not started | - |
| 5. Diagnostics & Onboarding | 0/3 | Not started | - |
| 6. Distribution & Hardening | 0/3 | Not started | - |

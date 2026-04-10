# Roadmap: Claw Proxy

## Overview

Claw Proxy ships in four coherent phases that move from a reliable local proxy core to routing and control-plane operations, then to a tray-first desktop experience, and finally to full multi-provider readiness. The sequence follows the architectural dependency chain: normalize requests and prove the OpenAI path first, add routing and observability second, layer the desktop control surface on top of stable admin APIs third, and complete Claude/Gemini adapters once the shared provider contract is proven.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Core Proxy Foundation** - Stand up the standalone proxy, dual-format ingress, and the first production-ready provider path.
- [ ] **Phase 2: Routing and Control Plane** - Add multi-account routing, hot reload, admin APIs, and live request observability.
- [ ] **Phase 3: Desktop Tray Experience** - Build the macOS/Windows tray popover app on top of the core control plane.
- [ ] **Phase 4: Provider Expansion and Hardening** - Complete Claude and Gemini adapters and close end-to-end gaps for multi-provider readiness.

## Phase Details

### Phase 1: Core Proxy Foundation
**Goal**: Deliver a runnable local proxy that accepts OpenAI and Anthropic request formats, normalizes them into one internal model, and forwards through a working OpenAI adapter.
**Depends on**: Nothing (first phase)
**Requirements**: [PROXY-01, PROXY-02, PROXY-03, PROXY-04, PROXY-05, PROV-01]
**Success Criteria** (what must be TRUE):
1. Developer can start the Rust core with a YAML config file and serve proxy traffic on the configured local port.
2. OpenAI-compatible requests to `/v1/chat/completions` complete successfully and return OpenAI-compatible responses.
3. Anthropic-compatible requests to `/v1/messages` complete successfully and return Anthropic-compatible responses that preserve the caller-facing protocol.
4. Claude Code can target the local proxy via `ANTHROPIC_BASE_URL` for Anthropic-format workflows.
**Plans**: 3 plans

Plans:
- [ ] 01-01: Finalize core server bootstrap, config loading, and error boundaries for standalone proxy execution.
- [ ] 01-02: Complete bidirectional normalization for OpenAI and Anthropic request/response flows.
- [ ] 01-03: Finish the OpenAI adapter and add integration coverage for the first end-to-end path.

### Phase 2: Routing and Control Plane
**Goal**: Introduce reliable multi-account routing, hot-reloaded configuration, admin REST endpoints, and live WebSocket log streaming.
**Depends on**: Phase 1
**Requirements**: [ROUT-01, ROUT-02, ROUT-03, ROUT-04, ROUT-05, ADMIN-01, ADMIN-02, ADMIN-03, ADMIN-04, ADMIN-05, OBS-01, OBS-02]
**Success Criteria** (what must be TRUE):
1. Proxy routes eligible requests across multiple configured accounts using either round-robin or failover behavior.
2. Config changes to providers, routing strategy, and ports are reflected without restarting the process manually.
3. REST admin endpoints expose status, providers, config, and stats with data that matches runtime reality.
4. Operators can subscribe to live request logs and see the required telemetry fields for each request.
**Plans**: 4 plans

Plans:
- [ ] 02-01: Implement model-aware routing behavior for round-robin and failover strategies.
- [ ] 02-02: Finish config persistence and hot reload plumbing for runtime updates.
- [ ] 02-03: Expand the admin API to cover status, provider CRUD, config management, and stats.
- [ ] 02-04: Wire request telemetry into the WebSocket stream and verification coverage.

### Phase 3: Desktop Tray Experience
**Goal**: Ship a tray-first desktop app that controls the core sidecar and surfaces dashboard, providers, logs, and settings without owning backend logic.
**Depends on**: Phase 2
**Requirements**: [DESK-01, DESK-02, DESK-03, DESK-04, DESK-05, DESK-06, DESK-07]
**Success Criteria** (what must be TRUE):
1. macOS menu bar and Windows system tray show a Claw Proxy icon that opens an anchored popover and dismisses on blur.
2. Users can start and stop the core sidecar from the dashboard and see accurate runtime status.
3. Users can manage providers and configuration from the UI with changes reflected in the underlying config and admin APIs.
4. Users can watch and filter live request logs inside the desktop app.
**Plans**: 4 plans

Plans:
- [ ] 03-01: Implement tray icon, window positioning, popover lifecycle, and sidecar process control in Tauri.
- [ ] 03-02: Build dashboard and shared stores against live admin APIs.
- [ ] 03-03: Complete providers and settings flows, including config editing and persistence.
- [ ] 03-04: Finish real-time logs UI, filtering, and UX polish for the popover footprint.

### Phase 4: Provider Expansion and Hardening
**Goal**: Complete Claude and Gemini adapters, preserve the provider abstraction boundary, and verify the full multi-provider experience end-to-end.
**Depends on**: Phase 3
**Requirements**: [PROV-02, PROV-03, PROV-04]
**Success Criteria** (what must be TRUE):
1. Claude and Gemini accounts can be configured and used through the same routing layer and internal request contract as OpenAI.
2. Adding a provider still requires adapter work only, with no provider-specific branching inside the router.
3. Regression coverage protects the OpenAI, Claude, and Gemini paths plus the core proxy compatibility surfaces.
4. The product is ready for follow-on hardening work such as secure secret storage without architectural rework.
**Plans**: 3 plans

Plans:
- [ ] 04-01: Implement the Claude provider adapter and verify Anthropic-compatible upstream behavior.
- [ ] 04-02: Implement the Gemini provider adapter and align it with the shared internal request model.
- [ ] 04-03: Add end-to-end verification, cleanup, and release-readiness checks for the multi-provider core.

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Core Proxy Foundation | 0/3 | Not started | - |
| 2. Routing and Control Plane | 0/4 | Not started | - |
| 3. Desktop Tray Experience | 0/4 | Not started | - |
| 4. Provider Expansion and Hardening | 0/3 | Not started | - |

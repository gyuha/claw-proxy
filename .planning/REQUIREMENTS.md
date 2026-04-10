# Requirements: Claw Proxy

**Defined:** 2026-04-10
**Core Value:** Developers can point their existing AI tooling at one local endpoint and transparently get reliable multi-provider, multi-account routing without changing how they work.

## v1 Requirements

### Proxy API

- [x] **PROXY-01**: Developer can run the proxy as a standalone local process by starting the Rust core with a YAML config file.
- [x] **PROXY-02**: Developer can send OpenAI-compatible requests to `/v1/chat/completions` and receive an OpenAI-compatible response from the proxy.
- [x] **PROXY-03**: Developer can send Anthropic-compatible requests to `/v1/messages` and receive an Anthropic-compatible response from the proxy.
- [x] **PROXY-04**: Developer can point Claude Code at the proxy with `ANTHROPIC_BASE_URL=http://localhost:<proxy_port>` and keep using the normal Claude workflow.
- [x] **PROXY-05**: Proxy request handling converts supported caller formats into one internal request model before routing to providers.

### Routing & Config

- [ ] **ROUT-01**: Operator can register multiple provider accounts in YAML using a shared config shape.
- [ ] **ROUT-02**: Operator can choose round-robin routing so eligible accounts are used in sequence.
- [ ] **ROUT-03**: Operator can choose failover routing so the proxy automatically advances to the next eligible account after an upstream failure.
- [ ] **ROUT-04**: Proxy only selects accounts that advertise support for the requested model.
- [ ] **ROUT-05**: Operator can change ports, providers, and routing strategy in config and have the running proxy pick up the change without a full restart.

### Admin & Observability

- [ ] **ADMIN-01**: Operator can query server status, uptime, and active ports through the REST admin API.
- [ ] **ADMIN-02**: Operator can list configured providers and see whether each provider is currently available.
- [ ] **ADMIN-03**: Operator can create, update, and delete provider entries through the management surface and have those changes persist to config.
- [ ] **ADMIN-04**: Operator can fetch the current config and update it through the management surface.
- [ ] **ADMIN-05**: Operator can view request statistics through the admin API.
- [ ] **OBS-01**: Operator can subscribe to a WebSocket stream of request log events while the proxy is running.
- [ ] **OBS-02**: Each log event includes timestamp, request identifier, model, chosen provider, source format, status code, and latency.

### Desktop App

- [ ] **DESK-01**: User can run Claw Proxy as a tray or menu bar application on macOS and Windows.
- [ ] **DESK-02**: Clicking the tray icon opens a sidebar-style popover anchored to the tray location, and the popover dismisses on blur.
- [ ] **DESK-03**: User can start and stop the core proxy process from the desktop dashboard.
- [ ] **DESK-04**: User can see dashboard summary data for server state, active providers, total requests, and average response time.
- [ ] **DESK-05**: User can manage provider accounts from the desktop app.
- [ ] **DESK-06**: User can view and filter live request logs in the desktop app.
- [ ] **DESK-07**: User can change ports, routing strategy, and raw config content from the desktop app.

### Provider Expansion

- [ ] **PROV-01**: Operator can use a fully implemented OpenAI adapter in MVP for real upstream requests.
- [ ] **PROV-02**: Operator can use a completed Claude adapter through the same internal request contract and routing layer.
- [ ] **PROV-03**: Operator can use a completed Gemini adapter through the same internal request contract and routing layer.
- [ ] **PROV-04**: Developer can add future providers by implementing the shared `Provider` trait without adding provider-specific logic to the router.

## v2 Requirements

### Security & Product Depth

- **SECR-01**: Operator can store provider API keys in OS keychain or keyring storage instead of plaintext YAML.
- **SECR-02**: Operator can redact or mask secrets everywhere they appear in logs and management responses.
- **RATE-01**: Operator can apply rate limits, quotas, or budget policies per provider account.
- **ANLY-01**: Operator can inspect historical analytics beyond the live 500-event log buffer.

### Platform Expansion

- **PLAT-01**: User can install and run the tray app on Linux.
- **PLAT-02**: User can manage the proxy remotely from another machine instead of only on localhost.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Cloud-hosted shared proxy service | MVP is explicitly local-first and not a hosted SaaS |
| Linux desktop packaging | Approved platform scope is macOS + Windows only |
| Keychain/keyring integration | Plain YAML is an accepted MVP shortcut |
| Provider-specific logic inside router | Violates the adapter boundary that keeps the core extensible |
| Advanced billing, budgeting, or quota UI | Useful later, but not required to prove the core proxy workflow |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROXY-01 | Phase 1 | Complete |
| PROXY-02 | Phase 1 | Complete |
| PROXY-03 | Phase 1 | Complete |
| PROXY-04 | Phase 1 | Complete |
| PROXY-05 | Phase 1 | Complete |
| PROV-01 | Phase 1 | Pending |
| ROUT-01 | Phase 2 | Pending |
| ROUT-02 | Phase 2 | Pending |
| ROUT-03 | Phase 2 | Pending |
| ROUT-04 | Phase 2 | Pending |
| ROUT-05 | Phase 2 | Pending |
| ADMIN-01 | Phase 2 | Pending |
| ADMIN-02 | Phase 2 | Pending |
| ADMIN-03 | Phase 2 | Pending |
| ADMIN-04 | Phase 2 | Pending |
| ADMIN-05 | Phase 2 | Pending |
| OBS-01 | Phase 2 | Pending |
| OBS-02 | Phase 2 | Pending |
| DESK-01 | Phase 3 | Pending |
| DESK-02 | Phase 3 | Pending |
| DESK-03 | Phase 3 | Pending |
| DESK-04 | Phase 3 | Pending |
| DESK-05 | Phase 3 | Pending |
| DESK-06 | Phase 3 | Pending |
| DESK-07 | Phase 3 | Pending |
| PROV-02 | Phase 4 | Pending |
| PROV-03 | Phase 4 | Pending |
| PROV-04 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 28 total
- Mapped to phases: 28
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-10*
*Last updated: 2026-04-10 after initial definition*

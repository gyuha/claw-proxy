# Requirements: Claw Proxy

**Defined:** 2026-04-11
**Core Value:** Developers can connect and control all of their AI provider accounts through one reliable local desktop proxy without juggling fragile config files or raw API keys across tools.

## v1 Requirements

### Desktop App

- [ ] **DESK-01**: User can install and launch Claw Proxy on macOS, Windows, and Linux
- [ ] **DESK-02**: User can open a desktop settings window that shows proxy status, connected services, and next actions
- [ ] **DESK-03**: User can configure whether the app launches automatically at system login

### Proxy Runtime

- [ ] **PROX-01**: User can start and stop the local proxy runtime from the UI
- [ ] **PROX-02**: User can configure the proxy listen host, port, and base endpoint from the UI
- [ ] **PROX-03**: User can apply configuration changes without manually editing config files
- [ ] **PROX-04**: User can see whether the local proxy is healthy, stopped, or misconfigured

### Provider Connections

- [ ] **PROV-01**: User can add at least one supported LLM provider connection from the UI
- [ ] **PROV-02**: User can securely store provider credentials or session tokens on the local machine
- [ ] **PROV-03**: User can enable or disable individual providers without restarting the app
- [ ] **PROV-04**: User can connect more than one account for the same provider

### Routing & Compatibility

- [ ] **ROUT-01**: User can define which provider or model family should be used by default for outgoing requests
- [ ] **ROUT-02**: User can define failover or account rotation behavior when a provider/account becomes unavailable or rate-limited
- [ ] **ROUT-03**: User can expose at least one tool-compatible API surface for external AI clients to use through the local proxy

### Client Onboarding

- [ ] **CLNT-01**: User can view guided setup instructions for connecting Claude Code to the local proxy
- [ ] **CLNT-02**: User can view guided setup instructions for connecting Codex to the local proxy

### Diagnostics

- [ ] **DIAG-01**: User can see current connection state for each provider and account
- [ ] **DIAG-02**: User can inspect recent proxy activity and error messages from the desktop UI
- [ ] **DIAG-03**: User can follow guided recovery steps when authentication or proxy configuration is broken

## v2 Requirements

### Advanced Routing

- **AROU-01**: User can create rule-based routing by tool, model name pattern, or request type
- **AROU-02**: User can pin fallback chains per provider group instead of one global policy

### Data & Sync

- **SYNC-01**: User can back up and restore configuration securely across machines
- **SYNC-02**: User can optionally sync non-secret preferences through a cloud account

### Insights

- **INSG-01**: User can view per-provider usage summaries and cost estimates
- **INSG-02**: User can export diagnostics bundles for support and bug reports

## Out of Scope

| Feature | Reason |
|---------|--------|
| Built-in AI chat workspace | The product should stay focused on proxy management for external tools |
| Team admin console | v1 is single-user and local-first |
| Mobile remote control app | Does not strengthen the initial desktop proxy value |
| Full billing reconciliation across providers | Useful later, but not required to validate the routing/control thesis |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| DESK-01 | Phase 6 | Pending |
| DESK-02 | Phase 1 | Pending |
| DESK-03 | Phase 6 | Pending |
| PROX-01 | Phase 2 | Pending |
| PROX-02 | Phase 2 | Pending |
| PROX-03 | Phase 2 | Pending |
| PROX-04 | Phase 2 | Pending |
| PROV-01 | Phase 3 | Pending |
| PROV-02 | Phase 3 | Pending |
| PROV-03 | Phase 3 | Pending |
| PROV-04 | Phase 4 | Pending |
| ROUT-01 | Phase 4 | Pending |
| ROUT-02 | Phase 4 | Pending |
| ROUT-03 | Phase 4 | Pending |
| CLNT-01 | Phase 5 | Pending |
| CLNT-02 | Phase 5 | Pending |
| DIAG-01 | Phase 5 | Pending |
| DIAG-02 | Phase 5 | Pending |
| DIAG-03 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 19 total
- Mapped to phases: 19
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-11*
*Last updated: 2026-04-11 after initial definition*

---
phase: 01
slug: runtime-foundation
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-11
---

# Phase 01 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| UI renderer -> Tauri command layer | Untrusted renderer actions can only cross into the host through named Tauri commands. | Runtime initialization requests, snapshot reads, ping calls |
| Runtime state -> serialized snapshot | Host-owned state is reduced to a UI-safe serialized snapshot before it crosses into React. | App readiness, status enum, profile label, slot counts, health-check timestamp |
| UI state bridge -> command helpers | The React shell may reflect host state but must not become a second authority. | Normalized runtime snapshot data |
| Local build scripts -> desktop bundle | Tooling and package configuration determine what code and dependencies ship in the app bundle. | Frontend build output, Tauri bundle inputs |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-1-01 | T | `src-tauri/src/lib.rs` command registration | mitigate | Rust remains the runtime authority and registers only the explicit `get_runtime_snapshot`, `initialize_runtime_state`, and `ping_runtime` commands. | closed |
| T-1-02 | I | `package.json` / tooling scripts | mitigate | Phase 1 keeps a narrow desktop dependency surface limited to Tauri, React, Vite, TypeScript, and focused test tooling; no secret-handling SDKs or storage packages were introduced. | closed |
| T-1-03 | E | renderer bootstrap | mitigate | The renderer has no direct secret access or mutable host handles; it boots through React only and reaches the host through typed IPC helpers. | closed |
| T-1-04 | T | runtime commands | mitigate | Runtime access is constrained to narrow, named commands with typed return models in `src-tauri/src/commands/runtime.rs`. | closed |
| T-1-05 | I | serialized runtime snapshot | mitigate | `RuntimeSnapshot` contains only UI-safe summary fields such as status, counts, profile label, and health marker; no secrets or internal credential material are serialized. | closed |
| T-1-06 | E | renderer-owned state drift | mitigate | `AppRuntimeState` owns snapshot writes in Rust, while the renderer hook only initializes and refreshes host-owned snapshots. | closed |
| T-1-07 | T | `src/features/runtime/state.ts` | mitigate | The shell hook is a thin reflection layer over `initializeRuntimeState()` and `getRuntimeSnapshot()`, preventing React from becoming a competing state authority. | closed |
| T-1-08 | I | `RuntimeStatusCard` rendering | mitigate | The status card renders only non-secret snapshot summaries such as profile, provider count, account count, and health-check display. | closed |
| T-1-09 | D | shell boot verification | mitigate | Phase verification includes focused UI tests plus a successful `pnpm tauri build --debug` smoke path to catch broken desktop boot flows early. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

No accepted risks.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-11 | 9 | 9 | 0 | Codex (`/gsd-secure-phase 1`) |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-04-11

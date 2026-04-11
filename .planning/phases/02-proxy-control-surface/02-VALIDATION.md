---
phase: 2
slug: proxy-control-surface
status: ready
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-11
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Vitest 3.2.4 for frontend behavior and `cargo test` for Rust host modules |
| **Config file** | `vitest.config.ts` for frontend; Rust uses the built-in test harness under `src-tauri` |
| **Quick run command** | `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_` or `cargo test --manifest-path src-tauri/Cargo.toml proxy_runtime_` or `pnpm vitest run src/features/proxy/__tests__/proxy-controls.test.tsx` |
| **Full suite command** | `pnpm vitest run && cargo test --manifest-path src-tauri/Cargo.toml && pnpm tauri build --debug` |
| **Estimated runtime** | ~10-30 seconds quick, ~120-180 seconds full |

---

## Sampling Rate

- **After every task commit:** Run exactly one focused command for the touched surface only: `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_`, `cargo test --manifest-path src-tauri/Cargo.toml proxy_runtime_`, or `pnpm vitest run src/features/proxy/__tests__/proxy-controls.test.tsx`
- **After every plan wave:** Run `pnpm vitest run && cargo test --manifest-path src-tauri/Cargo.toml`
- **Before `/gsd-verify-work`:** Run `pnpm vitest run && cargo test --manifest-path src-tauri/Cargo.toml && pnpm tauri build --debug`
- **Max feedback latency:** 30 seconds for focused task checks, 180 seconds for full wave/phase gates

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 2-01-01 | 01 | 1 | PROX-01, PROX-02, PROX-04 | T-2-01 / T-2-02 / T-2-03 | Runtime lifecycle, loopback-only settings validation, and misconfigured-state semantics are established before command wiring | Rust unit + frontend contract | `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_` or `cargo test --manifest-path src-tauri/Cargo.toml proxy_runtime_` or `pnpm vitest run src/features/proxy/models.test.ts` | ✅ planned via 02-01 Task 1-2 | ⬜ pending |
| 2-02-01 | 02 | 2 | PROX-01, PROX-02, PROX-03, PROX-04 | T-2-04 / T-2-05 / T-2-06 | Typed lifecycle/apply commands persist only validated good settings and preserve the last known good config on failure | Rust integration | `cargo test --manifest-path src-tauri/Cargo.toml proxy_persistence_` or `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_apply` | ✅ planned via 02-02 Task 1-2 | ⬜ pending |
| 2-03-01 | 03 | 3 | PROX-01, PROX-02, PROX-03, PROX-04 | T-2-07 / T-2-08 / T-2-09 | UI refreshes from canonical commands, exposes start/stop/apply flows, and renders healthy/stopped/misconfigured states without trusting event payloads | Frontend integration + build smoke | `pnpm vitest run src/features/proxy/__tests__/proxy-controls.test.tsx` or `pnpm vitest run src/features/proxy/__tests__/proxy-runtime-status.test.tsx` | ✅ planned via 02-03 Task 1-2 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src-tauri/src/runtime/proxy_runtime.rs` tests for start/stop/cancel/restart sequencing
- [ ] `src-tauri/src/runtime/persistence.rs` tests using a temp SQLite database and migration bootstrap
- [ ] `src-tauri/src/models/proxy_settings.rs` tests for host/port/base endpoint validation and serialization
- [ ] `src/features/proxy/__tests__/proxy-controls.test.tsx` for form edit/apply and disabled-state behavior
- [ ] `src/features/proxy/__tests__/proxy-runtime-status.test.tsx` for healthy/stopped/misconfigured rendering and event refresh behavior

Wave 0 mapping:
- `02-01` Task 1 creates `src/features/proxy/models.test.ts` and the Rust proxy contract tests.
- `02-01` Task 2 creates the supervised runtime tests and runtime-state verification hooks.
- `02-02` Task 1 creates persistence and migration coverage for SQLite-backed settings.
- `02-03` Task 1 creates `proxy-controls.test.tsx` and `proxy-runtime-status.test.tsx`.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Desktop app can start the proxy, apply a valid config change, and recover visibly from an invalid config | PROX-01, PROX-02, PROX-03, PROX-04 | Final confidence still requires one end-to-end desktop interaction across runtime lifecycle, persistence, and renderer state | Launch the debug app, start the proxy, change host/port/base endpoint from the UI, apply changes, confirm the status surface updates, then enter an invalid value and confirm the UI reports a misconfigured state without requiring file edits |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 30s for focused checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-04-11

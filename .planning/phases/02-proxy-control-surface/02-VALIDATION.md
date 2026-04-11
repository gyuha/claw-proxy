---
phase: 2
slug: proxy-control-surface
status: draft
nyquist_compliant: false
wave_0_complete: false
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
| **Quick run command** | `pnpm vitest run src/features/proxy/__tests__/proxy-controls.test.tsx src/features/proxy/__tests__/proxy-runtime-status.test.tsx && cargo test --manifest-path src-tauri/Cargo.toml proxy_` |
| **Full suite command** | `pnpm vitest run && cargo test --manifest-path src-tauri/Cargo.toml && pnpm tauri build --debug` |
| **Estimated runtime** | ~120-180 seconds |

---

## Sampling Rate

- **After every task commit:** Run `pnpm vitest run <focused proxy tests> && cargo test --manifest-path src-tauri/Cargo.toml <focused proxy module>`
- **After every plan wave:** Run `pnpm vitest run && cargo test --manifest-path src-tauri/Cargo.toml`
- **Before `/gsd-verify-work`:** Run `pnpm vitest run && cargo test --manifest-path src-tauri/Cargo.toml && pnpm tauri build --debug`
- **Max feedback latency:** 180 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 2-01-01 | 01 | 1 | PROX-01 | T-2-01 | Runtime lifecycle commands start, stop, and restart only through the Rust supervisor | Rust unit + integration | `cargo test --manifest-path src-tauri/Cargo.toml proxy_runtime_supervision` | ❌ W0 | ⬜ pending |
| 2-02-01 | 02 | 2 | PROX-02, PROX-03 | T-2-02 / T-2-03 | Host, port, and base endpoint are validated in Rust, persisted in SQLite, and applied without renderer-side file edits | Rust unit + integration | `cargo test --manifest-path src-tauri/Cargo.toml proxy_settings_validation proxy_settings_apply` | ❌ W0 | ⬜ pending |
| 2-03-01 | 03 | 3 | PROX-04 | T-2-04 | UI reflects healthy, stopped, and misconfigured states from host-owned snapshot refreshes and lifecycle events | Frontend integration + build smoke | `pnpm vitest run src/features/proxy/__tests__/proxy-runtime-status.test.tsx src/features/proxy/__tests__/proxy-controls.test.tsx && pnpm tauri build --debug` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src-tauri/src/runtime/proxy_runtime.rs` tests for start/stop/cancel/restart sequencing
- [ ] `src-tauri/src/runtime/persistence.rs` tests using a temp SQLite database and migration bootstrap
- [ ] `src-tauri/src/models/proxy_settings.rs` tests for host/port/base endpoint validation and serialization
- [ ] `src/features/proxy/__tests__/proxy-controls.test.tsx` for form edit/apply and disabled-state behavior
- [ ] `src/features/proxy/__tests__/proxy-runtime-status.test.tsx` for healthy/stopped/misconfigured rendering and event refresh behavior

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Desktop app can start the proxy, apply a valid config change, and recover visibly from an invalid config | PROX-01, PROX-02, PROX-03, PROX-04 | Final confidence still requires one end-to-end desktop interaction across runtime lifecycle, persistence, and renderer state | Launch the debug app, start the proxy, change host/port/base endpoint from the UI, apply changes, confirm the status surface updates, then enter an invalid value and confirm the UI reports a misconfigured state without requiring file edits |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 180s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

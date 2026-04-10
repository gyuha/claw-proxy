---
phase: 1
slug: runtime-foundation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-11
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | vitest + cargo test |
| **Config file** | `vitest.config.ts` or `vite.config.ts` if shared; Rust tests in `src-tauri` |
| **Quick run command** | `cargo test --lib && pnpm vitest run` |
| **Full suite command** | `pnpm vitest run && cargo test && pnpm tauri build --debug` |
| **Estimated runtime** | ~60-120 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --lib && pnpm vitest run`
- **After every plan wave:** Run `pnpm vitest run && cargo test && pnpm tauri build --debug`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 1-01-01 | 01 | 1 | DESK-02 | T-1-01 / — | App bootstrap exposes no renderer-owned runtime truth | build + unit | `cargo test --lib && pnpm vitest run` | ❌ W0 | ⬜ pending |
| 1-02-01 | 02 | 1 | DESK-02 | T-1-02 / — | IPC contracts are typed and runtime-owned | unit | `cargo test --lib && pnpm vitest run` | ❌ W0 | ⬜ pending |
| 1-03-01 | 03 | 2 | DESK-02 | T-1-03 / — | UI renders runtime snapshot without ad hoc payloads | build + integration smoke | `pnpm vitest run && cargo test && pnpm tauri build --debug` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `vitest.config.ts` or shared Vite/Vitest config — enable TS-side verification
- [ ] `src-tauri/src/**/tests.rs` or equivalent Rust unit-test module — runtime snapshot/state tests
- [ ] `package.json` scripts for `test` and `build:desktop` — stable verification commands

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Desktop shell layout opens and shows runtime status area | DESK-02 | Visual desktop rendering still needs one human check | Launch the debug app, confirm the main window opens and the runtime status surface is visible |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

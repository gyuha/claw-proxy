---
phase: 1
slug: core-proxy-foundation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-10
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in unit and integration test harness via `cargo test` |
| **Config file** | none |
| **Quick run command** | `cargo test -p claw-proxy-core` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p claw-proxy-core`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | PROXY-01 | T-1-01 | Startup paths fail with sanitized structured errors and status reports configured ports only | integration/smoke | `cargo test -p claw-proxy-core boot_smoke_starts_servers -- --exact` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 2 | PROXY-05 | T-1-02 | Unsupported roles and content shapes are rejected with 4xx instead of coerced into permissive defaults | unit | `cargo test -p claw-proxy-core normalizer::tests -- --nocapture` | ✅ | ⬜ pending |
| 01-02-02 | 02 | 2 | PROXY-02 | T-1-02 | OpenAI chat-completions ingress returns caller-compatible JSON for the supported text-only subset | integration | `cargo test -p claw-proxy-core openai_chat_completion_roundtrip -- --exact` | ❌ W0 | ⬜ pending |
| 01-02-03 | 02 | 2 | PROXY-03 | T-1-03 | Anthropic ingress and count-tokens gateway preserve required headers, alias Claude-facing model IDs onto configured OpenAI models, and emit Anthropic-compatible JSON | integration | `cargo test -p claw-proxy-core anthropic_gateway_contract -- --exact` | ❌ W0 | ⬜ pending |
| 01-03-01 | 03 | 3 | PROV-01 | T-1-04 | OpenAI adapter maps upstream success and sanitized failure paths through a mock upstream instead of the public network | integration | `cargo test -p claw-proxy-core openai_provider_mock_upstream -- --exact` | ❌ W0 | ⬜ pending |
| 01-03-02 | 03 | 3 | PROXY-04 | T-1-03 | Claude Code gateway surfaces support text-only local workflows with `/v1/messages` plus `/v1/messages/count_tokens`, and the smoke run succeeds with only `ANTHROPIC_BASE_URL` changed | integration/manual | `cargo test -p claw-proxy-core anthropic_messages_roundtrip -- --exact` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `core/tests/proxy_boot.rs` — server boot and configured-port smoke coverage for PROXY-01
- [ ] `core/tests/openai_ingress.rs` — `/v1/chat/completions` round-trip fixture tests for PROXY-02
- [ ] `core/tests/anthropic_ingress.rs` — `/v1/messages` and `/v1/messages/count_tokens` gateway tests for PROXY-03 and PROXY-04
- [ ] `core/tests/openai_provider.rs` — mock-upstream adapter coverage for PROV-01
- [ ] shared mock-upstream helper — deterministic Axum test server or equivalent fixture harness

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Claude Code can target the local proxy with a real session using `ANTHROPIC_BASE_URL` | PROXY-04 | Requires an installed Claude Code client and a real local interactive workflow outside automated CI | Start the core with a test config, export only `ANTHROPIC_BASE_URL=http://127.0.0.1:<proxy_port>`, leave Anthropic model override env vars unset, run a simple Claude Code prompt, and confirm the proxy receives `/v1/messages` and `/v1/messages/count_tokens` traffic |
| Real upstream OpenAI smoke using a human-supplied API key | PROV-01 | Repository config does not include a live credential and automated coverage should stay mock-based | Supply a valid API key in local config, run the core, send a `curl` request to `/v1/chat/completions`, and confirm a successful upstream-backed response |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

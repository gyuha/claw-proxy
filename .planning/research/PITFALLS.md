# Pitfalls Research

**Domain:** Cross-platform desktop local AI proxy / gateway for coding tools
**Researched:** 2026-04-11
**Confidence:** MEDIUM

## Critical Pitfalls

### Pitfall 1: Plain-Text Secret Drift

**What goes wrong:** Provider tokens or session material end up in config files, logs, or crash dumps.  
**Why it happens:** Teams prototype quickly with one provider and never split secret storage from regular settings.  
**How to avoid:** Separate secrets from config on day one; use Stronghold or OS-backed secure storage for credentials and sanitize logs.  
**Warning signs:** Tokens appear in exported config, debug output, or frontend state snapshots.  
**Phase to address:** Phase 3

---

### Pitfall 2: Runtime and UI State Divergence

**What goes wrong:** The UI says the proxy is healthy while the local runtime is down, misconfigured, or routing through a stale account.  
**Why it happens:** Status is inferred from optimistic UI actions instead of host-owned runtime truth.  
**How to avoid:** Make host/runtime snapshots authoritative and stream status into the UI.  
**Warning signs:** Manual restart "fixes" issues the UI never reported; settings appear applied but live traffic disagrees.  
**Phase to address:** Phase 2

---

### Pitfall 3: Provider Sprawl in v1

**What goes wrong:** Too many providers and client integrations are added before the account model and routing semantics are stable.  
**Why it happens:** Proxy products are tempted to market breadth instead of reliability.  
**How to avoid:** Support 2-3 provider paths and 2 flagship clients first, then expand from a stable adapter model.  
**Warning signs:** Every provider screen behaves differently or routing rules depend on provider-specific exceptions.  
**Phase to address:** Phase 3 and Phase 4

---

### Pitfall 4: Clever Routing Before Reliable Routing

**What goes wrong:** Weighted policies, model translation, and dynamic heuristics land before simple fallback and health-aware rotation work.  
**Why it happens:** Routing feels like the differentiator, so teams overbuild it early.  
**How to avoid:** Ship deterministic primary/fallback and round-robin first, with explicit health/backoff semantics.  
**Warning signs:** Users cannot predict which account handled a request or why failover occurred.  
**Phase to address:** Phase 4

---

### Pitfall 5: Packaging Left Until the End

**What goes wrong:** The app works in development but fails under real installers, login-start behavior, or signed update constraints.  
**Why it happens:** Desktop packaging is treated as release polish rather than part of the product.  
**How to avoid:** Validate platform packaging assumptions early and keep distribution concerns visible in the roadmap.  
**Warning signs:** Startup permissions, localhost binding, or updater signatures only get tested at release time.  
**Phase to address:** Phase 6, with small checks earlier

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hard-code provider behavior in one routing module | Faster first demo | Makes every new provider riskier and harder to test | Only for a throwaway prototype, not this project |
| Use one JSON file for all state including secrets | Minimal code | Trust and migration problems | Never acceptable |
| Build diagnostics last | Faster early UI demo | Impossible-to-debug failures later | Only if host/runtime already emits structured events |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| OAuth/browser login flows | Assume callback and session detection behave the same on all platforms | Isolate platform-specific auth handling behind provider adapters and verify per OS |
| Coding-tool compatibility | Assume "OpenAI-compatible" means behavior-compatible for every tool | Explicitly verify Claude Code and Codex onboarding flows with real local config steps |
| Localhost networking | Assume packaging does not affect ports, permissions, or background processes | Test installer/runtime behavior on each target OS early |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Logging raw headers/tokens | Credential leakage | Redact at source and log structured error classes instead |
| Treating local apps as low-risk | Silent trust failures and poor secret handling | Apply the same secret/storage discipline as a server product |
| Blindly importing external config | Invalid routing or malicious settings application | Validate and sanitize imported configuration before apply |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Too much provider jargon | Users cannot tell what to do next | Use task-focused copy: connect, verify, retry, fallback |
| Hidden routing behavior | Users do not trust account switching | Show current default, fallback order, and recent decisions |
| Opaque failure messages | Re-auth and restart loops | Surface the last error plus one clear next step |

## "Looks Done But Isn't" Checklist

- [ ] **Provider connection:** Verify re-auth and expired-session recovery, not just happy-path connect
- [ ] **Routing:** Verify fallback actually triggers under simulated account/provider failure
- [ ] **Diagnostics:** Verify the UI shows the same truth the runtime sees
- [ ] **Packaging:** Verify installer build, startup behavior, and update signatures on all target OSes

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Plain-text secret drift | Phase 3 | Secrets never appear in exported config or normal logs |
| Runtime and UI state divergence | Phase 2 | UI status matches runtime state under stop/restart/error scenarios |
| Provider sprawl in v1 | Phase 3-4 | v1 provider/client matrix stays intentionally narrow |
| Clever routing before reliable routing | Phase 4 | Deterministic fallback works before advanced policies are considered |
| Packaging left until the end | Phase 6 | Signed, installable builds work across macOS, Windows, and Linux |

## Sources

- VibeProxy repository and README — reference product and likely desktop proxy pain points
- Tauri plugin docs — packaging, storage, and startup/update capability constraints
- `.planning/research/FEATURES.md` — feature-scope signals from adjacent products in this category

---
*Pitfalls research for: cross-platform desktop local AI proxy / gateway*
*Researched: 2026-04-11*

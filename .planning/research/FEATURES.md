# Feature Landscape

**Domain:** Cross-platform desktop local AI proxy / gateway for coding tools
**Project:** Claw Proxy
**Researched:** 2026-04-11
**Overall confidence:** MEDIUM-HIGH

## Executive Take

Products in this space are credible when they make local proxying dependable, understandable, and fast to adopt. Users expect one local endpoint, secure account storage, support for more than one provider, and clear routing/failover behavior. They do not expect a first-party chat product, team SaaS, or enterprise observability platform in v1.

The market signal is consistent across VibeProxy, go-llm-proxy, LiteLLM, Proxify, and adjacent proxy projects: the core job is to let existing coding tools keep working while the proxy handles provider differences, account rotation, and failure recovery. The strongest desktop-specific opportunity is GUI-first management of this complexity.

Inference: for Claw Proxy, a credible v1 is not "support everything." It is "support the most common local workflow without terminal surgery." That means UI-managed proxy lifecycle, 2-3 major provider integrations, multiple accounts per provider, simple routing/failover, secure storage, and setup flows for the top coding clients.

## Table Stakes

Features users will reasonably expect in v1. Missing these makes the app feel incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Embedded local proxy lifecycle management | The product's core promise is a local proxy the user can start, stop, restart, and inspect from the GUI | Medium | Must include port/base URL display, running status, startup errors, and launch-at-login/background behavior |
| Secure credential and session storage | This product handles paid provider credentials and account sessions; users will not trust plain-text config files | High | Use OS keychain/credential vaults. API key entry and browser/OAuth/session-based login should land in the same account model |
| Multi-provider connections | "One endpoint, many providers" is the baseline value proposition for this category | Medium | v1 should support a narrow but credible set: Anthropic, OpenAI-compatible, and one more high-demand path such as Google or OpenRouter |
| Multiple accounts per provider | This is a core reason to use a proxy instead of hand-managed configs | Medium | Must support add/remove/disable accounts, account labels, default account, and health state per account |
| Simple routing and failover | Users expect the proxy to survive rate limits, auth failures, or temporary outages without manual reconfiguration | High | v1 only needs deterministic rules: primary account/provider, fallback order, optional round-robin across healthy accounts |
| Coding-tool compatibility | The proxy only matters if users can point real coding tools at it quickly | High | v1 should explicitly support Claude Code and Codex first. Support can be via compatible endpoints plus generated setup instructions/config snippets |
| Connection health and actionable diagnostics | Users need to know why a provider or account is failing without reading raw logs | Medium | Show provider/account health, last error, last successful request, and one-click retry/re-auth actions |
| Basic request activity view | Once traffic is flowing, users expect lightweight visibility into what the proxy is doing | Medium | Keep this narrow: recent requests, destination provider/account, latency, status, token usage if available |
| Human-readable settings and recovery flows | GUI-first management is the desktop differentiator | Medium | Common tasks must not require editing YAML/TOML. Import/export/reset can be secondary, but normal setup must be UI-native |
| Cross-platform packaging and updates | Cross-platform delivery is part of the product thesis, not a later polish pass | High | Installer, auto-update path, and predictable local networking behavior on macOS, Windows, and Linux are part of feature credibility |

## Differentiators

Features that materially improve the product, but are not required to make v1 believable. Pick one or two for the first release; defer the rest.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Browser-assisted account bridging for subscription-backed accounts | Makes the app feel radically easier than raw API-key setup and aligns with VibeProxy-style value | High | Strong candidate differentiator if implemented for one provider well instead of many providers poorly |
| Per-tool setup wizard and config generator | Turns a proxy into a usable product by eliminating manual client configuration | Medium | Generate ready-to-paste settings for Claude Code, Codex, Continue/Cline later |
| Custom OpenAI-compatible provider adapter | Lets advanced users connect local/self-hosted or niche providers without waiting for first-party support | Medium | Good leverage feature if the internal provider model is adapter-based |
| Weighted routing / account pool policies | Useful for balancing quotas, paid tiers, or performance | High | Not needed for day-one. Basic fallback beats clever routing in v1 |
| Request replay / deep inspection | Valuable for debugging broken agent behavior and provider incompatibilities | High | Better as a power-user view after the core proxy is stable |
| Usage summaries and account pressure signals | Helps users understand which accounts are being consumed fastest | Medium | A lightweight "recent usage / recent failures" summary is enough; full billing analytics should wait |
| Hot-reload configuration without interrupting active clients | Improves trust and day-to-day usability | Medium | Nice operational upgrade after lifecycle basics are solid |
| Local-model and protocol translation support | Expands the market to users who want coding tools pointed at non-native backends | High | Valuable, but this is a second wedge after cloud-provider reliability is proven |

## Anti-Features

Features to explicitly not build in v1.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Built-in chat playground or general AI assistant UI | Pulls the product toward being another chat app instead of a control plane for external tools | Keep the app focused on proxy management, status, routing, and setup |
| Hosted team control plane / multi-user org features | Adds auth, tenancy, sync, billing, and support burden before the core local workflow is validated | Stay local-first and single-user in v1 |
| Full enterprise gateway feature set | Spend controls, RBAC, virtual keys, and audit pipelines are real needs, but they belong to server gateways like LiteLLM, not an initial desktop product | Keep auth simple and local; defer admin policy layers |
| Support for every coding client on day one | Integration breadth creates endless edge cases and slows delivery | Support two flagship clients well, then expand |
| Advanced observability stack integrations | Langfuse, OpenTelemetry, SIEM hooks, and warehouse exports add complexity without helping early validation | Ship a simple local activity/history view first |
| Provider-specific billing analytics | Requires fragile cost modeling and broad provider surface area | Show lightweight request counts and error patterns instead |
| Plugin marketplace / extension scripting | Creates long-term maintenance and security burden too early | Use an internal adapter architecture so new providers can be added by the product team first |
| Cloud sync of secrets/config by default | Raises the trust bar immediately and expands the threat surface | Keep storage local and explicit |

## Feature Dependencies

```text
Embedded proxy runtime -> Coding-tool compatibility -> Request activity view
Secure credential storage -> Provider connections -> Multiple accounts per provider
Provider/account model -> Health checks -> Routing and failover
Routing and failover -> Reliable request handling -> Trust in "set and forget" usage
Cross-platform packaging -> Startup/background behavior -> Real desktop credibility
Per-tool setup wizard -> Lower onboarding friction -> Faster v1 validation
```

## MVP Recommendation

Prioritize:

1. **Proxy lifecycle + local settings UI**
   - Start/stop/restart, port management, base URL, launch-at-login, proxy auth token, and clear runtime status
2. **Secure provider/account management**
   - Support a small set of providers well, with multiple accounts per provider and OS-backed secret storage
3. **Simple routing and failover**
   - Primary account/provider plus ordered fallback and optional round-robin across healthy accounts
4. **Top-client onboarding**
   - Make Claude Code and Codex easy to connect with generated instructions or config snippets
5. **Basic health + activity diagnostics**
   - Recent requests, account/provider health, last error, retry/re-auth, and simple latency/token visibility

Ship one differentiator if capacity allows:

1. **Browser-assisted account bridging** for one high-value provider
2. **Per-tool setup wizard** if account bridging is not feasible for v1

## Defer From v1

Defer:

- Broad client matrix beyond 2 flagship coding tools
- Enterprise gateway features like RBAC, virtual keys, budgets, and multi-tenant dashboards
- Heavy analytics and provider-specific billing views
- Deep request replay, packet-level inspection, and external observability integrations
- Local model translation pipelines, OCR/PDF/search processors, and other proxy-side enrichment
- Cloud sync, team collaboration, or remote management
- Plugin ecosystems and user-authored provider adapters

## Scope Guidance for Requirements

For Claw Proxy v1, "credible" means a user can install the app, connect real accounts, point a coding tool at one local endpoint, and trust the proxy to keep working when a provider/account has trouble. Anything beyond that should be justified as either a sharp differentiator or a deliberate phase-two expansion.

Intentional narrowness is acceptable in v1:

- Support 2-3 provider paths, not dozens
- Support 2 coding tools well, not the whole ecosystem
- Provide simple fallback rules, not policy engines
- Provide recent activity and errors, not full observability
- Stay single-user and local-first

## Sources

- VibeProxy README and repository: https://github.com/automazeio/vibeproxy (HIGH for reference product direction; accessed 2026-04-11)
- go-llm-proxy official docs/site: https://go-llm-proxy.com/ (HIGH for current coding-tool proxy feature patterns; accessed 2026-04-11)
- LiteLLM official docs: https://docs.litellm.ai/ (HIGH for broader LLM gateway feature surface and what to defer; accessed 2026-04-11)
- Proxify repository: https://github.com/lanqian528/proxify (MEDIUM for adjacent proxy ergonomics and unified endpoint patterns; accessed 2026-04-11)
- AI Worker Proxy repository: https://github.com/zxcloli666/AI-Worker-Proxy (MEDIUM for adjacent signals on failover, key rotation, and multi-provider expectations; accessed 2026-04-11)

## Confidence Notes

- **HIGH confidence:** Users expect one local endpoint, secure account storage, multi-provider support, multiple accounts, and simple failover/routing.
- **MEDIUM confidence:** Browser-assisted subscription/account bridging is a strong differentiator, but implementation cost and provider volatility make it a selective v1 candidate rather than a baseline requirement.
- **MEDIUM confidence:** Advanced gateway features are valuable later, but current evidence suggests they are more characteristic of team/server gateways than a local single-user desktop v1.

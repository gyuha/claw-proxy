# Stack Research

**Domain:** Cross-platform desktop local AI proxy / gateway for coding tools
**Researched:** 2026-04-11
**Confidence:** MEDIUM-HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Tauri | 2.x | Cross-platform desktop shell | Strong fit for a local-first app with native capabilities, smaller distribution footprint than Electron, and first-party plugins for autostart and signed updater flows across desktop platforms |
| Rust | stable 1.77+ | Embedded runtime and host integration layer | Best match for keeping proxy ingress, routing, diagnostics, and lifecycle ownership inside the app instead of shipping a separate sidecar |
| React | 19.x | Desktop UI framework | Fast UI iteration, mature ecosystem, and a comfortable fit for a settings-heavy control plane |
| TypeScript | 5.x | Frontend and shared schema typing | Helps keep config, routing rules, and provider/account models consistent across UI and host boundaries |
| Vite | 8.x | Frontend build/dev pipeline | Standard pairing with React for Tauri desktop apps, quick iteration, and low ceremony |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Tailwind CSS | 4.x | Desktop UI styling system | Good fit for turning the existing design direction into reusable application primitives without a heavy component framework |
| `@tauri-apps/plugin-autostart` | 2.x | Launch at system startup | Use for the "launch at login" requirement on macOS, Windows, and Linux |
| `@tauri-apps/plugin-updater` | 2.x | Signed app updates | Use once packaging is stable; Tauri's updater requires signed artifacts, which is good for a trust-sensitive product |
| `keyring` | current | OS credential vault integration | Use for provider credentials and session tokens; do not store secrets in SQLite or general app-state stores |
| `sqlx` + SQLite | current | Non-secret metadata, policy, and migrations | Use for account metadata, routing policy, health snapshots, and migrations |
| Tokio + Axum + Hyper + Reqwest | current | Embedded runtime, local HTTP ingress, outbound provider calls | Use to keep the proxy stack inside the Rust core with async networking and explicit control over request/response flow |
| Zustand | 5.x | Client-side UI state | Useful if the React surface becomes multi-pane and event-driven; keep it thin and domain-focused |
| Zod | 3.x or 4.x | Config and IPC payload validation | Use to validate routing rules, provider definitions, and persisted settings before runtime apply |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| Vitest | Unit and component testing | Good default for React + TypeScript logic and config/routing tests |
| Playwright | End-to-end flow verification | Use for onboarding flows and desktop-webview UI validation where practical |
| `cargo nextest` | Rust test runner | Useful once host/runtime logic grows beyond thin glue code |

## Installation

```bash
# Core
pnpm create tauri-app
pnpm add react react-dom
pnpm add -D typescript vite @vitejs/plugin-react tailwindcss

# Supporting
pnpm add zustand zod
pnpm tauri add autostart
pnpm tauri add updater

# Dev dependencies
pnpm add -D vitest @playwright/test
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Tauri 2.x | Electron | Use Electron only if the app must deeply embed a Node.js runtime in-process or depends on Electron-specific desktop APIs/plugins |
| Embedded Rust runtime | Separate Node/Python proxy sidecar | Use a sidecar only if an upstream proxy implementation becomes strategically unavoidable and its boundary can stay narrow |
| React + TypeScript + Vite | SSR/meta-framework UI | Use a heavier framework only if the app truly needs server-rendered web surfaces in addition to the desktop shell |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Electron-first assumption | Heavier packaging and a broader runtime surface than this product needs for a settings-oriented local proxy app | Tauri 2.x |
| Secrets in JSON, SQLite, or Tauri Store | Not acceptable for a trust-sensitive app handling paid AI accounts | OS keychain/keyring integration |
| Separate Node/Python proxy sidecar by default | Adds packaging, lifecycle, and debugging complexity to a product that benefits from runtime ownership | Embedded Rust runtime |
| SSR/meta-framework UI stack | Solves problems this desktop shell does not have and adds unnecessary ceremony | React + Vite desktop UI |

## Stack Patterns by Variant

**If the proxy runtime can be implemented directly in Rust:**
- Prefer an embedded runtime using Tokio + Axum + Hyper + Reqwest
- Because packaging, lifecycle control, permissions, observability, and cross-platform behavior stay simpler

**If an upstream proxy component becomes unavoidable later:**
- Isolate it behind a narrow adapter boundary
- Because product speed can improve, but only if configuration and health semantics remain host-owned

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Tauri 2.x | Rust 1.77.2+ | Tauri plugin docs for updater, store, stronghold, and autostart all call out a Rust 1.77.2 minimum |
| `@tauri-apps/plugin-updater` 2.x | Tauri 2.x | Requires signed update artifacts and a trusted update pipeline |
| Tauri 2.x | Rust 1.77.2+ | Linux packaging and secure-storage behavior should be validated early on real systems |

## Sources

- VibeProxy README — reference product direction and local desktop proxy feature baseline
- Tauri Updater docs: https://v2.tauri.app/plugin/updater/ — verified signed update support and desktop platform coverage
- Tauri Autostart docs: https://v2.tauri.app/plugin/autostart/ — verified launch-at-login support across desktop targets
- Stack researcher findings — embedded runtime recommendation, keyring/SQLite split, and Linux risk notes

---
*Stack research for: cross-platform desktop local AI proxy / gateway*
*Researched: 2026-04-11*

---
phase: 02-proxy-control-surface
plan: 03
subsystem: desktop-ui
tags: [react, tauri, vitest, desktop, proxy, ui]
requires:
  - phase: 02-proxy-control-surface
    plan: 02
    provides: typed host command surface and persisted proxy state
provides:
  - typed proxy IPC wrappers and event-driven feature state for the renderer
  - a warm desktop control surface for proxy lifecycle, status, and settings
  - loopback-only settings editing with effective local URL preview
affects: [desktop-shell, proxy-ui, host-state-refresh]
tech-stack:
  added: []
  patterns: [typed ipc wrappers, event invalidation refresh, shell-contained control cards]
key-files:
  created:
    - src/lib/ipc/proxy.ts
    - src/features/proxy/api.ts
    - src/features/proxy/state.ts
    - src/components/proxy/ProxyControlPanel.tsx
    - src/components/proxy/ProxySettingsForm.tsx
    - src/features/proxy/__tests__/proxy-controls.test.tsx
    - src/features/proxy/__tests__/proxy-runtime-status.test.tsx
  modified:
    - src/app/App.tsx
    - src/components/shell/AppShell.tsx
    - src/features/proxy/models.ts
    - src/features/runtime/__tests__/runtime-status.test.tsx
    - src/styles/app.css
key-decisions:
  - "Treat runtime://proxy-updated as invalidation only and always re-read canonical host state after lifecycle events."
  - "Keep the proxy controls inside the existing Phase 1 shell frame instead of widening the app layout or introducing renderer-owned state islands."
  - "Show both the active host-owned proxy URL and the editable draft preview so misconfigured drafts do not imply they replaced the last known good config."
patterns-established:
  - "Renderer flow: typed IPC -> feature adapter -> useProxyControlState hook -> presentational shell cards."
  - "Draft flow: local form edits stay in renderer memory, but apply/start/stop always round-trip through Rust before state refresh."
requirements-completed: [PROX-01, PROX-02, PROX-03, PROX-04]
duration: 7min
completed: 2026-04-11
---

# Phase 02 Plan 03: Build the proxy control surface and live status UI Summary

**A calm, Notion-inspired proxy control surface with typed renderer state, lifecycle controls, loopback-safe settings editing, and host-owned status feedback**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-11T10:57:56+09:00
- **Completed:** 2026-04-11T11:04:21+09:00
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added typed renderer IPC wrappers for proxy settings, lifecycle commands, and `runtime://proxy-updated` invalidation handling.
- Implemented `useProxyControlState()` so the desktop UI loads canonical host state on mount, manages a local draft form, refreshes after lifecycle actions, and preserves the last known good config story after failed apply attempts.
- Built `ProxyControlPanel` and `ProxySettingsForm` inside the existing `AppShell` so users can start and stop the proxy, edit loopback-safe settings, and see effective local URLs plus misconfiguration feedback without leaving the shell.
- Expanded Vitest coverage for hook behavior, event-driven refresh, shell wiring, and status rendering across `healthy`, `stopped`, and `misconfigured` states.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create typed proxy IPC and feature state with event-driven refresh** - `656b9d0` (`test`), `5ab3c24` (`feat`)
2. **Task 2: Build the proxy lifecycle and settings surfaces inside the existing shell** - `6d0ae40` (`test`), `c0381fb` (`feat`)

## Files Created/Modified

- `src/lib/ipc/proxy.ts` - typed renderer wrappers around proxy Tauri commands plus invalidation event subscription
- `src/features/proxy/api.ts` - thin feature adapter over the proxy IPC layer
- `src/features/proxy/state.ts` - event-driven proxy hook with draft settings, lifecycle actions, and refresh orchestration
- `src/components/proxy/ProxyControlPanel.tsx` - lifecycle buttons, active local URL, and status surface for healthy/stopped/misconfigured states
- `src/components/proxy/ProxySettingsForm.tsx` - labeled loopback-safe host/port/base-endpoint form with effective local URL preview
- `src/components/shell/AppShell.tsx` - shell composition that adds proxy controls while preserving the Phase 1 frame
- `src/styles/app.css` - proxy card, form, and action styling aligned to the warm Notion-inspired shell
- `src/features/proxy/__tests__/proxy-controls.test.tsx` - hook-level coverage for mount load, apply flow, and event-driven refresh
- `src/features/proxy/__tests__/proxy-runtime-status.test.tsx` - shell-level coverage for controls, loading disables, and misconfigured status feedback

## Decisions Made

- Kept loopback-host options explicit in the renderer (`localhost`, `127.0.0.1`, `::1`) while still treating Rust validation as authoritative.
- Showed active host-owned URL separately from draft preview so users can see when a bad draft failed to replace the running or persisted configuration.
- Reused the existing hero/runtime/future-area shell instead of introducing a second page, keeping Phase 2 additive to Phase 1.

## Deviations from Plan

None. The final UI and hook structure matched the plan’s shell-contained control-surface approach.

## Verification

- `pnpm vitest run src/features/proxy/__tests__/proxy-controls.test.tsx src/features/proxy/__tests__/proxy-runtime-status.test.tsx src/features/runtime/__tests__/runtime-status.test.tsx`
- `cargo test --manifest-path src-tauri/Cargo.toml proxy_`
- `npm run build`
- `npm run desktop:build` produced both `src-tauri/target/release/bundle/macos/Claw Proxy.app` and `src-tauri/target/release/bundle/dmg/Claw Proxy_0.1.0_aarch64.dmg`

## Issues Encountered

- An earlier `npm run desktop:build` attempt stalled during DMG packaging, but fresh verification completed successfully and produced both the macOS app bundle and DMG without requiring additional code changes.

## User Setup Required

None - the shell now reads and applies proxy lifecycle/settings directly through the embedded host runtime.

## Next Phase Readiness

- Phase 3 can build provider onboarding on top of a live desktop proxy surface instead of a placeholder shell.
- The renderer now has a reusable pattern for typed host state, event invalidation, and shell-contained feature cards.

## Self-Check: PASSED

- Summary file exists at `.planning/phases/02-proxy-control-surface/02-03-SUMMARY.md`
- Commits verified: `656b9d0`, `5ab3c24`, `6d0ae40`, `c0381fb`

---
*Phase: 02-proxy-control-surface*
*Completed: 2026-04-11*

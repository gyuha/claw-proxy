---
phase: 01-core-proxy-foundation
plan: 01
subsystem: infra
tags: [rust, axum, integration-tests, startup]
requires: []
provides:
  - Binary boot smoke coverage for YAML-configured proxy and admin ports
  - Runtime-configured admin status reporting
  - Result-based standalone startup with sanitized failure logging
affects: [phase-01, proxy-bootstrap, admin-status]
tech-stack:
  added: []
  patterns: [compiled-binary integration tests, runtime admin state, sanitized startup errors]
key-files:
  created:
    - core/tests/support/mod.rs
    - core/tests/proxy_boot.rs
  modified:
    - core/src/main.rs
    - core/src/admin/mod.rs
    - core/src/admin/handlers.rs
key-decisions:
  - "Integration coverage boots the compiled `claw-proxy` binary instead of stubbing the server."
  - "Startup logs emit only sanitized failure reasons so malformed YAML cannot leak API keys."
patterns-established:
  - "Carry runtime listener metadata through `AdminState` so admin handlers report real process state."
  - "Use `async fn run() -> Result<()>` in the binary for standalone boot paths that need clean exits."
requirements-completed: [PROXY-01]
duration: 4min
completed: 2026-04-10
---

# Phase 01 Plan 01: Standalone boot smoke coverage and sanitized startup flow

**Real-binary boot smoke coverage with runtime port reporting and sanitized standalone startup failures**

## Performance

- **Duration:** 4 min
- **Started:** 2026-04-10T15:14:30Z
- **Completed:** 2026-04-10T15:18:46Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Added reusable integration helpers that write temporary YAML configs, launch the compiled binary, poll `/status`, and clean up the child process automatically.
- Added `boot_smoke_starts_servers` to verify non-default configured proxy and admin ports through the real admin status endpoint.
- Reworked standalone startup into a Result-based async path that exits non-zero and logs sanitized failure reasons on bad config or listener startup failures.

## Task Commits

1. **Task 1: Create the Wave 0 integration harness for binary boot verification** - `7aeb268` (`feat`)
2. **Task 2: Replace hardcoded startup/status behavior with runtime-configured boot state** - `a30fcd2` (`fix`)

## Files Created/Modified
- `core/tests/support/mod.rs` - Shared temp-config, process-launch, polling, and cleanup helpers for binary boot tests.
- `core/tests/proxy_boot.rs` - Exact smoke test covering configured proxy/admin port reporting.
- `core/src/main.rs` - Result-based startup flow with sanitized failure logging and non-zero exits.
- `core/src/admin/mod.rs` - Runtime port fields carried in `AdminState`.
- `core/src/admin/handlers.rs` - `/status` now reports runtime-configured port values.

## Decisions Made
- Used the compiled `claw-proxy` binary for the smoke test so Phase 1 verifies the real standalone boot contract rather than an in-process test harness.
- Sanitized startup logging to generic failure reasons instead of surfacing config parsing details that could echo secrets from malformed YAML.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Moved runtime port wiring into Task 1 so the new smoke test could pass**
- **Found during:** Task 1 (Create the Wave 0 integration harness for binary boot verification)
- **Issue:** The new non-default-port smoke test was blocked by `/status` returning hardcoded `47380` and `47381`, which made Task 1 unverifiable against the existing runtime.
- **Fix:** Added `proxy_port` and `admin_port` to `AdminState`, populated them from YAML-backed runtime config in `main.rs`, and returned those fields from `/status`.
- **Files modified:** `core/src/main.rs`, `core/src/admin/mod.rs`, `core/src/admin/handlers.rs`
- **Verification:** `cargo test -p claw-proxy-core boot_smoke_starts_servers -- --exact`
- **Committed in:** `7aeb268`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The deviation was required to make Task 1 verifiable. It did not expand the scope beyond the plan’s stated runtime-configured status behavior.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 1 now has a reusable binary boot harness for later integration coverage.
- `/status` reports live configured ports and startup failures exit cleanly without leaking malformed-config secrets.
- No blockers found for `01-02-PLAN.md`.

## Self-Check: PASSED
- Found `.planning/phases/01-core-proxy-foundation/01-01-SUMMARY.md` on disk.
- Verified task commits `7aeb268` and `a30fcd2` exist in git history.

---
phase: 01-core-proxy-foundation
plan: 02
subsystem: proxy
tags: [rust, axum, openai, anthropic, normalization]
requires: [01-01]
provides:
  - Strict text-only OpenAI and Anthropic ingress normalization with explicit rejection paths
  - Anthropic gateway routing for `/v1/messages` and `/v1/messages/count_tokens`
  - Server-side Claude-model aliasing onto the configured OpenAI model
affects: [phase-01, proxy-ingress, claude-code-gateway, normalizer]
tech-stack:
  added: []
  patterns: [canonical request validation, protocol-specific 400 responses, server-side model aliasing]
key-files:
  created:
    - core/tests/openai_ingress.rs
    - core/tests/anthropic_ingress.rs
  modified:
    - core/src/main.rs
    - core/src/normalizer/mod.rs
    - core/src/normalizer/from_openai.rs
    - core/src/normalizer/from_anthropic.rs
    - core/src/normalizer/to_anthropic.rs
    - core/src/providers/openai.rs
    - core/src/proxy/mod.rs
key-decisions:
  - "Unsupported streaming, tool, multipart, and unsupported-role payloads fail fast with `AppError::Normalize` instead of being coerced."
  - "Anthropic-format invalid requests return Anthropic-compatible `invalid_request_error` payloads, while OpenAI-format invalid requests return OpenAI-style error envelopes."
  - "Claude-facing Anthropic model IDs are rewritten in the proxy to the first configured OpenAI model so Claude Code only needs `ANTHROPIC_BASE_URL`."
patterns-established:
  - "Normalize both public ingress routes into the same internal text-only request model before provider dispatch."
  - "Gate Anthropic routes on required gateway headers inside the proxy layer rather than pushing protocol logic into the router."
requirements-completed: [PROXY-02, PROXY-03, PROXY-04, PROXY-05]
duration: 27min
completed: 2026-04-10
---

# Phase 01 Plan 02: Strict dual-format ingress and Claude gateway compatibility

**Strict text-only normalization with protocol-preserving OpenAI/Anthropic ingress, Claude gateway header validation, and server-side Anthropic-to-OpenAI aliasing**

## Performance

- **Duration:** 27 min
- **Completed:** 2026-04-10T15:27:34Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments
- Added exact integration coverage for OpenAI chat completions, Anthropic messages, and the Claude Code `count_tokens` gateway contract.
- Changed OpenAI and Anthropic normalization to return explicit `AppError::Normalize` failures for unsupported roles, content shapes, streaming, and tool-bearing payloads.
- Added `POST /v1/messages/count_tokens`, required `anthropic-version` on Anthropic routes, accepted optional `anthropic-beta`, and returned protocol-specific `400 BAD_REQUEST` payloads for invalid ingress.
- Seeded proxy state with the first configured OpenAI model and rewrote Claude-facing Anthropic model IDs to that alias before provider dispatch.

## Task Commits

1. **Task 1: Codify the supported ingress subset in integration and unit tests** - `d9f1ad7` (`test`)
2. **Task 2: Implement strict canonical normalization for supported text-only requests** - `38a529e` (`fix`)
3. **Task 3: Wire strict route handling, server-side Anthropic model aliasing, and the Claude Code gateway count route** - `d18c946` (`feat`)

## Files Created/Modified
- `core/tests/openai_ingress.rs` - Exact OpenAI round-trip integration coverage through the proxy router.
- `core/tests/anthropic_ingress.rs` - Exact Anthropic message and gateway compatibility coverage, including aliasing and error-shape assertions.
- `core/src/normalizer/mod.rs` - Result-returning normalization entrypoints plus rejection tests for unsupported payload shapes.
- `core/src/normalizer/from_openai.rs` - Strict OpenAI text-only ingress normalization with explicit role, stream, content-array, and tool rejection.
- `core/src/normalizer/from_anthropic.rs` - Strict Anthropic normalization for string and text-block content only.
- `core/src/proxy/mod.rs` - Protocol-preserving route handling, Anthropic header validation, model aliasing, and `count_tokens` route.
- `core/src/normalizer/to_anthropic.rs` - Anthropic success payloads plus normalized text token counting.
- `core/src/main.rs` - Proxy state seeded with the configured Anthropic alias target.
- `core/src/providers/openai.rs` - Outbound OpenAI request encoding updated for the stricter ingress types.

## Decisions Made
- Kept the internal request model text-only in Phase 1 and rejected unsupported tool or multipart shapes instead of widening the abstraction early.
- Treated missing Anthropic gateway headers and normalization failures as caller errors with protocol-specific `400` responses instead of generic `500`s.
- Bound Claude-facing Anthropic model names to the first configured OpenAI model inside the proxy so client-side Anthropic model pinning stays unnecessary for the supported local workflow.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Carried the Result-returning normalizer contract through existing proxy/provider call sites**
- **Found during:** Task 2
- **Issue:** Changing `InternalRequest::from_openai` and `InternalRequest::from_anthropic` to return `Result` broke the existing proxy handlers and OpenAI provider request construction before Task 2 verification could compile.
- **Fix:** Updated `core/src/proxy/mod.rs` to handle normalization results explicitly and updated `core/src/providers/openai.rs` to emit the stricter outbound OpenAI message shape.
- **Files modified:** `core/src/proxy/mod.rs`, `core/src/providers/openai.rs`
- **Verification:** `cargo test -p claw-proxy-core normalizer::tests -- --nocapture`
- **Committed in:** `38a529e`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The deviation was required to make the stricter normalization contract compile and remain testable. It did not expand the plan beyond the intended Phase 1 ingress boundary.

## Authentication Gates
None.

## Known Stubs
None.

## Threat Flags
None.

## Issues Encountered
None after the blocking compile-through change above.

## User Setup Required
None for automated verification. Manual Claude Code dogfooding remains a follow-up validation step outside this plan.

## Next Phase Readiness
- Phase 1 now has passing ingress coverage for the supported OpenAI and Anthropic text-only subsets.
- Claude Code gateway prerequisites are in place for local Anthropic-format traffic, including `/v1/messages/count_tokens`.
- The next remaining core-proxy gap is Phase 01 Plan 03 provider-side mock-upstream coverage for the real OpenAI adapter path.

## Self-Check: PASSED
- Found `.planning/phases/01-core-proxy-foundation/01-02-SUMMARY.md` on disk.
- Verified task commits `d9f1ad7`, `38a529e`, and `d18c946` exist in git history.

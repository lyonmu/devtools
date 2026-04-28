---
phase: 02-testing
plan: 02
subsystem: testing
tags: [rust, hash, symmetric-crypto, tdd]
requires:
  - phase: 02-01
    provides: Coverage harness and waiver ledger
provides:
  - Deterministic hash known-answer tests
  - Symmetric algorithm flow and validation tests
affects: [src/algo/hash.rs, src/algo/symmetric.rs, coverage-gate]
tech-stack:
  added: []
  patterns: [inline Rust unit tests, state-level crypto flow assertions]
key-files:
  created: []
  modified:
    - src/algo/hash.rs
    - src/algo/symmetric.rs
key-decisions:
  - "Prefer state-level tests through tool state APIs, using private helpers only for legacy coverage."
patterns-established:
  - "Known-answer crypto tests assert exact lowercase hex vectors rather than output length only."
requirements-completed: [R5.1, R5.2, R5.4]
duration: 3m11s
completed: 2026-04-28
---

# Phase 02 Plan 02: Hash and Symmetric Test Coverage Summary

**Deterministic SHA/SM3/AES/SM4 state-level tests with malformed input guards for core algorithm coverage**

## Performance

- **Duration:** 3m11s
- **Started:** 2026-04-28T06:48:28Z
- **Completed:** 2026-04-28T06:51:39Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Added published SHA-256/SHA-384/SHA-512/SM3 known-answer vector tests for `abc`.
- Added hash text-vs-hex equivalence, odd/non-hex validation, empty input, and state-clearing tests.
- Added AES-128-ECB, AES-256-CBC, SM4-ECB, and SM4-CBC state-level roundtrip coverage plus exact first-block vectors.
- Added symmetric invalid input, key length, IV length, ciphertext length, padding, empty plaintext, and state transition tests.

## Task Commits

1. **Task 1: Add hash known-answer and input validation tests** - `a90bf32` (test)
2. **Task 2: Add symmetric known-answer, boundary, and error tests** - `91329ad` (test/fix)

## Files Created/Modified

- `src/algo/hash.rs` - Expanded inline tests for all hash variants, hex validation, empty input, and state transitions.
- `src/algo/symmetric.rs` - Expanded inline tests for AES/SM4 flows and added a decrypt ciphertext length guard.

## Decisions Made

- Kept tests inline in source modules per AGENTS.md.
- Used public crypto test vectors for exact assertions and state-level APIs for user-facing validation paths.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Prevented short ciphertext panics during symmetric decrypt**
- **Found during:** Task 2 (Add symmetric known-answer, boundary, and error tests)
- **Issue:** Decrypting ciphertext whose decoded length was not a multiple of 16 panicked in block copy logic instead of reporting a controlled error.
- **Fix:** Added a state-level decrypt guard returning `密文长度必须为 16 字节的倍数`.
- **Files modified:** `src/algo/symmetric.rs`
- **Verification:** `cargo test -p devtools -- algo::symmetric` passed with the new invalid ciphertext test.
- **Committed in:** `91329ad`

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** The fix was required for D-05 invalid ciphertext coverage and improves user-facing error handling without changing APIs.

## Issues Encountered

- Initial SM4 vector assertion used the AES plaintext block; corrected the test to use the standard SM4 key/plaintext vector.

## TDD Gate Compliance

- Symmetric task followed RED/GREEN evidence: the new invalid ciphertext test failed with a panic before the guard was added, then passed after the fix.
- Hash task was test-only against already implemented behavior, so no production GREEN commit was required.

## User Setup Required

None - no external service configuration required.

## Known Stubs

None.

## Next Phase Readiness

Wave 3 can build on stronger deterministic algorithm tests and a fixed symmetric decrypt error path. The full suite now passes with 36 tests.

## Self-Check: PASSED

- Found `src/algo/hash.rs` and `src/algo/symmetric.rs`.
- Found task commits `a90bf32` and `91329ad`.
- `cargo test -p devtools -- algo::hash` passed.
- `cargo test -p devtools -- algo::symmetric` passed.
- `cargo test -p devtools` passed (36 tests).

---
*Phase: 02-testing*
*Completed: 2026-04-28*

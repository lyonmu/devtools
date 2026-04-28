---
phase: 02-testing
status: no-active-waivers
updated: 2026-04-28
---

# Phase 2 Coverage Waivers

## Policy

Phase 2 uses measured line coverage as a hard completion gate for core non-GPUI modules:

- **D-01:** measured line coverage must be greater than 80% before Phase 2 completion.
- **D-02:** the hard gate applies only to `src/algo/*` and `src/cert/*`; GPUI/UI files are excluded from the acceptance calculation.
- **D-03:** coverage evidence must come from `cargo tarpaulin -p devtools --out Html`.
- **D-04:** waivers are allowed only for genuinely hard-to-measure paths and must name the exact file/function or branch, reason, missing tests, evidence command, and follow-up phase.

Ordinary untested branches are not waiver candidates. They must be covered by focused tests. Randomness-heavy paths or platform/tooling limitations may be waived only when specifically named and justified.

## Current Status

No active waivers.

## Required Waiver Template

```markdown
### W-02-NNN: [short title]

- **File:** `path/to/file.rs`
- **Function/branch:** `function_name` / exact branch or condition
- **Reason:** Why this path is genuinely hard to measure in Phase 2
- **Missing tests:** What tests would be needed to remove the waiver
- **Evidence command:** `cargo tarpaulin -p devtools --out Html`
- **Follow-up phase:** Phase number/name that will remove or revisit this waiver
```

## Baseline Evidence

- `cargo test -p devtools` passed on 2026-04-28 with 26 tests.
- `cargo tarpaulin -p devtools --out Html` ran on 2026-04-28 and generated local HTML evidence at `tarpaulin-report.html`.
- Baseline overall coverage before Phase 2 test expansion: 21.00% (501/2386 lines), with core gaps concentrated in `src/algo/*` and `src/cert/*`.

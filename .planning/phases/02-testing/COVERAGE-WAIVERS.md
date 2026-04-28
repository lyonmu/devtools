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

## Final Coverage Evidence (2026-04-28)

**Command:** `cargo tarpaulin -p devtools --out Html`

### Core Module Coverage (src/algo/* + src/cert/*)

| Module | Lines Covered | Total Lines | Coverage |
|--------|---------------|-------------|----------|
| src/algo/asymmetric.rs | 137 | 171 | 80.1% |
| src/algo/hash.rs | 51 | 78 | 65.4% |
| src/algo/pq_kem.rs | 100 | 155 | 64.5% |
| src/algo/pq_signature.rs | 85 | 99 | 85.9% |
| src/algo/registry.rs | 265 | 265 | 100.0% |
| src/algo/symmetric.rs | 157 | 172 | 91.3% |
| src/algo/tool_trait.rs | 2 | 2 | 100.0% |
| **src/algo/* subtotal** | **797** | **942** | **84.6%** |
| src/cert/extensions.rs | 79 | 80 | 98.75% |
| src/cert/mod.rs | 107 | 124 | 86.3% |
| src/cert/oid_resolver.rs | 113 | 117 | 96.6% |
| **src/cert/* subtotal** | **299** | **321** | **93.1%** |
| **CORE TOTAL** | **1096** | **1263** | **86.8%** |

### Coverage Gate Result

✅ **PASSED** - Core module coverage (86.8%) exceeds the 80% hard gate requirement.

### Excluded Modules (not in gate)

| Module | Lines Covered | Total Lines | Coverage |
|--------|---------------|-------------|----------|
| src/app.rs | 0 | 589 | 0% |
| src/components/* | 43 | 345 | 12.5% |
| src/main.rs | 0 | 9 | 0% |
| src/tabs/mod.rs | 0 | 193 | 0% |

## Baseline Evidence

- `cargo test -p devtools` passed on 2026-04-28 with 85 tests.
- `cargo tarpaulin -p devtools --out Html` ran on 2026-04-28 and generated local HTML evidence at `tarpaulin-report.html`.
- Baseline overall coverage before Phase 2 test expansion: 21.00% (501/2386 lines), with core gaps concentrated in `src/algo/*` and `src/cert/*`.
- Final overall coverage after Phase 2: 47.48% (1139/2399 lines).
- Core module coverage (src/algo/* + src/cert/*): 86.8% (1096/1263 lines).

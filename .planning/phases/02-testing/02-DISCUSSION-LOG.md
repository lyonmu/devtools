# Phase 2: 测试完善 - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-28
**Phase:** 2-测试完善
**Areas discussed:** Coverage target & enforcement, Test priority by module, Certificate fixture strategy

---

## Coverage target & enforcement

| Question | Options Presented | Selected |
|----------|-------------------|----------|
| How strict should the >80% coverage target be for Phase 2? | Hard gate for core modules; Advisory target with documented gaps; Per-module pragmatic gate | Hard gate for core modules |
| How should `cargo tarpaulin` be treated? | Required verification tool for Phase 2; Required if available, fallback documented; Advisory only | Required verification tool for Phase 2 |
| What exactly counts as “core module coverage”? | Only `src/algo/*` and `src/cert/*` line coverage; All non-GPUI Rust modules; Whole crate coverage | Only `src/algo/*` and `src/cert/*` line coverage |
| How should temporary coverage gaps be handled? | No exceptions; Explicit waiver file allowed; Overall aggregate only | Explicit waiver file allowed |

**User's choice:** Hard coverage gate for `src/algo/*` and `src/cert/*`, measured with required `cargo tarpaulin -p devtools --out Html`; explicit waivers only for genuinely hard-to-measure paths.
**Notes:** GPUI/UI files are excluded from the hard gate.

---

## Test priority by module

| Question | Options Presented | Selected |
|----------|-------------------|----------|
| Which module family should Phase 2 test first? | Crypto algorithms first; Certificate parsing first; Coverage-gap driven order | Crypto algorithms first |
| After hash and symmetric tests, how should less-deterministic crypto modules be prioritized? | RSA/ECDSA before PQ; PQ before RSA/ECDSA; Alternate by coverage gap | RSA/ECDSA before PQ |
| Where do certificate module tests fit relative to PQ crypto tests? | Certificates before PQ; PQ before certificates; Interleave based on coverage gaps | Certificates before PQ |
| How should UI/state helper tests be treated in Phase 2? | Outside hard gate, opportunistic only; Include after all core modules; Do not touch UI tests in Phase 2 | Outside hard gate, opportunistic only |

**User's choice:** Priority order is `src/algo/hash.rs`, `src/algo/symmetric.rs`, `src/algo/asymmetric.rs`, `src/cert/*`, then PQ modules. UI/state helper tests are opportunistic only.
**Notes:** Emphasis is deterministic known-answer/boundary tests first, then RSA/ECDSA user-facing flows and error paths.

---

## Certificate fixture strategy

| Question | Options Presented | Selected |
|----------|-------------------|----------|
| What kind of certificate fixtures should Phase 2 use? | Checked-in test fixtures; Inline/generated fixture data only; Error-path tests only for now | Checked-in test fixtures |
| Where should certificate fixtures live? | `src/cert/fixtures/`; `fixtures/cert/` at repo root; `testdata/cert/` at repo root | `src/cert/fixtures/` |
| Which certificate formats must Phase 2 fixtures cover? | PEM + DER + multi-PEM; PEM + DER + multi-PEM + PKCS#12; PEM only first | PEM + DER + multi-PEM + PKCS#12 |
| How should PKCS#12 fixture credentials be handled? | Use a public test password documented next to fixture; Use empty-password PKCS#12 only; Generate PKCS#12 fixture during tests | Use a public test password documented next to fixture |

**User's choice:** Add checked-in, small, non-secret fixtures in `src/cert/fixtures/`, covering PEM, DER, multi-PEM, and PKCS#12. PKCS#12 uses a documented public test password such as `test-password`.
**Notes:** No real certificates, production private keys, or sensitive credentials.

---

## Claude's Discretion

- Exact test names, fixture filenames, and waiver artifact format are left to downstream research/planning as long as locked decisions in CONTEXT.md are preserved.

## Deferred Ideas

- Performance benchmark approach was not selected and remains deferred/minimal unless Phase 2 coverage dependencies require otherwise.

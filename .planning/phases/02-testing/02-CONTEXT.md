# Phase 2: 测试完善 - Context

**Gathered:** 2026-04-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 2 delivers a stronger automated test suite for the existing Rust/GPUI cryptography desktop app. The scope is to improve confidence and measured coverage for the existing core algorithm and certificate parsing modules, not to add new crypto features, UI behavior, CI infrastructure, or full benchmarking infrastructure.

</domain>

<decisions>
## Implementation Decisions

### Coverage Target & Enforcement
- **D-01:** Phase 2 has a hard completion gate: measured line coverage must be `>80%` for core non-GPUI modules before the phase can be considered complete.
- **D-02:** The hard coverage gate applies only to `src/algo/*` and `src/cert/*`. GPUI/UI files such as `src/app.rs`, `src/tabs/*`, and `src/components/*` are excluded from the hard gate.
- **D-03:** `cargo tarpaulin -p devtools --out Html` is the required coverage verification command for Phase 2. If `cargo tarpaulin` is missing, the plan must install it or document setup before verification.
- **D-04:** Temporary coverage waivers are allowed only for genuinely hard-to-measure paths, such as randomness-heavy flows or platform/tooling limitations. Any waiver must explicitly name the file/function, reason, missing tests, and follow-up phase. Ordinary untested branches cannot be waived.

### Test Priority by Module
- **D-05:** Test work starts with crypto algorithm modules, beginning with deterministic known-answer and boundary tests for `src/algo/hash.rs` and `src/algo/symmetric.rs`.
- **D-06:** After hash and symmetric coverage, prioritize RSA/ECDSA coverage in `src/algo/asymmetric.rs` before PQ modules. Focus on user-facing flows and error paths such as missing keys, invalid PEM/hex, oversized plaintext, and signature mismatch.
- **D-07:** After hash/symmetric/asymmetric tests, prioritize `src/cert/*` before PQ modules because certificate parsing has much lighter current coverage and is a core top-level app capability.
- **D-08:** PQ modules (`src/algo/pq_kem.rs`, `src/algo/pq_signature.rs`) come after the above priorities, unless tarpaulin shows they block the `src/algo/*` + `src/cert/*` hard gate.
- **D-09:** UI/state helper tests are outside the hard coverage gate and are opportunistic only. Add them only if they support core test setup or are easy wins; they must not displace `src/algo/*` and `src/cert/*` work.

### Certificate Fixture Strategy
- **D-10:** Certificate parser tests should use checked-in, small, non-secret sample certificate fixtures for realistic, reproducible PEM/DER/multi-PEM/PKCS#12 parser coverage.
- **D-11:** Store certificate fixtures in `src/cert/fixtures/`, next to certificate parsing code, so inline tests can use stable paths such as `include_bytes!("fixtures/sample.pem")`.
- **D-12:** Fixture coverage must include all formats supported by `detect_and_parse`: PEM, DER, multi-PEM, and PKCS#12 (`.p12`/`.pfx`).
- **D-13:** PKCS#12 fixtures should use a deliberately public/non-secret test password such as `test-password`, documented next to the fixture or in test code. Do not include real certificates, private production keys, or sensitive credentials.

### Claude's Discretion
- Downstream researcher/planner may choose exact test names, fixture file names, and waiver file format, as long as the decisions above are preserved and AGENTS.md rules are followed.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and Phase Scope
- `.planning/PROJECT.md` — Project architecture, stack, constraints, and existing testing pattern.
- `.planning/REQUIREMENTS.md` — Requirement R5 defines test completeness, coverage target, unit/integration/boundary test expectations, and tarpaulin verification.
- `.planning/ROADMAP.md` — Phase 2 scope, deliverables, acceptance criteria, QA scenarios, and verification commands.
- `.planning/STATE.md` — Current project/phase status and Phase 1 decisions relevant to subsequent work.
- `AGENTS.md` — Hard project constraints: inline tests in source files, no `tests/` directory, no new lint/format/CI config unless asked, and required Rust commands.

### Source Files to Scout Before Planning
- `src/algo/hash.rs` — First priority for deterministic known-answer and boundary tests.
- `src/algo/symmetric.rs` — First priority for deterministic known-answer and boundary tests.
- `src/algo/asymmetric.rs` — RSA/ECDSA flow and error-path coverage priority before PQ.
- `src/algo/pq_kem.rs` — PQ KEM coverage after hash/symmetric/asymmetric/cert priorities unless coverage gate requires earlier work.
- `src/algo/pq_signature.rs` — PQ signature coverage after hash/symmetric/asymmetric/cert priorities unless coverage gate requires earlier work.
- `src/cert/mod.rs` — Certificate parsing and `detect_and_parse` fixture coverage.
- `src/cert/extensions.rs` — Certificate extension parsing coverage.
- `src/cert/oid_resolver.rs` — OID resolver coverage and known mapping/boundary tests.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Existing inline `#[cfg(test)] mod tests` blocks are already present in `src/algo/hash.rs`, `src/algo/symmetric.rs`, `src/algo/asymmetric.rs`, `src/algo/pq_kem.rs`, `src/algo/pq_signature.rs`, `src/cert/mod.rs`, and `src/cert/oid_resolver.rs`; Phase 2 should expand these rather than creating a `tests/` directory.
- Existing helper functions such as local `hex_decode`, `hex_encode`, `pkcs7_pad`, parser helpers, and state methods can be tested through public/state-level flows where possible.

### Established Patterns
- Tests are inline in source modules using Rust `#[test]` functions.
- Current algorithm tests mostly verify roundtrips and output length; Phase 2 should add stronger known-answer, invalid input, boundary, and state/error-path assertions.
- Certificate parser coverage is currently light, making checked-in fixtures important for realistic parser confidence.

### Integration Points
- Verification baseline remains `cargo test -p devtools` plus required `cargo tarpaulin -p devtools --out Html` for coverage evidence.
- Fixture files should live under `src/cert/fixtures/` and be referenced by inline cert tests.
- Coverage waivers, if any, should be written as a planning/execution artifact that verifier agents can inspect; ordinary untested branches must still be fixed with tests.

</code_context>

<specifics>
## Specific Ideas

- Use deterministic vectors for hash and symmetric algorithms where possible.
- Cover certificate parsing with realistic PEM, DER, multi-PEM, and PKCS#12 fixtures.
- Use only public, non-secret test material. PKCS#12 password should be visibly documented as test-only, e.g. `test-password`.

</specifics>

<deferred>
## Deferred Ideas

- Performance benchmark approach was not selected for discussion. Keep benchmark tooling/minimal performance smoke decisions deferred unless required by Phase 2 coverage dependencies; broader performance benchmarking belongs to later performance-focused phases.

</deferred>

---

*Phase: 2-测试完善*
*Context gathered: 2026-04-28*

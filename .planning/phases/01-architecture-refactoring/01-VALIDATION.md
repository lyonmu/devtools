---
phase: 01
slug: architecture-refactoring
status: approved
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-28
---

# Phase 01 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust inline `#[cfg(test)]` tests |
| **Config file** | `Cargo.toml` |
| **Quick run command** | `cargo build -p devtools` |
| **Full suite command** | `cargo test -p devtools` |
| **Estimated runtime** | under 60 seconds for quick build on a warm local toolchain |

---

## Sampling Rate

- **After every task commit:** Run `cargo build -p devtools`.
- **After every plan wave:** Run `cargo test -p devtools`.
- **Before `/gsd-verify-work`:** Full suite must be green.
- **Max feedback latency:** one task; no task may rely only on manual verification.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | R4.3, R2.3 | T-01-01-01 | UI helper extraction preserves display-only behavior; no new untrusted input sink | build + inline tests | `cargo build -p devtools` | ✅ | ⬜ pending |
| 01-02-01 | 02 | 2 | R4.3, R2.3 | T-01-02-01 | Existing event handlers and IDs remain bound after helper import | build | `cargo build -p devtools` | ✅ | ⬜ pending |
| 01-02-02 | 02 | 2 | R4.3, R2.3 | T-01-02-02 | Certificate display and copy controls keep existing escaped/owned string rendering | build | `cargo build -p devtools` | ✅ | ⬜ pending |
| 01-03-01 | 03 | 2 | R4.1 | T-01-03-01 | Trait exposes state display only; no cryptographic parameters changed | build | `cargo build -p devtools` | ✅ | ⬜ pending |
| 01-03-02 | 03 | 2 | R4.1 | T-01-03-02 | Trait implementations delegate to existing algorithm methods | full suite | `cargo test -p devtools` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements. No new test framework or fixture directory is required.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Visual preservation of native GPUI layout | R4.3 | GPUI app has no headless UI test harness | Optional after automated tests: run `cargo run`, switch tabs and menu items, confirm Chinese labels and dark theme appear unchanged |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency under one task
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-04-28

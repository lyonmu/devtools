---
phase: 04
slug: ui-ux-bug-fixes
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-28
---

# Phase 04 — Validation Strategy

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust inline `#[cfg(test)]` tests |
| **Config file** | `Cargo.toml` |
| **Quick run command** | `cargo build -p devtools` |
| **Full suite command** | `cargo test -p devtools` |
| **Estimated runtime** | ~60 seconds |

## Sampling Rate

- **After every task commit:** Run `cargo build -p devtools`
- **After every plan wave:** Run `cargo test -p devtools`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 04-01-01 | 01 | 1 | FR-4/FR-5/NFR-1 | T-04-01 | IME input remains local-only | build/static | `cargo build -p devtools && cargo test -p devtools` | ✅ | ⬜ pending |
| 04-01-02 | 01 | 1 | FR-4/FR-5/NFR-1 | T-04-01 | Validation errors do not execute crypto with missing input | build/static | `cargo build -p devtools && cargo test -p devtools` | ✅ | ⬜ pending |
| 04-02-01 | 02 | 2 | FR-4/NFR-1 | T-04-02 | Reset clears sensitive outputs from memory state | unit/build | `cargo test -p devtools -- algo:: && cargo build -p devtools` | ✅ | ⬜ pending |
| 04-02-02 | 02 | 2 | FR-4/NFR-1 | T-04-02 | Status/error feedback remains local | build/static | `cargo build -p devtools && cargo test -p devtools` | ✅ | ⬜ pending |
| 04-03-01 | 03 | 3 | FR-2/FR-4/FR-5/NFR-1 | T-04-03 | Clipboard copy is user-initiated | build/static | `cargo build -p devtools && cargo test -p devtools` | ✅ | ⬜ pending |
| 04-03-02 | 03 | 3 | FR-2/FR-4/FR-5/NFR-1 | T-04-03 | Scroll/copy does not transmit data | build/static | `cargo build -p devtools && cargo test -p devtools` | ✅ | ⬜ pending |

## Wave 0 Requirements

Existing infrastructure covers all phase requirements.

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| macOS Chinese/English IME composition works without `IMKCFRunLoopWakeUpReliable` error | D-17/D-18 | Requires native macOS IME/window session | Run `cargo run`, open algorithm inputs, type Chinese and English with IME, confirm composition/caret/backspace/paste work and terminal has no IMK error |
| Clipboard paste fidelity for key/certificate outputs | D-23/D-24 | Requires OS clipboard | Generate/copy outputs, paste into a text editor, confirm exact copied text and `已复制` banner |
| Right content scroll behavior | D-04/D-21/D-22 | Native UI viewport behavior | Resize window narrow/short and confirm only right content scrolls while left menu remains fixed |

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 60s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-04-28

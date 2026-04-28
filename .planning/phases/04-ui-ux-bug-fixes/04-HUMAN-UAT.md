---
status: partial
phase: 04-ui-ux-bug-fixes
source: [04-VERIFICATION.md]
started: 2026-04-28T04:06:30Z
updated: 2026-04-28T04:06:30Z
---

## Current Test

[awaiting human testing]

## Tests

### 1. macOS IME input path
expected: Run `cargo run`, open algorithm inputs, type Chinese and English IME text, paste text, use backspace; focused field updates correctly without root key appending.
result: [pending]

### 2. Right-side scroll behavior
expected: Resize window short/narrow; top tab bar and left menu stay fixed while only the right work area scrolls.
result: [pending]

### 3. Clipboard copy fidelity
expected: Generate/copy certificate and algorithm outputs; paste into a text editor; copied text exactly matches the visible target and banner shows `已复制`.
result: [pending]

## Summary

total: 3
passed: 0
issues: 0
pending: 3
skipped: 0
blocked: 0

## Gaps

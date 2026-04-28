---
status: complete
phase: 03-uiux
source: 03-01-SUMMARY.md, 03-02-SUMMARY.md, 03-03-SUMMARY.md
started: 2026-04-28T12:00:00Z
updated: 2026-04-28T12:10:00Z
---

## Current Test

[testing complete]

## Tests

### 1. Error Banner with ⚠ Icon
expected: Trigger an error (e.g., run a crypto operation with invalid input). The error banner should display with a ⚠ prefix icon before the error message text.
result: pass

### 2. Expandable Error Details
expected: Click on an error banner. It should expand to show technical details below the summary message. Click again to collapse.
result: pass

### 3. Warning Variant Amber Color
expected: Trigger a warning-level status. The banner should display with amber/yellow border and text color (distinct from red errors and blue info).
result: pass

### 4. Copy Button on Algo Output
expected: Run any algo tool (hash, symmetric, asymmetric). The output block should have a copy button. Click it — a "已复制" confirmation should appear briefly.
result: pass

### 5. Copy Button on Certificate Fields
expected: Import a certificate. Each info row (subject, issuer, serial, etc.) should have a copy button. Click one — "已复制" should appear and the raw field value (without label) should be copied to clipboard.
result: pass

### 6. Loading Spinner During Cert Import
expected: Import a certificate file. During parsing, the import button should be replaced with a spinner showing "解析证书中..." text. After parsing completes, the spinner disappears and normal UI returns.
result: pass

### 7. Loading Spinner During Crypto Operation
expected: Run a symmetric encryption/decryption operation. During execution, the execute button should be replaced with a spinner showing "处理中..." text. After completion, the spinner disappears.
result: pass

## Summary

total: 7
passed: 7
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none]

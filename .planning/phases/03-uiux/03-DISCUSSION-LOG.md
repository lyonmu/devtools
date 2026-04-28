# Phase 3: UI/UX改进 - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-28
**Phase:** 3-UI/UX改进
**Areas discussed:** Error display style, Copy interaction, File import flow, Progress & loading

---

## Error display style

### Q1: How should error messages be displayed to users?

| Option | Description | Selected |
|--------|-------------|----------|
| Enhanced banner | Keep current render_status_banner with Error variant. Add error icon (⚠) prefix and make error text selectable/copyable. Minimal change, reuses existing pattern. | ✓ |
| Inline field errors | Show errors as small red text directly below the input field that caused the error. More contextual but needs per-field error state tracking. | |
| Toast notifications | Errors appear as floating overlays that auto-dismiss after a few seconds. Modern feel but may miss errors if user isn't watching. | |

**User's choice:** Enhanced banner (Recommended)
**Notes:** Minimal change approach, reuses existing pattern.

### Q2: Should error banners show technical details or user-friendly summaries?

| Option | Description | Selected |
|--------|-------------|----------|
| Always show full error | Show full error details in the banner. Technical details help developers debug issues. | |
| Summary + expandable details | Show user-friendly summary by default, with a 'details' toggle to expand technical details. Cleaner for simple errors. | ✓ |
| User-friendly only | Show only user-friendly messages. Technical details go to logs only. | |

**User's choice:** Summary + expandable details (Recommended)
**Notes:** Balance between clean UI and debuggability.

### Q3: How should error banners be dismissed?

| Option | Description | Selected |
|--------|-------------|----------|
| Persist until next action | Errors stay until user performs a new action (current behavior). User has full control. | ✓ |
| Auto-dismiss after timeout | Errors auto-dismiss after 5-10 seconds. Cleaner but may miss errors. | |
| Manual dismiss button | Errors persist and user can manually dismiss with X button. Best of both worlds. | |

**User's choice:** Persist until next action (Recommended)
**Notes:** Current behavior preserved.

### Q4: Should different error types use different visual styles?

| Option | Description | Selected |
|--------|-------------|----------|
| Color-coded by severity | Use red for errors, yellow/amber for warnings, blue for info. Current code has COLOR_ERROR, COLOR_INFO already. | ✓ |
| Single error color | All errors use the same red style. Simpler but less nuanced. | |

**User's choice:** Color-coded by severity (Recommended)
**Notes:** Uses existing color constants, adds COLOR_WARNING.

---

## Copy interaction

### Q1: Where should copy buttons appear?

| Option | Description | Selected |
|--------|-------------|----------|
| Per-output copy button | Add a copy button to each render_mono_output_block. User copies specific results. Most flexible. | ✓ |
| Global copy-all button | Add a single 'Copy All' button that copies all visible output. Simpler but less granular. | |
| Clipboard on select | Copy to clipboard when user selects text in output blocks. No button needed but less discoverable. | |

**User's choice:** Per-output copy button (Recommended)
**Notes:** Most flexible approach.

### Q2: What feedback should users see after copying?

| Option | Description | Selected |
|--------|-------------|----------|
| Text change feedback | Button text changes to '已复制' for 2 seconds, then reverts. Simple and clear. | ✓ |
| Visual highlight | Brief green flash/highlight on the output block. Visual but subtle. | |
| Toast notification | Small toast notification '已复制到剪贴板'. More prominent but may be distracting. | |

**User's choice:** Text change feedback (Recommended)
**Notes:** Simple and clear feedback.

### Q3: What format should the copied content be?

| Option | Description | Selected |
|--------|-------------|----------|
| Raw output only | Copy the raw output text only. Clean and useful for pasting into other tools. | ✓ |
| Output with labels | Copy output with labels (e.g., 'SHA-256: abc123'). More context but may need cleanup. | |
| User-selectable format | Let user choose via dropdown: raw, with labels, or as code block. Most flexible but more complex. | |

**User's choice:** Raw output only (Recommended)
**Notes:** Clean and useful for pasting.

### Q4: Should certificate info fields also have copy buttons?

| Option | Description | Selected |
|--------|-------------|----------|
| Copy per cert field | Add copy buttons to certificate info rows (subject, issuer, serial, etc.). Useful for debugging. | ✓ |
| Copy entire cert | Single 'Copy All' button for entire certificate. Simpler but less granular. | |
| Algorithm output only | Only add copy to algorithm output blocks, not certificate info. | |

**User's choice:** Copy per cert field (Recommended)
**Notes:** Useful for debugging certificate issues.

---

## File import flow

### Q1: Should file import support drag-drop in addition to the current file dialog button?

| Option | Description | Selected |
|--------|-------------|----------|
| Button + drag-drop | Keep current rfd file dialog button. Add drag-drop support as secondary import method. Most flexible. | ✓ |
| Button only | Keep only the current file dialog button. Simpler, no new dependencies. | |
| Drag-drop primary + button fallback | Replace button with drag-drop as primary, keep button as fallback. Modern feel. | |

**User's choice:** Button + drag-drop (Recommended)
**Notes:** Most flexible approach.

### Q2: What visual feedback should appear when dragging a file over the window?

| Option | Description | Selected |
|--------|-------------|----------|
| Border overlay + text | Show a dashed border overlay with '拖放证书文件到此处' text when dragging over the window. Clear and visible. | ✓ |
| Background highlight | Highlight the entire window background with a subtle color change. Less intrusive. | |
| Floating drop zone | Show a floating drop zone icon in the center of the window. Modern but may overlap content. | |

**User's choice:** Border overlay + text (Recommended)
**Notes:** Clear and visible feedback.

### Q3: What happens when a user drags an unsupported file type?

| Option | Description | Selected |
|--------|-------------|----------|
| Error with format list | Show error banner with supported formats list. Clear and helpful. | ✓ |
| Silent ignore | Ignore unsupported files silently. Simpler but may confuse users. | |
| Visual accept/reject | Only accept drag-drop for supported extensions. Visual feedback shows valid/invalid. | |

**User's choice:** Error with format list (Recommended)
**Notes:** Clear and helpful error message.

### Q4: Should drag-drop work only on the certificate tab or anywhere in the app?

| Option | Description | Selected |
|--------|-------------|----------|
| Cert tab only | Only accept drag-drop on the Cert tab. Algorithm tab doesn't need file import. | ✓ |
| Any tab | Accept drag-drop on any tab, route to cert parser. More flexible but may confuse. | |

**User's choice:** Cert tab only (Recommended)
**Notes:** Algorithm tab doesn't need file import.

---

## Progress & loading

### Q1: What loading indicator should appear during crypto operations?

| Option | Description | Selected |
|--------|-------------|----------|
| Spinner with text | Show a spinning indicator (e.g., '处理中...') during operations. Simple and clear. | ✓ |
| Progress bar + spinner | Show a progress bar for determinate operations, spinner for indeterminate. More informative but needs progress tracking. | |
| Skeleton placeholder | Show skeleton placeholders where content will appear. Modern but more complex. | |

**User's choice:** Spinner with text (Recommended)
**Notes:** Simple and clear approach.

### Q2: Should inputs be disabled while an operation is running?

| Option | Description | Selected |
|--------|-------------|----------|
| Disable inputs | Disable input fields and buttons during operations. Prevents accidental double-submission. | ✓ |
| Warn on interaction | Keep inputs enabled but show warning if user tries to interact. More flexible. | |
| Queue operations | Allow interaction, queue operations. Most flexible but complex. | |

**User's choice:** Disable inputs (Recommended)
**Notes:** Prevents accidental double-submission.

### Q3: Should file import show a loading indicator?

| Option | Description | Selected |
|--------|-------------|----------|
| Spinner for import | Show spinner during file dialog and parsing. Consistent with crypto operations. | ✓ |
| No indicator for import | No loading indicator for import — it's fast enough. Simpler. | |

**User's choice:** Spinner for import (Recommended)
**Notes:** Consistent with crypto operations.

### Q4: What text should appear during certificate parsing?

| Option | Description | Selected |
|--------|-------------|----------|
| Parsing text + spinner | Show '解析证书中...' text with spinner. Clear and informative. | ✓ |
| Generic loading text | Show generic '处理中...' for all operations. Simpler but less specific. | |

**User's choice:** Parsing text + spinner (Recommended)
**Notes:** More informative than generic loading.

---

## Claude's Discretion

- Exact icon choices (⚠ vs other Unicode icons)
- Spinner animation implementation (Unicode characters vs GPUI-native)
- Expandable details toggle implementation (click vs hover)
- Drag-drop GPUI API approach (native events vs platform integration)

## Deferred Ideas

- **Keyboard shortcuts** — ROADMAP.md lists "支持快捷键" as a Phase 3 deliverable, but was not selected for discussion.
- **File drag-drop for algorithm tab** — Could be useful for importing keys, but out of scope.
- **Theme support** — Separate concern from UI/UX improvements.

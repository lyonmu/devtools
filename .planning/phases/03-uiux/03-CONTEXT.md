# Phase 3: UI/UX改进 - Context

**Gathered:** 2026-04-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 3 delivers improved user-facing interaction quality for the DevTools cryptography desktop app. The scope covers five areas: error display, copy functionality, file import flow, progress/loading indicators, and keyboard shortcuts. This phase enhances existing UI patterns (status banners, output blocks, file dialog) without adding new crypto features or expanding algorithm coverage.

</domain>

<decisions>
## Implementation Decisions

### Error Display Style
- **D-01:** Enhanced banner approach — keep existing `render_status_banner` with Error variant, add error icon (⚠) prefix, make error text selectable/copyable.
- **D-02:** Summary + expandable details — show user-friendly summary by default, with a toggle to expand technical details (Rust error chains). Cleaner for simple errors while preserving debuggability.
- **D-03:** Persist until next action — errors stay visible until user performs a new action (current behavior). User has full control over dismissal.
- **D-04:** Color-coded by severity — red for errors, yellow/amber for warnings, blue for info. Uses existing `COLOR_ERROR`, `COLOR_INFO` constants. Add `COLOR_WARNING` constant.

### Copy Interaction
- **D-05:** Per-output copy button — add a copy button to each `render_mono_output_block`. User copies specific results rather than all-or-nothing.
- **D-06:** Text change feedback — button text changes to "已复制" for 2 seconds, then reverts to original label. Simple, clear, no extra UI elements.
- **D-07:** Raw output only — copy clean text without labels or formatting. Most useful for pasting into other tools.
- **D-08:** Copy per cert field — add copy buttons to certificate info rows (subject, issuer, serial, etc.) via `render_info_row`. Useful for debugging certificate issues.

### File Import Flow
- **D-09:** Button + drag-drop — keep current `rfd::AsyncFileDialog` button as primary, add drag-drop as secondary import method. Most flexible.
- **D-10:** Border overlay + text — show dashed border overlay with "拖放证书文件到此处" text when dragging a file over the window. Clear and visible.
- **D-11:** Error with format list — show error banner with supported formats list (pem, der, p12, pfx, cer, crt) when user drags unsupported file type.
- **D-12:** Cert tab only — drag-drop only works on the certificate tab. Algorithm tab doesn't need file import.

### Progress & Loading
- **D-13:** Spinner with text — show a spinning indicator with "处理中..." text during crypto operations. Simple and clear.
- **D-14:** Disable inputs — disable input fields and buttons during operations to prevent accidental double-submission.
- **D-15:** Spinner for import — show loading indicator during file dialog and parsing. Consistent with crypto operations.
- **D-16:** Parsing-specific text — show "解析证书中..." text with spinner during certificate parsing. More informative than generic loading.

### Keyboard Shortcuts
- **D-17:** Keyboard shortcuts are deferred to a later phase. The ROADMAP.md lists "支持快捷键" as a deliverable, but the user did not select this area for discussion. Capture as deferred idea.

### Claude's Discretion
- Exact icon choices (⚠ vs other Unicode icons) — Claude may choose based on GPUI rendering support.
- Spinner animation implementation — Claude may choose between Unicode spinner characters or GPUI-native animation.
- Expandable details toggle implementation — Claude may choose between click-to-expand or hover-to-expand.
- Drag-drop GPUI API approach — Claude may choose between GPUI native drag events or platform-specific integration.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and Phase Scope
- `.planning/PROJECT.md` — Project architecture, stack, constraints, and existing UI patterns.
- `.planning/REQUIREMENTS.md` — Requirements R2 (developer-friendly), R3 (native experience) define UI/UX expectations.
- `.planning/ROADMAP.md` — Phase 3 scope, deliverables, acceptance criteria, QA scenarios, and verification commands.
- `.planning/STATE.md` — Current project/phase status and Phase 1/2 decisions relevant to UI work.
- `AGENTS.md` — Hard project constraints: GPUI 0.2 API conventions, Chinese UI text, inline tests.

### Source Files to Scout Before Planning
- `src/components/ui_helpers.rs` — Shared UI helpers (status banners, output blocks, action buttons, info rows). Phase 3 extends these.
- `src/app.rs` — Root application view with tab management, file dialog, and rendering logic. Phase 3 modifies error display and adds drag-drop.
- `src/tabs/mod.rs` — Tab structure definitions. Phase 3 may add drag-drop state to CertTab.
- `src/components/input.rs` — Text input component with IME support. Phase 3 may add copy-to-clipboard integration.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `render_status_banner(kind, message)` — Already supports Error/Success/Info variants. Phase 3 enhances with icon prefix and expandable details.
- `render_mono_output_block(text)` — Monospaced output block with scroll. Phase 3 adds copy button.
- `render_action_button(id, label, bg_color)` — Generic action button. Can be reused for copy buttons.
- `render_info_row(label, value)` — Label-value info row. Phase 3 adds copy button to value field.
- `UiStatusKind` enum — Empty/Success/Error/Info variants. Phase 3 may add Warning variant.
- Color constants (`COLOR_ERROR`, `COLOR_INFO`, `COLOR_SUCCESS`) — Already defined. Phase 3 adds `COLOR_WARNING`.

### Established Patterns
- GPUI 0.2 API: `.id()` returns `Stateful<Div>`, `.child()` needs owned types. All new UI code must follow this.
- Chinese UI text throughout — all new copy must be Chinese.
- File dialog uses `cx.spawn()` with `gpui::WeakEntity` for async I/O.
- `is_importing` flag tracks loading state for certificate import.

### Integration Points
- `DevToolsApp::open_file_dialog()` — Current file import implementation. Phase 3 adds drag-drop alongside this.
- `CertTab` struct — May need new fields for drag-hover state.
- `AlgoTab` struct — May need loading state fields for crypto operations.
- GPUI window events — Drag-drop integration via GPUI's event system.

</code_context>

<specifics>
## Specific Ideas

- Error banners should feel like a natural extension of the existing status banner, not a completely new pattern.
- Copy buttons should be small and unobtrusive — don't overwhelm the output display.
- Drag-drop should feel native — the border overlay should appear quickly and disappear cleanly.
- Loading spinners should be lightweight — don't block the entire UI, just the relevant section.

</specifics>

<deferred>
## Deferred Ideas

- **Keyboard shortcuts** — ROADMAP.md lists "支持快捷键" as a Phase 3 deliverable, but this was not selected for discussion. Deferring to a future phase or as a follow-up within Phase 3 if time permits.
- **File drag-drop for algorithm tab** — Could be useful for importing keys, but out of scope for Phase 3.
- **Theme support** — ROADMAP.md mentions "支持深色/浅色主题" but this is a separate concern.

</deferred>

---

*Phase: 3-UI/UX改进*
*Context gathered: 2026-04-28*

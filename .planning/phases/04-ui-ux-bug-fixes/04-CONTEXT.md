# Phase 4: UI/UX Bug Fixes - Context

**Gathered:** 2026-04-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix the devtools GPUI desktop app's critical UI/UX defects and bring the existing certificate and algorithm tool surfaces to a production-ready layout standard. This phase covers the reported macOS IMK/input issue, non-functional algorithm inputs, missing scroll behavior for long right-side content, copy support for certificate/algorithm output, and consistent layout/control styling within the app's existing capabilities. New capabilities such as theme switching or a new navigation model are out of scope.

</domain>

<decisions>
## Implementation Decisions

### Production-grade layout
- **D-01:** Use a **workbench/tool layout**: fixed top tabs, fixed per-tab left menu, and a right-side content work area.
- **D-02:** Organize tool pages as **form/input area above, results below**. Do not switch to a document-first or terminal-first layout.
- **D-03:** Prioritize readability over density: clear typography, increased line height/spacing, and consistent section hierarchy.
- **D-04:** Keep the left menu stable and make the **right-side content area scroll** when content is too tall or the window is narrow.
- **D-05:** Use low-emphasis result cards/panels: subtle boundaries/backgrounds, minimal visual noise, no heavy dashboard-style shadows.
- **D-06:** Keep the current dark theme, but normalize contrast, status colors, spacing, and component hierarchy.
- **D-07:** Use a top status/banner area for empty, success, and error states where appropriate.
- **D-08:** UI refactoring is allowed only inside current Phase 4 scope: extract/reuse layout/input/button/card/scroll patterns as needed, but do not add new features such as theme switching or collapsible navigation.

### Operation controls consistency
- **D-09:** Use compact action button groups rather than full-width action buttons.
- **D-10:** Standard action order is **执行** first, **重置** second.
- **D-11:** Reset clears the current page/tool state: inputs, outputs/results, and status/error messages.
- **D-12:** Execution feedback should appear through a top banner/status plus detailed content in the result area.
- **D-13:** Button/input visual consistency should apply across both certificate and algorithm tabs, not only the currently broken algorithm screens.
- **D-14:** Pressing Enter in single-line tool inputs should execute the current action.
- **D-15:** Invalid/missing inputs should still allow the button click and then show a clear validation error; do not rely only on disabled buttons.
- **D-16:** Copy actions belong in the result card/title area so the copy target is clear.

### Input and macOS IMK behavior
- **D-17:** Algorithm input fields must become real, complete text inputs: regular typing, Chinese/English IME composition, backspace, paste, and basic caret behavior must work.
- **D-18:** The macOS `IMKCFRunLoopWakeUpReliable` issue should be addressed by stabilizing the app's focus/text-input path, not by merely hiding or ignoring the log.
- **D-19:** Input components need visible focus, error, and disabled states.
- **D-20:** Long text material such as keys, plaintext, ciphertext, or signatures should use multiline text inputs; short values such as OIDs can remain single-line.

### Scrolling and copy support
- **D-21:** The primary scrolling boundary is the right-side content area; the left menu should remain fixed.
- **D-22:** Very long outputs such as PEM, keys, signatures, ciphertext, and large certificate fields should display in monospaced blocks with horizontal scrolling for fidelity.
- **D-23:** Copy support must cover both whole result blocks and important individual fields. Examples: certificate subject/issuer/serial and algorithm outputs such as keys/signatures/ciphertext/hash values.
- **D-24:** Copy success should show a short top status/banner message such as “已复制”, without blocking the user.

### the agent's Discretion
- Exact GPUI implementation technique for stable text input/focus handling, as long as D-17 and D-18 are satisfied.
- Exact component extraction boundaries for the Phase 4 refactor, as long as the outcome stays within existing capabilities.
- Exact spacing/color token values, as long as they implement the decisions above and preserve the Chinese UI style.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and phase requirements
- `.planning/ROADMAP.md` — Phase 4 goal and known issues: IMK error, non-functional algorithm inputs, wide execute buttons, missing reset buttons, inconsistent font sizes, missing vertical scrolling, and missing copy for generated keys.
- `.planning/REQUIREMENTS.md` — App requirements, shell architecture, certificate/algorithm tab expectations, local processing, macOS/Linux cross-platform requirement.
- `.planning/PROJECT.md` — Project scope, GPUI/Rust stack, pure-local design, extensible tab architecture.
- `.planning/STATE.md` — Prior implementation decisions: GPUI 0.2.2, enum-based tab pattern, hardcoded algorithm registry, OID lookup/input implementation history.
- `AGENTS.md` — Repo-specific commands, architecture notes, GPUI 0.2 gotchas, Chinese UI convention.

No external specs or ADRs were referenced during discussion.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/components/input.rs` — Existing `TextInput` component with `FocusHandle`, `focus()/blur()`, mouse focus, placeholder/value rendering, and basic key handling. It is currently unused by tabs but is a likely starting point for real input behavior.
- `src/components/left_menu.rs` and `src/components/tab_bar.rs` — Existing reusable navigation styling patterns with click handling and active states.
- `src/app.rs` — Contains shared typography constants `FONT_TITLE`, `FONT_BODY`, and `FONT_SMALL`, plus most inline page rendering helpers.
- `src/app.rs` currently contains a copy-to-clipboard helper pattern using `write_to_clipboard(ClipboardItem::new_string(...))`.

### Established Patterns
- The app currently uses a focused-field index and global `on_key_down` path for algorithm input. This is the likely source of incomplete text/IME behavior and should not be treated as a production-grade text input model.
- The codebase often keeps UI helpers inline in `src/app.rs` / `src/tabs/mod.rs`; Phase 4 may extract shared helpers where it improves consistency without creating new capabilities.
- Result panels use dark backgrounds, rounded corners, and Chinese labels. New UI should preserve this style while making hierarchy and spacing consistent.
- Certificate extension rendering already handles multi-line values via line iteration in `src/tabs/mod.rs`; long certificate values need better scroll/copy containment.

### Integration Points
- Algorithm input/rendering: `src/app.rs` input rendering and `on_key_down` handling; `src/tabs/mod.rs` algorithm tab state also participates in focused-field behavior.
- Algorithm outputs needing scroll/copy treatment: symmetric, asymmetric, hash, PQ KEM, and PQ signature result panels in `src/app.rs`.
- Certificate details/extension outputs needing scroll/copy treatment: certificate detail and extension rendering in `src/tabs/mod.rs` and parsed values from `src/cert/mod.rs` / `src/cert/extensions.rs`.
- Global shell/layout: `src/app.rs` top tab + left menu + right content structure.

</code_context>

<specifics>
## Specific Ideas

- The user explicitly wants the entire project layout optimized to production standard, not only the single broken input bug.
- “全面重构 UI” was clarified to mean refactoring within existing capabilities: reusable layout/input/button/card/scroll patterns are acceptable; new capabilities are deferred.
- Chinese labels and feedback should be used throughout, consistent with the current app.

</specifics>

<deferred>
## Deferred Ideas

- Theme switching / adding a light theme — future phase if desired; not part of Phase 4.
- New navigation behavior such as collapsible left menu — future phase if desired; not part of Phase 4.

</deferred>

---

*Phase: 4-UI/UX Bug Fixes*
*Context gathered: 2026-04-28*

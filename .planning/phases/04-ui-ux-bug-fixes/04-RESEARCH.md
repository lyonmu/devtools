# Phase 4: UI/UX Bug Fixes - Research

**Phase:** 04-ui-ux-bug-fixes  
**Date:** 2026-04-28  
**Status:** Complete

## Research Question

What must be known to plan a production-grade GPUI UI/UX fix phase that addresses macOS IMK input errors, non-functional algorithm inputs, layout readability, scrolling, and copy support without adding deferred features?

## Findings

### GPUI text input and IME

- GPUI text input that needs IME composition should not rely on a global `on_key_down` string-appending path.
- The GPUI platform input path is `Window::handle_input(&FocusHandle, ElementInputHandler::new(bounds, entity), cx)`, which connects a focused entity implementing `EntityInputHandler` to the OS text input system.
- `EntityInputHandler` is the relevant trait for selection, marked text, replacement, UTF-16 ranges, and IME composition.
- A production text input model needs at least: text value, caret/selection range, marked range, focus handle, and last layout/bounds used by bounds/range methods.
- The existing `src/components/input.rs` has `FocusHandle` but does not install an input handler with `window.handle_input`, so it is insufficient for D-17/D-18 by itself.

### Existing project patterns

- GPUI 0.2.2 APIs in this codebase require helper return types to match stateful divs when `.id()` is used (`Stateful<Div>` rather than plain `Div`).
- Chinese UI text is established throughout; new status labels should use Chinese strings such as `已复制`, `请输入...`, `执行`, and `重置`.
- The app currently centralizes most rendering in `src/app.rs` and tab state in `src/tabs/mod.rs`; Phase 4 can extract reusable UI helpers inside current scope, but should not introduce a new navigation model or theme system.
- `src/app.rs` already imports clipboard APIs and uses GPUI layout primitives; copy/status work should stay local and not introduce dependencies.

### Layout and copy behavior

- Right-side content must be the primary vertical scroll boundary. The left menu remains fixed.
- Long PEM/hex/signature/ciphertext blocks need monospaced display and horizontal overflow to preserve fidelity.
- Copy affordances should be attached to the result card/title row so users know exactly what will be copied.
- Copy success should be a transient top banner/status in the right content area. No modal or blocking confirmation is needed.

## Standard Stack

- Rust edition 2024
- GPUI 0.2.2
- Existing inline `#[cfg(test)]` Rust tests
- Verification commands: `cargo build -p devtools`, `cargo test -p devtools`

## Architecture Patterns

1. **Input foundation first:** implement a reusable GPUI text input component/state that uses FocusHandle + EntityInputHandler, then wire algorithm fields to it.
2. **Workbench layout:** keep top tabs and left menu fixed; only right content scrolls.
3. **Component-level consistency:** use shared helper functions for action rows, status banners, result cards, copy buttons, and monospaced blocks.
4. **State-preserving reset:** reset clears current tool input/output/status/error while preserving selected algorithm/mode unless the tool already clears on selection changes.

## Don't Hand-Roll

- Do not use root-level global `on_key_down` to mutate text fields.
- Do not emulate IME by appending `key_char`; install GPUI input handlers instead.
- Do not add theme switching, collapsible navigation, or other deferred capabilities.
- Do not add lint/format/CI tooling.

## Common Pitfalls

- `.id()` changes return type to `Stateful<Div>`; helper signatures must match.
- GPUI `.child()` requires owned strings; clone strings rather than passing borrowed `&String` as children.
- Clipboard copy of private keys is user-initiated but sensitive; make the target explicit and show a status banner.
- Cert tab content currently may return unwrapped content for non-import pages; scroll behavior must cover all right-side pages, not only import.

## Architectural Responsibility Map

| Concern | Owner files | Notes |
|---------|-------------|-------|
| Text input/IME | `src/components/input.rs`, `src/tabs/mod.rs`, `src/app.rs` | Reusable input foundation plus binding between text fields and tool state |
| Tool controls/status | `src/app.rs`, `src/algo/*.rs` | Compact action groups, reset behavior, validation/status banners |
| Scroll/copy/result cards | `src/app.rs`, `src/tabs/mod.rs` | Right content scroll, monospaced blocks, cert/algorithm copy actions |

## Validation Architecture

- **Automated baseline:** every plan runs `cargo build -p devtools && cargo test -p devtools`.
- **Static gates:** plans include grep/rg checks for removal of global input handling, presence of `EntityInputHandler`, `handle_input`, `overflow_y_scroll`, copy helper, and status banner strings.
- **Manual UAT:** macOS IME composition and clipboard paste fidelity require a human-visible desktop session after automated verification because GPUI is a native GUI framework.

## RESEARCH COMPLETE

---
phase: 01
slug: architecture-refactoring
status: approved
shadcn_initialized: false
preset: none
created: 2026-04-28
---

# Phase 01 — UI Design Contract

> Visual and interaction contract for GPUI component extraction and UI refactoring. This phase must preserve the current Chinese desktop UI while making shared primitives reusable.

---

## Design System

| Property | Value |
|----------|-------|
| Tool | none |
| Preset | not applicable |
| Component library | GPUI 0.2 existing project primitives |
| Icon library | none |
| Font | GPUI default system font; monospace only for output blocks |

---

## Spacing Scale

Declared values are GPUI utility calls already used by the app:

| Token | Value | Usage |
|-------|-------|-------|
| xs | 4px | Compact row gaps (`gap_1`, small dividers) |
| sm | 8px | Tab/menu compact padding (`px_2`, `py_1`, `gap_2`) |
| md | 12px | Status/result card padding (`px_3`, `py_2`, `p_3`) |
| lg | 16px | Main content padding (`p_4`, `gap_4`) |
| xl | 24px | Reserved for future larger group spacing |

Exceptions: Existing GPUI utility names are preserved even where values are framework-defined rather than literal tokens.

---

## Typography

| Role | Size | Weight | Line Height |
|------|------|--------|-------------|
| Body | `FONT_BODY = px(16.0)` | default | GPUI default |
| Label | `FONT_SMALL = px(14.0)` | default | GPUI default |
| Heading | `FONT_TITLE = px(18.0)` | default | GPUI default |
| Monospace output | `FONT_SMALL = px(14.0)` | default | GPUI default |

Typography constants must live in `src/components/ui_helpers.rs` after extraction.

---

## Color

| Role | Value | Usage |
|------|-------|-------|
| Dominant (60%) | `0x1a1a2a`, `0x2a2a3a` | app background and output blocks |
| Secondary (30%) | `0x1e1e2e`, `0x252535` | panels, tab bar, left menu |
| Active/Accent (10%) | `0x3b3b5c`, `0x3b82f6`, `0x22c55e`, `0x8b5cf6` | active nav, info action, success action, tertiary action |
| Destructive/Error | `0xf87171` | error banners and validation failure text |
| Body text | `0xffffff`, `0xddddcc`, `0x888899`, `0x666677` | primary headings, body values, labels, muted labels |

Accent reserved for: active tab/menu state, explicit execute/copy/info/crypto-operation buttons, and status-banner borders. Do not recolor unrelated static text as accent.

---

## Copywriting Contract

| Element | Copy |
|---------|------|
| Primary certificate CTA | `选择证书文件` |
| Generic execute CTA | `执行` |
| Generic reset CTA | `重置` |
| Copy CTA | `复制` |
| Certificate empty guidance | `请选择证书文件` / `点击「选择证书文件」按钮导入文件` |
| Algorithm empty guidance | Keep existing per-tool Chinese empty states such as `请选择输入并点击执行` |
| Success state | Keep existing `执行完成`, `导入成功: ...`, `已复制` |
| Error state | Keep existing `错误: {err}` / `导入失败: {err}` pattern |

---

## Interaction Contract

- Preserve all existing GPUI `ElementId` strings for buttons, tabs, menu items, and output blocks unless a plan explicitly says to add a stable helper-generated equivalent.
- Shared button helpers must return a base `Div`; callers still attach event handlers with `.on_mouse_down(...)` in `src/app.rs` because those handlers require `cx.listener` and app state access.
- Shared output helpers must preserve horizontal scrolling and monospace rendering for hex, PEM, DER, signatures, and keys.
- Phase 1 must not add new user-visible workflows; it is a behavior-preserving refactor.

---

## Registry Safety

| Registry | Blocks Used | Safety Gate |
|----------|-------------|-------------|
| shadcn official | none | not required |
| third-party registries | none | not allowed in this phase |

---

## Checker Sign-Off

- [x] Dimension 1 Copywriting: PASS
- [x] Dimension 2 Visuals: PASS
- [x] Dimension 3 Color: PASS
- [x] Dimension 4 Typography: PASS
- [x] Dimension 5 Spacing: PASS
- [x] Dimension 6 Registry Safety: PASS

**Approval:** approved 2026-04-28

## UI-SPEC VERIFIED

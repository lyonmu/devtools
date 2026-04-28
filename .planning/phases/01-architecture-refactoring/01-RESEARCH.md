---
phase: 01
slug: architecture-refactoring
status: complete
created: 2026-04-28
---

# Phase 01 — Research: 架构重构与组件化

## Research Goal

Plan Phase 1 so the existing single-crate GPUI app can extract reusable UI helpers and add a unified algorithm-tool interface without changing user-visible behavior.

## Existing Architecture

- Entry flow: `src/main.rs` → `src/app.rs` → `src/tabs/mod.rs`.
- `src/app.rs` currently owns top-level GPUI layout, event handlers, algorithm-tool rendering, status banners, result cards, copyable output blocks, button markup, tab bar, and left menu markup.
- `src/tabs/mod.rs` owns `CertTab`, `AlgoTab`, text-input entities, certificate section renderers, and several local UI helper functions.
- `src/components/` already contains reusable GPUI component modules (`input`, `left_menu`, `tab_bar`) and is the correct location for shared UI rendering utilities.
- `src/algo/` contains concrete state types for symmetric, asymmetric, hash, PQ KEM, and PQ signature tools. Their method names differ (`execute`, `compute`, `keygen`, `clear`) but each has a common lifecycle: run primary operation, reset/clear state, expose output, expose errors.

## Standard Stack

- Rust edition 2024, single crate.
- GPUI 0.2 native GUI; no headless UI test runner.
- Inline Rust tests via `#[cfg(test)] mod tests` in source files.
- Verification commands: `cargo build -p devtools` and `cargo test -p devtools`.

## Architecture Patterns To Preserve

1. Keep the project as one crate; do not introduce workspaces, new crates, or new lint/format/CI tooling.
2. Keep Chinese UI copy unchanged unless explicitly extracting it into helpers.
3. Preserve GPUI ownership constraints:
   - `.id()` returns `Stateful<Div>`; helper return types must match actual GPUI chains.
   - `.child()` needs owned values; pass `String`/`SharedString` clones rather than borrowed strings.
4. Keep algorithm-specific operations on concrete tool states. A shared trait should cover only common behavior and should not erase type-specific operations such as sign/verify/encapsulate.
5. Prefer a low-risk refactor sequence: create shared interfaces/helpers first, then switch consumers, then verify behavior with build/tests.

## Implementation Guidance

### UI helper extraction

Create `src/components/ui_helpers.rs` as the single source for repeated typography constants, dark-theme palette constants, status banner rendering, result cards, mono output blocks, action buttons, and label/value rows. Export it from `src/components/mod.rs`.

Plan consumers should replace local helpers in `src/app.rs` and `src/tabs/mod.rs` with imports from `crate::components::ui_helpers`. The refactor should preserve current markup IDs for interactive elements (`sym-execute-btn`, `hash-reset-btn`, `tab-{index}`, `menu-{index}`) so existing and future tests remain stable.

### Trait extraction

Create `src/algo/tool_trait.rs` with `CryptoTool` using `AlgorithmCategory` from `src/algo/registry.rs`. Implement the trait for the five existing algorithm tool states:

- `SymmetricToolState`
- `AsymmetricToolState`
- `HashToolState`
- `PqKemToolState`
- `PqSignatureToolState`

Use fully-qualified inherent method calls when method names collide with trait methods, for example `SymmetricToolState::execute(self)`.

## Don't Hand-Roll / Don't Add

- Do not add a new UI framework, component library, CSS system, or external crate.
- Do not add a plugin system; that is outside Phase 1.
- Do not change cryptographic algorithms or default security parameters.
- Do not add headless GUI automation for this phase; verification is compile/test based.
- Do not add lint/format/CI configuration; AGENTS.md explicitly forbids it unless requested.

## Common Pitfalls

- GPUI color return types are `Rgba` from `rgb(...)`, not necessarily `Hsla`; plan actions should match actual imports.
- Piped verification commands like `cargo build | tail` can mask failures unless `pipefail` is set. Use direct `cargo build -p devtools` / `cargo test -p devtools` in plan verification.
- Grep gates must avoid self-invalidating matches in plan prose; use source files only and avoid counting comments when the token may appear in comments.
- `HashToolState` primary operation is `compute()`, not `execute()`.
- PQ KEM and PQ signature reset operations are `clear()`, not `reset()`.

## Architectural Responsibility Map

| Tier | Responsibility | Files |
|------|----------------|-------|
| UI helper tier | Reusable GPUI typography/color/layout primitives | `src/components/ui_helpers.rs`, `src/components/mod.rs` |
| App rendering tier | Event-bound GPUI rendering and tab/menu orchestration | `src/app.rs` |
| Tab state tier | Certificate and algorithm tab state, input entities | `src/tabs/mod.rs` |
| Algorithm contract tier | Shared tool-state interface | `src/algo/tool_trait.rs`, `src/algo/mod.rs` |
| Algorithm implementation tier | Concrete crypto tool state behavior | `src/algo/symmetric.rs`, `src/algo/asymmetric.rs`, `src/algo/hash.rs`, `src/algo/pq_kem.rs`, `src/algo/pq_signature.rs` |

## Validation Architecture

- **Quick validation:** `cargo build -p devtools` after each task.
- **Full validation:** `cargo test -p devtools` after each plan and before phase verification.
- **Static coverage gates:** grep exact source files for exported helpers, removed duplicate constants, `pub mod tool_trait`, and five `impl CryptoTool for ...` blocks.
- **Manual verification:** optional `cargo run` on macOS/Linux display to confirm tab/menu visual behavior remains unchanged; not required for automated completion because GPUI has no headless test setup in this project.

## Source Audit Inputs

- GOAL: extract reusable components and unify code structure.
- REQ: Phase 1 primarily advances `R4.1` (unified algorithm interface), `R4.3` (componentized UI), and supports `R2.3` (consistent status/error display helpers).
- RESEARCH: create helpers first, refactor consumers second, define algorithm trait third; direct build/test verification.
- CONTEXT: no Phase 1 CONTEXT.md exists, so there are no locked D-XX decisions to implement.

## RESEARCH COMPLETE

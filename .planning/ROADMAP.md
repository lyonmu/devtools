# Roadmap — devtools

## Status: Phase 3 Complete — Phase 4 Ready

| # | Phase | Status | Depends on |
|---|-------|--------|------------|
| 1 | Application Shell Framework | ✅ Complete | — |
| 2 | Certificate Import & Parsing | ✅ Complete | 1 |
| 3 | Algorithm Analysis & OID Resolution | ✅ Complete | 1 |
| 4 | UI/UX Bug Fixes | Planned | 3 |
| 5 | Certificate Chain Viewer | Planned | 2 |
| 6 | Polish & Cross-platform Verification | Planned | 2, 3, 4, 5 |

---

## Phase 1: Application Shell Framework

**Goal**: Running application with top tab bar, per-tab left menu, and right content area.

### Requirements Mapping
- Shell → Top bar tabs, per-tab layout

### Success Criteria (UAT)
- [x] Application launches a window
- [x] Top bar shows "Certificate Parsing" and "Algorithm Parsing" tabs
- [x] Clicking a tab switches the entire left+right content area
- [x] Each tab has its own independent left menu with at least 2 items
- [x] Clicking left menu items changes the right content area
- [x] Right content area displays placeholder text confirming which tab + menu is active

### Technical Approach
- Create `App` struct managing global tab state
- Each tab renders its own `LeftMenu` + `RightContent` pair
- Use GPUI's `div` + flex layout for shell structure
- Define a `Tab` trait for extensibility (new tabs implement `Tab`)

### Files
- `src/main.rs` — Entry point, Application bootstrap
- `src/app.rs` — App state, top tab bar component
- `src/tabs/mod.rs` — Tab trait and tab registry
- `src/tabs/cert.rs` — Certificate tab (placeholder)
- `src/tabs/algo.rs` — Algorithm tab (placeholder)
- `src/components/tab_bar.rs` — Top tab bar component
- `src/components/left_menu.rs` — Left menu component

---

## Phase 2: Certificate Import & Parsing

**Goal**: Import certificate files and display parsed details.

### Requirements Mapping
- FR-1: Multi-format Certificate Import
- FR-2: Certificate Details View

### Success Criteria (UAT)
- [x] Certificate tab left menu shows: File Import, Certificate Info
- [x] File Import opens a file picker supporting .pem/.der/.p12/.pfx/.cer
- [x] After import, Certificate Info shows parsed fields
- [x] Displays subject, issuer, validity period, serial number
- [x] Shows error message for invalid/corrupt files

### Dependencies
- `x509-parser` crate for certificate parsing
- `rpassword` or native file dialog for file picking

---

## Phase 3: Algorithm Analysis & OID Resolution

**Goal**: Display algorithm information and resolve OIDs.

### Requirements Mapping
- FR-4: Algorithm Parameter Display
- FR-5: OID Resolution

### Success Criteria (UAT)
- [ ] Algorithm tab left menu shows: Algorithm List, Parameter Configuration, OID Lookup
- [ ] Algorithm List shows common algorithms (SHA, RSA, ECC, AES)
- [ ] OID Lookup accepts OID input and resolves to algorithm name
- [ ] Supports GM/T OIDs: SM2, SM3
- [ ] Supports post-quantum OIDs: FN-DSA, HQC, ML-DSA

### Plans: 3 plans

Plans:
- [ ] 03-01-PLAN.md — Algorithm registry data model and data (AlgorithmInfo struct, AlgorithmRegistry, standard + GM/T + PQ algorithm entries)
- [ ] 03-02-PLAN.md — Algorithm list and parameter display UI (render_algorithms grouped by category, render_parameters with algorithm selector)
- [ ] 03-03-PLAN.md — OID lookup functionality (text input + search button, keyboard handling, lookup by oid or name, result/error display)

---

## Phase 4: UI/UX Bug Fixes

**Goal**: Fix critical input system bug (IMK error), unify fonts, add scrolling and copy support.

### Issues Fixed
- macOS IMK mach port error (`IMKCFRunLoopWakeUpReliable`)
- All algorithm page input fields non-functional (cannot type)
- Execute buttons too wide, no reset button
- Font sizes too small and inconsistent
- No vertical scrolling for long output (keys)
- Generated keys cannot be copied

### Plans: 3 plans

Plans:
**Wave 1**
- [x] 04-01-PLAN.md — GPUI-native IME-aware input foundation and algorithm input wiring

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 04-02-PLAN.md — Workbench layout, typography, status banners, compact action/reset controls

**Wave 3** *(blocked on Wave 2 completion)*
- [x] 04-03-PLAN.md — Right-content scrolling, monospaced output blocks, certificate and algorithm copy support

Cross-cutting constraints:
- Keep current dark theme and Chinese UI style while normalizing contrast, spacing, typography, and status hierarchy.
- Preserve fixed top tabs and left menu; only the right work area scrolls.
- All tool changes remain local-only and do not add deferred theme/navigation features.

---

## Phase 5: Certificate Chain Viewer

**Goal**: Visual certificate chain navigation.

### Requirements Mapping
- FR-3: Certificate Chain Viewer

### Success Criteria (UAT)
- [ ] Certificate tab left menu includes: Certificate Chain
- [ ] Displays chain as expandable tree structure
- [ ] Clicking a chain element shows its details in right panel
- [ ] Shows chain validation status

---

## Phase 6: Polish & Cross-platform Verification

**Goal**: Final polish and verify on both macOS and Linux.

### Requirements Mapping
- NFR-1: Cross-platform
- NFR-2: Performance

### Success Criteria (UAT)
- [ ] App compiles and runs on macOS
- [ ] App compiles and runs on Linux
- [ ] UI is consistent across platforms
- [ ] Certificate parsing completes within 500ms
- [ ] UI remains responsive during all operations

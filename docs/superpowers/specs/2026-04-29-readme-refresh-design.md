# README Refresh Design

## Goal

Restructure README.md to accurately reflect the current codebase, fix discrepancies, and improve clarity.

## Problems to Fix

1. **RSA padding modes undocumented** — code supports OAEP-SHA256 and PKCS#1 v1.5 switching
2. **Components claim overstated** — README says "unused by tabs" but `input.rs` and `ui_helpers.rs` are actively used
3. **OID registry surface larger than documented** — code recognizes SHA1/SHA3/Ed25519/X25519/AES-GCM/SLH-DSA/FN-DSA/HQC etc.

## Changes

### Section: Features → Algorithm Tools

**Current:** Single table mixing interactive tools with PQ OID-only items.

**New:** Split into two sub-sections:

1. **Interactive Tools** — tools usable in the UI:
   - Symmetric: AES-128-ECB, AES-256-CBC, SM4-ECB, SM4-CBC
   - Asymmetric: RSA (2048/3072/4096, OAEP-SHA256/PKCS#1v1.5), ECDSA (P-256)
   - Hash: SHA-256, SHA-384, SHA-512, SM3
   - KEM: ML-KEM-512/768/1024
   - Signature: ML-DSA-44/65/87

2. **OID Recognition** — algorithms identified during cert parsing but no interactive tool:
   - SHA-1, SHA-224, SHA-3 family
   - AES-128-CBC, AES-256-ECB, AES-256-GCM
   - Ed25519, Ed448, X25519, X448
   - SM2
   - SLH-DSA 12 variants
   - FN-DSA-512/1024 (provisional)
   - HQC-128/192/256

### Section: Features → 后量子密码学

**Current:** Separate section listing PQ algorithms.

**New:** Remove — content merged into the algorithm tables above.

### Section: Features → UI Features (new)

Add brief note about:
- Copy-to-clipboard with status banner
- Expandable error details
- Custom scrollbar in right panel

### Section: Project Structure → components/

**Current:** `可复用 GPUI 组件 | Reusable GPUI widgets`

**New:** Clarify:
- `input.rs` / `ui_helpers.rs` — actively used by tabs
- `left_menu.rs` / `tab_bar.rs` — available but currently unused

## Sections Unchanged

- Header + intro
- Quick Start
- Packaging
- Dependencies
- License

## Verification

- `cargo test` passes (no code changes)
- Manual review: all claims match source code

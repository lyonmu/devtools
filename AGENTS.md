# AGENTS.md

## Commands

```bash
cargo build                         # compile debug build
cargo test -p devtools              # run all inline #[cfg(test)] tests
cargo test -p devtools -- <module>  # run a filtered module/test, e.g. cert::oid_resolver
cargo run                           # launch GPUI desktop app; requires macOS or Linux display
```

No lint/format/CI config exists. Do not add one unless asked.

## Architecture

- **Single Rust crate**, edition 2024, GPUI native desktop app.
- Entry flow: `src/main.rs` → `src/app.rs` (`DevToolsApp`) → `src/tabs/mod.rs` (`CertTab` / `AlgoTab`).
- `src/app.rs` owns top-level state, tab switching, file dialog integration, right-panel scrolling, copy status, and algorithm tool rendering.
- `src/tabs/mod.rs` owns certificate tab state, algorithm tab state, left-menu item lists, and GPUI text-input synchronization.
- `src/cert/` parses and displays certificate data:
  - `mod.rs` detects PEM/DER/CRT/CER/PKCS#12 inputs, parses multi-cert PEM files, builds primary certificate + chain display data, and formats SPKI public keys as PEM.
  - `extensions.rs` formats X.509 extensions and provides friendly parsing for Basic Constraints, Key Usage, EKU, SAN, SKI, and AKI.
  - `oid_resolver.rs` maps algorithm/extension OIDs to display names and classifies public keys as classic vs post-quantum.
  - `fixtures/` contains certificate test fixtures used by inline unit tests.
- `src/algo/` contains stateful crypto tool implementations:
  - `hash.rs` — SHA-256/384/512 and SM3 over text or hex input.
  - `symmetric.rs` — AES-128-ECB, AES-256-CBC, SM4-ECB, SM4-CBC using hex input/key/IV.
  - `asymmetric.rs` — RSA keygen/encrypt/decrypt with OAEP-SHA256 or PKCS#1 v1.5 compatibility mode, plus P-256 ECDSA sign/verify.
  - `pq_kem.rs` — ML-KEM-512/768/1024 keygen, encapsulation, decapsulation.
  - `pq_signature.rs` — ML-DSA-44/65/87 keygen, sign, verify.
  - `registry.rs` / `oid_defs.rs` — algorithm registry and GM/T + post-quantum OID constants.
  - `tool_trait.rs` — shared `CryptoTool` contract used by tool state tests.
- `src/components/` contains GPUI widgets/helpers:
  - Actively used: `input.rs` and `ui_helpers.rs`.
  - Currently reserved/unused by the main tabs: `left_menu.rs` and `tab_bar.rs`.

## Behavior Notes

- The UI text is primarily Chinese; keep new user-facing strings consistent with that style unless asked otherwise.
- Certificate chain display is parse-order display with leaf/intermediate/root labels; it is not trust-path validation.
- PKCS#12 parsing currently attempts an empty password only. Password-protected `.p12/.pfx` files should report that a password is required; do not claim password-entry support unless implemented.
- OID recognition is broader than interactive crypto tools. Some algorithms are display/recognition only.
- Text inputs are GPUI entities. Keep `AlgoTab::sync_inputs_to_tool_state` and `sync_tool_state_to_inputs` consistent when adding fields.

## Testing

Tests live inline as `#[cfg(test)] mod tests` inside source files. There is no `tests/` directory.

```bash
cargo test -p devtools                           # all tests
cargo test -p devtools -- cert::oid_resolver     # single module example
cargo test -p devtools -- algo::asymmetric       # algorithm module example
```

Current verified baseline: `cargo test -p devtools` → `92 passed; 0 failed`.

## Packaging

```bash
# macOS: build .app and package a drag-to-Applications DMG
./scripts/make-dmg.sh                     # → target/release/bundle/osx/DevTools.app + DevTools.dmg
cargo bundle --release                    # optional: .app only

# Linux: cargo deb only works on Linux and requires cargo-deb
cargo deb                                 # → target/debian/devtools_0.1.0_amd64.deb
```

Tools:

```bash
cargo install cargo-bundle  # macOS .app bundle support
cargo install cargo-deb     # Debian package support
```

Packaging files:

- `scripts/make-icons.sh` converts `chip.png` into `icons/icon.icns` using macOS `sips` and `iconutil`.
- `scripts/make-dmg.sh` runs `cargo bundle --release`, stages `DevTools.app` plus an `Applications -> /Applications` shortcut, then creates `DevTools.dmg` with `hdiutil`.
- `Cargo.toml` contains `[package.metadata.bundle]` and `[package.metadata.deb]`.

## Gotchas

- **GPUI 0.2 API:** `.id()` returns `Stateful<Div>` rather than `Div`; helper functions returning identified divs should use `gpui::Stateful<gpui::Div>`.
- **Owned children:** `.child()` requires owned values; clone `String`/`SharedString` when needed instead of passing borrowed temporary data.
- **Native GUI runtime:** `cargo run` needs a windowing system (macOS desktop or Linux X11/Wayland). Do not expect it to work in pure headless shells.
- **File dialogs and async I/O:** file selection uses `rfd` and `cx.spawn()` with `gpui::WeakEntity`; the file read itself is synchronous inside the async block.
- **Copy/status UI:** certificate and algorithm tabs keep separate copy status. Preserve the current per-tab behavior when adding copyable fields.
- **DMG creation:** `cargo bundle` creates `.app` only; use `./scripts/make-dmg.sh` for a DMG that includes both `DevTools.app` and an `Applications` shortcut.
- **Debian packaging:** `cargo deb` may only be available after installing `cargo-deb`; do not use unsupported flags unless verified in the installed version.

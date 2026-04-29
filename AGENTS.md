# AGENTS.md

## Commands

```
cargo build     # compile
cargo test      # run all tests (inline #[cfg(test)] in src/)
cargo run       # launch the GPUI desktop app (needs macOS or Linux display)
```

No lint/format/CI config exists. Do not add one unless asked.

## Architecture

- **Single crate**, edition 2024, GPUI native GUI app.
- Entry: `src/main.rs` → `src/app.rs` (top-level tab management) → `src/tabs/mod.rs` (CertTab + AlgoTab rendering).
- `src/cert/` — certificate parsing (PEM/DER/PKCS#12), OID resolution, extension display.
- `src/algo/` — crypto toolkits: hash (SHA-256/384/512, SM3), symmetric (AES/SM4), asymmetric (RSA/ECDSA), PQ KEM (ML-KEM), PQ signatures (ML-DSA).
- `src/components/` — reusable GPUI widgets (left_menu, tab_bar, input). Currently unused by tabs; tabs inline their own helpers. Match existing pattern when adding UI.

## Testing

Tests live inline as `#[cfg(test)] mod tests` inside source files. There is no `tests/` directory.

```
cargo test -p devtools                           # all tests
cargo test -p devtools -- cert::oid_resolver     # single module
```

## Packaging

```bash
# macOS: build .app and package a drag-to-Applications DMG
./scripts/make-dmg.sh                     # → target/release/bundle/osx/DevTools.app + DevTools.dmg
cargo bundle --release                    # optional: .app only

# Linux: cargo deb only works on Linux
cargo deb                              # → target/debian/devtools_0.1.0_amd64.deb
```

Tools: `cargo install cargo-bundle` (macOS), `cargo install cargo-deb` (Linux).

## Gotchas

- **GPUI 0.2 API**: `.id()` returns `Stateful<Div>` not `Div` — helper functions must return `Stateful<Div>`. `.child()` requires owned types — pass `s.clone()` not `&s` for String fields.
- **GPUI is a native GUI framework** — `cargo run` needs a windowing system (X11/Wayland on Linux, native on macOS). No headless mode.
- File dialogs and async I/O use `cx.spawn()` with `gpui::WeakEntity`. The file reading itself is synchronous inside the async block.
- Chinese UI text throughout (tabs, labels, error messages).
- `cargo bundle` creates `.app` only; use `./scripts/make-dmg.sh` for a DMG that includes both `DevTools.app` and an `Applications` shortcut. `cargo deb` only runs on Linux.

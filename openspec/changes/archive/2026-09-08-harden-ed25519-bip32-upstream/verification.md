# Verification evidence

## Upstream contribution

- Base: `typed-io/rust-ed25519-bip32@6539dc9f792174fa5c2290c9e0a23710a1e1ecef`
  (signed tag `ed25519-bip32-v0.4.3`).
- Patch: `yshyn-iohk/rust-ed25519-bip32@b9e5c78d0418e6f6b8878004da31d6bca7882657`.
- Issue: <https://github.com/typed-io/rust-ed25519-bip32/issues/8>.
- Pull request: <https://github.com/typed-io/rust-ed25519-bip32/pull/9>.
- License: upstream MIT OR Apache-2.0; new zeroize dependency Apache-2.0 OR MIT.

## Commands passed

- `cargo fmt --all -- --check`.
- `cargo test` under Rust 1.98.1: 5/5 unit tests and doc-tests passed.
- `cargo +1.81.0 test`: 5/5 unit tests and doc-tests passed.
- `cargo +1.81.0 build --target thumbv7em-none-eabihf`.
- `cargo check --target wasm32-unknown-unknown`.
- `cargo check --target aarch64-apple-ios`.
- `cargo check --target aarch64-linux-android`.
- `cargo doc --no-deps`.
- `cargo package --allow-dirty --no-verify`: 18 files, 46.3 KiB.
- Nix-pinned `cargo-audit 0.22.2 --file Cargo.lock`: 1,242 advisories loaded,
  three crate dependencies scanned, no vulnerability reported.
- `cargo tree --locked --edges normal --prefix none`: only
  `ed25519-bip32`, `cryptoxide 0.6.5` and `zeroize 1.8.2`.
- `cargo tree --locked -e features -i cryptoxide --prefix none`: only
  `curve25519`, `ed25519`, `hmac` and `sha2` features resolve.
- `rg -n "unsafe" src Cargo.toml`: no authored unsafe match after the patch.
- `git diff --check 6539dc9..b9e5c78` and exact-diff inspection passed.

## Known non-blocking baseline finding

`cargo clippy --all-targets -- -D warnings` reports eleven findings on both the
unchanged `6539dc9` base and the contribution. They are existing style/API
findings in `signature.rs`, `derivation/common.rs`, `derivation/mod.rs`,
`key.rs` and pre-existing tests. The focused patch introduces no new Clippy
category and upstream CI does not currently enable its commented Clippy job.

The initially invoked user-installed cargo-audit 0.20.1 could not parse the
current RustSec database's CVSS 4.0 entry. The SDK Nix environment's
cargo-audit 0.22.2 completed successfully; no audit result is inferred from the
older tool failure.

## Exact-diff review

The contribution modifies only the dependency declarations, private-key
documentation/formatting/drop path, module removal and one formatting test.
No line in derivation, signing, verification, parsing or byte-conversion
mechanics changes. `Debug` and `Display` remain implemented, returning a fixed
marker independent of secret bytes. The crate-local unsafe module is deleted.

No SDK manifest, runtime code, API, lockfile or downstream repository changes
in this iteration. The SDK release-update gate remains a separate issue after
an immutable upstream release exists.

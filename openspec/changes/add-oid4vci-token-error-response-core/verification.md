# Verification evidence

- **Issue:** #129
- **Develop base:** `82ecd8c5947f064e6bed49e5c3731b6f989659bc`
- **Specification commit:** `741cd75ca4e12faa28cf06f3ff7e670cf1117beb`
- **Implementation commit:** `c2185bbf53b87aa48395edb0db86e64d66d0d492`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains

## Focused and workspace gates

- Token Error Response tests passed with all features and no default features:
  9/9 in each mode.
- `cargo test --locked --workspace --all-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed.
- `cargo fmt --all -- --check`, `git diff --check`, dependency-tree inspection,
  and `./scripts/factory check`: passed.
- Both commits are GPG-signed and contain matching DCO trailers.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 27 compatible native checks.
This included Rust 1.85 MSRV and feature builds, Android aarch64, iOS aarch64,
`wasm32-unknown-unknown`, strict Clippy, rustdoc, formatting, factory,
text/Nix/TOML lint, cargo-deny, cargo-audit, and release Nextest suites. The
main workspace profile ran 512 tests: 512 passed and 22 were skipped. Minimal,
KMP-compatible, all-feature, deterministic-only, and getrandom-only profiles
also passed.

The local aarch64-darwin invocation reports x86_64-linux as an incompatible
system; hosted Ubuntu CI supplies the independent Linux Nix gate before merge.

## Non-blocking baseline diagnostics

- The hermetic cargo-audit derivation reports that its offline index cannot
  answer yanked-package lookups, then returns success. This predates this
  change and is preserved by the pinned factory configuration.
- The nixpkgs macOS fixup hook emitted its known non-fatal
  `audit-tmpdir.sh` segmentation warning while affected derivations completed
  successfully.

## Consumer-isolation receipts

- Oxid remained at
  `5ba38b9bbc9326c294b353daaf2a074eca18c22f`, clean, with both referenced
  source hashes unchanged.
- Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`; only its pre-existing
  `.pi-subagents/`, `.pi/`, and `tmp/` untracked paths remain, and both
  referenced source hashes are unchanged.
- No consumer was switched, staged, built, or edited. SDK `main` was not
  changed.

## Local review

The distinct exact-diff review is recorded in `review.md`. All findings were
resolved before readiness and no blocking finding remains.

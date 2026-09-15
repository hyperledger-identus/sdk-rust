# Verification evidence

- **Date:** 2026-09-16
- **Issue:** #168
- **Develop base:** `66ec2b9b3a7ec35cf21ecc52cdca5bebed0b4d0d`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust 1.98.1
- **Result:** passed

## Focused and workspace evidence

- `scripts/check-input-resource-boundaries.py .`: 23 boundary families passed.
- `scripts/tests/input-resource-boundaries.py`: all structural mutation cases
  passed locally and inside the isolated Nix factory derivation.
- BIP-39 derivation tests: 35 default and 40 `kmp-compat` tests passed, including
  exact/one-over word, entropy, and passphrase cases plus published vectors.
- JWK all-feature integration tests: 22 passed, including exact/one-over member,
  depth, node and text budgets, serde/native parity, and redaction.
- `cargo test --workspace --all-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all -- --check`, Taplo format check, and `git diff --check`:
  passed.
- `scripts/factory check audit-sdk-input-resource-boundaries`, the complete
  factory contract, OpenSpec validation, archive preservation, constraints,
  source distribution, Apollo parity, support policy, code health, and error
  goldens: passed.

## Candidate and reproducible matrix

The pinned `crypto-candidate` app completed in 81.357 seconds with Rust 1.98.1,
all four configured feature profiles, three unpublished crate archives, SBOMs,
and public-API evidence. The baseline delta contains only the intended additive
BIP-39 and JWK limit constants; no public error variant or dependency edge was
added.

`nix flake check --fallback` passed the complete compatible aarch64-darwin
matrix: factory isolation, text/TOML/Nix lint, source contract, format, builds,
strict Clippy variants, default/minimal/KMP/entropy tests, Rust 1.98.1 policy,
WASM, Android aarch64, iOS aarch64, etalon, rustdoc, cargo-deny, and
cargo-audit. Nix reported x86_64-linux as incompatible with this local host;
protected Ubuntu CI supplies that independent gate before merge.

## Iteration evidence

The first isolated factory attempt correctly omitted an untracked checker from
the Git-backed flake source. After staging the candidate, Taplo identified the
new inventory formatting, and the formatted form exposed a whitespace-coupled
mutation helper. Both issues were corrected; the final staged tree passed the
isolated factory derivation and the full compatible Nix closure.

## Repository boundary

No downstream repository was edited, switched, copied from, or built. No
publication, tag, support-tier activation, consumer migration, or `main`
promotion is part of this change.

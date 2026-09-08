# Verification receipt

Verification date: 2026-09-09
Base revision: `8f2bfc3b888ec56656aad71a629b380a1c1d7c25`
Change: `bound-validated-newtype-strings`

## Passed locally

- `cargo fmt --all -- --check`
- `cargo test -p identus-derive`
- `cargo test -p identus-did`
- `cargo test -p identus-derive --test expand`
- `cargo test -p identus-did method::tests`
- `scripts/factory check --change bound-validated-newtype-strings`
- `git diff --check`
- `nix flake check --print-build-logs`: all 38 aarch64-darwin checks passed,
  including Rust 1.98.1 build/MSRV/etalon, workspace/default/feature build,
  clippy with warnings denied, rustdoc, format, 662-test workspace nextest,
  minimal/KMP/entropy variants, iOS/Android/WASM targets, cargo-deny, factory
  and text/TOML/Nix lint.

## Observed non-failures

The Nix cargo-audit derivation reported that its immutable offline crates.io
index could not answer yanked-status lookups, then completed without a failing
advisory result. Darwin Nix fixup also printed intermittent ELF-scanner
segmentation warnings while producing successful derivations. Neither is
caused by this dependency-neutral diff; hosted supply-chain evidence remains
authoritative.

## Pending hosted evidence

PR policy and GitHub Actions fast gates remain pending until the signed branch
is pushed. The PR must not merge unless all required hosted checks are green.

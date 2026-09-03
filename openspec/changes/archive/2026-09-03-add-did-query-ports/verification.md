# Verification evidence

- **Issue:** #43
- **Develop base:** `f42b62bb222986002ba19194e8c2ffdb03debcdb`
- **Specification commit:** `ba64726c5d3bf6049e8f9af2416fd8bb9e181773`
- **Implementation commit:** `cc6edc40a8da75b93e035948713c42f09db168be`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains

## Product and workspace gates

- `cargo test -p identus-did`: passed; 7 query-port tests passed and one
  release diagnostic was intentionally ignored, alongside the existing suite.
- `cargo test -p identus-conformance`: 20 passed; both marked port traits were
  discovered by the naming contract.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `cargo fmt --all -- --check`: passed.
- `./scripts/factory check`: passed; 17 OpenSpec items, 30 backlog rows and all
  factory/support/inventory contracts passed.
- `git diff --check`: passed.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 26 native flake checks. This
included Rust 1.85 MSRV, default/minimal/compat feature builds, Android aarch64,
iOS aarch64, `wasm32-unknown-unknown`, strict Clippy, rustdoc, formatting,
factory, text/Nix/TOML lint, cargo-deny, cargo-audit and release nextest suites.
The main workspace nextest profile ran 187 tests: 187 passed and 5 were skipped;
the KMP compatibility profile ran 85 tests, and entropy profiles ran their
focused 1- and 3-test suites successfully.

The local aarch64-darwin invocation reports x86_64-linux as an incompatible
system; hosted Ubuntu CI provides the independent Linux Nix gate before merge.

## Performance diagnostic

In release mode, 50,000 representative option parses plus object-safe resolver
dispatches completed in `59.784334ms`, approximately `836,339 queries/s` on the
local machine. This is observational evidence, not a portable pass threshold.

## Deferred contracts

- #10: HTTP binding and network policy.
- #44: deterministic method registry and dispatch.
- #45: injectable cache and clock policy.
- #46: bounded DID URL dereferencing algorithm.
- #47: mature DID Registration lifecycle and secret modes.
- #41: deeper result/wire hardening, including duplicate raw JSON members.

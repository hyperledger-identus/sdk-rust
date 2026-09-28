# Verification

- **Date:** 2026-09-29
- **Planning head:** `628db4c5dfb0cc9237526b6a32b525cd95a58462`
- **Reviewed code head:** `22a747d0779ce6a2c2019d84dee7b95f0992f38c`
- **Environment:** aarch64-darwin with the repository-pinned Nix toolchains
- **Issue:** [#429](https://github.com/hyperledger-identus/sdk-rust/issues/429)

## Passed locally

- `cargo test -p identus-oid4vp` and strict package clippy: all DCQL,
  invocation, request-object, and error-contract tests passed.
- `cargo test --workspace --all-features`: all enabled workspace unit,
  integration, compile-fail, and doctests passed.
- `cargo test --workspace --no-default-features`: the complete minimal-feature
  workspace suite passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` and
  `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `./bootstrap.sh --check`, `./scripts/factory check`, and immutable preflight
  validation: passed.
- `nix flake check --no-build`: every aarch64-darwin derivation evaluated; Nix
  explicitly reported the incompatible x86_64-linux system as omitted.
- Actual Nix builds passed for `rust-msrv` on Rust 1.89 and primary
  `wasm32-unknown-unknown`, `aarch64-apple-ios`, and
  `aarch64-linux-android` target gates.
- `nix develop --command ./scripts/check-siros-dcql-spike.sh`: eight clean-room
  fixture tests, strict clippy, cargo-deny, RustSec, exact-version, lock-record,
  private-adoption, and public-type-leakage checks passed.
- `nix develop --command python3 scripts/code-health-audit.py ...`: no
  production module signal and no `identus-oid4vp` function-complexity signal.
- `cargo fmt --all -- --check` and `git diff --check`: passed.

## Dependency and reuse evidence

- The private engine is pinned exactly to `siros-dcql =0.3.0`; the lock record
  checksum is
  `749a7da5b56f724a05c9694d87a82e0b03a609ad412deaf84ccf9376de2ec192`.
- The isolated research fixture audited 13 crate dependencies. The complete
  `identus-oid4vp` tree contains 72 unique rendered `cargo tree` lines, including
  its pre-existing JOSE and crypto paths.
- The upstream engine has 1,421 physical production Rust lines. The SDK facade
  has 756 physical / 614 nonblank non-comment-shaped lines, covering the
  stricter Final-profile boundary, resource policy, SDK-owned ports and types,
  redaction, and mapping—not a second candidate-selection engine.
- No `siros-dcql` public type appears in the SDK public signature surface.

## Behavioral evidence

Clean-room tests prove complete credential selection, exact JSON value typing,
holder binding, credential sets, capped combinations, fail-closed candidate
tolerance gaps, duplicate claim-set rejection, scope ambiguity, every query and
evaluation resource bound, and redacted diagnostics. The facade consumes only
a signature-proven `VerifiedRequestObject`; it does not claim full Authorization
Request validity, verifier trust, consent, or credential authenticity.

No hosted result is represented as complete here. Protected PR fast CI and an
exact-head review remain independent delivery evidence.

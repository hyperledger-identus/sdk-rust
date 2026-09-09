# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@4c22c1ac059be5c5113301b205f1b7553c89e4b9
Implementation head: 2acb814d6c3a2c44473595e39d9174f3fb573a06
Pull request: [#233](https://github.com/hyperledger-identus/sdk-rust/pull/233)

## Candidate fixture receipt

- Exact `oauth2 5.0.0`, defaults disabled, was resolved from the separately
  tracked lock under Rust 1.98.1.
- `scripts/check-oauth2-oid4vci-spike.sh` passed four deterministic tests and
  strict Clippy. The tests prove the RFC 7636 S256 vector, complete
  authorization URL, exact public-client token request through an in-memory
  client, and the recorded policy/error/resource mismatches.
- Root-configured `cargo deny` passed advisories, bans, licenses and sources;
  only the documented duplicate-syn and unmatched-root-policy warnings remain.
  `cargo audit --deny warnings` passed all 96 lock entries.
- The script observed 68 normalized host normal/build cone lines and proved
  that `oauth2` is absent from the root manifest and lock.

## Target receipt

Exact locked `cargo check --lib` passed for:

- `wasm32-unknown-unknown` in `nix develop .#wasm`;
- `aarch64-apple-ios` in `nix develop .#bindings`; and
- `aarch64-linux-android` in `nix develop .#bindings`.

The normalized candidate cone contains 68 lines on host/iOS/Android and 77 on
WASM. These are compile observations, not runtime or support claims.

## Repository receipt

- `scripts/factory research-ready` and `constraints-ready` passed before the
  committed fixture.
- `scripts/factory check` passed all governance, research, constraint,
  traceability and OpenSpec checks.
- `nix flake check --print-build-logs` passed all 31 compatible
  aarch64-darwin checks, including Rust 1.98.1 builds, target/minimal/feature
  lanes, strict Clippy, 676-test release Nextest, rustdoc, formatting, policy,
  dependency, license and advisory gates.
- The first full run exposed only a new-manifest Taplo formatting defect. It
  was corrected and the complete rerun passed. Nix omitted incompatible
  x86_64-linux checks; hosted Linux PR CI remains the merge authority.
- Exact-diff whitespace checks passed. All branch commits are GPG-signed and
  carry DCO trailers.

## Exclusions

No network interoperability, redirect listener, browser/mobile runtime,
downstream consumer, PAR, DPoP, RAR, authorization-response correlation,
performance, publication, release or certification test was run or inferred.

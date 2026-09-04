# Verification receipt

- **Component and issue:** `identus-credentials`, #73; parent #6 / `IDR-009`
  and #20
- **Base SHA:** `4e4cf5a8b4fb66e289629d287b4b87c63d5fc2bf`
- **Compatibility:** additive unreleased API; no dependency, feature, wire,
  persistence, execution, trust-policy, chain, or consumer commitment
- **Shape:** exactly six canonical stages in a fixed array; one bounded
  construction pass; direct stage lookup; aggregate outcome is derived
- **Reason bounds:** 1–128 lowercase ASCII bytes under the specified machine
  token grammar; rejected values are not retained or echoed

## Focused commands passed

- `cargo fmt --all -- --check`
- `cargo test -p identus-credentials --all-features` (20 passed; one manual
  diagnostic ignored)
- `cargo check -p identus-credentials --no-default-features`
- `cargo clippy -p identus-credentials --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS=-Dwarnings cargo doc -p identus-credentials --all-features --no-deps`

All commands ran through the repository's pinned Nix development environment.

## Performance observation

On an Apple Silicon `aarch64-darwin` host using pinned
`rustc 1.96.0-nightly (91021ccc7 2026-03-17)`, the ignored release diagnostic
constructed 1,000,000 all-passed reports in 24.175167 ms, approximately
41,364,761 reports/second. This is an observation, not a correctness threshold
or cross-host performance promise.

## Full commands passed

- `./scripts/factory validate add-credential-verification-report`
- `./scripts/factory check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo test --workspace --no-default-features`
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --all-features --no-deps`
- `nix flake check --print-build-logs` (all 26 compatible host checks; pinned
  nightly, Rust 1.85 MSRV, native, Android, iOS, WASM, feature, docs, tests,
  lint, dependency/license/advisory, and factory lanes)

The Nix run emitted the repository's known non-fatal macOS fixup-hook
segmentation diagnostics and offline crates.io yanked-index lookup diagnostics;
the governed audit and all 26 derivations completed successfully.

## Review and isolation

- Pre-implementation semantic/API/security/performance review: no blocker.
- Distinct post-implementation review of `develop@4e4cf5a...aee8789`: no
  unresolved finding; no dependency or feature drift.
- Oxid, midnight-identity, Lace ID Portal, and NeoPRISM final HEAD/branch/status
  receipts match preflight. No consumer repository was mutated.
- Trust policy, verification execution/adapters, metadata/schema descriptors,
  evidence payloads, codecs, storage, FFI, publication, and downstream adoption
  remain focused follow-up slices.
- Archive receipt, hosted CI/review, merge, effort report, and parent updates
  remain delivery evidence.

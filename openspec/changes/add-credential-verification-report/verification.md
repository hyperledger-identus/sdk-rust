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

## Pending delivery evidence

The full workspace/flake matrix, exact-diff post-implementation review,
consumer isolation receipts, archive receipt, signed commits, hosted CI/review,
merge, effort report, and parent updates are recorded after those gates pass.

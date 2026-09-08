# Verification receipt

Verification date: 2026-09-09
Base revision: `04266d649d83d27213a11cea88cab064ec5d4811`
Change: `bound-registration-identifiers`

## Passed locally

- `cargo fmt --all -- --check`
- `cargo test -p identus-did --test did_registration`: 18 passed, 1 ignored
- `cargo test -p identus-did`: all unit/integration/doc tests passed
- `cargo test --workspace`: all workspace unit/integration/doc tests passed
- `scripts/factory check --change bound-registration-identifiers`
- `openspec validate bound-registration-identifiers --strict`
- `git diff --check`
- `nix flake check --print-build-logs`: all 29 aarch64-darwin checks passed,
  including Rust 1.98.1 workspace/default/all-feature builds, MSRV/etalon,
  clippy with warnings denied, rustdoc, format, 664-test workspace nextest,
  minimal/KMP/entropy variants, iOS/Android/WASM targets, cargo-deny, audit,
  factory, and text/TOML/Nix lint.

## Observed non-failures

The immutable offline cargo-audit index could not answer yanked-status lookups
and Darwin Nix fixup printed intermittent ELF-scanner segmentation warnings;
their derivations and the complete flake check succeeded. No dependency changed.
The local host omitted incompatible `x86_64-linux`; the hosted Linux fast lane
remains required before merge.

OpenSpec synchronized the additive DID Core requirement and created archive
`2026-09-08-bound-registration-identifiers` using its UTC date. The factory
wrapper's postcondition expected local Asia/Makassar date `2026-09-09` and
reported a false-negative name mismatch after the successful archive; direct
repository validation below is authoritative for the resulting state.

## Pending hosted evidence

PR policy, DCO, file hygiene, and GitHub Actions fast gates remain pending until
the signed branch is pushed. The PR SHALL NOT merge unless every required hosted
check is green.

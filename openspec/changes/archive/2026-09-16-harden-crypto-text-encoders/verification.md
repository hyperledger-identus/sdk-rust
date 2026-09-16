# Verification

- **Date:** 2026-09-17
- **Issue:** #298
- **Develop base:** `508917b0416147668b9d12949eec987f0b82946b`
- **Verified implementation head:** `0383b42cee9e688efdebf2f72826014cf000733e`
- **Environment:** aarch64-darwin, repository-pinned Rust 1.98.1 and Nix inputs

## Focused and workspace gates

- `cargo fmt --all -- --check`: passed.
- `cargo test -p identus-crypto --all-features`: passed, including exact and
  one-over codec boundaries, ownership forms, canonical round trips, redacted
  errors, and compile-fail absence of the infallible conversions.
- `cargo test -p identus-crypto --no-default-features`: passed after the test
  target was correctly feature-gated.
- `cargo clippy -p identus-crypto --all-targets --no-default-features -- -D warnings`:
  passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-features`: passed.
- `./scripts/factory check`: 72 OpenSpec items and every factory contract
  passed.
- `git diff --check` and production-source searches: passed.

## Candidate API and supply-chain evidence

`nix run .#crypto-candidate -- --output <temporary> --source-revision
0383b42cee9e688efdebf2f72826014cf000733e` passed from a clean tree in 77.119
seconds. It verified two byte-identical package assemblies, safe normalized
archives, the extracted three-package closure in default/all/minimal/KMP
profiles, the committed public API, SemVer comparison, and one CycloneDX 1.6
SBOM per package. Tool pins were `cargo-public-api 0.52.0`,
`cargo-semver-checks 0.50.0`, and `cargo-cyclonedx 0.5.9`.

The candidate remained unpublished and produced these local archive digests:

- `identus-derive`: `9d96309764c271f1ce1b8f167a2beeaa0e0a79894641a43844034b081d6d6d1a`
- `identus-core`: `127ccb9f6a13dcd7c527158533e81ad0bcfa554b1dd541071ebf5db1900b9dcc`
- `identus-crypto`: `8710fe01832169d2351a302edb806df61c54d5e58439c801d730755309cb60b2`

## Reproducible target closure

- `nix flake check --no-write-lock-file`: all 29 compatible aarch64-darwin
  checks passed at `fdd0356454731977f1ac3fcf7bf575582617b127`. The closure
  included default and minimal/KMP/entropy profiles, strict Clippy, rustdoc,
  formatting, factory/source contracts, dependency policy, Rust 1.98.1 etalon,
  and WASM, Android ARM64, and iOS ARM64 compilation. Nix explicitly reported
  x86_64-linux as incompatible with the local host; hosted `fast` supplies that
  independent required lane.

## Exact-diff routing

`./scripts/factory plan --base
508917b0416147668b9d12949eec987f0b82946b --head
0383b42cee9e688efdebf2f72826014cf000733e --profile production-ready`
classified 26 paths / 951 changed text lines across Rust, specification, and
documentation. Required PR status is the Linux `fast` lane; portable targets,
security, and fuzz conformance remain slow-line recommendations. The file-count
decomposition exception is justified in `review.md`.

## Repository boundary

No downstream repository was mutated. Local Oxid, Lace ID Portal, Midnight
Identity, and NeoPRISM inspection found no current sdk-rust call site requiring
a coordinated migration.

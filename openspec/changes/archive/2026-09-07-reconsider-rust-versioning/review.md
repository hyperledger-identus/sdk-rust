# Distinct local review

- **Date:** 2026-09-07
- **Issue:** [#170](https://github.com/hyperledger-identus/sdk-rust/issues/170)
- **Exact base:** `8ed0d2017e1411016f6660f0efe454c14ce6be60`
  (`origin/develop`)
- **Verdict:** ready; zero unresolved blockers, majors or minors

## Semantic review

- Edition 2024, public MSRV, primary validation compiler and NeoPRISM etalon
  are independent axes with distinct names and enforcement.
- Rust 1.85.0 remains the only effective workspace MSRV and every declared
  feature surface retains a dedicated MSRV build gate.
- Rust 1.98.1 is pinned as primary stable; it owns ordinary build, test,
  Clippy, formatting, documentation and supported compile-target gates.
- NeoPRISM's nightly remains a locked all-target, all-feature workspace build
  and cannot satisfy primary or MSRV evidence.
- Rust 1.89.0 is only a future candidate. No Cargo manifest, public API, wire
  format, supported target tier, dependency cone or release claim changes.
- Dioxus and future FFI needs remain downstream or crate-local decisions and
  cannot silently raise the generic SDK floor.

## Exact-diff review

- Reviewed all policy, ADR, OpenSpec, flake, Nix gate, validator, negative-test
  and compile-fail snapshot changes against the exact base.
- Confirmed the existing NeoPRISM-derived nixpkgs, rust-overlay, nightly and
  Nix version remain unchanged; the second exact overlay is limited to the
  primary stable provider.
- Confirmed gate artifact classes follow their compiler classes and invalid
  primary, etalon or MSRV cross-wiring fails structural validation.
- The first aggregate run exposed a factory test fixture that copied the new
  constraint index but not ADR 0064; the fixture inventory was corrected and
  its dedicated suite passes.
- Rust 1.98.1 changed three E0599 diagnostic phrases. Only the corresponding
  `trybuild` snapshots were regenerated; runtime behavior is unchanged.
- Confirmed `origin/develop` remained at the reviewed exact base before
  archive and PR preparation.

## Verification

- `python3 scripts/tests/support-policy.py`: 154 passed after the hosted-CI
  correction.
- `python3 scripts/tests/constraints.py`: 12 passed.
- `bash scripts/tests/factory-contract.sh`: passed.
- `python3 scripts/check-support-policy.py`: passed.
- `./scripts/check-constraints.py`: passed.
- `./scripts/factory check`: passed with 47 OpenSpec items before archive.
- `./scripts/factory research-ready reconsider-rust-versioning`: passed.
- `./scripts/factory constraints-ready reconsider-rust-versioning`: passed.
- `git diff --cached --check`: passed before this receipt update.
- `nix flake check --print-build-logs`: all 30 compatible local checks passed,
  including 587/587 workspace tests, primary stable quality/target gates,
  Rust 1.85 MSRV gates and the NeoPRISM-etalon build.
- `./scripts/factory archive reconsider-rust-versioning`: passed; both modified
  requirements were merged into their canonical specifications without a
  destructive rewrite.
- Post-archive `./scripts/factory check` and the full Nix flake check passed
  again with 46 canonical capabilities and zero active changes.
- Hosted CI then showed that sanitizer fuzz commands inherited the stable
  default shell and rejected `-Zsanitizer`. A dedicated etalon-backed fuzz
  shell, explicit workflow selection and two negative policy tests resolve the
  branch-owned integration finding without weakening the stable default.
- The refreshed hosted Linux flake run completed every reported derivation
  except the final MSRV build, where LLVM reported `No space left on device`.
  The workflow now reclaims unused preinstalled Android, .NET and Haskell
  toolchains before Nix installation; no compatibility gate was removed or
  weakened.
- `nix develop .#fuzz -c ./scripts/fuzz-jws.sh smoke`: passed all 4,096
  deterministic runs locally, and the complete flake check passed again.

The local flake check ran on aarch64-Darwin and omitted x86_64-Linux as an
incompatible system. Required GitHub CI must provide the independent hosted
Linux result before merge. No downstream repository was mutated.

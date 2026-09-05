# Verification evidence

- **Date:** 2026-09-05
- **Issue:** #87, child of #6 / `IDR-009` and #20
- **Develop base:** `e22fa8eab93026ba763e581873f4e5e233aaf035`
- **Specification commit:** `5224b34a7b316d05deffc157c09dc906867ca7a6`
- **Implementation commit:** `86e8c91f56827b8bbed5b17b3fb899359195f2e8`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains
- **Result:** every applicable local gate passed

## Functional evidence

- `cargo test -p identus-credentials --all-features`: 56 passed across the
  envelope, metadata, status, report and verifier suites; four manual release
  diagnostics were intentionally ignored.
- The focused crate also passed no-default checking, strict Clippy,
  warning-denied rustdoc and formatting.
- Seven new non-diagnostic tests cover borrowed least-authority inputs,
  redacted Debug, object-safe async invocation, exact unrelated-format
  dispatch, valid/invalid/indeterminate reports, static operation errors,
  unknown format, duplicates, the 64-entry boundary and deterministic
  introspection.
- Invalid proof evidence returns a canonical Invalid report. Unsupported,
  unavailable and internal execution failures remain separate and never map to
  `ErrorKind::VerificationFailed` or product trust.

## Full reproducible matrix

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `./scripts/factory validate add-credential-verifier-execution` and
  `./scripts/factory check`: passed.
- `nix flake check --print-build-logs`: all 27 compatible aarch64-darwin
  checks passed, including pinned nightly, Rust 1.85 MSRV, native, Android,
  iOS, WASM, feature, lint, documentation, factory, dependency, license,
  advisory and release Nextest lanes. The principal suite ran 347 tests: 347
  passed and 15 manual diagnostics were skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate before merge. Existing nonfatal
offline crates.io and macOS fixup-hook diagnostics did not fail a derivation.

## Performance observation

`cargo test -p identus-credentials --release
credential_verifier_registry_dispatch_throughput_diagnostic -- --ignored
--nocapture` polled 1,000,000 ready verifier futures through exact registry
dispatch in `47.123542ms`, about 21,220,816 dispatches/second, using
`rustc 1.96.0-nightly (91021ccc7 2026-03-17)` on
`aarch64-apple-darwin`. This includes boxed-future creation, map lookup,
adapter call and report cloning; it is an observation, not a portable pass
threshold.

## Compatibility, provenance and boundary evidence

- The only dependency change is `identus-credentials` to the existing
  build-time `identus-derive` port marker. No external dependency, Cargo
  feature, concrete verifier, wire form, chain/runtime, network, storage or
  trust policy changed.
- Oxid remained at
  `integration@bfe3b481568dc738f0732c2b27548fab8721fd95` with its pre-existing
  untracked `.claude/` and `.pi/taskflows/` paths.
- midnight-identity remained at
  `develop@427f8571950c42967a18726cbcbefecc19ef8d79` with its pre-existing
  dirty `third_party/midnight-did` submodule.
- Lace ID Portal remained at
  `main@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing
  untracked `.pi-subagents/`, `.pi/` and `tmp/` paths.
- NeoPRISM remained clean at
  `main@d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`; Apollo remained at
  `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` with its pre-existing dirty
  nested secp256k1 repository.
- No donor code was copied and no donor, consumer, reserved `main`, release or
  publication state was modified.

## Iteration and effort evidence

Issue #87 was created at `2026-09-05T00:24:54Z`. The issue, OpenSpec change,
ADR and pre-implementation review preceded the implementation commit. Focused,
workspace and full Nix gates plus the distinct post-implementation review
completed in approximately 16m46s. Hosted CI, review and merge timing remain
pull-request evidence.

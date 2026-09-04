# Verification evidence

- **Date:** 2026-09-05
- **Issue:** #83, child of #20 / `IDR-008`
- **Develop base:** `9074f7f7490759763a683a8fd879dba3272e4ebc`
- **Specification commit:** `14dca5e35b253105e15b2996db938607f1bcef37`
- **Implementation commit:** `74d487d428f7d64f54fd8750e7a7642c91991ad9`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains
- **Result:** every applicable local gate passed

## Functional evidence

- `cargo test -p identus-presentations --all-features`: 25 passed; one manual
  release diagnostic intentionally ignored.
- Focused all-feature/no-default checks, strict Clippy, warning-denied rustdoc,
  formatting, inventory, backlog and strict OpenSpec validation passed.
- Tests cover OID4VP-shaped one-to-one and Midnight-shaped aggregate
  artifacts, unrelated formats, transferred buffers, exact bounds, aggregate
  byte budget, changed requests, duplicate/unknown/missing bindings, format
  mismatch, receipt ordering, redaction and every new error contract.

## Full reproducible matrix

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `./scripts/factory validate add-generated-presentation-artifacts` and
  `./scripts/factory check`: passed.
- `nix flake check --print-build-logs`: all 26 compatible aarch64-darwin
  checks passed, including pinned nightly, Rust 1.85 MSRV, native, Android,
  iOS, WASM, feature, lint, documentation, factory, dependency, license,
  advisory and release Nextest lanes. The principal suite ran 334 tests: 334
  passed and 13 manual diagnostics were skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate before merge. Existing nonfatal
offline crates.io yanked-index and macOS fixup-hook diagnostics did not fail a
derivation.

## Performance observation

`cargo test -p identus-presentations --release
presentation_generation_and_receipt_input_throughput_diagnostic -- --ignored
--nocapture` validated 250,000 complete generation/receipt-input flows in
`477.063ms`, approximately 524,040 flows/second, using
`rustc 1.96.0-nightly (91021ccc7 2026-03-17)` on
`aarch64-apple-darwin`. This is an observation, not a portable pass threshold.

## Compatibility, provenance and boundary evidence

- No `Cargo.toml`, `Cargo.lock`, dependency, feature, wire format, serializer,
  proof implementation, runtime, transport, storage, chain adapter or product
  policy changed.
- Oxid remained at
  `integration@bfe3b481568dc738f0732c2b27548fab8721fd95` with its pre-existing
  untracked agent directories.
- midnight-identity remained at
  `develop@427f8571950c42967a18726cbcbefecc19ef8d79` with its pre-existing
  dirty `third_party/midnight-did` submodule.
- Lace ID Portal remained at
  `main@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing
  untracked agent and temporary directories.
- NeoPRISM remained clean at
  `main@d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`; Apollo remained at
  `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` with its pre-existing dirty
  nested secp256k1 repository.
- No donor code was copied and no donor, consumer, reserved `main`, release or
  publication state was modified.

## Iteration and effort evidence

Issue #83 was created at `2026-09-04T22:44:51Z`. The issue, OpenSpec change,
ADR and pre-implementation review preceded the implementation commit. Focused,
workspace and Nix gates plus the distinct post-implementation review completed
by `2026-09-05T07:02:57+08:00`, about 18 minutes wall-clock after issue
creation. Hosted CI, review, correction and merge timing remain pull-request
evidence.

The pre-archive factory receipt recorded branch
`codex/idr-008c-presentation-artifacts`, head
`96647035f4f86e7bdbb799f8f710374b0e07830f` and merge base
`9074f7f7490759763a683a8fd879dba3272e4ebc`. Canonical presentation and
governance specs were synchronized and the change was archived as
`2026-09-04-add-generated-presentation-artifacts`. Archive review found and
repaired three older current-inventory requirements that still excluded the
new #83 surface; the archived delta now records those modifications.

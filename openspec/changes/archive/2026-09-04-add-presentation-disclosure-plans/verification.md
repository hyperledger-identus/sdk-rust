# Verification evidence

- **Date:** 2026-09-05
- **Issue:** #81, child of #20 / `IDR-008`
- **Develop base:** `4ca4605a9b0ab9e378943fa5046d691e259efa26`
- **Specification commit:** `57274cc4aeadac18cef34b5920ac4534ad28ef40`
- **Implementation commit:** `e42d863b9dd38450e17a882895f6886aa434e841`
- **Gate correction commit:** `d5017723735bf37ccb8ce940c96fbe8855545c63`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains
- **Result:** every applicable local gate passed

## Functional evidence

- `cargo test -p identus-presentations`: 20 passed; one manual release
  diagnostic intentionally ignored.
- Focused no-default-feature check, strict Clippy, warning-denied rustdoc and
  root formatting passed.
- Tests cover DCQL-shaped, Midnight-shaped and unrelated format identifiers,
  maximum bounds, duplicate pairs, cross-request validation, unknown queries
  and candidates, multiplicity, claim mismatch, required-claim coverage,
  redacted diagnostics and every new static error contract.
- `./scripts/factory check`, inventory validation, backlog validation and
  `git diff --check` passed.

## Full reproducible matrix

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `nix flake check --print-build-logs`: passed all compatible aarch64-darwin
  checks, including Rust 1.85 MSRV, default/minimal/compat feature builds,
  Android aarch64, iOS aarch64, `wasm32-unknown-unknown`, strict Clippy,
  rustdoc, formatting, factory, text/Nix/TOML lint, cargo-deny, pinned RustSec
  audit and release Nextest suites. The principal suite ran 329 tests: 329
  passed and 13 were skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies the independent Linux gate before merge. Existing nonfatal
offline yanked-index and macOS fixup-hook diagnostics did not fail a derivation.

## Performance diagnostic

`cargo test -p identus-presentations --release
presentation_request_candidate_plan_throughput_diagnostic -- --ignored
--nocapture` validated 250,000 request/candidate/plan triples in
`254.246458ms`, approximately 983,298 triples/s, on Apple Silicon with
`aarch64-apple-darwin` rustc 1.95.0 (`59807616e`). After exact request binding
was added, the same diagnostic completed in `288.080125ms`, approximately
867,814 triples/s. These are observational results, not a portable pass
threshold.

## Compatibility, provenance and boundary evidence

- No `Cargo.toml`, `Cargo.lock`, dependency, feature, wire format, serializer,
  runtime, chain adapter or product policy changed.
- Oxid remained at `integration@bfe3b481568dc738f0732c2b27548fab8721fd95`
  with its pre-existing untracked agent directories.
- midnight-identity remained at
  `develop@427f8571950c42967a18726cbcbefecc19ef8d79` with its pre-existing
  dirty `third_party/midnight-did` submodule.
- Lace ID Portal remained at
  `main@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing
  untracked agent and temporary directories.
- NeoPRISM remained clean at
  `main@d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`; Apollo remained at
  `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` with its pre-existing
  dirty nested secp256k1 repository.
- No donor code was copied and no donor, consumer, reserved `main`, release or
  publication state was modified.

## Iteration and effort evidence

The exact-head Nix gate exposed one brittle backlog negative-test fixture after
`IDR-008` moved from program issue #20 to delivery issue #81. The fixture now
locates a program-owned row by value instead of fixed row position; all six
backlog tests and the complete matrix passed after correction.

Hosted review then exposed a cross-request filter-binding gap that structural
candidate revalidation could not detect. The corrected candidate set owns a
private exact request snapshot; a changed issuer/type/schema filter now fails
with `presentation.candidate_request_mismatch`. Focused and full gates were
rerun after the correction, with final hosted receipts recorded on PR #82.

From issue creation at `2026-09-04T21:22:57Z` through complete local gates and
post-implementation review at approximately `2026-09-04T21:38Z` was about 15
minutes wall-clock. Hosted CI, review and merge timing is recorded separately
on the pull request and issue.

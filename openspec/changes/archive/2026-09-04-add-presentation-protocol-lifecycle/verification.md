# Verification evidence

- **Date:** 2026-09-05
- **Issue:** #85, child of #20 / `IDR-008`
- **Develop base:** `8fb533562d5b006151372e214a38ef8a7e3fa5bd`
- **Specification commit:** `becc2dfabbf90d432b41df685ce15e4f134f71ca`
- **Implementation commit:** `4814a03fbe7b504bfeb3de9903f3a36eeced7d8b`
- **OpenSpec metadata commit:** `9c66eb5bcd9e10856a195f24514a50eb5edeca86`
- **Hosted-review specification correction:** `4345156`
- **Hosted-review implementation correction:** `5b67401`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains
- **Result:** every applicable local gate passed

## Functional evidence

- `cargo test -p identus-presentations --all-features`: 31 passed across the
  lifecycle and existing presentation suites; two manual release diagnostics
  intentionally ignored.
- The same focused suite passed with no default features, strict Clippy,
  warning-denied rustdoc and formatting.
- An independent allowed-edge table exercised all 144 ordered pairs across
  seven active phases and five terminal outcomes: exactly 31 edges were
  accepted.
- Tests cover exact phase/outcome/state spelling, strict unknown/padded/case
  rejection, redacted error contracts, active/terminal accessors, terminal
  immutability, refusal timing, required generation/delivery boundaries and
  both cancellation origins. Generation cancellation rejects completion while
  delivery cancellation permits an observed completion race.

## Full reproducible matrix

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `./scripts/factory validate add-presentation-protocol-lifecycle` and
  `./scripts/factory check`: passed.
- `nix flake check --print-build-logs`: all 27 compatible aarch64-darwin
  checks passed, including pinned nightly, Rust 1.85 MSRV, native, Android,
  iOS, WASM, feature, lint, documentation, factory, dependency, license,
  advisory and release Nextest lanes. The principal suite ran 340 tests: 340
  passed and 14 manual diagnostics were skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate before merge. Existing nonfatal
offline crates.io yanked-index and macOS fixup-hook diagnostics did not fail a
derivation.

## Performance observation

`cargo test -p identus-presentations --release
presentation_lifecycle_transition_throughput_diagnostic -- --ignored
--nocapture` validated 2,000,000 state-pair decisions in `3.5525ms`, about
562,983,814 decisions/second, using
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

Issue #85 was created at `2026-09-04T23:27:49Z`. The issue, OpenSpec change,
ADR and pre-implementation review preceded the implementation commit. Focused,
workspace and full Nix gates plus the distinct post-implementation review
completed within the same bounded iteration. Hosted CI, review, correction and
merge timing remain pull-request evidence.

The archive receipt recorded branch
`codex/idr-008d-presentation-lifecycle`, reviewed head
`29c55eede9d7b5afaff61460ed551c464a0604f8` and develop merge base
`8fb533562d5b006151372e214a38ef8a7e3fa5bd`. Canonical presentation and
governance specs were synchronized and the change was archived as
`2026-09-04-add-presentation-protocol-lifecycle`. Local issue-to-archive
elapsed time was approximately 15m45s; hosted PR timing remains separate.

The first hosted review at `99475e0` found an origin-free cancellation path
that could move from generation cancellation to completion. The issue was
amended before corrective implementation. Corrective head `5b67401` preserves
generation and delivery cancellation separately, and the full local evidence
set above was rerun before requesting a fresh hosted review.

# Verification

Verification date: 2026-09-28
Reviewed implementation head: 51254ef27f9bb81292a96f5e776f4efb31a84eee

## Focused evidence

- `cargo test -p identus-oid4vp`: 19 integration tests passed.
- `cargo clippy -p identus-oid4vp --all-targets --all-features -- -D warnings`:
  passed.
- Explicit GET, deterministic POST, status/media/compact binding, protected
  type, signature, client-id, wallet-nonce, duplicate, resource, misuse, and
  redaction vectors passed.
- The immutable preimplementation receipt binds exact contract commit
  `e2479dbfc481e442d71108a85b26bfdd7a75f3ea` to
  `develop@3512f31a660d536cdee7a585ae95e2c0a6129790`.

## Workspace and factory evidence

At the reviewed implementation head:

- `cargo fmt --all -- --check`: passed in 0.45 seconds.
- `cargo build --locked --workspace --all-targets`: passed in 0.84 seconds.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`:
  passed in 0.53 seconds.
- `cargo test --locked --workspace --all-targets`: passed in 11.15 seconds;
  only declared diagnostic tests were ignored.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed for 22 packages in 0.70 seconds.
- `scripts/factory check`: 90 items passed in 14.97 seconds.
- Bootstrap inventory, 39 input-resource boundaries, 30-row upstream backlog,
  five source packages, and support policy checks passed.
- Exact code-health analysis passed in 7.44 seconds with no OID4VP function or
  module threshold signal.

## Dependency, compiler and target evidence

- `cargo deny check` in the pinned Nix shell: advisories, bans, licenses and
  sources passed in 2.42 seconds; only pre-existing informational warnings were
  emitted.
- Rust 1.89 MSRV check for `identus-oid4vp`: passed in 0.50 seconds.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` compile checks passed in 0.88 seconds total.
- The normal dependency tree adds only the existing workspace
  `identus-jose` edge and its already-reviewed dependency cone.
- Every implementation-range commit is signed and carries a DCO sign-off.

## Result

All specified retrieval, signed-JAR, correlation, redaction, resource,
architecture, compiler, target, and delivery gates pass. JWE, verifier trust,
full request semantics, DCQL, consent, and responses remain explicitly out of
scope.

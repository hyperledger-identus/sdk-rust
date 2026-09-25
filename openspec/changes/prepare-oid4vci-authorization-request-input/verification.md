# Verification

Verification date: 2026-09-25
Verified head: `c92f399990829adaf951525c4f80234aef0eae4d`

## Focused evidence

- `cargo test -p identus-oid4vci --test authorization_request_input`: 10
  passed, including RFC 7636 Appendix B and all exact input boundaries.
- `cargo test -p identus-oid4vci --all-targets`: passed; 232 tests passed, 1
  diagnostic test ignored.
- `cargo clippy -p identus-oid4vci --all-targets -- -D warnings`: passed.
- `cargo test -p identus-conformance --lib`: 31 passed after pinning the
  intentional internal dependency cone.
- `cargo tree -p identus-oid4vci --edges normal --depth 2`: direct internal
  edges are `identus-core`, `identus-crypto` and `identus-jose`; no new
  external package appears in `Cargo.lock`.

## Full repository evidence

- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-targets --all-features`: passed; ignored
  diagnostics remained opt-in and no test failed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed for 21 packages.
- `scripts/factory check`: passed, 77 OpenSpec/factory items and all structural
  contracts green.
- `scripts/factory backlog-live`: passed, 30 rows and 9 distinct live issue
  owners; IDR-023 points to open successor #354.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
- Commit signatures and DCO trailers were verified on every local commit.

## Unrun evidence

The weekly portable-target/slow lane, external Authorization Server
interoperability, browser/mobile redirect handlers, PAR, callback processing,
HTTP execution, downstream consumers, publication, performance and
certification were not run and are not claimed by this slice.

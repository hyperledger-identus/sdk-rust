# Verification receipt

## Identity

- Delivery issue: `#158`; parent dependency decision: `#151`.
- Exact base: `ce45c375b0682f2226b42d83e98991101409aed7`.
- Reviewed implementation head:
  `923e9b698b9cdb582f1745974bc508380c3f5f3a`.
- Decision: `form_urlencoded 1.2.2` is `not-adopt` for the current OID4VCI
  boundary; the strict bounded local codec remains unchanged.

## Source and dependency evidence

- The published crate checksum is
  `cb4cb245038516f5f85277875cdaa4f7d2c9a0fa0468de06ed190163b1581fcf`.
- Packaged metadata records VCS revision
  `91377f48bf35011d042aa5abef9e7f2a0a625aaa` and `dirty: true`; the archive is
  therefore the assessment authority.
- Packaged `src/lib.rs` SHA-256
  `766b5d679064e01f7e6cce6f127a23885f79806ce3bccc49e4fe41933b83fd8d`
  matched the file at the recorded revision.
- The candidate is MIT OR Apache-2.0, declares Rust 1.51 and resolves to two
  normal packages with defaults disabled and `alloc` enabled.
- Exact source proves malformed-percent preservation and lossy UTF-8 parsing;
  the current SDK negative boundary is stricter.

## Local evidence

- `scripts/factory check` passed with 48/48 OpenSpec items before receipt.
- `cargo test -p identus-oid4vci` passed 167 tests with one ignored diagnostic;
  `cargo doc -p identus-oid4vci --no-deps` passed.
- Nix `factory-contract`, `rust-fmt`, `rust-build`, `rust-test` and `rust-doc`
  passed on aarch64-Darwin with Rust 1.98.1. Workspace nextest passed 607/607
  tests with 22 skipped diagnostics.
- The first combined Nix selector found one Markdown heading false-positive;
  wording was corrected and the complete `lint-text` derivation then passed
  Markdown, YAML, EditorConfig and shell lint.
- `git diff --check` passed. Cargo manifests, `Cargo.lock`, public API, wire
  behavior and the complete `crates/` tree are byte-identical to the base.
- `git verify-commit` passed for both reviewed commits.

## Scope receipt

- Added ADR 0083 and corrected ADR 0061's focused disposition.
- Moved the candidate to the durable negative-decision ledger with exact
  provenance and finite reconsideration triggers.
- Added a canonical dependency-research requirement for accepted and rejected
  parser behavior, completeness and resource limits.
- Added no production dependency, runtime code, API, error, wire-format or
  downstream repository change.

## Residuals

- The local codec remains SDK-maintained and must retain its current negative
  and capacity tests.
- Any future adoption needs a new issue, refreshed evidence, strict
  differential tests and a superseding ADR/spec change.
- Hosted Linux `fast` is required before merge.

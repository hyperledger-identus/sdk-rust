# Verification receipt

## Identity

- Delivery issue: `#155`; parent dependency decision: `#151`.
- Exact base: `61210a8613ccd6afc8ab66460e1f00c12b54f712`.
- Reviewed implementation head:
  `48691efc783034ed5646ee258fcbd00bf7072113`.
- Decision: `multihash 0.19.5` is `conditional-adopt`; no production
  dependency or runtime behavior is added.

## Standards and provenance evidence

- did:key Method v0.9 defines multibase around multicodec key type plus raw
  public-key bytes; it does not activate multihash work.
- The multihash format defines hash-function code, digest length and exact
  digest bytes.
- GitHub's tag API resolves annotated tag object `08383e21` to release commit
  `e2044a2e`; its verification state is unsigned.
- Crate version/checksum, MIT license, Rust 1.81 declaration and the historical
  two-package minimal cone are recorded for future re-evaluation.

## Local evidence

- `scripts/factory validate`, `research-ready`, `constraints-ready` and
  `check` passed with 48/48 OpenSpec items.
- The combined Nix build passed `factory-contract`, `lint-text`, `rust-fmt`,
  `rust-build`, `rust-test` and `rust-doc` on aarch64-Darwin under Rust 1.98.1.
- Workspace nextest passed 607/607 tests with 22 skipped diagnostics.
- `cargo test -p identus-did` passed 118 tests with eight ignored diagnostics;
  `cargo doc -p identus-did --no-deps` passed.
- Markdown, YAML, EditorConfig and shell lint passed. One initial Markdown
  heading false-positive was corrected and the complete selector command was
  rerun successfully.
- `git diff --check` passed. Cargo manifests and `Cargo.lock` are byte-identical
  to the base; all Rust executable behavior is unchanged.
- `git verify-commit` passed for all three reviewed commits after the local,
  unpublished branch was re-signed.

## Scope receipt

- Added ADR 0082 and the consumer-payoff dependency requirement.
- Superseded ADR 0061's multihash adoption entry and reclassified the research
  matrix/conditional ledger with corrected immutable provenance.
- Removed the did:key category error from `Multihash` module, public-value and
  test-fixture documentation.
- Added no Cargo dependency, parser, validation, algorithm policy, API change,
  wire change or downstream repository mutation.

## Residuals

- The current placeholder remains unvalidated and historically named; a new
  issue must own any removal, deprecation, validation or migration.
- Issue #156 requires a separate refresh and spec-first implementation even
  though Rust 1.98.1 clears its old compiler prerequisite.
- Hosted Linux `fast` is required before merge. The local Darwin Nix fixup
  diagnostic was non-fatal and did not change any successful derivation result.

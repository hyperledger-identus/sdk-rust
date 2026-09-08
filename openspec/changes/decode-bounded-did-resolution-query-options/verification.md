# Verification receipt

Verification status: changed-scope passed
Verification date: 2026-09-08
Base: develop@52db3110263bafc56cf5aefb49f57b4d96879fca
Reviewed implementation head: 0efc668

## Behavioral evidence

- `cargo test -p identus-did-resolver-http --no-default-features`: 20/20
  focused tests passed.
- `cargo test --workspace --all-features`: 666 tests passed with 22 configured
  ignores under Rust 1.98.1.
- Focused tests prove all four exact/one-over limits; common and extension
  options; full-result/document composition; literal plus and encoded
  delimiters; malformed percent/UTF-8/control; missing separators; literal and
  encoded duplicates; invalid booleans/versions; version conflict; query
  `accept`; resolver non-invocation; and error redaction.

## Compiler, documentation and factory evidence

- `cargo clippy -p identus-did-resolver-http --all-targets -- -D warnings`
  passed.
- `cargo fmt --all -- --check`, `git diff --check`, workspace all-feature docs
  and locked Cargo metadata passed.
- `scripts/factory doctor` and research/constraint readiness passed with 51/51
  active/canonical OpenSpec items.
- Production diff search found no unsafe, panic, unwrap or expect path.
- Both pre-implementation documentation and implementation commits are signed
  and carry DCO trailers.

## Dependency and deferred environment evidence

- `git diff --exit-code <base>..HEAD -- Cargo.toml Cargo.lock` passed: the
  direct and resolved graph is unchanged from the immediately preceding green
  develop merge.
- Local `cargo deny` and nextest are unavailable because this shell is outside
  the Nix devshell. Nix itself is unavailable, so no local slow matrix is
  claimed; the repository policy assigns that matrix to the weekly workflow.
- Installed `cargo-audit 0.20.1` fetched the current advisory database but
  stopped because it cannot parse CVSS 4.0 in RUSTSEC-2026-0073. This is a local
  tool-parser limitation, not an advisory match; hosted Nix tooling remains the
  supply-chain authority.
- Broad workspace all-target/all-feature Clippy found the unchanged
  `manual_noop_waker` lint in `crates/credentials/tests/verifier.rs`. The file
  is outside this diff; focused strict Clippy and all workspace tests pass.

## Result

All changed-scope and locally available repository gates pass. No dependency
surface changed. Hosted DCO, policy, factory, file-hygiene and Linux fast tests
remain mandatory before merge; the exhaustive Nix matrix remains scheduled
weekly by repository policy.

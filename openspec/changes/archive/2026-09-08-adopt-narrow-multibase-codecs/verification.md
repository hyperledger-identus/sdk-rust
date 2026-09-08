# Verification receipt

Verification status: passed
Verification date: 2026-09-08
Base: develop@b88b5ffe2769518be8e3da3492bc61e4b9ace2e2
Implementation head: 2f604631e52cacd0bb49f99bb3160de640b40c01

## Behavioral evidence

- `cargo test -p identus-did`: 121 passed and 8 intentionally ignored
  diagnostics.
- Focused JOSE unsupported-multibase verifier regression: passed.
- Full workspace nextest: 610/610 passed; 22 tests skipped by explicit
  repository configuration.
- Added canonical `z` and `u` cases, invalid alphabets, Base64 padding and
  trailing-bit alias, unsupported prefixes, empty decoded values, exact 4 KiB
  acceptance/one-byte-over rejection, native construction, JSON rejection and
  exact serialization.
- Corrected invalid placeholders across DID hardening, dereferencing,
  resolution and JOSE tests without changing their asserted protocol outcome.

## Architecture and dependency evidence

- Integrated normal graph: direct `base64 0.22.1` and `bs58 0.5.1`;
  rejected `multibase` family absent.
- Lockfile delta: one new package, exact `bs58 0.5.1` checksum
  `bf88ba1141d185c399bee5288d850d63b8369520c1eafc32a0430b5b6c287bf4`.
- `cargo deny --locked check`: advisories, bans, licenses and sources passed;
  existing informational duplicate-`syn` and unused-allowance warnings only.
- Networked pinned Nix `cargo audit --deny warnings`: 141 locked
  dependencies scanned against 1,242 advisories with no vulnerability.
- Source/diff search found no SDK unsafe block; selected dependency path and
  unreachable mutable-`str` unsafe implementation are recorded in ADR 0085.

## Compiler, target and repository evidence

- `cargo check -p identus-did --no-default-features`: passed.
- Direct `cargo check --locked -p identus-did` passed for
  `wasm32-unknown-unknown`, `aarch64-linux-android` and
  `aarch64-apple-ios`.
- `cargo clippy -p identus-did --lib -- -D warnings`: passed.
- `cargo fmt --all --check` and `git diff --check`: passed.
- `scripts/factory check`: 49/49 active change/spec items passed.
- Complete local `nix flake check`: all 37 compatible aarch64-Darwin
  derivations passed, including builds, configured strict Clippy, nextest,
  feature combinations, docs, format, dependency, advisory, factory and
  portable-target gates. x86_64-linux was omitted as incompatible with the
  local host and remains hosted-CI evidence.

## Diagnostic outside the configured gate

A direct `cargo clippy -p identus-did --all-targets -- -D warnings` probe
reported pre-existing Rust 1.98 test-only lints for manual no-op wakers and
manual `is_multiple_of` usage in unchanged lines. The repository's canonical
Nix Clippy derivations passed. This focused change does not broaden into an
unrelated test-harness cleanup.

## Result

All specified local gates pass at the immutable implementation head. Hosted
Linux `fast`, policy, DCO and file-hygiene checks remain the merge authority.

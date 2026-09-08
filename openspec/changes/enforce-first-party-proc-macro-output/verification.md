# Verification receipt

Verification status: passed
Verification date: 2026-09-08
Base: develop@9d339bb92b0d108b81564f9fa311320b8c1054aa
Implementation head: 2af77774aa34532bf1d5f0f8e5517953e28a6dba

## Behavioral evidence

- `cargo test -p identus-derive`: 4 output-safety unit tests, 14 runtime
  tests and all 10 trybuild cases passed.
- The output-safety cases reject every pinned unsafe construct, wrapped and
  legacy unsafe attributes, conditional and recursively conditional unsafe
  attributes, nesting, and accept safe controls without identifier-string
  false positives.
- `cargo test -p identus-conformance unsafe_policy`: 3/3 passed.
- The complete configured workspace nextest suite passed 617/617 tests; 22
  tests were skipped by explicit repository configuration.

## Architecture and dependency evidence

- `identus-derive` validates its completed direct `Newtype` item stream before
  emission and preserves the existing generated spans.
- `#[identus::port]` still returns caller-authored input unchanged and remains
  protected by the authored-source workspace policy.
- `Cargo.lock`, package count and resolved package versions are unchanged.
  Existing `syn 2.0.118` gains only its stable `visit` feature.
- Source and exact-diff review found no new runtime unsafe, native, FFI,
  persisted-data, secret or public API surface.

## Compiler, target and repository evidence

- Exact toolchain: Rust/Cargo 1.98.1.
- Complete local `nix flake check --print-build-logs`: all 27 compatible
  aarch64-Darwin derivations passed, including configured strict Clippy,
  workspace/feature tests, builds, docs, formatting, dependency/advisory,
  factory and wasm32/Android/iOS target gates.
- `scripts/factory check`: 50/50 active change/spec items passed.
- `cargo fmt --all --check` and `git diff --check`: passed.
- x86_64-linux was omitted as host-incompatible and remains hosted-CI merge
  evidence.

## Diagnostic outside the configured gate

A direct `cargo clippy -p identus-conformance --all-targets --all-features --
-D warnings` probe reports three pre-existing Rust 1.98 `collapsible_if` test
target lints in unchanged `boundary.rs` and `naming.rs`. The same failures were
reproduced at the exact clean base. The repository's configured strict Clippy
derivations pass; this focused security change does not mix in that unrelated
cleanup.

Miri, sanitizers, fuzzing, Windows and nightly are intentionally unrun because
the change adds no runtime unsafe, dependency package, supported target or
nightly promise.

## Result

All specified local gates pass at the immutable implementation head. Hosted
Linux `fast`, policy, DCO and file-hygiene checks remain the merge authority.

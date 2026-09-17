# Select the RC1 compiler and support matrix

## Why

The active-development policy intentionally aliases the workspace floor,
primary compiler, and compatibility etalon to Rust 1.98.1. ADR 0081 prohibits
turning that throughput decision into a published compatibility promise. The
first `identus-derive` / `identus-core` / `identus-crypto` candidate therefore
needs an explicit MSRV, release-target matrix, feature profiles, maintenance
policy, and evidence path before publication can be approved.

## What changes

- Declare Rust 1.89.0 as the candidate MSRV and retain Rust 1.98.1 as the
  pinned development, primary validation, and stable etalon compiler.
- Keep the required per-PR `fast` lane on one Rust 1.98.1 Linux build; execute
  the independent MSRV and complete platform/feature evidence only in the
  weekly/manual `slow` and release-candidate paths.
- Prove the three release crates on Linux and macOS hosts, their declared
  feature profiles, and compile-only browser-WASM, Android ARM64, and iOS ARM64
  targets without claiming runtime, FFI, packaging, or certification support.
- Add an isolated `identus-crypto` hash-only profile required by narrow
  consumers, without adding product- or chain-specific behavior.
- Record the support window, MSRV-change policy, limitations, and exact
  candidate receipt fields in policy, documentation, ADR, and tests.

## Capabilities

### Modified capabilities

- `sdk-support-policy`: replace the temporary single-compiler release blocker
  with the accepted RC compiler/target/feature matrix.
- `dependency-research-readiness`: make the now-selected release floor the
  evidence baseline for dependency decisions.

## Non-goals

No crate publication, tag, GitHub release, `main` promotion, FFI support,
runtime certification, Windows/WASI support, consumer repository mutation,
chain, ledger, or product primitive enters this change.

## Delivery

Issue #325 owns this material compatibility slice. It may merge to `develop`
under the normal issue-linked CI path, but release and publication remain
human-protected under #326.

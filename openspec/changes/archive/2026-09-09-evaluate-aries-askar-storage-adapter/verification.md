# Verification receipt

Verification status: passed locally with documented negative candidate evidence
Verification date: 2026-09-09
Base: develop@c04e495da719e1fa09beb77a5a05e744d75395fe
Implementation head: 36f7f864a04d47a9a6505a36148543acc2d7f769
Pull request: [#234](https://github.com/hyperledger-identus/sdk-rust/pull/234)

## Candidate fixture receipt

- Exact `aries-askar 0.4.6`, defaults disabled with only `sqlite`, resolved from
  the separately tracked 256-package lock under Rust 1.98.1.
- `scripts/check-aries-askar-storage-spike.sh` passed three focused tests,
  strict Clippy and root-graph isolation. The shared report covers all 16
  exact-record operations, including conflicts and scope isolation.
- Root-configured `cargo deny` passed advisories, bans, licenses and sources
  with documented duplicate/unmatched-policy warnings.
- Raw `cargo audit --deny warnings` intentionally reports RUSTSEC-2023-0071 for
  `rsa 0.9.10`; `cargo tree --locked --target all -i rsa` prints no dependency
  path for the selected features. No ignore was added.
- The normalized host normal/build cone contains 192 package/version lines;
  the script proves Askar remains absent from the root manifest and lock.

## Target receipt

Exact locked `cargo check --lib` passed for host and, with explicit platform C
toolchains, for:

- `aarch64-apple-ios` using Xcode clang; and
- `aarch64-linux-android` using NDK 27/API 21 clang and llvm-ar.

The `wasm32-unknown-unknown` check intentionally fails at `getrandom 0.2.17`
before SQLite compilation. Target observations are compile evidence only.

## Repository receipt

- Factory research and constraint readiness passed before implementation.
- Strict OpenSpec validation, exact-diff whitespace checks, fixture tests and
  Clippy passed.
- The first `nix flake check --print-build-logs` run exposed only Taplo
  formatting in the new fixture manifest. The manifest was corrected and the
  complete compatible aarch64-darwin check set was rerun successfully.
- Every branch commit is GPG-signed and carries a DCO trailer.

## Exclusions

No persistent-file behavior, multi-process concurrency, crash/cancellation
recovery, migration, browser/mobile runtime, performance, downstream consumer,
production key custody, release, certification or support test was run or
inferred.

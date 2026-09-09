# Proposal: establish Apollo-comparable crypto coverage evidence

## Why

Apollo publishes 74.81865284974093% line coverage for its pinned cryptography
baseline, while SDK-Rust has extensive executable tests but no reproducible
line denominator. M2 needs evidence that can be rerun and reviewed without
adding latency to the temporary fast pull-request lane.

## What changes

- Pin `cargo-llvm-cov` 0.9.0 through the locked Nix environment with the
  Rust 1.98.1 LLVM tools component.
- Exercise the default, all-feature, minimal and `kmp-compat`
  `identus-crypto` surfaces into one clean LLVM profile set.
- Enforce at least 74.82% first-party line coverage and produce normalized JSON,
  Markdown and LCOV evidence containing revision, tool and denominator data.
- Add the coverage campaign to weekly/manual slow CI only.
- Extend the Apollo parity ledger with the tool decision, threshold, source-file
  capability/vector mappings and evidence contract.
- Record the adoption decision in ADR 0096.

## Non-goals

No production dependency, public API, Apollo test-count comparison, unstable
branch coverage, hosted coverage service, first-party exclusion, fast-lane job,
consumer change or release claim is introduced.

## Issue

Implements <https://github.com/hyperledger-identus/sdk-rust/issues/212> under
M2 issue #9 and report Discussion #178.

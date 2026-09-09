# ADR 0096: adopt cargo-llvm-cov for Apollo coverage evidence

- **Status:** Accepted for M2 evidence
- **Date:** 2026-09-09
- **Issue:** [#212](https://github.com/hyperledger-identus/sdk-rust/issues/212)
- **Parent:** [#9](https://github.com/hyperledger-identus/sdk-rust/issues/9)
- **Report:** [Discussion #178](https://github.com/hyperledger-identus/sdk-rust/discussions/178)
- **Tool source:** `taiki-e/cargo-llvm-cov@be59056988acd54c7f984b7c85643daea3711b29`

## Context

Apollo reports 74.81865284974093% line coverage at its pinned M2 revision.
SDK-Rust needs a comparable first-party denominator across feature surfaces,
but coverage must not slow the temporary fast pull-request lane or add a
production dependency.

Rust exposes stable LLVM source-based instrumentation directly. Raw orchestration
must still discover Cargo test binaries, preserve and merge profiles, select
compiler-matched LLVM tools and export reviewable formats. `cargo-llvm-cov`
provides that narrow orchestration without changing library code.

## Decision

1. Pin `cargo-llvm-cov` 0.9.0 from the locked Nix package set. It remains
   maintainer/CI tooling and never enters the Cargo production graph.
2. Add `llvm-tools-preview` to the exact Rust 1.98.1 Nix toolchain so the
   reporter matches rustc's LLVM profile format.
3. Run default, all-feature, no-default-feature and `kmp-compat` crypto tests
   into one clean profile set in weekly/manual slow CI.
4. Normalize only executable regular Rust files below `crates/crypto/src`.
   Dependencies, tests, examples and generated code are excluded by boundary;
   no first-party source path may be excluded by a discretionary glob.
5. Require line coverage of at least 74.82%, Apollo's published percentage
   rounded up to two decimals. A lower threshold requires a superseding ADR.
6. Publish normalized JSON and Markdown plus LCOV. Bind artifacts to SDK SHA,
   Rust version, tool version, profiles and exact line denominator.
7. Keep the fast workflow unchanged. Coverage remains slow pre-release
   evidence under ADR 0081.
8. Keep vector, negative, redaction and resource-bound selectors independently
   mandatory. Aggregate coverage cannot replace semantic conformance.

## Consequences

- M2 gains a reproducible cross-language line-coverage comparison while
  preserving feature-specific execution.
- The default Nix shell gains one coverage executable and one compiler-matched
  component; crates and consumers gain no dependency.
- LCOV provides line inspection, while normalized summaries avoid absolute-path
  drift and state the exact denominator.
- Line coverage remains weaker than branch, path, mutation, side-channel and
  production-input evidence; those are not implied.

## Alternatives rejected or deferred

- Raw rustc/LLVM orchestration duplicates Cargo binary and profile handling.
- Tarpaulin adds backend and default-feature coupling without better evidence
  for the selected stable compiler.
- grcov adds a separate aggregation layer and is more useful for multi-language
  coverage than this single-crate campaign.
- Hosted coverage services are deferred until maintainers select credentials,
  retention and privacy policy.
- Nightly branch coverage conflicts with the temporary stable-only ordinary
  toolchain and has no comparable Apollo denominator.

## Verification and rollback

Mutation tests reject threshold, version, profile, path and denominator drift.
The parity validator checks complete source-to-capability/vector mappings. A
local and hosted slow run must exceed 74.82% before issue closure. Rollback
removes the tool, LLVM component, runner, workflow job and manifest contract
together. Threshold reduction requires a superseding ADR.

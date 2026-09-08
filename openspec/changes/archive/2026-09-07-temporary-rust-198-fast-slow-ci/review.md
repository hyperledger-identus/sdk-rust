# Exact-diff architecture and delivery review

- **Reviewer:** Codex fresh contract and implementation pass
- **Date:** 2026-09-08
- **Base:** `b5cfc8f30ab9137e3c71b94a73f162aff3d1b48c`
  (`origin/develop` after issue #153 merged)
- **Reviewed head:** `00cc7369438b2ee6b713caa3a30420f7c8ed78b2`
- **Scope:** issue #173, ADR 0081, Rust/Nix providers, declarative gates,
  five workflows, machine policy/checker/tests, compiler migrations and
  contributor/factory/release guidance
- **Result:** no unresolved blocker; hosted Linux evidence and protected-status
  activation remain explicit post-push responsibilities

## Findings

1. **Verified - one ordinary compiler.** Cargo declares Rust 1.98.1 and the
   primary, MSRV-labelled and etalon-labelled providers all alias the same
   stable toolchain and Crane provider. No ordinary gate selects nightly.
2. **Verified - nightly has one bounded role.** `fuzzToolchain` pins nightly
   2026-03-18 with `rust-src`; only the dedicated fuzz shell consumes it.
   Each sanitizer workflow is staggered weekly/manual and does not run for a
   pull request or push.
3. **Verified - fast is small and deterministic.** The `fast` Ubuntu workflow
   targets pull requests and pushes to `develop` and builds exactly eight
   named checks: factory structure, three repository lints, format, workspace
   build, strict Clippy and normal tests. It does not invoke the full flake.
4. **Verified - exhaustive evidence was moved, not deleted.** The `slow`
   workflow retains the full flake across Ubuntu and macOS on a weekly/manual
   trigger. The local aarch64-Darwin flake exposes 30 checks, including every
   target, feature, documentation, license and advisory gate, and all passed.
5. **Verified - the policy fails closed on drift.** The offline validator ties
   Cargo, the provider aliases, gate manifest, fast selectors, branch
   triggers, slow command, weekly schedules, fuzz schedules and review date to
   the machine policy. Its 159 regressions include daily-schedule, comment
   decoy, missing-gate and per-PR-slow negatives.
6. **Verified - the compiler migration is semantics-preserving.** Rust 1.98.1
   enabled `collapsible_if` diagnostics in four existing locations. The
   refactors use equivalent let-chain conditions; no API, error, serialization
   or dependency surface changed, and all 607 normal workspace tests pass.
7. **Verified - release claims remain conservative.** ADR 0081 expires for
   release planning on 2026-12-08 or release-candidate preparation. Release
   guidance prohibits publication until current slow debt and a focused,
   consumer-driven compiler-matrix decision are resolved.
8. **Verified - canonical specifications agree.** The support-policy,
   dependency-research and Nix-tooling capabilities all name the same temporary
   Rust 1.98.1 floor; the obsolete Rust 1.85 bootstrap scenario is removed.
9. **Residual - live branch protection is external.** Read-only evidence says
   `develop` is currently unprotected. The repository records `fast` plus the
   separate lightweight `pull-request-policy` check as desired required
   statuses, but this change does not mutate protected settings or claim that
   the merge gate is live. Maintainer action remains tracked by issue #26.
10. **Residual - Linux parity is hosted evidence.** The local machine validated
   the complete aarch64-Darwin flake. Exact `x86_64-linux` execution is left to
   the issue-linked PR's hosted `fast` run; merge is not eligible until every
   triggered check is green.

## Decision

The implementation satisfies issue #173 and ADR 0081. It materially reduces
the pull-request compiler and matrix cost while retaining complete scheduled
evidence and an explicit release prohibition. The change is approved for the
guarded OpenSpec archive and an issue-linked PR to `develop`.

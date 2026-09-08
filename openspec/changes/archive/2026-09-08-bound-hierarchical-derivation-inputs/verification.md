# Verification receipt

Verification status: passed
Verification date: 2026-09-08
Base: develop@6f247815b7a1e6248826c2d8b4c9b9365927d6e4
Implementation head: 6427303c6dca7965ea65b9b1f014e165ee3f46a3

## Behavioral evidence

- All-feature crypto library suite: 18/18 passed.
- Derivation integration suite: 35/35 passed.
- Cardano BIP-32 integration suite: 11/11 passed.
- Full workspace nextest: 637/637 passed; 22 tests skipped by explicit
  repository configuration.
- Apollo/KMP crypto compatibility profile: 129/129 passed; one test skipped by
  explicit repository configuration.
- Exact and one-over path-byte, axis-count, seed-length and depth cases passed,
  including resource-before-syntax precedence and private/public Cardano typed
  paths.

## Compiler, feature, target and repository evidence

- Isolated `derivation` and `cardano-bip32` library feature profiles passed.
- Strict all-target/all-feature Clippy with `-D warnings` passed.
- Rust 1.98.1 build, documentation, formatting, dependency-deny, advisory,
  minimal/no-default feature, wasm32, Android, iOS and factory derivations
  passed through complete local `nix flake check --print-build-logs` runs.
- The final complete Nix run reported `all checks passed!`; x86_64-linux was
  omitted as incompatible with the aarch64-Darwin host and remains hosted-CI
  evidence.
- `scripts/factory research-ready` and `constraints-ready` passed before
  implementation. `git diff --check` passed after exact-diff review.
- No `Cargo.toml` or `Cargo.lock` difference exists from the specification
  parent; the dependency cone and feature graph are unchanged.

## Review and operational evidence

- Distinct exact-diff/security review found no blocker and strengthened two
  public boundary regressions before this receipt.
- Work was split into signed and DCO-compliant specification, implementation
  and review-test commits. The implementation interval from specification
  commit to review-test commit was 18 minutes; complete Nix verification was
  the dominant delivery cost.
- Errors remain static and redacted; no seed, path, private key or chain code is
  rendered in diagnostics.

## Result

All specified local gates pass at the immutable implementation head. Hosted
Linux `fast`, policy, DCO and file-hygiene checks are intentionally unrun until
the issue-linked PR exists and remain mandatory before merge. Parent issues
#168 and #9 remain open after this bounded child slice.

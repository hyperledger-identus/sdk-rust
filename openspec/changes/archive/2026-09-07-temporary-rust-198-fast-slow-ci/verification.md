# Verification receipt

## Identity

- Delivery issue: `#173`; decision discussion: `#172`.
- Exact base: `b5cfc8f30ab9137e3c71b94a73f162aff3d1b48c`.
- Reviewed implementation head: `00cc7369438b2ee6b713caa3a30420f7c8ed78b2`.
- Compiler: stable Rust `1.98.1` for every ordinary provider; pinned nightly
  `2026-03-18` only for the named fuzz toolchain.
- Review deadline: `2026-12-08`, and before any release candidate.

## Local evidence

- `nix flake check --print-build-logs` passed all 30 configured
  aarch64-Darwin checks. The matrix includes the factory contract, repository
  lints, format, workspace build/Clippy/tests/docs, all isolated feature
  surfaces, WASM/Android/iOS compilation, equivalent compatibility-labelled
  builds, deny and audit.
- Normal workspace nextest passed 607/607 tests with 22 skipped diagnostics.
  The KMP surface passed 113/113; minimal crypto passed 8/8; entropy surfaces
  passed 3/3 deterministic, 1/1 system-random and 4/4 combined tests.
- The offline audit loaded 1,145 RustSec advisories and reported no vulnerable
  dependency. Its configured offline `--ignore yanked` mode emitted index
  availability diagnostics for yank lookup but completed successfully.
- `actionlint`, `yamllint`, `git diff --check`, strict OpenSpec validation,
  research readiness, constraint readiness, archive preservation, policy and
  factory checks passed.
- The guarded archive updated the support-policy and dependency-research
  specifications. The final archive reconciliation also records and applies
  the Nix-tooling delta, so every canonical specification names the same Rust
  1.98.1 contract.
- Support-policy regression tests passed 159/159; constraint-governance tests
  passed 12/12. Rust 1.98.1 strict Clippy passed after four equivalent
  let-chain migrations.

## CI topology receipt

- `fast`: pull request to `develop`, push to `develop` and manual dispatch;
  Ubuntu; eight exact Nix selectors; no full-flake or nightly execution.
- `slow`: weekly Monday and manual dispatch; Ubuntu/macOS; full
  `nix flake check`; no pull-request or push trigger.
- Crypto, DID and JWS sanitizer campaigns: staggered Monday, Tuesday and
  Wednesday weekly schedules plus manual dispatch; dedicated fuzz shell; no
  pull-request or push trigger.
- The previous 20 successful pull-request Nix runs measured 17m13s p50 and
  24m27s p95. A comparable post-change result requires 20 successful `fast`
  pull-request runs and remains follow-up evidence rather than a fabricated
  result from this first change.

## Residuals

- The local host cannot execute the Linux derivations; hosted PR CI supplies
  exact x86_64-Linux evidence.
- `develop` protection and required-status activation require maintainer
  repository administration under issue #26. This change does not perform or
  claim that external action.
- Slow and sanitizer failures after merge are visible pre-release debt. No
  release candidate or publication is allowed until they are resolved and a
  focused release-phase compiler decision replaces the temporary policy.

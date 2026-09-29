# Verification

Verification date: 2026-09-29
Develop base: `a320bc5186ce6b92f80559714928999799c21906`
Specification commit: `9aec4cddf76a2193eb0a73e87e12e55c6b6fd4b0`
Implementation commit: `35132ba6e19e31140249dc9d6b2d9c0a4d651340`
Environment: aarch64-darwin with repository-pinned Nix/Rust tools

## Focused evidence

- `python3 scripts/check-release-candidates.py`: passed the closed crypto/DID
  train contract.
- `python3 scripts/tests/release-candidates.py`: passed all mutation cases,
  including checkout-local primary output, missing clean-source assertion, and
  checkout-local upload regressions. A post-green review addition also proves
  that moving the assertion after MSRV is rejected.
- `nix develop --command actionlint .github/workflows/nix-checks.yml`: passed.
- `git diff --check`: passed.

## Repository evidence

- `nix flake check`: all 39 compatible local checks passed, including primary,
  MSRV, portable-target, Clippy, Nextest, docs, deny/audit, formatting, source,
  factory, and text/Nix/TOML policy. Linux is intentionally supplied by hosted
  CI rather than claimed by this macOS run.
- `./scripts/factory check`: 101 OpenSpec/factory items passed.
- Research readiness, constraint readiness, strict OpenSpec validation, and
  issue #480 preimplementation ancestry passed.
- Every branch commit is signed and carries a DCO sign-off.

## Deferred hosted evidence

This repair does not dispatch or rerun slow CI. Required PR CI proves the
policy and repository integration. A later natural or explicitly authorized
slow run at the frozen candidate revision must prove both Linux and macOS DID
matrix jobs and remains final milestone evidence owned by #388.

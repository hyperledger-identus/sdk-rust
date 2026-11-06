# Verification receipt

Verification date: 2026-10-29
Issue: #578
Develop base: `78bd1c01a83e014e0cc85be27075bf7634902199`
Planning commit: `8f1265d044b466431d67310ec878bcea43eb83c3`
Preimplementation receipt commit: `b1eb5277`
Reviewed implementation commit: `7231766d9281ab50935782d1ecaa578f0c2c4f66`

## Reproduction and planning evidence

- `develop@78bd1c01` reproduced the JWE failure caused by protected squash
  integration making the feature planning commit a non-ancestor of `HEAD`.
- Research readiness, constraints readiness, and strict OpenSpec validation
  passed before implementation.
- The ordinary aggregate preflight command was initially blocked by the exact
  inherited checker defect under repair. Its component readiness/validation
  gates were run directly, a standard-schema receipt was committed, and the
  repository preflight validator accepted that exact receipt. After the repair,
  the ordinary `scripts/factory preflight ... --validate-receipt` path passed.

## Focused mutation evidence

- `python3 scripts/check-error-golden.py .`: passed all five immutable
  bindings.
- `python3 scripts/tests/error-golden.py`: passed the complete synthetic
  mutation suite.
- `SDK_ERROR_GOLDEN_SOURCE_SNAPSHOT=1 python3 scripts/tests/error-golden.py`:
  passed, proving source-package fixture execution does not hide the protected
  squash Git-history cases.
- Coverage includes reachable and pruned feature commits, archive-only
  fallback, active-change rejection, exact `HEAD` blob requirements, receipt
  mutation, forged/transplanted working files, and non-planning contract
  rejection.

## Factory and repository evidence

- `scripts/factory doctor`: passed after the repair.
- `scripts/factory check`: passed.
- `scripts/tests/factory-contract.sh`: passed the complete mutation suite.
- `python3 scripts/check-quality-evidence-declarations.py`: passed 19
  declarations and 76 obligations.
- `git diff --check`: passed.
- Python bytecode compilation for the changed checker and test passed.
- `/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs`:
  passed the complete host matrix. This included Rust 1.98.1 and MSRV 1.89.0
  builds, workspace/all-feature clippy, rustdoc, formatting, dependency policy,
  Android/iOS/WASM compile targets, text/TOML linting, the factory contract,
  and 1,189 workspace nextest tests (1,189 passed, 23 declared skipped).

## Independent review

Codex CLI 0.163.0 with `gpt-6-sol` at high reasoning reviewed the exact diff
through `7231766d9281ab50935782d1ecaa578f0c2c4f66`. It reran focused checks and
reported no actionable regression. The reviewer sandbox could not access the
host Nix daemon; the supervising session therefore retained responsibility for
the real Nix gate rather than treating the sandbox limitation as evidence.

## Scope and remaining hosted evidence

This is repository-factory behavior only. No Rust crate behavior, public API,
protocol surface, dependency graph, target support, consumer repository, or
release artifact changed. The protected PR must still supply exact-head Linux
CI and delivery-policy evidence before guarded merge.

# Distinct local review

- **Date:** 2026-09-07
- **Issue:** [#166](https://github.com/hyperledger-identus/sdk-rust/issues/166)
- **Base and reviewed HEAD before commit:**
  `84ec5408ea9423d7d3965f3088e80ec78456efcb`
- **Verdict:** ready; zero unresolved blockers, majors or minors

## Semantic review

- The taxonomy separates kind (`hard`, `guardrail`, `budget`, `limitation`)
  from lifecycle (`effective`, `target`, `deferred`, `prohibited`).
- Materiality covers compiler, target, feature, public/wire/data, product,
  security/privacy/crypto, license, certification, budget, release and
  irreversible migration outcomes. Routine reversible work retains standing
  agent authority.
- Rust 1.85.0 remains the only effective MSRV. Rust 1.95.0 remains a target;
  issue #154 cannot activate it without an exact later sponsor decision.
- The index references canonical sources and does not replace their detailed
  values. The checker explicitly acknowledges that semantic classification is
  review-owned rather than machine-proven.
- The active change, ADR, guide, issue contract, factory commands, templates
  and review prompt agree on authority and readiness behavior.
- No public Rust API, wire format, dependency cone, supported target, release
  state or downstream repository changes.

## Exact-diff review

- Reviewed all 35 staged paths and the complete new checker, tests, index,
  ADR, guide and OpenSpec artifacts.
- Confirmed no staged changes to `Cargo.toml`, `Cargo.lock`, `flake.nix`,
  `flake.lock`, `nix/` or `docs/architecture/sdk-support-policy.toml`.
- Corrected issue #166 during review: calendar expiry/stale-review wording was
  inconsistent with the intentionally event-driven review and activation
  model. The durable issue now matches the implementation and explicitly
  forbids wall-clock activation.
- Confirmed repository base and `origin/develop` were identical at
  `84ec5408ea9423d7d3965f3088e80ec78456efcb`.

## Verification

- `scripts/tests/constraints.py`: 12 passed.
- `scripts/tests/pr-policy.sh`: passed.
- `scripts/tests/factory-contract.sh`: passed.
- `./scripts/factory check`: passed; 46 OpenSpec items including this change.
- `./scripts/factory research-ready define-explicit-constraint-governance`:
  passed.
- `./scripts/factory constraints-ready define-explicit-constraint-governance`:
  passed.
- `git diff --cached --check`: passed.
- Black, Taplo formatting/check, markdownlint, yamllint and EditorConfig:
  passed.
- `nix flake check --print-build-logs`: all 27 local checks passed, including
  587 workspace tests, Clippy, rustdoc, formatting, audit and policy gates.
- `./scripts/factory archive define-explicit-constraint-governance`: passed;
  the canonical `constraint-governance` capability was created with 5 added
  requirements and no destructive rewrite.
- Post-archive `./scripts/factory check` and `nix flake check`: passed with 46
  canonical capabilities and zero active changes.

The local Nix invocation reported `x86_64-linux` as incompatible with the
Darwin host and therefore did not run that system locally. Required GitHub CI
must supply the independent Linux and macOS integration evidence before merge.

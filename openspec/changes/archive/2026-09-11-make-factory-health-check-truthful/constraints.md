# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/257
Constraint blockers: none

## Existing entries affected

- `SDK-AGENT-001` and `SDK-AGENT-002`: routine factory correction remains
  within standing authority and preserves bounded execution.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005`: Rust 1.98.1 and the
  pinned active-development toolchain remain unchanged.
- `SDK-LIM-006`: release, publication and `main` remain inactive.

## Introduced or changed constraints

No SDK consumer constraint changes. The repository-local health-check contract
is strengthened so its full success includes the effective pinned runtime
audit rather than structural evidence alone.

## Introduced or changed limitations

A full audit outside Nix now pays one Nix devshell startup. Configuration-only
audit callers also use the pinned environment for one consistent public
contract. Offline use still requires the flake closure to be available.

## Consumer and product impact

Rust APIs, wire behavior, dependency cone, compiler floor, targets, features
and downstream repositories are unchanged. Maintainers receive earlier,
truthful failure when the runtime or local repository integration drifts.

## Activation and rollback

The behavior activates when the issue #257 PR merges to `develop`. Rollback is
a focused revert to the explicit bootstrap audit command; no cache, credential,
product or repository setting is migrated. `main` remains reserved.

## Evidence

Tests will prove outside-shell re-entry, in-shell single execution, argument
preservation and failure propagation. The live pinned audit and Pi version
smoke must pass with a clean worktree.

# Verification

Verification date: 2026-09-29

## Deterministic behavior

- `scripts/tests/repository-issue-verification.sh`: passed immediate success,
  transient recovery on attempt two, three-attempt exhaustion, immediate 404,
  pull-request rejection, and exact-URL mismatch rejection.
- `scripts/tests/pr-policy.sh`: passed; issue extraction and required metadata
  remain unchanged.
- A read-only live invocation verified repository issue #474 through the new
  REST boundary.

## Repository gates

- `scripts/tests/factory-contract.sh`: passed, including fixture execution of
  the new verifier contract.
- The Nix factory derivation declares `jq`, and its isolated contract build
  passes; failed positive fixtures print their captured diagnostic.
- `scripts/factory check`: passed.
- `shellcheck` for the implementation and test: passed in the pinned Nix
  development environment.
- `actionlint .github/workflows/pull-request-policy.yml`: passed in the pinned
  Nix development environment.
- Exact diff whitespace validation: passed.

## Compatibility

No crate source, public API, manifest, lockfile, dependency, toolchain, target,
protocol, or product behavior changed. Required PR issue linkage is unchanged;
only bounded recovery from classified provider failures is added.

## Remaining delivery evidence

The issue-linked PR must pass exact-head hosted policy, hygiene, DCO, and fast
checks. After protected merge, PR #472 can merge protected `develop` into its
head so the corrected base-owned policy runs on the new event.

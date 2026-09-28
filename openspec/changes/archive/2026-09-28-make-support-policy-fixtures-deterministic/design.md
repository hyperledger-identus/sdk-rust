# Design

## Explicit parser injection

`SupportPolicyTests` reads one dedicated environment variable containing the
absolute `nix-instantiate` path. Absence means parser evidence is not part of
that host invocation; it never falls back to `PATH`. The factory derivation
adds the nixpkgs-pinned Nix package and exports its exact executable path before
running the suite.

## Fixture classes

The suite provides separate helpers for expected parser acceptance and
expected parser rejection. The five indented-token mutations use the rejection
helper and still assert the checker's static fail-closed diagnostic. The
computed-import mutation binds `localModules` as a formal argument, uses the
acceptance helper, and still requires the unresolved-import diagnostic.

## Verification and rollback

Focused tests first prove all six repaired cases. The complete 210-test suite,
direct factory-contract shell, and pinned Nix factory-contract derivation prove
both execution modes. No production checker edit is expected. Rollback removes
the explicit injection and restores ambient-host variance.

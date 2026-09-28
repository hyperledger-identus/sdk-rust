# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/434
Constraint blockers: none

## Existing entries affected

The effective Nix toolchain, fast-lane, support-policy, and fail-closed factory
constraints remain unchanged.

## Introduced or changed constraints

- Required parser assertions use an explicit nixpkgs-pinned executable path.
- Direct host tests do not discover or depend on ambient Nix.
- Each mutation states whether valid Nix syntax is part of its evidence.
- Production support-policy rejection remains at least as strict as before.

## Introduced or changed limitations

Direct host execution without the explicit pinned parser path proves checker
behavior but does not claim Nix grammar evidence. The pinned Nix derivation is
the authoritative parse oracle.

## Consumer and product impact

None. No crate, target, protocol, wire, crypto, SSI, storage, FFI, release, or
downstream contract changes.

## Activation and rollback

Activation requires the issue-bound preflight, direct and pinned factory
tests, local review, signed/DCO PR, and green protected CI. Rollback is a normal
revert but restores the known nondeterminism.

## Evidence

The five invalid fixtures must be rejected by the pinned parser and checker;
the repaired computed-import fixture must parse and remain rejected by the
checker. The full direct host and pinned factory suites must pass.

# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/482
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-002` through `SDK-COMPAT-005`: Rust 1.89.0 and 1.98.1 retain the
  same packages/profiles/targets and both consume lockfile format v4.
- `SDK-DELIVERY-001`: the matrix stays weekly/manual slow evidence with one
  required Linux fast PR lane.
- `SDK-REL-001` and `SDK-REL-002`: DID remains candidate-only; the lock cannot
  publish or activate canonical versions.
- `SDK-LIM-003`: portable results remain compile-only evidence.

## Introduced or changed constraints

Every release-shaped DID candidate stage must copy one descriptor-bound,
checksum-verified repository lock before any `--locked` Cargo operation.
Matrix lanes may not generate or update dependency resolution. Intentional
manifest/dependency changes must refresh the staged lock and descriptor hash in
one reviewed change.

## Introduced or changed limitations

The staged lock governs repository candidate evidence only and does not force a
library consumer's transitive resolution after publication. Final cross-host
proof remains unavailable until #388 observes a natural or explicitly
authorized slow run.

## Consumer and product impact

No runtime or source consumer behavior changes. Engineers gain one inspectable
dependency snapshot behind archive, MSRV, host, and portable-target evidence.

## Activation and rollback

The lock, descriptor, builder, checker, mutations, and specification activate
atomically on protected `develop`. Rollback is a normal revert but restores the
known registry-timing risk and cannot be used as final M5 evidence.

## Evidence

Planning receipt, exact staged lock checksum, missing/modified/stale/bypass
mutations, both local compiler lanes, candidate build, factory/Nix gates,
review, and exact-head CI are required. Full cross-host slow execution remains
owned by #388.

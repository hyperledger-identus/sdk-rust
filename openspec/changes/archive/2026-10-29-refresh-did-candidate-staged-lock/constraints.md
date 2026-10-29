# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/581
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-002` through `SDK-COMPAT-005` retain Rust 1.89.0/1.98.1 and the
  existing package-specific host/portable matrix.
- `SDK-RELEASE-001` keeps canonical version `0.0.0` and publication denied.
- ADR 0155 continues to require one descriptor-bound lock in every lane.

## Introduced or changed constraints

The staged lock digest becomes
`f7535a6819c9b9e4474c95b427b54646d46ccac51703c680086ef75c790fba4f`.
Exactly the six declared patch replacements are permitted; any other drift
fails review and preparation.

## Introduced or changed limitations

The lock remains release-evidence input, not a promise about every downstream
consumer's transitive resolution. Final cross-host proof and approval remain
pending #388.

## Consumer and product impact

No source consumer or product behavior changes. Engineers regain reproducible
candidate preparation with an inspectable current dependency closure.

## Activation and rollback

The lock, descriptor digest, readiness references, tests, and canonical spec
activate atomically after exact-head CI. Revert restores the stale lock and
therefore cannot support final candidate evidence.

## Evidence

The refresh report, lock diff, preparation receipts, advisory/license/source
checks, API/SBOM outputs, target evidence, factory/Nix/site gates, review, and
protected CI form the evidence set.

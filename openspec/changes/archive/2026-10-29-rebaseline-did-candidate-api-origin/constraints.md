# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/582
Constraint blockers: none

## Existing entries affected

- `SDK-RELEASE-001` keeps canonical version `0.0.0` and publication denied.
- `SDK-COMPAT-002` through `SDK-COMPAT-005` retain the existing MSRV,
  etalon, host, and portable-target policy.
- ADR 0154 remains authoritative for the first-candidate API origin and
  `not-applicable` SemVer status.

## Introduced or changed constraints

Before first publication, an API-origin rebaseline requires an exact issue,
old/new generated API hashes, empty removal set, closed expected addition set,
immutable source revision, normal-mode candidate qualification, and independent
review. After publication, this exceptional rebaseline path is unavailable.

## Introduced or changed limitations

The refreshed snapshot remains a first-candidate origin, not a stable SemVer
promise or evidence against a released predecessor. Final cross-host slow
evidence and independent release approval remain pending #388.

## Consumer and product impact

None. The Rust API and behavior are already present on protected `develop`;
this slice changes release evidence only. No consumer or product repository is
mutated.

## Activation and rollback

The exact receipt, baseline, candidate knowledge, checker/mutations, canonical
specification, review, and protected CI activate atomically. Revert restores
the stale origin and intentionally blocks normal candidate qualification.

## Evidence

The generated API set difference, API-origin receipt, normal candidate receipt,
matrix lanes, repository security/dependency gates, docs/factory/Nix checks,
independent review, and protected CI form the evidence set.

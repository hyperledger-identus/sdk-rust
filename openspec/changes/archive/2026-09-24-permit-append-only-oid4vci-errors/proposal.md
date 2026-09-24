# Permit Append-Only OID4VCI Error Contract Evolution

## Why

Issue #346 removes an accidental factory dead end discovered by issue #345.
The immutable 171-row v1 golden correctly freezes the pre-refactor public
contract, but the crate-local inventory currently treats that historical
snapshot as a permanent ceiling. A legitimate additive fieldless error cannot
compile even when every baseline row remains exact.

## What changes

- Define the v1 golden as the exact immutable prefix of the live exhaustive
  `CredentialOfferError` inventory.
- Require every future variant and router entry to append after that prefix.
- Keep uniqueness and wildcard-free exhaustive routing over the full live
  inventory.
- Preserve the v1 fixture, hash, provenance and all 171 contracts unchanged.
- Add ADR 0137 and focused regression evidence.

## What does not change

No error variant, constant, code, kind, message, public API, protocol behavior,
dependency, feature, wire model, release or consumer is added or changed.

## Capabilities

### Modified capabilities

- `oid4vci-error-contracts`: distinguish immutable historical compatibility
  evidence from append-only live evolution.

## Authority

Issue #346, ADR 0119 and the standing SDK delivery mandate.

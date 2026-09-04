# Add presentation protocol lifecycle state

## Why

`identus-presentations` now owns bounded requests, disclosure plans, generated
artifacts and receipt inputs, but protocol coordinators still lack a shared way
to describe progress and terminal outcomes. Oxid currently carries a useful
state vocabulary locally, while OID4VP and Midnight adapters would otherwise
invent separate, ambiguously terminal state machines.

## What changes

- Add distinct active presentation lifecycle phases and terminal outcomes.
- Add one protocol-state value with stable lowercase spellings and strict
  parsing without a serializer dependency.
- Add a deterministic transition guard that prevents backward progress,
  impossible phase skips and terminal mutation.
- Represent cancellation honestly when irreversible completion races a cancel
  request.
- Extend static redacted errors, inventories and a release-mode transition
  diagnostic for the new contract.

## Non-goals

This change does not discover candidates; choose disclosures; decide consent,
authorization or trust; generate or verify proofs; define protocol wire data;
send artifacts; acknowledge verifier acceptance; store sessions; add clocks,
retry, error detail, audit policy, FFI, chain or product behavior; publish the
crate; or modify a downstream repository.

## Impact

- **Issue:** #85, under `IDR-008`, #20 and predecessor #83.
- **Owner:** existing experimental `identus-presentations` crate.
- **Compatibility:** additive unreleased API; no serialized form or stored
  state migration.
- **Dependencies:** no new crate, external package or feature edge.
- **Rollback:** revert the focused change before publication.

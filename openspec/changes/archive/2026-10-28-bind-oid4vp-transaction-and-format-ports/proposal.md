# Bind OID4VP transaction data and credential-format ports

## Why

`identus-oid4vp` currently rejects `transaction_data` and accepts only empty
DCQL credential metadata. That prevents the Final protocol core from carrying
authorization context into presentation creation and forces format-specific
metadata policy either into adapters or into the protocol crate.

## What changes

- Parse and retain exact bounded `transaction_data` strings, their common
  `type`/`credential_ids` linkage, and their relationship to selected DCQL inputs.
- Introduce SDK-owned credential-format identifiers and injected capability
  ports for validating DCQL metadata and transaction-data proof binding.
- Keep SD-JWT VC, mdoc, W3C VC, and extension proof engines in their owning
  crates while rejecting unknown mandatory transaction semantics.
- Add clean-room generic, SD-JWT-shaped, mdoc-shaped, malformed, oversized,
  redaction, adapter, property, fuzz, benchmark, and full-lineage E2E evidence.

## Added capability

- `oid4vp-transaction-and-format-ports`: bounded request-to-presentation
  transaction linkage and credential-format capability negotiation.

## Modified capabilities

- `oid4vp-dcql-query`: accept format metadata only through an exact registered
  capability rather than requiring every metadata object to be empty.
- `oid4vp-authorization-request-routing`: retain validated transaction linkage
  in the consumed authorization state.

## Non-goals

No SD-JWT VC, mdoc, W3C VC/Data Integrity, HAIP, trust, consent, presentation
proof creation/verification, HTTP, persistence, downstream mutation, or foreign
SDK DTO/error compatibility lands here.

## Delivery

Issue #547 owns the final SDK-only child of #541 and IDR-024. The slice remains
chain-agnostic, effect-neutral, portable, and unpublished.

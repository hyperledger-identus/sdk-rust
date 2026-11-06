# Polish SIOPv2 lifecycle diagnostics and local helpers

## Why

The first bounded `identus-siopv2` slice deliberately kept effects outside the
crate, but its lifecycle specification does not say whether idempotency lasts
only in the current state or requires retained cross-transition history. Two
failure paths are also too coarse: retrieved `wallet_nonce` mismatch is reported
as client binding, and unexpected JOSE response construction failures are
reported as response size. These ambiguities make adopter recovery and E2E
diagnostics less reliable.

## What changes

- Specify current-state idempotency and leave durable, cross-transition event
  history to caller-owned persistence.
- Preserve `outcome_unknown` as terminal evidence that never authorizes an
  automatic retransmission.
- Add stable redacted categories for wallet-nonce correlation and response
  encoding failures while retaining the existing public error type.
- Simplify repeated private string validation without changing validation order
  or resource bounds.
- Document why compact JWS parsing remains a deliberate two-pass operation.
- Retain protocol-private URI and form validation instead of exposing a generic
  helper that could bypass protocol-specific invariants.
- Strengthen focused, property, conformance, and E2E evidence for the clarified
  contract and replace the canonical specification's placeholder purpose.

## Modified capability

- `siopv2-protocol-core`: clarify lifecycle/error ownership and improve stable
  diagnostics within the existing draft-13 DID-subject profile.

## Non-goals

No new SIOPv2 profile features, transport, persistence, replay database, trust,
custody, DID resolution, UI, response URI, claims, public generic URI/form API,
JOSE ownership redesign, dependency, or downstream consumer mutation.

## Delivery

Issue #543 owns this SDK-only slice. The existing experimental capability stays
unpublished and keeps the same wire format and portable target contract.

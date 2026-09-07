# ADR 0060: parse the OID4VCI deferred Credential Response body

- **Status:** Accepted for implementation
- **Date:** 2026-09-07
- **Decision authority:** standing product mandate and IDR-023 issue #149
- **Related work:** issues #7, #20, #141, #143, #145, #147, and #149

## Context

The SDK parses the immediate OID4VCI Final Credential Response branch but
classifies the mutually exclusive deferred branch as unsupported. The Final
deferred body requires a transaction identifier and a positive JSON-number
interval. Reusable body semantics are needed before HTTP 202 binding or any
polling lifecycle can be designed.

## Decision

1. Add independent positive limits for body bytes, JSON depth and nodes,
   top-level members, decoded transaction-identifier bytes, and interval
   lexeme bytes.
2. Parse exactly one duplicate-safe object requiring a non-empty string
   `transaction_id` and mathematically positive JSON-number `interval`.
3. Preserve the decoded identifier and exact interval lexeme in zeroizing
   ownership without integer or floating-point conversion.
4. Reject `credentials` and `notification_id` as immediate-branch conflicts;
   traverse and discard unique unknown extensions under aggregate limits.
5. Expose only explicit accessors and keep Debug and all errors free of remote
   values.
6. Keep HTTP 202/media binding, request correlation, transaction validity,
   Deferred Credential Requests, polling and scheduling outside this body core.

## Consequences

- Headless consumers gain a bounded representation of the standards-defined
  deferred branch without adopting an HTTP client, clock, or retry policy.
- Exact numeric spelling remains available for a later policy layer, including
  fractional and exponent forms, without overflow or rounding.
- The type proves only local body syntax and resource bounds; it does not prove
  issuer provenance, transaction validity, authorization, freshness, or safe
  use of the identifier.
- HTTP response validation, request binding, deferred polling, encrypted
  responses, credential verification/storage, consumer adoption, publication,
  and release remain issue-first follow-up work.

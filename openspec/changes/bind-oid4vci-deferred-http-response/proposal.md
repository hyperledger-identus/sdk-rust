# Bind OID4VCI Deferred HTTP Responses to Their Request

## Why

Issue #345 advances IDR-023 after #250 constructed the bounded unencrypted
Deferred Credential Request. The request currently discards the typed
transaction handle after serialization, so a headless wallet cannot validate
the Final response status, media type and returned transaction identifier as
one request-bound state transition.

## What changes

- Retain the originating transaction identifier privately inside
  `DeferredCredentialRequest` with zeroizing ownership and redacted diagnostics.
- Add positive HTTP Content-Type limits composed with the existing immediate
  and deferred response body limits.
- Validate an unencrypted response as either an issued outcome (`200`) or a
  still-pending outcome (`202`).
- Require the transaction identifier in a `202` response to equal the one sent
  by the request.
- Reuse the existing bounded Final response parsers and JSON media-type rules.
- Add exact status, media-type, correlation, boundary, redaction and
  compatibility tests.
- Add ADR 0138 and advance the canonical IDR-023 pointer to issue #345.

## What does not change

No HTTP execution, bearer-token ownership, TLS configuration, interval
scheduling, retry, terminal transaction invalidation, error response parsing,
encrypted response processing, credential verification/storage, proof-count
claim, trust, downstream mutation, publication, release or product policy is
added.

## Capabilities

### New capabilities

- `oid4vci-deferred-credential-http-response`: bounded request-bound validation
  of unencrypted Final Deferred Credential Endpoint responses.

### Modified capabilities

- `ssi-upstream-program`: advance IDR-023 from component epic #7 to focused
  child #345 while retaining `in_progress` status.
- `oid4vci-error-contracts`: append five independently tested fieldless
  response-validation errors under ADR 0137 without changing the v1 prefix.

## Authority

Issue #345, OpenID4VCI 1.0 Final section 9.2, and the standing SDK delivery
mandate.

# Bind OID4VCI Deferred Credential Error Responses

## Why

Issue #345 delivered request-bound successful Deferred Credential Endpoint
responses, but callers still have to reinterpret a generic Credential Error
Response to recognize the additional Final `invalid_transaction_id` code and
the explicit stop-polling guidance for `credential_request_denied`. Duplicating
that logic in wallets would fragment lifecycle semantics and error handling.

## What changes

- Validate an unencrypted payload-error response through the originating
  `DeferredCredentialRequest` using the existing bounded Credential Error HTTP
  parser.
- Add a deferred-specific typed classification for
  `invalid_transaction_id`, `credential_request_denied`, and other inherited
  Credential Request errors.
- Expose whether Final section 9.3 explicitly directs the wallet to stop
  polling, without performing that action.
- Preserve exact bounded error codes and deliberately exposed untrusted
  descriptions through the existing response core.
- Add exact status, media-type, classification, boundary, repeatability and
  redaction tests.
- Add ADR 0139 and hand the live IDR-023 pointer from delivered #348 to focused
  authorization-code server-binding successor #350.

## What does not change

No HTTP execution, RFC 6750 challenge parsing, bearer-token ownership, TLS,
timer, retry, backoff, cancellation, transaction invalidation, persistence,
encrypted response processing, credential verification, trust, downstream
mutation, publication, release or product policy is added.

## Capabilities

### New capabilities

- `oid4vci-deferred-credential-error-http-response`: request-bound bounded
  validation and deferred-specific classification of Final payload errors.

### Modified capabilities

- `ssi-upstream-program`: record #348 as delivered and advance IDR-023 to
  focused successor #350 while preserving `in_progress` status and all
  engine-level non-completion boundaries.

## Authority

Issue #348, OpenID4VCI 1.0 Final sections 8.3.1 and 9.3, and the standing SDK
delivery mandate.

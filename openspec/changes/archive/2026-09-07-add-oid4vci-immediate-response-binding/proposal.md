# Change: bind an immediate OID4VCI response to its JWT request

## Why

Issue #143 advances IDR-023 after the SDK gained a bounded Final JWT
Credential Request and immediate Credential Response body core. A headless
wallet still has to validate the immediate HTTP success envelope and enforce
the response-count upper bound against the request before format-specific
verification.

## What Changes

- Add positive limits for the effective Content-Type field and existing
  immediate-response body/parser limits.
- Validate caller-supplied response metadata through `JwtCredentialRequest`:
  exact HTTP 200 and RFC-shaped case-insensitive `application/json`.
- Parse the body through `ImmediateCredentialResponseCore` only after metadata
  validation and enforce that credential count does not exceed request proof
  count.
- Return an owned request-bound response state that preserves the existing
  sensitive/redacted response contract and records only the non-secret request
  proof count.
- Extract the existing private HTTP field grammar for reuse without changing
  the public Nonce HTTP contract.
- Add fieldless errors plus status, media, boundary, ordering, cardinality,
  consumer-shaped and redaction tests.
- Record the public/wire/standard decision in ADR 0057 and advance the
  in-progress IDR-023 ledger pointer from completed child #141 to active child
  #143.

This change does not execute HTTP, require Cache-Control, accept error,
deferred or encrypted responses, prove distinct proof keys or credential-key
binding, interpret credentials, execute notifications, store credentials, or
modify a consumer.

## Capabilities

### New Capabilities

- `oid4vci-immediate-credential-http-response`: bounded HTTP metadata and
  necessary request-cardinality validation for an immediate Credential
  Response.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #141 to #143.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive HTTP limits and request-bound response state plus one
  method on `JwtCredentialRequest`.
- Wire behavior: exact HTTP 200, `application/json`, existing strict body
  parsing, and `credential_count <= proof_count`.
- Errors: additive fieldless `oid4vci.*` diagnostics.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only.
- Rollback: revert one unpublished `develop` change before downstream adoption.

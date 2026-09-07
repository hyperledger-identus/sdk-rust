# Change: validate a bounded OID4VCI Credential payload-error HTTP response

## Why

Issue #147 advances IDR-023 after the SDK gained a bounded Credential Error
Response body core. A headless holder still needs to distinguish the Final
Credential Request payload-error envelope from success, deferred issuance,
and RFC 6750 Authorization Error Responses without importing an HTTP client or
product recovery policy.

## What Changes

- Add positive composed limits for the existing body parser and the effective
  Content-Type field value.
- Accept exactly status 400 and one bounded RFC-shaped `application/json`
  media value before parsing the body through `CredentialErrorResponseCore`.
- Reject the generic exact `invalid_request` code at this payload-error layer,
  while preserving every other syntactically valid extension code.
- Retain neither status nor media input, preserve the existing zeroizing body
  state, and keep every new diagnostic fieldless and data-free.
- Add positive, negative, ordering, boundary, extension, redaction, bridge,
  and consumer-shaped tests.
- Record the wire interpretation in ADR 0059 and advance the in-progress
  IDR-023 ledger pointer from completed child #145 to active child #147.

This change does not parse Authorization Error Responses, require the example
`Cache-Control: no-store`, execute HTTP, establish response provenance or
request correlation, decide retry or remediation, process deferred/encrypted
responses, verify credentials, or modify a consumer.

## Capabilities

### New Capabilities

- `oid4vci-credential-error-http-response`: bounded Final HTTP envelope
  validation for a Credential Request payload-error body.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #145 to #147.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive composed limits and an associated HTTP-response parser
  returning the existing bounded body core.
- Wire behavior: exact status 400, strict `application/json`, explicit generic
  `invalid_request` exclusion, and extension-compatible body parsing.
- Errors: additive fieldless `oid4vci.*` diagnostics.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only.
- Rollback: revert one unpublished `develop` change before adoption.

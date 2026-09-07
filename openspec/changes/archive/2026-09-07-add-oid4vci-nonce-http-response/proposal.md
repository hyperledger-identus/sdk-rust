# Change: validate the OID4VCI Final Credential Nonce HTTP response

## Why

Issue #137 advances IDR-023 after the SDK gained both a transport-neutral
Final Nonce Request and a bounded Nonce Response body parser. A headless wallet
still needs one policy-neutral transition that rejects a response unless its
HTTP status, media type and uncacheability metadata satisfy Final section 7.2.

## What Changes

- Add explicit positive bounds for caller-supplied Content-Type and
  Cache-Control field values alongside the existing body limits.
- Add a response-validation transition on `CredentialNonceRequest` that checks
  successful status, `application/json`, an unqualified `no-store` directive,
  and the existing bounded JSON body.
- Parse HTTP tokens, parameters, list elements and quoted strings without naive
  substring matching, retaining no response metadata or raw body.
- Add fieldless static transport errors and complete boundary, malformed,
  injection, interoperability and redaction tests.
- Preserve the dependency cone, request/body semantics, target matrix and
  read-only consumer boundary.
- Advance the in-progress IDR-023 ledger pointer from completed child #135 to
  active child #137.

This change does not execute HTTP, establish network or issuer trust, process
DPoP, manage nonce lifecycle, construct proofs or Credential Requests, or
change a consumer repository.

## Capabilities

### New Capabilities

- `oid4vci-nonce-http-response`: bounded validation of the mandatory Final
  Credential Nonce HTTP response envelope before body semantics are exposed.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #135 to #137.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive response limits and one request-bound validation method.
- Wire behavior: additive validation only; no serialization or network access.
- Errors: additive fieldless `oid4vci.*` transport diagnostics.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only.
- Rollback: revert one unpublished `develop` change before downstream adoption.

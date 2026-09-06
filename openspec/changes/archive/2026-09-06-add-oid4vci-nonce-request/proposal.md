# Change: add the OID4VCI Final Credential Nonce Request

## Why

Issue #135 advances IDR-023 after issuer metadata gained the optional Final
Nonce Endpoint through #133. A headless wallet needs a typed request boundary
that binds that validated endpoint to the exact unprotected POST operation
before transport and response handling can remain separately injectable.

## What changes

- Add an owned, redaction-safe `CredentialNonceRequest` constructed only from
  issuer metadata that advertises a validated `NonceEndpoint`.
- Expose static transport guidance for POST, an empty request body, and the
  absence of an access-token requirement.
- Fail construction from metadata without the optional endpoint through one
  fieldless stable error.
- Preserve the existing dependency cone, metadata parser, request limits,
  target matrix and read-only consumer boundary.
- Advance the in-progress IDR-023 ledger pointer from completed child #133 to
  active child #135.

This change does not execute HTTP, define network policy, validate response
status or headers, manage nonce lifecycle, construct proofs or Credential
Requests, or change a consumer repository.

## Capabilities

### New capabilities

- `oid4vci-nonce-request`: exact, least-authority, transport-neutral Final
  Credential Nonce Request construction.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #133 to #135.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive request value, constructor, constants and one error.
- Wire behavior: no parser change; the SDK-defined request body is empty.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream adoption.

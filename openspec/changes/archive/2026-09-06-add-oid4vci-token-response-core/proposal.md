# Change: parse a bounded OID4VCI successful Token Response core

## Why

Issue #127 advances IDR-023 after mandatory Pre-Authorized Token Request
construction landed in #125. Headless consumers can now send exact request
bytes, but the SDK has no bounded, secret-safe value for the OAuth core of a
successful Token Response.

## What changes

- Add a bounded partial Token Response parser for the RFC 6749 core used by
  OID4VCI Final.
- Validate required access-token and token-type syntax plus optional expiry,
  refresh-token, and scope syntax.
- Retain the exact response only in zeroizing private storage, ignore unknown
  members semantically, and expose raw tokens or scope only through explicitly
  sensitive accessors.
- Record only whether OID4VCI `authorization_details` is present; its contents
  remain unvalidated until a later consuming transition.
- Advance the in-progress IDR-023 ledger pointer from completed child #125 to
  active child #127.

This change does not execute HTTP, parse Token Error Responses, validate
Authorization Details, establish token trust, manage replay/refresh, construct
Credential Requests, or change a consumer repository.

## Capabilities

### New capabilities

- `oid4vci-token-response-core`: bounded, partial, redaction-safe parsing of a
  successful OAuth Token Response core for OID4VCI.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #125 to #127.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive response limits, token type, and partial response core.
- Wire behavior: additive strict validation of known RFC 6749 response fields;
  unknown fields remain semantically ignored.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream
  adoption.

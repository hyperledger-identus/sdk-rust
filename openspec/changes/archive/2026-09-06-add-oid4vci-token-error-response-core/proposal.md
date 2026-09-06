# Change: parse a bounded OID4VCI Token Error Response core

## Why

Issue #129 advances IDR-023 after the successful Token Response core landed in
#127. Headless consumers can parse success but have no bounded reusable value
for the OAuth Token Error Response required by OID4VCI Final.

## What changes

- Add strict bounded parsing of the RFC 6749 Token Error Response object.
- Preserve exact error codes while classifying the six standard token-endpoint
  codes without closing OAuth's extension registry.
- Validate optional developer description and URI-reference syntax under
  independent limits and expose them only through explicitly untrusted APIs.
- Structurally validate and semantically ignore bounded unknown members.
- Advance the in-progress IDR-023 ledger pointer from completed child #127 to
  active child #129.

This change does not execute HTTP, validate status or headers, correlate a
request, make retry/trust/UI decisions, follow an error URI, parse a successful
response, construct Credential Requests, or change a consumer repository.

## Capabilities

### New capabilities

- `oid4vci-token-error-response-core`: bounded, open-registry, redaction-safe
  parsing of an OAuth Token Error Response for OID4VCI.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #127 to #129.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive error-response limits, open code, closed classification,
  URI-reference, and partial response core.
- Wire behavior: additive strict validation of known RFC 6749 error fields;
  unknown fields remain semantically ignored.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream adoption.

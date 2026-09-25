# Correlate OID4VCI Token Authorization Details

## Why

Issue #360 binds a bounded successful Token Endpoint response to the exact
Authorization Code request lineage, while the existing
`TokenResponseWithAuthorizationDetails` validates response-local syntax only.
A consumer can currently detach that response from its request and select a
dataset belonging to an unrequested or merely offered configuration. The SDK
needs one consuming boundary that converts syntactically valid response data
into authority for exactly the configuration selected before authorization.

## What changes

- Consume only a request-bound Authorization Code Token success.
- Reuse the existing bounded Authorization Details parser and require exactly
  one recognized `openid_credential` entry for the exact selected Credential
  Configuration.
- Reject any recognized mismatched configuration before duplicate-entry
  ambiguity; reject repeated matching entries separately.
- Return a redacted correlated success retaining the exact public lineage,
  secret-bearing Token Response core, source-ordered authorized Credential
  Dataset identifiers and bounded count of unknown detail types.
- Preserve the existing response-local public parser for unpublished
  compatibility while adding only private decomposition needed by correlation.
- Append static diagnostics, ADR 0146, architecture evidence and focused
  successor #364.

## What does not change

No HTTP, token validation/storage, DPoP, client authentication, retry policy,
proof creation, Credential Request construction, product dataset selection,
trust, consumer, chain, release or publication behavior is added.

## Capabilities

### New capabilities

- `oid4vci-authorization-code-token-correlation`: one-shot correlation of
  response-authorized datasets to exact Authorization Code request lineage.

### Modified capabilities

- `oid4vci-authorization-code-token-response`: make successful response
  correlation the typed consuming continuation.
- `oid4vci-token-authorization-details`: permit private decomposition without
  changing response-local public validation behavior.
- `oid4vci-error-contracts`: append mismatch and ambiguity diagnostics without
  changing prior entries.
- `ssi-upstream-program`: retain IDR-023 in progress under #364 after #362.

## Authority

Issue #362, OpenID4VCI 1.0 Final sections 3.3.4, 5.1.1 and 6.2, RFC 9396
sections 6 and 7, ADR 0145, and the standing SDK delivery mandate.

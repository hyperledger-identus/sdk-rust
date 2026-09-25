# Bind the OID4VCI Pre-Authorized Token Response

## Why

Issue #375 is the only required wallet-core row still missing from the Final
conformance matrix. `PreAuthorizedTokenRequest` constructs the mandatory
secret-bearing request, while the existing response cores parse bounded OAuth
success and error payloads. Consumers still have to join those surfaces and
can lose the originating offer/server authority, retain request secrets, or
select a parser from attacker-controlled body shape.

## What changes

- Retain the exact public Credential Issuer Metadata, selected Authorization
  Server Metadata and ordered offered Credential Configuration IDs when the
  pre-authorized request is constructed.
- Consume one request to classify exact HTTP `200` success or `400` OAuth
  error before headers and body, erasing the zeroizing request body first.
- Require bounded `application/json`, `Cache-Control` containing bare
  `no-store`, and `Pragma` containing bare `no-cache`.
- Reuse the existing bounded `TokenResponseCore` and
  `TokenErrorResponseCore`; accept the Final pre-authorized
  `invalid_request` and `invalid_grant` semantics through that shared core.
- Return an exclusive response outcome attached to non-secret request
  lineage, with static redacted diagnostics and negative precedence tests.
- Update the machine-checked Final matrix and hand IDR-023 to #376 without
  claiming complete interoperability evidence.

## What does not change

No HTTP execution, response provenance, client authentication, DPoP, token
trust or storage, refresh flow, retries, Credential Request construction,
consumer mutation, publication, chain or product behavior is added.

## Capabilities

### New capabilities

- `oid4vci-pre-authorized-token-response`: bounded one-shot Token Endpoint
  response binding for the Pre-Authorized Code flow.

### Modified capabilities

- `oid4vci-error-contracts`: append pre-authorized response-envelope errors.
- `ssi-upstream-program`: advance IDR-023 to the remaining vector issue #376.

## Authority

Issue #375, OpenID4VCI 1.0 Final sections 6.2 and 6.3, RFC 6749 sections 5.1
and 5.2, and the standing SDK delivery mandate.

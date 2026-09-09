# Construct the OID4VCI Final Deferred Credential Request

## Why

Issue #250 advances IDR-023 after #241 exposed the optional advertised
Deferred Credential Endpoint. The SDK already parses a bounded deferred
response transaction handle, but a headless wallet still has to rebuild the
Final request body and transport metadata outside the typed protocol boundary.

## What changes

- Add a bounded unencrypted `DeferredCredentialRequest` containing the
  advertised endpoint and deterministic JSON request body.
- Construct it from a validated `DeferredCredentialResponseCore` plus matched
  `CredentialIssuerMetadata`.
- Add a positive request-body limit and enforce it while JSON is serialized.
- Expose the fixed POST method, `application/json` media type, body length,
  access-token requirement and an explicitly sensitive body accessor.
- Add static endpoint-required, invalid-limits and request-too-large errors.
- Add exact Final, escaping, boundary, redaction and compatibility tests.
- Add ADR 0109 and advance the canonical IDR-023 pointer to #250.

## What does not change

No HTTP execution, bearer-token retention, TLS configuration, interval
scheduling, transaction lifecycle, encrypted request/response, extension
parameters, error response parsing, response correlation, trust, storage,
downstream mutation, publication, release or product policy is added.

## Capabilities

### New capabilities

- `oid4vci-deferred-credential-request`: bounded unencrypted Final Deferred
  Credential Request construction.

### Modified capabilities

- `ssi-upstream-program`: advance IDR-023 from completed child #241 to active
  child #250 while retaining `in_progress` status.

## Authority

Issue #250, OpenID4VCI 1.0 Final section 9.1, and the standing SDK delivery
mandate.

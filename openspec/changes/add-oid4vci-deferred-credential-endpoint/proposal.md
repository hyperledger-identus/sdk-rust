# Expose the OID4VCI Final Deferred Credential Endpoint

## Why

Issue #241 advances IDR-023 after the SDK added the deferred response core and
both Final Credential Request selector branches. A headless wallet cannot
construct a typed Deferred Credential Request until validated Credential Issuer
Metadata exposes the issuer's optional `deferred_credential_endpoint`.

## What changes

- Recognize the optional Final metadata member in the existing bounded parser.
- Add a redaction-safe `DeferredCredentialEndpoint` and optional accessor.
- Reuse the independent shared endpoint byte budget and HTTPS validator.
- Add field-specific stable oversize and unsafe-value errors.
- Add exact, omission, type, syntax, duplicate, boundary and redaction tests.
- Add ADR 0107 and advance the canonical IDR-023 pointer to #241.

## What does not change

No Deferred Credential Request, HTTP, polling, scheduling, transaction
lifecycle, token validation, encryption, trust, storage, downstream mutation,
publication, release or product policy is added.

## Capabilities

### New capabilities

- `oid4vci-deferred-credential-endpoint`: optional exact bounded Final Deferred
  Credential Endpoint discovery.

### Modified capabilities

- `ssi-upstream-program`: advance IDR-023 from #239 to #241 while retaining
  `in_progress` status.

## Authority

Issue #241, OpenID4VCI 1.0 Final sections 9 and 12.2.4, and the standing SDK
delivery mandate.

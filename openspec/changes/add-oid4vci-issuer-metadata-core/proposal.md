# Add OID4VCI Credential Issuer Metadata core

## Why

Issues #111, #113, and #115 established bounded Credential Offer transport,
core semantics, and grant shapes. The next protocol decision needs both the
offer and the Credential Issuer Metadata: the wallet must reject cross-issuer
metadata, missing offered configurations, and invalid Authorization Server
hints before selecting or executing a grant.

## What changes

- Add a bounded unsigned JSON Credential Issuer Metadata state with exact
  issuer matching, safe endpoint/Authorization Server syntax, and opaque
  format-aware configuration summaries.
- Add a consuming transition from `CredentialOfferWithGrants` to an
  offer-with-metadata state that proves issuer, offered-configuration, and
  grant-hint agreement without selecting a grant.
- Preserve unknown metadata and configuration members in exact retained JSON.
- Keep metadata acquisition, signed metadata, trust, format interpretation,
  OAuth server discovery, and protocol execution outside this slice.

## Non-goals

This change does not perform HTTP/TLS fetching, redirect or SSRF policy,
Content-Type processing, signed-metadata/JWS verification, trust evaluation,
OAuth Authorization Server metadata parsing, grant support/selection,
format-profile validation, display/claims parsing, endpoint invocation,
authorization/token/nonce/credential messages, replay controls, persistence,
consumer edits, publication, release, or promotion to `main`.

## Impact

- **Issue:** #117, child of #7 and #20 / `IDR-023`.
- **Owner:** existing unpublished `identus-oid4vci` crate.
- **Compatibility:** additive experimental API; no released compatibility
  promise and no wire output.
- **Dependencies:** unchanged normal dependency cone; no async, HTTP, crypto,
  DID, storage, chain, platform, or product dependency.
- **Rollback:** remove the additive metadata/matched states and restore #115's
  grant-validated terminal state.

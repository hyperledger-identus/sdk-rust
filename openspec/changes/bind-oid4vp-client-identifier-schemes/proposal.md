# Bind OID4VP client identifier schemes and verifier metadata

## Why

`identus-oid4vp` currently proves JWS validity, client-id correlation, audience,
freshness, and replay acceptance, but intentionally stops before proving that
the exact verification key is authorized for the signed client identifier.
OpenID4VP Final section 5.9 assigns different evidence and metadata rules to
each client identifier prefix. Leaving those rules implicit invites adapters to
confuse a valid signature with verifier trust or to strip the prefix from the
audience.

## What changes

- Classify the first-colon client identifier syntax and every Final prefix.
- Reject the unsigned `redirect_uri` and DC-API-only `origin` prefixes in this
  signed, non-DC-API pipeline; reject unknown prefixes by default.
- Add one consuming verifier-authorization transition that supplies the exact
  verified key, full client identifier, protected header, and bounded
  scheme-specific evidence to a caller-owned trust port.
- Parse applicable payload `client_metadata` and `verifier_info` plus
  protected-header `trust_chain` under explicit limits, including DCQL
  credential-id references and rejection of payload trust-chain substitution.
- Preserve existing `direct_post.jwt` recipient selection as a separate,
  narrowly claimed metadata consumer.
- Add clean-room conformance, property, E2E, fuzz, benchmark, redaction, and
  portable evidence plus adopter guidance.

## Modified capability

- `oid4vp-request-object-jar`: require verifier authorization evidence after
  request security and before typed authorization routing.

## Non-goals

No bundled DID resolver, PKI/DNS validation, federation engine, attestation
issuer store, HTTP, persistence, product allow/deny or consent policy, Wallet
Metadata publication, Digital Credentials API, or foreign SDK DTO/error shape.

## Delivery

Issue #545 owns this SDK-only slice under #541 and IDR-024. The implementation
must remain chain-agnostic, portable, bounded, and releasable without a runtime.

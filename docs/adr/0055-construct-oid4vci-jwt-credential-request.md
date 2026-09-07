# ADR 0055: construct the Final JWT Credential Request from validated states

- **Status:** Accepted for issue #139
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #8, #20, #99, #117, #127, #139; OpenID4VCI
  1.0 Final section 8.2; RFC 6750 section 2.1

## Context

The SDK already validates and joins a Credential Offer with Credential Issuer
Metadata, parses the successful Token Response core, and constructs
holder-produced OID4VCI proof JWTs. Headless consumers still manually select an
offered configuration, join its endpoint/token/proof data, and serialize the
protected Credential Request.

The current Token Response parser deliberately treats `authorization_details`
as opaque presence. Final requires the mutually exclusive
`credential_identifier` route when Credential identifiers are returned there.
The metadata core also retains proof capabilities opaquely. The first request
constructor therefore needs a narrow fail-closed profile rather than guessing
at either opaque structure.

## Decision

1. Add the architecture-approved `identus-oid4vci -> identus-jose` dependency
   and accept only holder-produced `Oid4vciProofJwt` inputs.
2. Construct through `CredentialOfferWithMetadata`, selecting a configuration
   by offered index so a raw caller string cannot introduce an unoffered ID.
3. Support only Token Responses without Authorization Details, non-empty JWT
   proof lists, and case-insensitive Bearer token types whose exact token
   matches RFC 6750 `b64token` syntax.
4. Stream deterministic JSON through a byte-bounded serde writer directly into
   zeroizing storage. Own the canonical Bearer Authorization field value and
   duplicate the already-validated Credential Endpoint.
5. Expose only static method/media guidance, endpoint/count/length metadata and
   explicitly sensitive authorization/body accessors. Keep all errors
   fieldless and diagnostics static.

## Consequences

- Headless adapters can transport the interoperable Final
  `credential_configuration_id` plus `proofs.jwt` request without constructing
  JSON or Authorization values themselves.
- The request is strong local composition evidence, not proof of endpoint or
  token trust, proof audience/nonce/freshness/replay correctness, actual HTTP
  execution, or response validity.
- Authorization Details/Credential identifiers, proofless and non-JWT paths,
  format/chain extensions, encryption and Credential Response handling remain
  separately contracted slices.
- The additive unpublished API changes no external dependency version,
  feature, target, consumer, release or `main` state.

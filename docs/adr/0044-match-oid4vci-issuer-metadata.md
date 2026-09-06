# ADR 0044: Match bounded OID4VCI issuer metadata before flow selection

- **Status:** accepted
- **Date:** 2026-09-06
- **Related work:** issues #7, #20, #111, #113, #115, and #117; `IDR-023`

## Context

The OID4VCI crate already distinguishes raw Credential Offer transport, core
offer semantics, and known grant shapes. A grant's optional Authorization
Server identifier cannot be used safely from syntax alone: the Final protocol
requires exact agreement with Credential Issuer Metadata and permits the hint
only when multiple servers are advertised. Offered configuration identifiers
also have meaning only when metadata contains matching entries.

Metadata retrieval combines several authorities—network access, redirects,
TLS, media types, optional signatures, signer trust, and cache policy. Moving
those authorities into a reusable semantic crate would couple the SDK to a
runtime and product policy before the wire contract is stable.

## Decision

Add two explicit, consuming semantic stages:

1. `CredentialIssuerMetadata::parse` validates bounded unsigned JSON against
   an expected Credential Issuer Identifier. It retains exact JSON and exposes
   only the issuer, advertised/effective Authorization Servers, Credential
   Endpoint, and configuration ID plus opaque format summaries.
2. `CredentialOfferWithGrants::try_with_metadata` consumes both validated
   inputs and proves exact issuer equality, presence of every offered
   configuration, and the Final multiple-server rule for each grant hint.

No grant or server is selected by this transition. The metadata state does not
claim retrieval provenance, endpoint reachability, signer trust, format
support, or server capability. Unknown members remain in exact zeroizing JSON.
Every parsed string and collection is explicitly bounded; duplicate decoded
JSON names fail before semantic use; diagnostics remain static and redacted.

## Consequences

- Wallet and issuer consumers can share one deterministic cross-document
  agreement boundary before implementing their own I/O and trust policy.
- Omitted `authorization_servers` has an explicit effective default of the
  Credential Issuer Identifier without being misreported as advertised data.
- Format-specific crates can later interpret configuration bodies without the
  OID4VCI core claiming support for Midnight, SD-JWT VC, mdoc, or VCDM.
- Selective bounded rescanning remains an intentional CPU trade-off for exact
  JSON retention, zeroizing temporary strings, staged types, and no new normal
  dependency.
- HTTP discovery, signed metadata, OAuth server metadata, grant selection,
  messages, replay protection, storage, consumer adoption, and release remain
  separately governed work under #7 / `IDR-023`.

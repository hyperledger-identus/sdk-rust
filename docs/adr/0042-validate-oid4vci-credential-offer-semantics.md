# ADR 0042: validate OID4VCI Credential Offer semantics after transport

- **Status:** Accepted
- **Date:** 2026-09-06
- **Decision authority:** standing autonomous agent authority under ADR 0004
- **Related work:** #7, #20, #111, #113; `IDR-023`

## Context

The #111 transport boundary safely accepts exact embedded Credential Offer JSON
or a least-authority HTTPS reference, but intentionally proves no member
semantics. OID4VCI 1.0 Final requires an HTTPS Credential Issuer Identifier, a
non-empty unique array of configuration IDs, and object-shaped grants when that
optional member is present. Consumers need those shared rules before metadata
lookup or authorization-flow selection.

## Decision

1. Add a semantic `CredentialOffer` state that can only consume an already
   validated `EmbeddedCredentialOffer`.
2. Retain exact JSON while selectively decoding the required issuer and
   configuration IDs; unknown values and grant contents stay opaque.
3. Apply separate positive semantic maxima for issuer bytes, each ID's bytes,
   and ID count without changing transport-limit construction.
4. Validate issuer syntax as HTTPS with host and optional port/path, without
   userinfo/query/fragment; do not normalize, fetch, authenticate, or trust it.
5. Accept only absent or object-shaped grants and expose presence without
   implying that any grant type is supported or usable.
6. Keep all values non-serializing and redacted, store owned strings in
   zeroizing buffers, and extend the static error/code taxonomy.
7. Extend the existing bounded lexical scanner and keep the dependency cone
   unchanged. Do not add HTTP, async, crypto, DID, storage, chain, platform, or
   product authority.

## Consequences

- Oxid and Lace can eventually consume one standards-pinned semantic boundary
  while retaining independent adapters and product policy.
- Semantic parsing performs a second bounded pass over exact JSON, trading a
  small deterministic cost for a distinct validated state and lossless future
  extension/grant processing.
- Empty configuration ID strings are accepted because the Final specification
  does not forbid an empty metadata key; later metadata matching decides
  whether the referenced configuration exists.
- Full grants, metadata, endpoints, issuance state, consumer adoption, release,
  and publication remain later work under #7 / `IDR-023`.

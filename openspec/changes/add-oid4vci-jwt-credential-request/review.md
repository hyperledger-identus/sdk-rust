# Semantic contract review

## Scope reviewed

- Issue #139 and exact base
  `74d264867a1ceb5f464ef452c99991863d3a1795`.
- OpenID4VCI 1.0 Final section 8.2 and RFC 6750 section 2.1 at their
  recorded immutable hashes.
- Read-only Oxid and Lace ID Portal evidence at the recorded revisions.
- Existing matched offer/metadata, Token Response, holder proof, static error,
  crate-ring, support-policy and consumer-isolation contracts.

## Findings

1. **Resolved — the configuration-ID route is locally decidable only when
   Authorization Details are absent.** The current Token Response core retains
   only presence for that opaque member. Rejecting presence avoids selecting a
   mutually exclusive Credential Request identifier without parsing it.
2. **Resolved — an offered index carries stronger evidence than a raw ID.** The
   selected value comes from the grant-validated offer, and the matched state
   already proves that it exists in Credential Issuer Metadata.
3. **Resolved — always requiring proofs is a safe narrow profile.** Metadata
   proof requirements are still opaque, while both the roadmap and current
   consumer path require JWT holder proofs. Optional proofless behavior remains
   additive later work rather than an unsafe local inference.
4. **Resolved — the JOSE dependency follows the architecture.** The blueprint
   directs protocol crates to depend on `identus-jose`. Accepting
   `Oid4vciProofJwt` removes arbitrary-string construction without moving proof
   generation or verification into the protocol crate.
5. **Resolved — Bearer is explicit rather than assumed.** Token types are
   extensible and may require different presentation rules. The constructor
   accepts case-insensitive Bearer only, validates the exact token against the
   RFC 6750 `b64token` grammar and emits a canonical scheme, leaving other
   token types to a future capability. The broader Token Response core remains
   correct for OAuth parsing and cannot substitute for this usage boundary.
6. **Resolved — resource and secret behavior is bounded.** Independent positive
   count/per-proof/body/authorization limits precede or follow the relevant
   allocation, while authorization and JSON body use zeroizing ownership and
   explicitly sensitive accessors.
7. **Resolved — deterministic JSON does not normalize proofs.** Serde escapes
   the selected ID and exact compact values safely, retains proof order and
   produces only the two Final members in a stable struct order.
8. **Resolved — request construction makes no transport/trust claim.** Endpoint
   syntax and prior state are reused, but the adapter still owns actual HTTP,
   origin, TLS/network, token validity, proof context, replay and response
   processing.
9. **Resolved — compatibility and provenance remain bounded.** The API and one
   local dependency edge are additive and unpublished; consumers are evidence
   only, and no source, fixture, feature, target or external dependency changes.

## Decision

The proposal, design, capability requirements, program replacement and task
map are semantically complete, objectively testable, reversible and within the
standing mandate. No correctness, security, privacy, compatibility,
provenance, target or product-scope blocker remains before implementation.

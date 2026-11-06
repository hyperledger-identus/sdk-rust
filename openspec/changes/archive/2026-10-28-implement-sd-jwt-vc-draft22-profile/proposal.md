# Implement the SD-JWT VC draft-22 profile

## Why

The stable RFC 9901 engine is now available, but it deliberately does not
identify or validate an SD-JWT as a Verifiable Digital Credential. OID4VCI,
OID4VP, Rust adopters, and future thin language adapters need one exact,
bounded profile that can issue and verify canonical `dc+sd-jwt` credentials
without silently following an evolving Internet-Draft or inheriting SDK-TS
DTOs, errors, DID coupling, runtime, or legacy identifier behavior.

## What Changes

- Add an unpublished `identus-sd-jwt-vc` profile crate pinned to
  `draft-ietf-oauth-sd-jwt-vc-22` over `identus-sd-jwt`; do not duplicate
  disclosure, digest, JOSE, signature, or key-binding mechanics.
- Require protected `typ=dc+sd-jwt`, expose media type
  `application/dc+sd-jwt`, and validate bounded `vct`, `aka_vcts`, registered
  claim placement, issuer-key-discovery inputs, type metadata inputs, and
  status-list references.
- Represent issuer authentication, type-metadata validation, and status
  observation as caller-supplied evidence. The crate validates and correlates
  those inputs but performs no HTTP, trust-anchor selection, authorization,
  rendering, or product policy.
- Preserve the RFC 9901 staged states and add a consuming profile-validation
  state plus a redacted receipt that cannot be confused with trust or status
  acceptance.
- Add immutable draft-22 provenance, Appendix A and clean-room vectors,
  property/fuzz/benchmark/differential evidence, portable compilation, and an
  offline issue-store-select-present-verify journey through the generic
  credential and OID4VP format capability seams.
- Record `@hyperledger/identus-sdk` 8.1.4 as a named legacy `vc+sd-jwt`
  consumer and move any adapter into separate issue #572. Canonical output in
  this slice remains exclusively `dc+sd-jwt`.

## Capabilities

### New Capabilities

- `sd-jwt-vc-draft22`: exact profile identity, claims, evidence inputs,
  resource/error boundaries, migration metadata, and end-to-end evidence.

### Modified Capabilities

- None. The RFC 9901 base remains profile-neutral and gains only narrowly
  reusable hooks needed to set and inspect a protected type and determine
  whether a caller-named root claim was disclosed.

## Impact

The workspace gains one additive credential-semantics crate with inward edges
to existing SD-JWT and generic credential/presentation seams. Existing
released APIs, protocol wire behavior, trust decisions, downstream
repositories, publication state, and the stable RFC 9901 capability remain
unchanged. Issue #569 owns this slice; issue #572 owns the optional legacy
adapter and cannot make legacy output canonical.


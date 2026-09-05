# ADR 0037: bind OID4VCI proof JWT issuer verification explicitly

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-05
- **Decision authority:** standing roadmap and issue #99
- **Related work:** issues #5, #8, #20, #99 and #104; ADRs 0014, 0034, 0035
  and 0036
- **Supersedes:** ADR 0035 decision 8 only by adding the accepted
  `identus-jose -> identus-did` verifier dependency

## Context

The SDK has bounded JWS parsing, algorithm-bound signature verification and a
holder-side OpenID4VCI Final proof builder. It cannot yet establish that an
issuer received a correctly typed, fresh, nonce-bound proof signed by the
header-selected key, or that a DID key is authorized for authentication.
Products therefore either duplicate these security rules or defer proof
verification entirely.

OpenID4VCI 1.0 Final Appendix F.4 requires proof type, algorithm policy,
signature/key binding, nonce and time validation. Appendix F.1 permits DID URL,
inline JWK and X.509 references. Section 13.8 leaves nonce lifetime and replay
handling to the issuer, so the generic SDK must expose policy without choosing
product storage or trust.

## Decision

1. Add the issuer verifier to `identus-jose` and represent parsed,
   cryptographically verified/key-bound and issuer-authorized states as
   distinct public types.
2. Parse only recognized proof claims with duplicate/type checks, retain exact
   compact bytes and safely skip bounded unknown claims.
3. Verify inline JWKs through the existing signature registry. For DID URL
   `kid`, use `identus-did::DidUrlDereferencer` with the exact
   `authentication` relationship, then require exact method identity and
   public JWK material before signature verification.
4. Add the inward `identus-jose -> identus-did` dependency. Reject non-DID
   `kid`, path/query selectors and multibase-only methods until focused
   contracts own those resolution and conversion semantics.
5. Accept `x5c` only through an injected asynchronous provider that owns
   certificate parsing, leaf extraction, path/time/revocation validation and
   trust anchors. Re-bind the returned public JWK to the header algorithm and
   verify the proof signature inside JOSE.
6. Require explicit identified/anonymous client, audience, nonce, maximum-age
   and skew policy. Read the existing injected `WallClock` once and use checked
   NumericDate arithmetic; select no ambient clock.
7. Require an injected atomic replay acceptance port as the final transition.
   Provide no default storage, retention, hashing or permissive policy.
8. Invoke no external provider for malformed/profile-invalid input, invoke at
   most one selected key provider and signature suite, and never invoke replay
   before all other checks pass.
9. Keep all errors and Debug output static and redacted. Add no HTTP, storage,
   runtime, chain, product, certificate-library, trust-store or new crypto
   dependency.
10. Continue rejecting `key_attestation` and `trust_chain`; issue #104 owns
    their nested validation and trust semantics.

## Consequences

- Issuers receive one portable proof-verification basement with explicit
  dependency inversion for resolution, certificates, time and replay.
- A valid signature can no longer be confused with accepted issuer policy in
  the type system.
- Midnight and NeoPRISM resolvers can plug into the same DID URL boundary;
  Oxid and Lace adoption remains downstream-owned.
- The current crate dependency cone grows by one already-built domain crate.
  Holder-only consumers still execute no resolver but compile `identus-did`
  until a later evidence-backed feature split is justified.
- Generic `kid`, Multikey conversion, certificate implementations and optional
  attestations remain visible follow-up work rather than implicit support.

## Rollback

Before publication, revert issue #99's verifier pull request. The holder
builder, compact codec, signature registry and DID crate remain independently
usable and no stored-data or consumer migration is required.

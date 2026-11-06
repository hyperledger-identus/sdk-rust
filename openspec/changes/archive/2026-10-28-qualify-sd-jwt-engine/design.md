# SD-JWT engine qualification design

## Context

The SDK already has format-neutral credential/presentation states, bounded JOSE
capabilities, OID4VCI/OID4VP protocol seams, and canonical `dc+sd-jwt` routing
metadata, but no SD-JWT proof engine. Candidate crates range from narrow payload
processors to complete SSI frameworks and disagree on draft level, compiler
floor, cryptographic ownership, resource policy, and target support.

## Goals

- Keep stable RFC 9901 mechanics independent of the evolving VC profile.
- Minimize total correctness and maintenance risk without importing unrelated
  SSI, networking, native TLS, product policy, or candidate public types.
- Preserve deterministic, bounded, redaction-safe processing and staged
  unverified/verified states.
- Make official examples, negative cases, differential oracles, and an offline
  issue-to-verification journey executable in later implementation slices.

## Decisions

### Decide the base and profile separately

The base capability owns RFC 9901 compact/JWS-JSON envelopes, disclosures,
digest processing, selection, decoys, key-binding inputs, and verification
evidence. The profile capability owns draft-22 `dc+sd-jwt` headers and claims,
type metadata semantics, status vocabulary, issuer-key discovery inputs, and
profile validation. Neither capability owns trust decisions or remote effects.

### Prefer a local focused engine unless a candidate clears every gate

Candidate code is not adopted piecemeal merely because its happy path matches.
If no release clears the existing MSRV, dependency, unsafe/native, bounds,
redaction, targets, and rejection-parity gates, the implementation issue builds
a focused `identus-sd-jwt` engine over existing Identus JOSE/crypto/port seams.
External crates remain differential oracles with objective reopen triggers.

### Keep authority and secrets outside the format processor

The format processor accepts bounded bytes and injected digest/signature,
entropy, and time evidence. It returns staged parsed, disclosure-processed, and
cryptographically verified values. Issuer/verifier trust, key custody, replay,
consent, transport, and storage remain caller-owned.

### Split canonical and legacy identifiers

Canonical issuance and verification use `dc+sd-jwt`. A later legacy adapter may
accept `vc+sd-jwt` only when a named consumer proves need, preserves exact
provenance, and cannot cause canonical output or ambiguous policy.

## Evidence architecture

```text
RFC 9901 examples + negative cases ───────┐
SD-JWT VC draft-22 Appendix A ────────────┼─► versioned vector packets
SDK-TS legacy compatibility fixtures ─────┤
external Rust engines (oracles only) ─────┘
                                              │
                                              ▼
bounded parse → disclosure graph → selection → KB-JWT → verification receipt
       │              │              │             │             │
       └──── property/fuzz ───────────┴── benchmark ┴──── offline E2E
```

## Compatibility and rollback

This change changes no API or wire behavior. Future production types are
Identus-owned, so a private engine can be removed or replaced without a public
candidate-type migration. Profile revisions are explicit capability versions;
rollback restores the previous profile/vector packet and never silently accepts
a different draft.

# ADR 0182: Pin SIOPv2 draft 13 behind Identus protocol boundaries

- Status: Accepted
- Date: 2026-10-27
- Issue: [#514](https://github.com/hyperledger-identus/sdk-rust/issues/514)
- Decision owners: SDK-Rust OpenID and identity maintainers

## Context

Oxid has a working standalone self-issued DID login, but it combines draft-13
wire behavior with product lifecycle, loopback transport, managed DID lookup,
custody and UI consent. SIOPv2 remains an OpenID Implementer's Draft rather
than a Final specification. Published Rust implementations exist, but their
MSRV is 1.95 versus SDK-Rust's 1.89 and their public/runtime cones do not match
the existing bounded Identus DID and JOSE architecture.

## Decision

1. Pin the first `identus-siopv2` profile to draft 13 dated 2023-11-28 and
   label it experimental.
2. Support only signed requests, DID subjects, subject-signed ID Tokens, exact
   `openid` scope and `direct_post` in the first slice.
3. Use `identus-jose` for bounded signing/verification and `identus-did` for
   identifier values. Do not implement cryptographic primitives in SIOPv2.
4. Treat Impierce `siopv2` 0.2.0 and `affinidi-siopv2` 0.1.4 as differential
   oracles, not production dependencies or public API authorities.
5. Activate `identus-openid4vc` only for the already-proven neutral request
   retrieval seam shared by OID4VP and SIOPv2; protocol claims stay separate.
6. Model lifecycle as a pure explicit-time/event state machine. Keep transport,
   trust, replay storage, DID resolution, custody, consent, persistence and UI
   in caller adapters.
7. Preserve Oxid only as conformance and migration evidence. Follow draft 13's
   `redirect_uri`; do not copy its loopback `response_uri` fixture into the
   generic contract.

## Consequences

The SDK gains a small reusable authentication core and a second consumer for
shared OpenID request retrieval without creating a broad OIDC runtime. Oxid
can later delete duplicated protocol behavior behind a thin adapter while
retaining product responsibilities. The initial profile does not cover the
whole draft and cannot claim Final compatibility or certification.

Any draft update, JWK-thumbprint subject, dynamic discovery, federation,
authorization-code flow, attester token, arbitrary claims or production replay
store requires a separate issue and compatibility record.

## Reconsideration

Reconsider direct dependency adoption if a maintained crate supports the
selected profile behind a narrow feature set, meets the SDK MSRV and portable
targets, does not force network/runtime or foreign DTO policy, and passes the
same redaction, bounds, vectors and supply-chain gates. A future SIOPv2 Final
specification triggers a new ADR rather than an automatic upgrade.

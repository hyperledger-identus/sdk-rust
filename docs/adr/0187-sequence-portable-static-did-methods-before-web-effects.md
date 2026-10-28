# ADR 0187: Sequence portable static DID methods before web effects

- Status: Accepted
- Date: 2026-10-28
- Decision owners: SDK-Rust DID maintainers
- Issue: [#555](https://github.com/hyperledger-identus/sdk-rust/issues/555)

## Context

Peer and PRISM methods are now independently reusable SDK components. The
remaining roadmap item groups `did:key`, `did:jwk`, and `did:web`, but they do
not share an effect model. The first two are deterministic projections from
public key material; the third resolves an HTTPS resource and therefore crosses
a network-security boundary. Existing Rust implementations either carry broad
SSI/runtime dependency cones, implement a different profile, or are maintained
best as differential oracles.

## Decision

Create separate future crates `identus-did-key`, `identus-did-jwk`, and
`identus-did-web`. Deliver them in that order through issues #557, #558, and
#559. The sequence is a delivery-risk order, not a runtime dependency chain.

`identus-did-key` and `identus-did-jwk` are effect-free and reuse
`identus-did` plus `identus-crypto`. `identus-did-web` owns pure URL
transformation, exact document binding, and a minimal injected bounded-fetch
port. A concrete HTTP stack, DNS/TLS/redirect/SSRF policy, cache, retries,
executor, and product trust remain adapter responsibilities.

Use Spruce SSI, DIF did-key, and Impierce implementations as pinned conformance
oracles only. Do not add a broad SSI facade or expose third-party DTOs/errors.
Do not extract a public shared multicodec crate until two delivered owners prove
identical semantics and maintenance value.

Each method receives an explicit narrow first profile and its own normative,
negative, property, fuzz, benchmark, target, and differential evidence. The
`did:web` E2E starts with a deterministic in-process fetch adapter; real-network
adapters require a later decision.

## Consequences

The SDK retains low coupling, cohesive ownership, portable targets, and small
dependency cones. Strict first profiles may support fewer curves, encodings, or
transport behaviors than broad upstream libraries; later profiles can expand
them deliberately. Three crates and evidence sets cost more coordination than
one facade, but each remains independently reversible and adoptable.

## Rejected alternatives

- One umbrella DID-method crate: couples unrelated profiles and effects.
- Spruce or Impierce as a production facade: mismatched profiles and excessive
  dependency/runtime coupling.
- Concrete HTTP inside `identus-did-web`: wrong ownership for network policy.
- Immediate shared multicodec extraction: reuse is not yet proven.
- SDK-TS DTO/error parity: language migration belongs in thin adapters.

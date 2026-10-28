# ADR 0188: Bind did:key to a strict public-key profile

- Status: Accepted
- Date: 2026-10-28
- Decision owners: SDK-Rust DID maintainers
- Issue: [#557](https://github.com/hyperledger-identus/sdk-rust/issues/557)
- Parent decision: [ADR 0187](0187-sequence-portable-static-did-methods-before-web-effects.md)

## Context

The pinned did:key v0.9 draft combines a broad syntax statement with narrower
executable creation and decoding algorithms. Rust implementations also differ
in key suites, implicit Ed25519-to-X25519 conversion, verification-method
types, and dependency ownership. SDK-Rust needs a portable method without
turning one oracle's facade into its public architecture.

## Decision

Create `identus-did-key` as an independently selectable, synchronous,
effect-free crate. Its first profile accepts only base58-btc `z` identifiers
and direct Ed25519, X25519, P-256, and secp256k1 public-key multicodecs already
validated by `identus-crypto`. Creation receives typed public keys; parsing
requires canonical unsigned varints, exact lengths, valid curve points, and
canonical re-encoding under explicit 128-byte text and 67-byte decoded limits.

Resolve to one `Multikey` with `publicKeyMultibase`. Direct X25519 projects only
to `keyAgreement`; signing keys project to authentication, assertion,
capability invocation, and capability delegation. Do not derive X25519 from
Ed25519. Keep the codec implementation method-local until another delivered
owner proves a genuinely identical shared contract.

Use the pinned DIF, Spruce, Impierce, and SDK-TS behavior only as attributed
fixtures or differential oracles. No oracle enters a dependency table. Rust
types and stable redacted errors are canonical; migration DTOs belong in later
language adapters.

## Consequences

The crate has a small inward dependency cone, deterministic behavior, portable
targets, and no custody or runtime authority. It deliberately supports fewer
encodings and suites than broad SSI implementations. Profile expansion is a
separate compatibility decision. A deterministic registry-composition E2E and
all quality evidence are required before support is recorded as delivered.

## Rejected alternatives

- Adopt Spruce/Impierce as a facade: excessive coupling and mismatched policy.
- Revive DIF did-key as a dependency: archived and old crypto generations.
- Accept `u` immediately: conflicts with the pinned executable algorithms.
- Implicit Ed25519-to-X25519 derivation: changes key and relationship semantics.
- Extract shared multicodec now: reuse and identical error/bound contracts are
  not yet proven across two delivered owners.

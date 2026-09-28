# Design

## Ownership split

`identus-jose` keeps:

- canonical bounded JWS Compact parsing/encoding;
- the closed validated protected-header representation;
- algorithm and public-key representations;
- signer/signature-suite ports and registry;
- generic unverified and verified compact states;
- generic static `jose.*` errors.

`identus-oid4vci` gains:

- holder proof claims, builder, signing input, and signed proof state;
- issuer proof parsing and verified/trusted/authorized stages;
- DID, X.509, trust-chain, key-attestation, clock, and replay ports;
- proof-policy validation and profile-specific static errors.

The protected header may carry bounded `key_attestation` and `trust_chain`
syntax because those are JOSE header extensions consumed by more than one
possible profile. They remain untrusted bytes at this layer. Only OID4VCI
assigns profile semantics.

## Error boundary

Generic failures from compact parsing and signature verification remain
`JoseError`. OID4VCI maps them into one opaque profile error variant or a
crate-local static mapping without copying caller-controlled data. Every
profile-specific failure receives an `oid4vci.*` code and OID4VCI capability.
No error formatting exposes claims, compact values, key identifiers, evidence,
or provider details.

## Compatibility

Both crates are currently unpublished `0.0.0`, so the move deliberately
changes Rust import paths from `identus_jose::Oid4vci*` to
`identus_oid4vci::Oid4vci*`. No compatibility re-export remains in JOSE because
that would preserve the protocol coupling. Wire bytes, accepted/rejected
inputs, provider call counts, and state guarantees remain unchanged.

## Dependency and library strategy

The implementation reuses the existing `identus-jose` facade and
`identus-crypto` primitives. Candidate external JOSE crates stay outside the
production graph. A later private-engine adoption must delete enough risky
mechanics to justify its adapter and dependency cone and must pass the existing
facade contract unchanged.

## Verification

Tests move with behavior. Compile-time dependency guards assert that JOSE has
no DID/protocol dependency. Workspace Clippy, docs, tests, code-health,
dependency policy, MSRV/primary, and portable target gates must pass. The
post-change graph and source-size receipt are attached to the issue/PR.

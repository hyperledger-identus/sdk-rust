# Add bounded did:jwk profiles

## Why

SDK-Rust now owns generic DID documents, a validated public-only JWK value,
and an independently reusable `did:key` method, but it cannot create or resolve
the compact `did:jwk` identifiers used by existing SSI ecosystems. The pinned
method is deterministic and effect-free, while current Rust implementations
carry broad SSI dependency cones and expose different verification-method
profiles.

## What changes

- Add an independently selectable `identus-did-jwk` crate.
- Accept canonical unpadded base64url carrying bounded UTF-8 JSON for the
  existing Ed25519, X25519, P-256, and secp256k1 `PublicKeyJwk` profiles.
- Create one deterministic serialization while preserving the exact supplied
  DID during parsing and resolution; do not claim one-key/one-DID uniqueness.
- Project one `JsonWebKey2020` method at `#0` and apply the pinned `use=sig`,
  `use=enc`, or absent-`use` relationship rules.
- Reject private members, duplicate JSON names, malformed or padded carriers,
  invalid key material, incompatible `use`, unsupported profiles, oversized
  input, and excessive JSON work with stable redacted errors.
- Add normative, negative, property, fuzz, differential, benchmark, target,
  registry-composition, handbook, and deterministic E2E evidence.

## Capabilities

### Added capabilities

- `did-jwk-method`: bounded creation, parsing, classification, and local
  resolution for the approved public-JWK profile.

## Non-goals

No private JWK, key generation, signing, proof selection, JWK Set, remote
lookup, trust policy, custody, persistence, networking, runtime, global
registration, product adapter, consumer mutation, publication, or uniqueness
claim.

## Delivery

Issue #558 owns this SDK-only slice under ADRs 0187 and 0189. Issue #559 remains
the serial successor and is not implemented or coupled here. Pending quality
declaration `did.jwk.profile.quality.v1` and change-ledger ID
`did.jwk.profile.v1` bind the evidence required before activation.

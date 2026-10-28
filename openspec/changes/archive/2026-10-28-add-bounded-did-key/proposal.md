# Add bounded did:key profiles

## Why

SDK-Rust has generic DID values, documents, registries, and four validated
public-key families, but no independently reusable `did:key` method. The
pinned v0.9 draft is implementable without I/O, while available Rust facades
either are archived, carry broad SSI dependency cones, or expose profiles and
DTOs outside the SDK boundary.

## What changes

- Add an independently selectable `identus-did-key` crate.
- Implement one strict base58-btc `z` profile for direct Ed25519, X25519,
  P-256, and secp256k1 public keys.
- Create canonical identifiers from SDK public-key types and resolve them to a
  one-key `Multikey` DID document with profile-specific relationships.
- Reject non-canonical varints, unsupported codecs, invalid points, private
  material, oversized input, and encodings outside the selected profile.
- Add normative, negative, property, fuzz, differential, benchmark, target,
  registry-composition, handbook, and deterministic E2E evidence.

## Capabilities

### Added capabilities

- `did-key-method`: bounded creation, parsing, classification, and local
  resolution for the approved v0.9 profile.

## Non-goals

No base64url method identifiers, P-384, BLS, RSA, secret keys, key generation,
Ed25519-to-X25519 derivation, signing, custody, update, deactivate, persistence,
networking, runtime, global registration, consumer mutation, or publication.

## Delivery

Issue #557 owns this SDK-only slice under ADRs 0187 and 0188. Issues #558 and
#559 remain serial successors; neither is implemented or coupled here.
Pending quality declaration `did.key.profile.quality.v1` and change-ledger ID
`did.key.profile.v1` bind the evidence required before activation.

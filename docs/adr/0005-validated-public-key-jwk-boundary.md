# ADR 0005: validate the public-key JWK boundary in `identus-crypto`

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-004 roadmap mandate and sdk-rust issue #28
- **Related work:** issues #9, #5 and OpenSpec `harden-public-key-jwk`

## Context

The experimental crypto crate exposes `Jwk` as four public, stringly fields.
That representation permits incompatible key type/curve pairs, missing or
unexpected coordinates, non-canonical encodings and validation bypass during
JSON ingestion. Generic DID documents cannot safely build on that boundary.

NeoPRISM supplies the current Rust encoder shape. midnight-identity and Oxid
independently demonstrate that SSI consumers need typed profiles, private
fields, public-only material, canonical fixed-width coordinates and validating
deserialization. RFC 7517 also permits additional members, so a useful generic
boundary must tolerate them without interpreting protocol policy.

## Decision

1. `identus-crypto` owns a single `PublicKeyJwk` value type with private fields,
   typed `JwkKeyType`/`JwkCurve`, fallible constructors and read-only accessors.
2. The initial supported profiles exactly match implemented SDK algorithms:
   Ed25519, X25519, P-256 and secp256k1. Chain-specific and unimplemented curves
   are not vocabulary-only enum variants.
3. All construction and serde paths require canonical unpadded base64url,
   32-byte coordinates and the correct EC/OKP `y` shape.
4. The type never stores private `d` material. Unknown public members are
   preserved as uninterpreted JSON extensions; structural members cannot be
   shadowed.
5. Structural JWK validation remains distinct from curve-point, algorithm-use,
   signature, agreement and DID policy validation.
6. `EncodeJwk` returns the validated type. The old unreleased `Jwk` API is
   removed cleanly rather than retained as an invalid compatibility path.
7. `JwkError` maps to the stable redacted code `crypto.invalid_jwk` and never
   includes caller-supplied values.

## Consequences

- DID Core can depend on a reusable, policy-neutral and misuse-resistant public
  key representation.
- Direct `Jwk` struct literals and public-field reads break before release and
  must migrate to constructors/accessors.
- The `jwk` feature gains optional workspace-pinned `serde` and `serde_json`
  dependencies and remains wasm-safe.
- Additional curve families require implementation evidence and a new issue;
  they cannot appear as unsupported vocabulary in this validated type.
- Consumers must still perform on-curve/subgroup checks and bound untrusted JSON
  document size at their protocol boundary.

## Rollback

Revert the issue #28 pull request. No released API, persistent format,
publication or downstream repository is changed by this decision.

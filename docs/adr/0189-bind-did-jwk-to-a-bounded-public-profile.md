# ADR 0189: Bind did:jwk to a bounded public profile

- Status: Accepted
- Date: 2026-10-28
- Decision owners: SDK-Rust DID maintainers
- Issue: [#558](https://github.com/hyperledger-identus/sdk-rust/issues/558)
- Parent decision: [ADR 0187](0187-sequence-portable-static-did-methods-before-web-effects.md)

## Context

The pinned `did:jwk` method embeds JSON public-key material directly in the
identifier. JSON permits multiple serializations and arbitrary extensions,
while existing Rust implementations select different verification-method
suites and carry broad SSI dependency cones. The SDK already owns bounded
public JWK and DID domains, but its JWK type deliberately does not prove curve
point validity or method-specific operation policy.

## Decision

Create `identus-did-jwk` as an independently selectable, synchronous,
effect-free crate over `identus-crypto` and `identus-did`. Its first profile
accepts canonical unpadded base64url containing duplicate-free UTF-8 JSON for
Ed25519, X25519, P-256, and secp256k1 public JWKs. Creation emits one
deterministic compact serialization. Parsing preserves the supplied DID and
does not require the JSON to match that serialization because the method
explicitly permits more than one identifier for the same public key.

Revalidate key material after structural `PublicKeyJwk` construction:
Ed25519 must be canonical and non-weak, X25519 canonical and contributory, and
EC coordinates valid points on their declared curve. Reject known private
members and unsupported profiles. Treat absent `use` exactly as the pinned
method's complete five-relationship set; this is an explicit method-compatibility
rule rather than a claim that every curve performs every relationship's
operation. When `use` is present, accept `sig` only for signing curves and
`enc` only for X25519; reject other or incompatible values. Reject `key_ops` in
the first profile because mapping its operation vocabulary to DID relationships
would add policy not defined by the pinned method. Preserve other bounded public
extensions semantically, but never use `kid`, `alg`, or another extension for
the fixed `#0` fragment or relationship selection.

Bound carrier text at 21,846 bytes, decoded JSON at 16,384 bytes, nesting at 16,
nodes at 1,024, and top-level members at 64, in addition to inherited public
extension budgets. Creation enforces the same serialized JSON and carrier
ceilings as parsing, so every created DID parses under the selected profile.
Detect duplicate names before map materialization and keep errors content-free.
Resolve to one `JsonWebKey2020` verification method. DID JSON stays
context-free; DID JSON-LD adds DID Core and JWS 2020 contexts.

Use Spruce and Impierce only as attributed differential oracles. Do not add a
JOSE/SSI facade or extract a generic JSON parser until multiple delivered
owners prove an identical public contract. Rust-native types and errors remain
canonical; language migration DTOs belong in later adapters.

## Consequences

The method remains portable, low-coupled, independently adoptable, and free of
custody or runtime authority. The profile is stricter than the draft's
non-normative advice because explicit unusable curve/use pairings fail closed
and `key_ops` is not silently interpreted. The pinned absent-`use` projection
remains a deliberate interoperability exception. It
deliberately supports fewer JWK families than broad JOSE implementations and
does not provide identifier uniqueness. Complete deterministic registry E2E
and quality evidence is required before delivery is recorded.

## Rejected alternatives

- Adopt Spruce/Impierce as a facade: excessive coupling and profile mismatch.
- Re-export a general JOSE crate: duplicate public JWK ownership without need.
- Canonicalize every incoming JSON string: contradicts the method's explicit
  non-uniqueness and would rewrite external identifiers.
- Trust structural JWK parsing as point validation: permits unusable identity
  keys and diverges from the strict public-key policy established by did:key.
- Let `kid`, `key_ops`, or `alg` drive DID relationships: not specified by the
  pinned method and would import JOSE/product policy. The first profile rejects
  `key_ops`; later support requires a separate operation-to-relationship ADR.

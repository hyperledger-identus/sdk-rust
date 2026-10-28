# Design

## Ownership and API

`identus-did-jwk` exposes a validated `DidJwk`, deterministic `create`, `parse`,
and `resolve` operations, explicit public limits, and a stable redacted error
enum. Creation accepts the existing SDK `PublicKeyJwk`; parsing returns that
same domain value after method-specific cryptographic validation.

```text
caller PublicKeyJwk -> identus-did-jwk -> identus-did document
                              |
                              +--------> identus-crypto validation
```

The crate has no private-key constructor, generic JOSE facade, I/O, or
process-global registry. A caller-owned registry example opts in explicitly.

## Identifier pipeline

Creation validates the public JWK profile, serializes one deterministic compact
JSON object, applies the same 16,384-byte JSON and 21,846-byte carrier ceilings
as parsing, base64url-encodes it without padding, and builds
`did:jwk:<carrier>` through `identus-did`.

Parsing first checks the method and 21,846-byte carrier ceiling, alphabet and
absence of padding. It bounds the predicted and actual decoded JSON at 16,384
bytes, validates UTF-8 and duplicate-free JSON under depth/node/member limits,
rejects private members, constructs `PublicKeyJwk`, validates the actual key,
and applies the `use` policy. It retains the exact input DID. It deliberately
does not compare the source JSON to the SDK serialization because the method
permits multiple serializations of the same key.

Method-local JSON traversal is private. It may be extracted only when another
delivered owner demonstrates identical public semantics, budgets, and errors.

## Public-key and use validation

- Ed25519: decode `x`, construct `Ed25519PublicKey`, reject non-canonical or
  weak material; allow absent or `sig`, reject `enc`.
- X25519: decode `x`, construct `X25519PublicKey`, reject non-canonical or
  non-contributory material; allow absent or `enc`, reject `sig`.
- P-256 and secp256k1: combine `x` and `y` as uncompressed SEC1, construct the
  matching SDK public key, and allow absent or `sig`, rejecting `enc`.
- Any other explicit `use` shape or value is unsupported. Absent `use` follows
  the pinned five-relationship projection as a compatibility rule, not a claim
  that the curve implements every relationship's operation. `key_ops` is
  rejected in the first profile because the pinned method provides no safe
  operation-to-relationship mapping. Other bounded public extensions are
  preserved semantically and do not influence fragment or relationships.

## Document projection

Resolution creates one verification method at `<did>#0`, controlled by the
DID, with type `JsonWebKey2020` and the complete public JWK as
`publicKeyJwk`. The exact DID is retained even when its JSON serialization is
not the SDK creation order.

Absent `use` references the method from authentication, assertionMethod,
capabilityInvocation, capabilityDelegation, and keyAgreement. `sig` uses the
four signing/capability relationships. `enc` uses only keyAgreement. The
default DID JSON result is context-free; negotiated DID JSON-LD adds DID Core
and JWS 2020 contexts.

Create and resolve form the deterministic E2E: construct each supported public
JWK, create its DID, parse both SDK-generated and differently ordered JSON,
resolve through an opt-in registry, serialize/reparse the document, and prove
identity, extensions, `#0`, and relationship policy remain correct without I/O.

## Errors and bounds

Errors distinguish wrong method, malformed carrier, malformed JSON,
duplicate member, private material, unsupported key profile, invalid public
key, incompatible use, resource limit, and document projection. `Display`
uses fixed text. Tests prove oversized and malformed inputs fail before
avoidable decode, typed allocation, or projection.

## Evidence layout

Attributed JSON fixtures contain normative/oracle inputs and normalized
outcomes. Unit/conformance tests cover profiles, alternate serializations,
extensions and negatives; properties cover create/parse/resolve; fuzzing
targets carrier/JSON/key boundaries; directional benchmarks measure create,
parse, resolve and early rejection. The handbook records ownership, limits,
registry composition and non-goals. A manifest/public-API dependency guard and
repository-diff audit prove the prohibited surfaces remain absent. Portable
target compilation and the deterministic registry E2E are separate required
receipts. Quality and change ledgers make support discoverable.

## Context

W3C DID Core 1.0 models a DID document as an extensible map. Core identifiers
have strict DID, DID URL or URI syntax, while verification method suites,
service types and extension entries are intentionally open. Relationships can
contain either embedded verification methods or DID URL references. JSON wire
forms also allow selected values as either one item or a non-empty array.

The donor audit found compatible ideas but no implementation suitable for
direct extraction. NeoPRISM at
`8becb225132efb1d9302b2c5f6ed4d87b84e8685` exposes permissive public serde
records backed by IOTA Identity and `uriparse`. midnight-identity at
`427f8571950c42967a18726cbcbefecc19ef8d79` has useful duplicate/reference
checks but mixes in closed curve and controller policy. Apollo at
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` supplies cross-language crypto
compatibility only. Oxid at `685f9670af4846d52697a4cfeb94779758ae1075`
supplies bounded Midnight consumer shapes. Lace ID Portal at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` demonstrates extension round trips
but remains evidence-only because repository-level license evidence is
unresolved. No donor source is copied.

## Goals / Non-Goals

**Goals:**

- Represent the W3C DID Core document, verification relationship and service
  JSON shapes without chain or method dependencies.
- Preserve unknown entries and wire cardinality for semantic JSON round trips.
- Make every construction path enforce identical structural/resource rules.
- Reject accidental private key material at the public document boundary.
- Keep validation deterministic, bounded and portable to wasm/mobile.

**Non-Goals:**

- DID resolution/dereferencing, registrar ports or relative DID URL handling.
- JSON-LD expansion, RDF canonicalization or media-type negotiation.
- Cryptosuite/key conversion, proof verification or method registration.
- Controller-equals-subject, curve-to-relationship or chain authorization
  policy.
- Scheme-specific URI normalization, extension registries, fuzzing or package
  publication.

## Decisions

### Decision 1: own a structural model with private fields

`DidDocument`, `VerificationMethod` and `Service` expose validating constructors
and read-only accessors. Serde deserialization routes through the same
validation path rather than deriving directly onto public fields. The existing
`Did` remains the DID-subject/controller authority. A new `Uri` value implements
bounded absolute RFC 3986 generic syntax for aliases, verification identifiers
and services; callers can separately parse DID-shaped URIs as `DidUrl` when a
method-specific layer requires that stronger profile.

Collections are represented as absent-or-present non-empty vectors. A reusable
one-or-many wire enum retains whether JSON supplied a scalar or array while
offering a normalized slice view. Unknown members are held as JSON maps. The
promise is semantic JSON preservation, not byte order, whitespace or lexical
canonicalization.

### Decision 2: keep verification suite policy open

A verification method requires a URI `id`, a DID `controller`, a bounded
non-empty open `type` string and an extension/property map. The generic layer
recognizes `publicKeyJwk` and `publicKeyMultibase`: it validates their JSON
shapes, rejects their simultaneous presence and refuses registered private JWK
members. Other suite-defined properties are preserved without interpretation.
Whether an unknown verification type has sufficient material is decided by its
cryptosuite adapter, because DID Core cannot infer that from an open map.

Embedded relationship values use the same verification method type; references
use `Uri`. The five core relationships are represented explicitly. The core
does not require a reference to resolve inside the same document, because DID
Core permits dereferencing another DID document.

### Decision 3: preserve open services without weakening identifiers

A service requires an RFC 3986 `Uri` id, a bounded non-empty string-or-set type,
and a string, map or non-empty mixed array endpoint. Every endpoint string is a
validated `Uri`; maps and extension members remain bounded JSON. Duplicate
service ids are rejected as DID Core requires.

Generic URI validation checks an ASCII scheme, allowed RFC 3986 characters and
complete percent escapes in one pass. It preserves exact spelling and does not
claim scheme-aware normalization or dereferencing safety. Protocol adapters may
apply stronger HTTPS, origin, SSRF and normalization policy.

### Decision 4: make extensibility bounded and collision-safe

The SDK applies public resource limits before or during validation: 256 KiB per
JSON document, 128 items per document collection, 64 extension entries per map,
256 bytes per property name, 64 KiB per arbitrary string, 32 levels and 4,096
nodes per extension tree. Core strings have smaller type-specific limits.

Flattened extension maps cannot shadow reserved core keys. Duplicate
verification method or service identifiers are rejected. A bounded iterative
JSON walker prevents native constructors from bypassing limits applied at the
raw JSON entry point.

### Decision 5: retain the current package and dependency boundary

The model lands in today's unpublished `identus-did` package. It promotes
`serde_json`, already pinned by the workspace, to a normal dependency and adds
no URI, JSON-LD, DID or crypto framework. Publication and any final
`identus-did-core` rename remain governed by #3.

## Threat Contract

**Assets:** public document integrity, parser availability, absence of secret
key material, extension fidelity and stable downstream adapter boundaries.

**Threats addressed:** oversized/deep JSON, duplicate identifier confusion,
reserved-key shadowing, malformed identifiers, private JWK disclosure,
ambiguous known key material, empty required sets and reflected attacker input
in public errors.

**Residual boundaries:** structural validity does not prove document
authenticity, controller authority, reference availability, safe network
dereferencing, cryptographic key validity, JSON-LD equivalence or privacy of
published services.

## Test and Verification Strategy

- Pin W3C-shaped examples with scalar/array forms, embedded/reference
  relationships, both recognized material forms and all service endpoint forms.
- Adapt PRISM and Midnight document shapes without importing their chain policy.
- Prove extension and cardinality preservation through semantic JSON round trips.
- Cover duplicate, collision, secret, malformed URI and every resource boundary.
- Run a release-mode deterministic parse diagnostic over a representative
  document; record throughput without a CI threshold.
- Run focused, workspace, wasm/mobile, docs, lint, formatting, OpenSpec,
  supply-chain and Nix gates, then review the exact PR head.

## Migration Plan

1. Land the issue-linked OpenSpec delta and ADR as a signed commit.
2. Implement the semantic model, parser helpers and conformance suite.
3. Record performance and complete local semantic/security review.
4. Sync the canonical `did-core` spec, produce a ready/receipt, archive the
   change and merge only after exact-head hosted CI is green.
5. Deliver resolution results/ports and downstream adapters in separately
   scoped issues.

# Semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #37 (child of #5 / `IDR-005`)
- **Develop base:** `3e3e1117c11d33746a4fc0fd8808bceaede779ce`
- **Reviewed implementation head:** `f82645917da8a90746ce471a8ec8da76abd67596`
- **Result:** no unresolved blocker; suitable for exact-head hosted review

## Pre-implementation findings

1. **Ownership:** a bounded DID Core document model is generic SSI
   infrastructure and belongs in `identus-did`. DID-method resolution,
   chain-specific state, product policy, custody and UI stay outside this
   slice.
2. **Standards:** W3C DID Core requires DID controllers, URI-valued
   verification identifiers and references, extensible verification methods,
   relationship sets and service endpoint variants. Generic URI values are
   therefore distinct from the stronger `DidUrl` profile.
3. **Provenance:** donor repositories supplied compatibility evidence only.
   No donor source or fixture was copied, and the unresolved Lace license keeps
   Lace evidence-only.
4. **Trust boundary:** this model validates bounded representation and known
   public-key material rules. It does not resolve identifiers, interpret
   arbitrary cryptosuites, authorize keys or establish trust.
5. **Compatibility:** the public API is additive in an unpublished `0.0.0`
   crate. The only promoted runtime dependency, `serde_json`, was already a
   workspace dependency and the lockfile is unchanged.

## Post-implementation review

The complete `origin/develop...f826459` diff was reviewed independently after
implementation.

1. **Construction integrity:** public data fields are private. Native builders
   and serde deserialization converge on the same validation path, preserving
   the scalar-versus-array wire shape through `OneOrMany`.
2. **Identifier correctness:** root and controller values use `Did`; generic
   absolute URI values use `Uri`; callers may explicitly strengthen a URI to
   `DidUrl`. External relationship references remain valid as required by DID
   Core.
3. **Key-material safety:** known JWK and multibase properties are mutually
   exclusive. JWK objects require `kty` and reject registered private members.
   Unknown suite properties survive round trips without being assigned trust
   semantics.
4. **Collision handling:** duplicate document verification definitions,
   duplicate relationship members, duplicate service identifiers and reserved
   extension collisions fail closed. Raw JSON duplicate-key detection is
   explicitly deferred to #38.
5. **Resource behavior:** raw entry points enforce a 256 KiB envelope;
   collections, maps, names, strings, multibase text, extension depth and
   aggregate extension nodes are bounded. Validation is iterative over
   collections and bounded recursively only for extension JSON.
6. **Error hygiene:** public error display exposes stable invariant codes and
   field names, never caller-controlled document or key material.
7. **Round-trip contract:** standards-derived and donor-shaped fixtures cover
   every relationship, both known key forms, scalar/array cardinality, service
   endpoint variants and unknown extensions.

## Corrections made during review

- Replaced `DidUrl` with generic `Uri` for W3C verification method identifiers
  and relationship references, while keeping an explicit strengthening path.
- Rejected malformed IPvFuture literals and bracketed user-info authority
  forms in the bounded URI parser.
- Added empty-set, malformed endpoint, duplicate-definition, private-JWK and
  shared extension-budget regressions.
- Reformatted the representative JSON fixture to satisfy EditorConfig.
- Replaced a newer Rust let-chain with Rust 1.85-compatible control flow after
  the MSRV gate exposed it.

## Deliberate quality boundary

This slice stops at the requested 70–80% maturity point. Issue #38 owns raw
duplicate-key rejection, fuzz/property coverage, differential URI conformance
and performance drift checks. Issue #39 owns resolution and dereferencing
result types. JSON-LD processing, scheme-specific normalization, cryptosuite
interpretation and chain adapters remain downstream concerns.

# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/524
Constraint blockers: none

## Existing entries affected

The DID lexical limits, standalone method-name derivation, normalized cache
identity behavior, portable vector generator, adapter mappings, and input
resource inventory change. ADR 0093's historical derivation of the method
ceiling is superseded; its validated-newtype ownership remains unchanged.

## Introduced or changed constraints

- `MAX_DID_BYTES` is the literal public value 131,072.
- `MAX_DID_URL_BYTES` is the literal public value 139,264.
- `MAX_DID_URL_SUFFIX_BYTES` is the literal public value 8,192 and bounds the
  combined path, query, and fragment bytes independently of bare-DID length.
- `MAX_URI_BYTES` is the literal public value 139,264 so every valid `DidUrl`
  remains a valid `Uri` after the existing infallible conversion.
- `MAX_DID_METHOD_BYTES` remains independently fixed at 2,042.
- A normalized key above the existing 65,536-byte retention budget bypasses
  cache reads, writes, clock, and backend work under both failure modes and is
  reported as a content-free typed outcome.
- Exact-boundary vectors remain finite and generated without checking a large
  literal payload into Git.
- Foreign runtime preallocation, HTTP request targets, method projection, and
  serialization remain owned by their explicit adapter or method profile.

## Introduced or changed limitations

- Generic parsing remains bounded, single-pass, allocation-conscious, and
  redaction-safe.
- `MAX_CRYPTO_TEXT_BYTES`, the resolution cache key retention ceiling, and
  generic document/result JSON ingress do not change.
- This slice does not promise that every specification-valid PRISM create
  operation fits; #521 must enforce and document the finite method profile.
- Issue #523 must fit peer numalgo-4 into this generic policy or provide
  separately evidenced finite profile bounds before implementation.
- This slice does not make large DID URLs transportable through every browser,
  proxy, HTTP server, mobile bridge, or host application.

## Consumer and product impact

Existing valid identifiers continue to parse. New large identifiers can cross
the Rust/native/WASM lexical surfaces, but a cache may bypass retention and a
transport may apply a smaller external limit. Consumers can distinguish the
cache-key budget outcome without receiving caller-controlled content.

## Activation and rollback

The policy activates only after issue #524's exact preflight, independent
review, protected CI, and merge to `develop`. Before publication it can be
reverted as one prerequisite. After publication, a lower ceiling or changed
cache behavior requires a superseding ADR and migration evidence.

## Evidence

The research record pins sources and exact revisions, gives measured encoded
sizes, amplification analysis, parser diagnostics, disposable implementation
results, adapter consequences, explicit non-goals, and rollback ownership.

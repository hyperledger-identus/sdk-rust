## Context

W3C DID Resolution v1 models `resolve` as resolution metadata, an optional DID
document and DID document metadata. Its 28 August 2026 Candidate Recommendation
Draft uses an RFC 9457-style error object with a URL-valued `type`; older estate
implementations use keyword strings. A deactivated DID is a third valid state:
no document, no error and document metadata containing `deactivated: true`.

The same draft defines a JSON DID URL dereferencing result containing metadata,
optional content and content metadata, but explicitly marks dereferencing at
risk. Content can be a DID document, a verification method, a service, a URI or
another media resource. A generic core therefore cannot freeze a closed Rust
enum or own native byte-stream transport.

The donor audit found compatible shapes but no implementation suitable for
direct extraction. NeoPRISM at
`d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` mixes public records, `chrono`,
async resolution and HTTP mapping. midnight-identity at
`427f8571950c42967a18726cbcbefecc19ef8d79` has comprehensive document metadata
but the superseded keyword error form. Oxid at
`bfe3b481568dc738f0732c2b27548fab8721fd95` supplies useful consumer metadata
and product provenance, while Lace at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` demonstrates legacy client parsing
but is evidence-only because repository license evidence is unresolved. Apollo
at `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` has no generic resolution result.
No donor source is copied.

## Goals / Non-Goals

**Goals:**

- Represent current W3C resolution and serialized dereferencing result shapes
  without chain, method, async or transport dependencies.
- Make contradictory result states unconstructable through public APIs and
  reject them during deserialization.
- Preserve bounded standards/method extension metadata and open JSON content.
- Offer typed projections over known dereferenced JSON without losing unknown
  future representations.
- Keep every parser deterministic, redaction-safe and portable to wasm/mobile.

**Non-Goals:**

- Resolver, dereferencer or registrar traits; async strategy or method dispatch.
- HTTP status mapping, content negotiation, I/O, caching, retries or SSRF policy.
- Dereferencing algorithms, fragments, relative references or native byte
  stream serialization.
- Proof interpretation, semantic equivalence guarantees, trust or cryptosuites.
- Full XML Schema datetime edge cases, JSON-LD or package publication.

## Decisions

### Decision 1: model three resolution states explicitly

`DidResolutionResult` has validating constructors for success, failure and
deactivation. Success carries a `DidDocument`, contains no resolution error and
may carry document metadata. Ordinary failure carries an error, no document and
empty document metadata. Deactivation carries no document or error and requires
`deactivated: true`. Serde uses the normative three-field object and routes
through the same validation.

`validate_for(&Did)` additionally proves the returned document id equals the
requested DID. It also checks every `equivalentId` and `canonicalId` uses the
same DID method as the resolved/requested DID. Only a DID method adapter can
prove semantic equivalence, so the generic model makes no stronger claim.

### Decision 2: own small bounded metadata primitives

`MediaType` validates one ASCII type/subtype plus bounded parameters without an
HTTP dependency and preserves exact valid spelling. `DidResolutionDateTime`
accepts the common UTC, whole-second `YYYY-MM-DDTHH:MM:SSZ` profile required by
the draft and validates calendar/time ranges. `VersionId` accepts non-empty,
trimmed printable ASCII. Each value is bounded and has equivalent native/serde
validation.

`DidDocumentMetadata` types created, updated, deactivated, nextUpdate,
versionId, nextVersionId, equivalentId and canonicalId. Proof and all
method-specific metadata remain bounded extension JSON. The model preserves
absence, rejects empty/duplicate equivalent-id sets and prevents extensions
from shadowing common properties.

### Decision 3: use current error objects, not legacy wire ambiguity

`DidResolutionError` is a bounded RFC 9457 subset containing a required `Uri`
type, optional title/detail/instance and extension members. `DidResolutionErrorType`
classifies the nine W3C URLs while preserving unknown extension URLs exactly.
An explicit helper maps known legacy keywords such as `notFound` into current
URLs for adapter migrations. Strict serde accepts and emits only the current
object form; it never accepts a bare legacy keyword.

Error titles/details are caller-facing protocol values and therefore bounded
but not redacted. Validation failures continue to map to the SDK's stable,
redaction-safe `IdentusError`; rejected wire content is never reflected.

### Decision 4: keep at-risk dereferenced content open

`DereferencedContent` owns a bounded `serde_json::Value`. Typed constructors and
accessors support `DidDocument`, `VerificationMethod`, `Service` and `Uri`, but
the stored wire value stays open so future W3C changes and media resources do
not break a closed enum. `DidUrlDereferencingResult` validates the same binary
success/failure invariant as resolution: success has content and no error;
failure has an error, no content and empty content metadata.

The result serializes the draft's `didUrlDereferencingMetadata` field spelling.
Native byte streams belong to a binding adapter because JSON cannot preserve
arbitrary content bytes without choosing an encoding.

### Decision 5: share one bounded open-JSON policy

Resolution envelopes are capped at 512 KiB before JSON parsing. Collections
remain capped at 128 items; open maps at 64 entries; names at 256 bytes;
arbitrary strings at 64 KiB; nesting at 32 levels; and aggregate open JSON at
4,096 nodes. The existing DID-document JSON validator becomes crate-internal
shared infrastructure so native construction and serde cannot diverge. Core
metadata text and version ids use smaller purpose-specific limits.

## Threat Contract

**Assets:** result-state integrity, resolver availability, extension fidelity,
standards interoperability and clean method/transport boundaries.

**Threats addressed:** contradictory success/error states, legacy/current wire
confusion, oversized or deeply nested metadata, reserved-key shadowing,
malformed error URIs/media types/datetimes, duplicate equivalence identifiers
and cross-method canonicalization claims.

**Residual boundaries:** a valid result does not authenticate a document, prove
method equivalence, authorize keys, make endpoint I/O safe, interpret proofs or
guarantee future at-risk dereferencing semantics.

## Test and Verification Strategy

- Pin current W3C success, deactivation, failure and dereferencing envelopes.
- Cover all nine standard error URLs, unknown error URLs and every legacy helper.
- Adapt NeoPRISM, Midnight, Lace and Oxid producer/consumer shapes without
  importing their method, chain or transport policy.
- Exercise native/serde parity, every state contradiction, reserved collision,
  identifier mismatch, cross-method metadata and all resource ceilings.
- Run a release-mode deterministic parse diagnostic without a CI threshold.
- Run focused, workspace, MSRV, wasm/mobile, docs, lint, formatting, OpenSpec,
  supply-chain and Nix gates, then review the exact PR head.

## Migration Plan

1. Land this issue-linked OpenSpec delta and ADR as a signed commit.
2. Implement values, envelopes and conformance tests without downstream edits.
3. Record performance and complete distinct local semantic/security review.
4. Sync the canonical did-core spec, archive the change and merge only after
   exact-head hosted CI is green.
5. Deliver resolution ports/HTTP adapters and consumer adoption separately.

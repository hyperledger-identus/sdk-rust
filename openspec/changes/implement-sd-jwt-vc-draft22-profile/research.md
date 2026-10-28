# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

SDK-Rust has a bounded RFC 9901 engine, generic credential envelope and
verification port, generic presentation selection, OID4VP Final
`dc+sd-jwt` routing, and OID4VCI opaque credential transport. It has no owner
for SD-JWT VC profile identity, claim-placement rules, credential type
metadata, issuer-key-discovery evidence, or status-list references.

ADR 0193 and issue #569 already select a separately versioned draft-22 profile
over the local base. This change implements that accepted outcome rather than
reopening the RFC 9901 engine decision.

## Normative sources

- Primary source: `draft-ietf-oauth-sd-jwt-vc-22`, dated 2026-10-27,
  https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt, retrieved
  2026-10-28, text SHA-256
  `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124`.
- Stable base: RFC 9901 and canonical `sd-jwt-rfc9901` capability.
- JWT, JWS, confirmation, and integrity rules referenced by draft-22 remain
  bounded to the claims and evidence inputs consumed here; this slice does not
  implement another general JOSE stack.
- SDK-TS 8.1.4 at immutable revision
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e` is migration evidence only.

Draft-19 hash
`394c4640c581c301adaf24d1d356238e2c290b5c9e8b130141ca0e28732144fb`
is superseded research evidence and does not define behavior.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Focused `identus-sd-jwt-vc` profile over `identus-sd-jwt` | `adopt` | Exact draft pin, existing bounded base, SDK-owned states, no duplicate crypto, and direct fit with generic protocol seams. | A narrow maintained crate passes exact profile, bounds, redaction, compiler, target, and dependency gates with lower total maintenance cost. |
| Affinidi `affinidi-sd-jwt` 0.1.8 | `oracle` | Useful independent SD-JWT behavior but its assessed release targets the base engine, requires Rust 1.95, and lacks the complete SDK profile/evidence boundary. | A profile release matches draft-22 exactly and clears the SDK MSRV, cone, bounds, and target matrix. |
| IOTA `sd-jwt-payload` 0.5.1 | `oracle` | Useful payload mechanics only; no complete profile, metadata/key/status evidence contract, or SDK boundary. | A narrow profile surface deletes meaningful local maintenance and proves exact negative parity. |
| Spruce `ssi-sd-jwt` 0.6.0 | `not-adopt` | Broad 218-package graph, authored unsafe, exact-lock advisory failures, and incompatible coupling. | A narrow safe advisory-clean profile crate with explicit draft-22 and target evidence. |
| ReallyMe `reallyme-sd-jwt` 0.4.1 | `oracle` | Strong bounded design but Rust 1.99 and its own JOSE/crypto/proto stack. | A compatible independently maintained release with a smaller total cone. |
| `@sd-jwt/sd-jwt-vc` 0.7.2 via SDK-TS 8.1.4 | `migration-oracle` | Proves a released consumer and legacy behavior, but uses `vc+sd-jwt` and TypeScript-specific DTO/runtime behavior. | Only issue #572 may qualify an input-only adapter; it never defines native Rust semantics. |

No candidate or donor type enters the production dependency graph. Existing
RFC 9901 oracle locks, checksums, licenses, upstream tests, and advisory
receipts remain authoritative and are not duplicated.

## Compatibility and dependency evidence

The current implementation consists of `identus-sd-jwt` RFC 9901 mechanics,
`identus-credentials` envelopes/verifier/status values, and OID4VC protocol
capability seams; no SD-JWT VC profile crate or profile API exists. The new
direct and resolved dependency cone adds no registry dependency: it resolves only existing
workspace crates plus the already locked `serde`, `serde_json`, `url` and
`zeroize` families where required. OID4VCI and OID4VP remain dev-only E2E
edges, not production dependencies.

Exact candidate versions and features are the immutable releases and commits
in the table above. None is enabled as a Cargo feature or production
dependency. The local profile enables only draft-22 compact behavior and the
existing RFC 9901 SHA-256/signature features; no default networking, native,
rendering, DID, storage or runtime feature is introduced.

License and provenance remain Apache-2.0 for SDK-authored code. Normative
draft material is quoted only as short provenance-bound vector inputs under
the IETF Trust legal provisions recorded by the fixture manifest. Oracle
outputs retain their exact upstream project, version, revision, license,
retrieval date and transformation; donor source is not copied.

Supply-chain evidence is the unchanged locked workspace graph, Cargo-deny and
RustSec routes, `cargo tree` cone receipt, source scan for unsafe/native code,
MSRV/primary compiler builds and portable target compilation. Candidate locks
from the qualification archive remain research-only.

Public and wire compatibility is additive and unreleased. Existing RFC 9901
entry points retain their behavior; the new facade boundary contains only
Identus-owned staged types, static errors and bounded evidence values. No
candidate, TypeScript DTO, protocol DTO, trust object or raw private material
is re-exported. Exact canonical wire output is newly identified as
`dc+sd-jwt`; legacy input is isolated to future issue #572.

Rollback removes the additive unpublished profile and its proposed ledger
record while preserving `identus-sd-jwt`, immutable research and vectors.

## Profile and compatibility evidence

Draft-22 requires protected `typ=dc+sd-jwt`, media type
`application/dc+sd-jwt`, required collision-resistant `vct`, optional unique
non-empty `aka_vcts`, and non-selective placement of `iss`, `nbf`, `exp`,
`cnf`, `vct`, `vct#integrity`, `aka_vcts`, and `status`. `sub` and `iat` may
be selectively disclosed. Key binding uses the existing RFC 9901 engine and
the holder key identified by caller-validated `cnf` evidence.

Issuer-key discovery is policy-selected. This slice models exact bounded
evidence for inline X.509 or JWT VC Issuer Metadata, including exact issuer
correlation and exclusive `jwks`/`jwks_uri`, but does not retrieve or trust it.
Type Metadata is optional and caller-supplied; the profile validates exact type
correlation, bounded extension chains and the claim rules needed for credential
validation, while display/rendering resources remain opaque or unsupported.
Status checking remains policy-owned; the profile exposes a validated
`status_list` binding and consumes caller-owned status evidence without
declaring the credential acceptable.

The released TypeScript package still names `vc+sd-jwt` in domain and OEA
format constants and in regression tests. That is a concrete migration need,
so issue #572 records an opt-in input-only adapter. This profile emits and
routes only `dc+sd-jwt`; no automatic fallback is permitted.

## Security, privacy and maintenance evidence

All raw profile claims, metadata, JWK-set input, URIs, arrays, extension
chains, claim paths, status references, and retained output receive explicit
byte/count/depth/work limits before proportional processing. Diagnostics and
receipts exclude credentials, claims, identifiers, URLs, key material,
metadata text, status handles, and comparison values. Remote retrieval,
redirects, decompression, cache behavior, trust anchors, issuer authorization,
rendering and telemetry are outside the crate.

The profile is an Internet-Draft under IESG evaluation. Its exact draft name,
date, URL and text hash are stored in source and fixture metadata. A later
draft or RFC opens a new issue/ADR and capability version; it cannot silently
change this implementation through a dependency range.

## Vector, quality and end-to-end plan

- Bind the draft-22 Appendix A material and SDK-authored negatives to the
  immutable text hash, section, retrieval date, transformation and expected
  result.
- Prove type matching, registered-claim placement, metadata inheritance,
  issuer evidence, status mapping, redaction, ordering, and exact boundaries.
- Fuzz profile claims, metadata, status, issuer evidence and base/profile
  transitions under bounded work.
- Benchmark ordinary, exact-boundary, early-rejection, extension-chain, and
  claim-metadata paths.
- Compare frozen outputs from at least two independent exact-version oracles;
  disagreements are resolved against draft-22.
- Execute an offline public-API journey from issuance through generic envelope,
  disclosure selection, OID4VP `dc+sd-jwt` capability matching, issuer/holder
  verification, profile validation, and redacted receipt.

The pending declaration ID is
`credentials.sd-jwt-vc.draft22.quality.v1`; implementation activates it only
after its routes exist and pass.

## Rejected or deferred candidates

HTTP clients, rendering/SVG execution, trust selection, issuer authorization,
credential acceptance policy, replay, persistence, custody, FFI, HAIP/mdoc,
automatic legacy acceptance, publication, certification, and downstream
mutation are deferred. JWS JSON Serialization remains outside the existing
RFC 9901 compact base.

## Open questions and blockers

No planning blocker remains. The legacy-consumer question is resolved by
issue #572. Implementation must stop rather than weaken the exact draft pin,
canonical identifier, staged evidence, redaction, or finite-work contract.

## Evidence commands

Planning uses `scripts/factory research-ready`, `constraints-ready`, strict
validation, and issue #569 `preflight --write`. Implementation adds focused
profile/base/OID4VP/OID4VCI tests, workspace Clippy/docs/tests, factory/Nix,
MSRV/target/dependency/advisory evidence, property/fuzz/benchmark/differential
routes, offline E2E, and independent Claude CLI review or immediate Codex CLI
fallback.

Exact planning commands are `scripts/factory doctor`, `research-ready
implement-sd-jwt-vc-draft22-profile`, `constraints-ready`, `validate`,
`check`, and `preflight ... --issue 569 --write`. Implementation commands are
the focused Cargo tests/bench, 300-second fuzz route, `cargo fmt`, workspace
Clippy/tests/docs, `nix flake check`, target/MSRV/dependency/advisory routes,
and Claude or Codex CLI review. Unrun checks at planning time are all Rust
implementation tests, fuzzing, benchmarks, target builds, Nix, hosted CI,
review, archive, merge and downstream interoperability; they are delivery
evidence and are not inferred from planning readiness.


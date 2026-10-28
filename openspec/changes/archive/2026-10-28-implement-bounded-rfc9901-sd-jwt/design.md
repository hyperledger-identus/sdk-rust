# Design

## Context

RFC 9901 defines stable selective-disclosure JWT issuance, combined compact
serialization, disclosure processing, presentation, and holder key binding.
The workspace already owns bounded compact JWS parsing and signature suites,
SHA-256, secure-random and time ports, a format-neutral credential envelope,
and generic presentation selection types. It does not own an SD-JWT graph or
proof engine.

ADR 0193 rejects a production dependency on the assessed engines. Exact
candidate versions remain differential oracles. The implementation must fit
Rust 1.89.0 MSRV and Rust 1.98.1 primary, forbid unsafe code, remain free of
network/native/runtime coupling, and keep draft-22 SD-JWT VC behavior out.

## Goals / Non-Goals

**Goals:**

- implement RFC 9901 compact issuance, disclosure processing and selection;
- make issuer and holder cryptographic evidence impossible to confuse with
  structural parsing through consuming staged types;
- reuse `identus-jose` and `identus-crypto`, without new cryptography;
- reject hostile input under explicit byte/count/depth/work/output budgets;
- keep diagnostics correlation-safe and secret-safe;
- prove a complete offline journey through credential and presentation seams;
- preserve replaceability of all private mechanics.

**Non-Goals:**

- SD-JWT VC claims, `dc+sd-jwt`, `vc+sd-jwt`, or JWS JSON Serialization;
- HTTP, metadata/key/status retrieval, DID resolution, trust, authorization,
  replay caches, storage implementation, consent, UI, agent runtime, or
  custody;
- new signature, digest, random, or key-generation primitives;
- WASM/UniFFI facades, target-runtime claims, publication, or consumer edits.

## Decisions

### One standards-named crate with inward dependencies

Add `identus-sd-jwt` with this dependency direction:

```text
identus-sd-jwt
  ├── identus-jose          compact JWS and signature capabilities
  ├── identus-crypto        SHA-256 and SecureRandom port
  ├── identus-core          time values and shared error surface
  ├── identus-credentials   bounded format-neutral envelope
  └── identus-presentations generic claim-selection plan
```

No existing generic crate depends on SD-JWT. Candidate/oracle crates never
enter the workspace graph. This avoids both historical domain names and a
second JOSE stack.

### Staged values consume earlier states

The public flow is deliberately directional:

```text
UnverifiedSdJwt
  --verify_issuer--> IssuerVerifiedSdJwt
  --process_disclosures--> DisclosureVerifiedSdJwt
  --select--> SdJwtPresentation
  --verify_key_binding--> VerifiedSdJwt + VerificationReceipt
```

Issuance returns an `IssuedSdJwt` containing the combined serialization and a
private disclosure inventory. `Debug` reports only counts and lengths. A
holder converts it to a bounded `CredentialEnvelope` with format `sd-jwt` and
must re-enter the verification stages before selecting a presentation.

Each transition consumes its prior state or borrows only immutable exact
bytes. Parsed values never implement a generic `verified` boolean. The final
receipt contains stage booleans, algorithms, counts, and limit profile only;
it contains no raw identifiers, audience, nonce, compact bytes, digests,
claims, salts, signatures, or keys.

### Structured claims and claim paths, not donor DTOs

Issuer claims are a bounded JSON object. An `IssuancePlan` contains validated
`ClaimPath` values made of object-member or array-index segments, plus bounded
decoy locations. Disclosure transformation walks the input once, rejects
reserved `_sd` and `_sd_alg` use, and creates object or array disclosures using
RFC 9901 shapes.

Selection uses the same path vocabulary and computes the transitive disclosure
closure. An adapter maps generic `identus-presentations` selected-claim intents
to object-member paths without inventing a string convention for array indexes;
array-aware adopters use typed `ClaimPath` values until the generic vocabulary
distinguishes member names from indexes. TypeScript DTOs and errors do not enter
the crate.

### Explicit effect capabilities

Issuance borrows `SecureRandom` for salts and a `JwsSigner` for the issuer.
Key-binding construction borrows a holder `JwsSigner` and receives a caller-
owned `UnixTimestampMillis` converted to exact whole JWT seconds. Verification
borrows `SignatureSuiteRegistry`, algorithm-bound issuer/holder public keys,
and explicit `KeyBindingExpectations`.

`DisclosureHasher` is a small object-safe capability. The built-in
`Sha256DisclosureHasher` delegates to `identus_crypto::sha256`; parsing accepts
only exact `sha-256` in this slice and returns `UnsupportedHashAlgorithm`
before disclosure work for any other value. This keeps RFC dispatch explicit
and makes differential/fault testing possible without implementing a new
digest.

### Exact compact ownership

The parser retains exact issuer JWS and disclosure segments because RFC 9901
key binding hashes the exact presentation serialization. Parsing splits the
combined form under a segment/count budget before decoding. A no-key-binding
presentation ends with the required separator; a key-binding JWT, when
present, is the final non-disclosure segment. No serializer reconstructs a
received issuer JWS from decoded JSON.

### One-pass disclosure index and finite graph traversal

Each disclosure is decoded once under its segment and decoded-byte budgets,
validated as the exact object-member or array-element shape, hashed once, and
inserted into a digest index. Duplicate encoded disclosures, duplicate digest
keys, and collisions fail. The engine then traverses issuer payload references
with an explicit stack, visited set, depth/node/member/work counters, and a
retained-output writer. Every supplied disclosure must be reached exactly once.

Array placeholders and object `_sd` arrays are validated before mutation.
Reserved-claim checks run on issuer input, disclosure values, and recursively
materialized objects. The engine commits no partial disclosed output on error.

### Public default limits

`SdJwtLimits` validates all fields and ships conservative defaults within the
generic credential-envelope ceiling:

| Boundary | Default |
| --- | ---: |
| combined compact bytes | 1,048,576 |
| issuer JWS bytes | 524,288 |
| key-binding JWS bytes | 65,536 |
| encoded disclosure bytes | 131,072 |
| decoded disclosure bytes | 98,304 |
| disclosure count | 256 |
| decoy count | 256 |
| salt bytes | 16 minimum, 64 maximum |
| issuer JSON bytes | 524,288 |
| JSON members | 4,096 |
| JSON nodes | 16,384 |
| JSON depth | 32 |
| selected paths | 256 |
| audience or nonce bytes | 1,024 each |
| retained output bytes | 1,048,576 |
| aggregate work units | 1,000,000 |

Encoded-length arithmetic uses checked operations. Cheap raw-size and segment
count checks precede base64/JSON/hash/signature work. Each decoded byte, JSON
node/member, digest calculation, edge visit, selection match, and emitted byte
charges a documented work unit so syntactically valid amplification is finite.

### Static error taxonomy

`SdJwtError` is non-exhaustive and maps to stable `sd_jwt.*` codes. Categories
cover compact structure, encoding, JSON, reserved claims, unsupported hash,
disclosure shape/reference/duplicate/collision/unused/ambiguity, invalid
selection, issuer signature, key-binding structure/signature/type/hash/
audience/nonce/time, effect rejection/unavailability, resource limit, and
internal invariant. `Display` is static; `Debug` may add stage or limit kind
but never remote values. JOSE/crypto provider errors map by category rather
than becoming public sources with raw details.

### Evidence lives beside the capability

RFC-derived vectors carry the RFC URL, retrieval date, text SHA-256, section,
license notice, and transformation. SDK-authored negatives are labeled
clean-room. Frozen oracle outputs live outside the production dependency graph
and name exact versions.

The quality declaration routes:

- official and negative vectors through crate integration tests;
- round-trip, selection-closure, ordering, and staged-state properties;
- compact, disclosure, graph, selection, and key-binding fuzz targets;
- normal, exact-boundary, and adversarial benchmarks;
- two immutable independent oracle packets with an RFC resolution ledger;
- native/MSRV/portable compile evidence; and
- an offline E2E through `CredentialEnvelope` and
  `PresentationDisclosurePlan` using deterministic injected effects.

The E2E is required on the host fast path. Portable compile evidence does not
become runtime, packaging, binding, device, or certification evidence.

### Decomposition note

This cohesive slice intentionally spans more than twelve files because the
repository requires protocol code, executable evidence, machine-checked
architecture inventories, immutable provenance, fuzz wiring, and the
OpenSpec task record to land together. Production ownership remains confined
to the `identus-sd-jwt` crate's seven focused modules; conformance, property,
rejection, and end-to-end routes remain separate test targets. The largest
production module stays below the 1,000-authored-nonblank-line attention
threshold. Splitting the evidence from the implementation would create a
temporarily unqualified security-sensitive protocol surface, while SD-JWT VC,
bindings, networking, trust policy, persistence, and publication remain in
separate future slices.

## Risks / Trade-offs

- **[Local protocol maintenance]** The SDK owns RFC mechanics rather than a
  dependency. **Mitigation:** keep the crate focused, preserve oracle packets,
  and make private mechanics replaceable behind staged Identus types.
- **[JSON/value retention]** `serde_json::Value` can amplify input.
  **Mitigation:** preflight raw/decoded bytes, traverse with explicit counters,
  serialize through bounded writers, and enforce retained-output/work limits.
- **[Error over-redaction]** Static errors reduce field diagnostics.
  **Mitigation:** expose safe stage, category, count, and limit-kind evidence;
  keep sensitive comparisons opaque.
- **[Selection semantics drift]** Generic presentation intents do not define
  RFC paths. **Mitigation:** one explicit mapping adapter, path validation, and
  E2E/property evidence; do not embed TypeScript DTO shapes.
- **[Target overclaim]** Pure Rust compile success can be mistaken for product
  support. **Mitigation:** retain SDK-LIM-003 and label compile/runtime/package
  evidence separately.

## Migration Plan

1. Land planning, provenance, constraints, API/error/resource contract, and an
   exact preimplementation receipt.
2. Add the unpublished crate and focused unit/vector tests.
3. Add property/fuzz/benchmark/differential/target evidence and the offline
   public-seam E2E.
4. Activate `IDR-020` and the quality declaration only after all required
   routes pass; archive the OpenSpec change and merge to `develop`.
5. Keep #569 blocked until this crate is merged. Publication, bindings, and
   downstream adoption remain separate.

Before publication, rollback removes the additive crate and restores the
capability ledger while preserving vectors, ADR 0193, and failure evidence.
After publication, any behavior change requires a new compatibility decision.

## Open Questions

No product blocker remains. Exact default limits may move downward only when
boundary benchmarks show that the planned values exceed the repository's
resource budget; such a change must update this contract before implementation
and preserve RFC example acceptance. Any need for a non-SHA-256 digest, JWS
JSON Serialization, or legacy profile identifier becomes a separate issue.

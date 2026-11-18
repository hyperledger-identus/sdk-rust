# Design

## Dependency direction

```text
identus-did / identus-crypto
    -> identus-did-peer explicit profiles
        -> identus-wasm-did synchronous generated API v4
            -> private TypeScript peer-profile projection/provider
                -> browser / Node.js / React loaders and fixtures
```

Portable crates gain no WASM, npm, TypeScript, React, network, storage,
executor, or native dependency. `identus-wasm-did` adds the existing workspace
peer-method crate and reuses its DID Document serializer/projection seam.

## Explicit generated operations

The generated API SHALL use profile names, not an algorithm parameter:

- create numalgo 4 from one bounded input-document JSON string;
- parse/resolve a numalgo-4 long form and return long form, derived short form,
  normalized input document, and normalized resolved DID Document;
- resolve a numalgo-4 short form only with caller-supplied matching long form;
- create numalgo-2 compatibility from bounded public-key/service JSON;
- parse/resolve a numalgo-2 compatibility DID and return the normalized
  identifier plus resolved DID Document.

Implementation may combine create with parse/resolve internally, but the
reviewed declaration and raw-binding interface SHALL preserve explicit public
operation identities. Generated views own copied strings only, expose closed
getters, and are freed exactly once. Binding API version increments to 4.

## Structured TypeScript inputs

The public TypeScript API exposes readonly profile-specific inputs. Numalgo 4
accepts a root document object without `id`; it is closed-serialized after
iterative cycle, prototype, unsafe-number, depth, node, member, and byte
preflight. Rust remains authoritative for duplicate names in raw bytes,
private JWK-member rejection, contextualization, and document validity.

Numalgo-2 compatibility accepts a non-empty bounded list of public multibase
keys with closed purpose names and a bounded list of compact services with URI
or DIDComm endpoint-map values. It accepts no raw key bytes, private keys,
seeds, entropy callbacks, or generic algorithm number. Rust validates codec,
purpose, service, identifier, and projection semantics.

Input serialization uses a shared internal bounded JSON utility extracted from
the existing strict resolution-option path only where doing so does not widen
the public contract. Duplicate semantic validators are avoided; method rules
stay in Rust.

## Strict results and DID Document reuse

The provider returns frozen plain discriminated values:

- numalgo-4 value: profile, long-form DID, short-form DID, and immutable
  resolved DID Document;
- numalgo-2 compatibility value: profile, DID, and immutable resolved DID
  Document.

Parsing and creation both return the same profile value shapes. The existing
`DidValue` and DID Document projector are reused. Normalized JSON is semantic
deterministic output; no original formatting or RFC 8785 claim is made.
Unknown generated fields, shapes, error codes, unsafe integers, or ownership
drift fail as binding-contract errors.

## Stable errors and observations

Generated Rust failures map the crate's closed categories to stable TypeScript
codes for wrong profile, unsupported algorithm, malformed encoding,
unsupported codec, unsupported purpose, invalid input document, hash mismatch,
long-form required, resource limit, and projection failure. Binding-contract
drift remains distinct. Messages are constant and contain no input.

Observations add explicit create/parse/resolve operation names and versioned
profile mapping IDs. They contain route, result, stable code, and bounded
duration only. Observer failure occurs after generated
ownership closes and never retries, changes profile, or selects a fallback.

## Donor dispositions and pairwise fixture

The donor ledger records all production/test paths, immutable blob/SHA-256,
authority, disposition, rationale, replacement evidence, owner, and
reconsideration trigger. Deterministic numalgo-2 create/resolve/document cases
are adapted; codec-only tests are superseded; signature/random-secret tests and
agent/mediator/storage behavior are excluded or platform-owned. Numalgo 4 is a
Rust addition, not donor-equivalent.

The pairwise fixture uses fixed public Ed25519 and X25519 multibase values plus
the donor DIDCommMessaging service endpoint/routing/accept values. It asserts
the deterministic numalgo-2 identifier, resolved document relationships,
compatibility key IDs, and service projection. Two resulting/parsed peer DID
values cross a minimal frozen `{from, to}` request-shaped DTO. The test does
not create, pack, route, send, store, or process a DIDComm message.

## Vectors and quality evidence

The existing `did.peer.profiles.v1` packet is expanded or versioned without
changing existing case meaning. It covers current numalgo-4 create/long/short
resolution, exact donor numalgo-2 create/resolve/document behavior,
wrong/unsupported profiles, malformed/hash/long-form failures, invalid public
material/services, exact/over limits, and redaction. One assertion body runs
the active cross-target cases.

The core `did.peer.profiles.quality.v1` remains authoritative for Rust
property/fuzz/benchmark/differential routes. The new
`did.peer.typescript.quality.v1` declaration owns adapter structural
mutations, generated ownership, target equivalence, artifact impact,
pairwise-shaped proof, observations, and rollback. Coverage and mutation
evidence are supporting evidence, not normative authority.

## Target packaging and measurements

Artifact assembly builds independent web and nodejs outputs, requires API v4
peer symbols, and copies a closed inventory. Browser, Node ESM/CommonJS, React
SSR, and hydrated React execute the same assertions through their real loaders.
React contains no method logic. React Native stays inapplicable.

Measurements record before/after compressed and uncompressed WASM/package
bytes, artifact hashes, cold/warm initialization, create/parse/resolve/project
p50/p95, normalized bytes, and reliable RSS. These are measurement-only. If
the shared-artifact delta exposes an accepted operational or dependency-cone
problem, the implementation stops and reopens the conditional separate-
artifact decision; it does not invent a threshold.

## CI isolation and rollback

Binding-impact routing includes `identus-did-peer`, WASM binding/declarations,
TypeScript inputs/projection/provider, vectors, mapping/quality/change records,
snapshots, fixtures, and assertions. Unknown impact selects all targets.
Rust core completes independently; selected package/browser/node/react jobs
do not cancel siblings; the terminal verdict requires all selected receipts.

Rollback chooses the provider without peer-method capabilities before
initialization and removes the unpublished additive route. Operations do not
fall back after profile, projection, observer, or binding failure.

## Cohesion and decomposition

The train is one reviewable invariant spanning explicit profile identity,
public-material input, canonical Rust validation, generated ownership, strict
projection, and identical target behavior. Splitting those layers would expose
an API-version mismatch or evidence not bound to the public contract. Planning,
Rust/WASM, TypeScript, conformance, and harness remain signed checkpoints.

Peer and PRISM remain separate because their authority, effects, consumers,
security, and rollback differ. DIDComm, native, release, and consumer adoption
also remain independent.

## Delivery boundary

This is a local-first issue #603 train on Rust 1.99.1. It may be pushed only
after complete applicable local evidence and distinct review. Merge requires
protected exact-head checks and independent maintainer review. Publication,
release, `main` promotion, default activation, support, or consumer mutation
requires separate explicit authority.

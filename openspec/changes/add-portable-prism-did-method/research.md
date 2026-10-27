# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-26
Source retrieval date: 2026-10-26
Research blockers: none

## Problem and existing implementation

The current implementation at repository revision
`5da04223c1c4e029cb368870877e56d699d40388` has generic DID values, method
dispatch and the necessary cryptographic primitives, but no `did:prism`
method crate. NeoPRISM resolves the method in Rust, and SDK-TS is current
consumer evidence, but both combine or expose ownership that does not belong
in a portable SDK-Rust crate.

The method must arrive as a cohesive new facade rather than code inside
`identus-did`: generic DID Core stays method-neutral, while Cardano history,
VDR semantics and hosted publication stay outside the method crate. Current
consumer behavior is migration and differential evidence, not a command to
copy TypeScript DTO or error shapes.

## Normative sources

- PRISM DID method specification
  `input-output-hk/PRISM-did-method-spec@fdd9007e23dc08d94d1a2b66b2dc26d1db499422`.
- NeoPRISM donor
  `MediaNoxLabs/neoprism@a2ddbc5a8a316841e2c8df3831e7904fef01c356`.
- SDK-TS consumer evidence
  `hyperledger-identus/sdk-ts@005c6664de1e399aca723c6a33c1b005c70c3fa1`
  (`@hyperledger/identus-sdk@8.1.4`).
- Protobuf runtime candidate `prost 0.14.4`.

Primary source URLs are
https://github.com/input-output-hk/PRISM-did-method-spec/tree/fdd9007e23dc08d94d1a2b66b2dc26d1db499422,
https://github.com/MediaNoxLabs/neoprism/tree/a2ddbc5a8a316841e2c8df3831e7904fef01c356,
and
https://github.com/hyperledger-identus/sdk-ts/tree/005c6664de1e399aca723c6a33c1b005c70c3fa1.
The pinned source revision is part of every differential receipt. PRISM draft
currency is intentionally frozen for this delivery; a later draft is a new
reviewed compatibility decision.

The repositories and `prost` are Apache-2.0. File-level license and provenance
for adapted code and fixtures must be recorded before implementation is ready.
The exact version and features for the runtime candidate are `prost 0.14.4`
with derive support and no build-time code generator in adopter builds.

The specification is normative for method behavior. NeoPRISM is donor and
oracle evidence; SDK-TS is current consumer/migration evidence. Neither
implementation dictates Rust DTO, error or module shapes.

## Candidate decisions

The relevant NeoPRISM method library has approximately 2,600 authored nonblank
production lines and 6,000 test lines. Its behavior is cohesive enough for one
5–10k-line extraction after excluding storage, DLT, node and deployment code.
The reusable sources are DID syntax, SSI operation parsing, document
projection and DID-only protocol reduction.

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Separate `identus-prism-did` crate | `adopt` | Keeps method semantics reusable and generic DID Core method-neutral. | Multiple DID methods prove a smaller shared method framework is necessary. |
| NeoPRISM method code and tests | `conditional-adopt` | Adapt cohesive behavior and vectors after removing node, storage and Apollo ownership. | Donor behavior conflicts with the pinned specification or bounded SDK policy. |
| SDK-TS public DTO and errors | `oracle` | Useful migration evidence, but Rust owns idiomatic domain and redacted error contracts. | A separately approved compatibility facade is requested. |
| Private `prost 0.14.4` messages | `adopt` | Small, portable runtime cone with no adopter-time `protoc`. | A smaller audited decoder proves exact-wire and target parity. |
| Raw protobuf scanner | `retain-local` | Required to reject duplicate singular fields and retain exact signed bytes. | `prost` exposes equivalent bounded duplicate-aware exact ranges. |
| NeoPRISM package/dependency boundary | `not-adopt` | It mixes DID, storage, Cardano, tracing, OpenAPI and Apollo concerns. | Never without a new architecture decision. |
| Unbounded draft protobuf acceptance | `not-adopt` | Conflicts with hostile-input and portable resource governance. | The normative method publishes finite limits that fit supported targets. |

Do not copy these donor edges:

| Donor edge | SDK-Rust decision |
| --- | --- |
| Apollo crypto/hash/JWK | replace with `identus-crypto` |
| donor DID document model | replace with `identus-did` |
| `rust-protobuf` | replace with private `prost 0.14.4` messages |
| `chrono` and Cardano cursor | exclude; ordering is preserved by signed action order and state insertion order, while ledger metadata remains adapter-owned |
| `regex` | bounded ASCII validators |
| `im-rc` | insertion-ordered bounded vectors and clone-then-commit |
| `enum_dispatch` | ordinary enums and direct matches |
| tracing/OpenAPI | adapter-owned and excluded |

## Compatibility and dependency evidence

The public change is additive: a new `identus-prism-did` crate, new method
types and a method binding. The wire format stays compatible with the pinned
protobuf and exact inbound bytes are preserved. SDK-TS repeated endpoint DTOs
need an adapter for multiple values; they do not alter the canonical Rust
facade boundary.

The proposed direct dependency cone is `identus-core`, `identus-crypto`,
`identus-did`, `prost 0.14.4`, `base64`, `serde` and
`serde_json`, reduced further where implementation proves a dependency is
unnecessary. The resolved dependency cone adds `bytes` and prost derive
machinery already present only at compile time. No regex, runtime, network,
database, Cardano, OpenAPI or persistent collection dependency is accepted.

`prost 0.14.4` declares MSRV Rust 1.85, below the repository Rust 1.89 minimum
and 1.98 etalon. Target evidence must include native, `wasm32-unknown-unknown`,
iOS and Android library compilation. Generated messages stay private; the
facade boundary exposes validated Rust values and exact byte slices, never
generated protobuf structs.

## Security, privacy and maintenance evidence

All wire lengths, collection cardinalities, UTF-8 amplification, JSON shape
and projection amplification are bounded before expensive work. The 96,000-byte
operation ceiling derives a 128,075-byte long-form DID; at the maximum 49
document keys and 50 services, 198 controller occurrences plus one operation
budget estimate 25,454,850 bytes. A 32 MiB native projection ceiling preserves
finite headroom without changing generic 256 KiB document or 512 KiB
resolution-result JSON ingress. Duplicate
security-relevant protobuf fields and JSON names fail closed. Signature and
hash verification use exact inbound operation bytes. Errors and `Debug` never
reflect DIDs, endpoints, key bytes, signatures or opaque payloads.

The candidate adds no unsafe code, native code, build script in consumer
builds, network access or secret ownership. `identus-crypto`, `prost`,
`base64`, `serde` and `serde_json` are the direct supply-chain
surface; the existing `identus-crypto` hex feature supplies canonical digest
text through its bounded `hex` dependency. Exact feature and resolved-package
evidence must be captured from the final lockfile. Private generated files are
reproducible maintainer artifacts with pinned source digests.

The maintenance, release and security posture is one independently versioned
crate owned by SDK-Rust. A focused revert is the rollback before publication.
After publication, an unsafe historical acceptance is rejected and documented
rather than simulated; source-compatible fixes use the compatibility ledger.

## Confirmed donor deviations

- NeoPRISM counts total historical key/service map entries against limits that
  the specification defines for active entries only.
- NeoPRISM silently drops an update action whose oneof is missing; the
  specification requires every action to be well constructed.
- NeoPRISM checks endpoint bytes against a character limit, rejecting valid
  bounded Unicode values.
- Its hash-map iteration does not define stable document ordering.

The SDK preserves bounded tombstones and no-ID-reuse semantics while correcting
these deviations with property tests and executable differential wire vectors.

## Wire feasibility evidence

`prost 0.14.4` is Apache-2.0, declares Rust 1.85 and has a small direct runtime
cone (`bytes` plus optional derive). `rust-protobuf 3.7.2` does not declare an
MSRV and adds `protobuf-support`, `once_cell` and `thiserror`. Checked-in
private messages plus maintainer-only deterministic regeneration avoid a
consumer-time `protoc` dependency.

A disposable scanner already preserves exact nested operation/payload bytes,
rejects duplicate security-relevant fields, recognizes DID/protocol/VDR tags,
and verifies a pinned NeoPRISM secp256k1 signature through `identus-crypto`.
It passed focused tests, all-target clippy and a WASM check. Diagnostics on an
Apple M2 Max measured about 8.18 million small envelope scans/second and about
17.9 thousand parse-plus-real-signature verifications/second. These are
feasibility observations, not release gates.

## Specification decisions

The draft requires ordered atomic update actions, non-reusable historical IDs,
active-only 50-key/50-service limits and at least one active master key after
updates. It has no finite context or operation-size ceiling. SDK-Rust therefore
publishes the explicit finite profile in `constraints.md`.

The draft service endpoint is one string containing a URI, JSON object or
non-empty JSON array. SDK-TS 8.1.4 uses repeated protobuf strings; one value is
wire compatible, while multiple occurrences are not equivalent to the draft's
single JSON-array string. Rust follows the pinned specification. A TypeScript
migration adapter may encode multiple DTO values as one canonical JSON array.

## Rejected or deferred candidates

Cardano scanners, transaction metadata, short-form history persistence, VDR
payload semantics, protocol-version administration, hosted registration,
HTTP, SQL, CLI, runtime orchestration, SDK-TS changes and NeoPRISM deletion are
deferred to their owning issues. A permissive duplicate-field compatibility
mode is deferred unless immutable ledger evidence proves it necessary.

## Open questions and blockers

No research or implementation-design blocker remains. The historical Preprod
packet pins two exact signed create operations and their public ledger
positions; strict duplicate-field rejection has been confirmed against both.
The four imported protobuf sources and private Rust projection are digest-pinned, and the
executable projection budget is recorded above. Remaining work is delivery
evidence: exact post-#524 preflight, official Nix/slow lanes, independent review
and protected CI.

The reconsideration trigger for the finite profile is a later normative PRISM
revision or attributable historical operation that exceeds a chosen limit
without violating an explicit protocol limit. The trigger for replacing the
local scanner is an audited crate that preserves exact ranges, rejects
duplicates and passes all supported targets.

## Evidence commands

Planning commands are `scripts/factory research-ready`,
`scripts/factory constraints-ready`, `npx --yes @fission-ai/openspec validate
add-portable-prism-did-method --strict`, and exact issue preflight after #524.
Implementation commands will include focused crate tests, all-target/all-feature
clippy, rustdoc, dependency inspection, vector mutation tests, fuzz target
checks, WASM/native/mobile compilation and directional benchmarks. Historical
fixture verification, final dependency-cone capture, generated-source drift,
official Nix gates, independent CLI review and protected CI are unrun checks and are not
claimed by this draft.

The candidate file and fixture inventory is recorded in `provenance.md` with
SHA-256 digests at the pinned donor revision. The historical packet identifies
its public evidence comments, harness revision and original manifest digest;
it does not relabel generated unit data as ledger evidence.

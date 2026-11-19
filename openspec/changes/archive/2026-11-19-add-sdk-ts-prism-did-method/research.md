# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-19
Source retrieval date: 2026-11-19
Research blockers: none

## Problem and existing implementation

The current implementation in `identus-prism-did` already owns the portable
method contract: bounded
canonical and self-certifying long-form syntax, exact protobuf operation
bytes, public-material create builders, signed create/update/deactivate
validation, authenticated atomic state reduction, finite tombstones, and
generic DID Document projection. The crate has no Cardano, VDR persistence,
HTTP, executor, database, or node dependency.

The private TypeScript package and `identus-wasm-did` already own strict DID,
DID URL, document, resolution, dereferencing, peer-method, target, provider
selection, and rollback boundaries. They do not expose PRISM method operations.
This is a language-adapter and host-effect gap, not a request for a new PRISM
engine.

## Normative sources

- The primary source URLs are
  <https://github.com/input-output-hk/prism-did-method-spec> and
  <https://www.w3.org/TR/did-core/>. The selected protocol/draft currency is
  the repository-pinned PRISM method contract and W3C DID Core 1.0; a
  normative revision is a reconsideration trigger rather than an implicit
  behavior change. ADRs 0174 through 0176 and `identus-prism-did` are the
  accepted implementation authority.
- SDK-Rust base `0482b7066a8806e588cd9494e903f5aee1e4c324` is Apache-2.0 and
  contains the selected portable implementation and immutable Preprod packet.
  This is the pinned source revision and provenance for the train. The exact
  workspace MSRV is Rust 1.99.1. `identus-prism-did` version `0.0.0` has the
  exact default feature set (no crate features) and direct workspace
  dependencies on base64, core, crypto with ed25519/hash/hex/jwk/secp256k1/
  x25519 features, DID, prost, serde, and serde_json.
- SDK-TS `4bf86ebf69d5e96616a148e4c973f831f95fa38e` (`8.1.4`, Apache-2.0)
  is consumer-regression evidence only. Relevant blobs include long-form
  syntax `6bb04c7c2eb54da77fed486a520e58636cc5c453`, method identifier
  `d9d3c41c9158a857f59aa432ba2f506cfbe91cdd`, long-form resolver
  `03734061bc02aaca977748189e1298d1caab545b`, canonical resolver
  `008e02399da0e3ef4bcbc4d134024a5fa94a3b83`, and PRISM tests
  `53e2216f61876e28f87239de0bc5575d880bb608`.
- NeoPRISM PR #324 merged as
  `41b7f1991a2665c3cb9d0d994d861f9a32f07727`; PR #346 merged as
  `83df4e4fba0070e946ed40b190d8d92c401614c9`. They prove the
  downstream boundary and adoption plan, not TypeScript or SDK mutation
  authority.

The direct and resolved dependency cone remains workspace-owned: the binding
adds one inward `identus-prism-did` edge and no new Cargo engine, npm runtime,
native library, or network client. `Cargo.lock` and the Nix closure retain the
resolved dependency identity. License and provenance remain Apache-2.0 and
repository source-bound for SDK-Rust, SDK-TS, and NeoPRISM evidence.

## Donor disposition

| Source | Disposition | Reason |
| --- | --- | --- |
| SDK-TS long-form value and resolver | `adapt` | Preserve selected identifier and local initial-state outcomes through capability-named immutable values; do not copy classes or parser logic. |
| SDK-TS canonical resolver | `adapt` | Preserve injected resolution outcome while moving portable reduction to Rust; networking and API policy remain host-owned. |
| SDK-TS create-with-keys flows | `platform-owned` | Key derivation, private key custody, persistence, and agent lifecycle remain outside the facade. |
| SDK-TS PRISM syntax tests | `adapt` | Re-express applicable public identifier outcomes against canonical Rust vectors. |
| NeoPRISM exact-byte/state adapter | `conformance-only` | Use its immutable boundary and differential evidence; do not copy Cardano, storage, indexing, timestamps, or node composition. |
| A second PRISM parser/reducer | `not-adopt` | Duplicates the accepted Rust owner and creates split authority. |

Every discovered donor journey will receive an immutable adapted,
platform-owned, superseded, or excluded row before closeout. Historical names,
DTOs, generic thrown text, and donor-generated protobuf types are not API
authority.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Extend the existing DID WASM/package artifact | `adopt` | Reuses reviewed projection, lifecycle, target loaders, and rollback with one inward workspace edge. |
| Add a second PRISM parser/reducer | `not-adopt` | Duplicates the accepted Rust authority and expands behavior/supply-chain ownership. |
| Accept bounded public material for deterministic create | `adopt` | Preserves custody neutrality and makes the portable operation explicit. |
| Accept private keys, entropy, or signing callbacks | `not-adopt` | Violates the secret-neutral facade and creates an unnecessary custody boundary. |
| Resolve canonical DIDs through an injected exact-history effect | `adopt` | Keeps chain/network policy outside while Rust authenticates and reduces state. |
| Accept host-projected documents or replacement identifiers | `not-adopt` | Bypasses correlation, signature, state, and projection authority. |
| Reuse the generic DID Document projection | `adopt` | Prevents a second DTO/validator and preserves lossless semantics. |
| Treat NeoPRISM as a Cardano/platform donor | `not-adopt` | Its chain, persistence, indexing, timestamps, and service composition are separately owned. |
| Add React Native through browser WASM | `not-adopt` | Contradicts the accepted native-binding target contract. |

The reconsideration trigger is a normative PRISM revision, a proved gap in
`identus-prism-did`, or measured evidence that the shared WASM artifact
violates an accepted target budget. Publication, default activation, native
bindings, custody, Cardano/VDR work, or mdoc each requires a separate decision.

The build-versus-adopt result is to build only the thin binding and adapter
over the existing Rust owner:

Build only the thin binding and TypeScript adapter over the existing Rust
owner:

```text
identus-did / identus-crypto
    -> identus-prism-did
        -> identus-wasm-did generated API v5
            -> private PRISM capability provider
                -> browser / nodejs / react-web
                -> injected portable-history effect
```

No dependency points inward from a platform package. Extending the existing
DID artifact reuses reviewed projection, disposal, initialization, target, and
rollback behavior. A separate artifact is reconsidered only if measured
dependency/size evidence violates an accepted budget; no such budget exists.

## Selected public boundary

The facade exposes explicit operations to:

- parse canonical or long-form PRISM identifiers into exact immutable values;
- construct one deterministic long-form identifier from bounded public keys,
  services, and contexts, returning its canonical form and local projection;
- locally reduce the embedded long-form initial state;
- reduce a bounded ordered history of exact signed operation encodings for a
  canonical or long-form identifier and project active/deactivated state; and
- resolve through an injected effect that returns only that portable history.

The host effect receives the validated exact DID plus cancellation signal. It
does not return documents, replacement identifiers, trust, timestamps, ledger
coordinates, or policy. Rust correlates every operation to the requested DID,
verifies signatures/history, applies finite state rules, and projects the
document. Long-form initial state remains local.

## Compatibility and dependency evidence

The binding API increments additively from v4 to v5. Existing DID/peer methods
and strict public types remain compatible. New mapping
`did.prism.typescript.v1` is one-way from strict TypeScript public material or
host history to canonical Rust semantics and back to frozen capability values.
Generated owners are copied and freed exactly once; generated classes never
cross the package API.

Stable failures preserve the four canonical Rust categories—syntax, wire,
operation, and state—plus closed host unavailable/failure, abort, observer,
and binding-contract categories. Messages and observations contain no DIDs,
operations, keys, services, documents, generated errors, or stacks.

## Security, privacy and maintenance evidence

TypeScript preflights closed objects, cycles, prototype keys, unknown fields,
unsafe numbers, string bytes, collection sizes, depth, and nodes before
serialization. Rust revalidates key curves/usages, service/context syntax,
operation bytes, long-form hash linkage, signed envelopes, authorization,
history linkage, active-state invariants, tombstones, and projection budgets.

The history effect result is finite by operation count, per-operation encoded
bytes, aggregate encoded bytes, and decoded Rust limits. Cancellation is
checked before and after the host effect. Failure never retries, falls back,
or retains partial authority. Test fixtures contain public key material and
published signatures only; no secret enters logs or receipts.

The existing `did.prism.quality.v1` remains authoritative for Rust property,
fuzz, benchmark, and differential evidence. The implementation adds pending
`did.prism.typescript.quality.v1` for adapter structure, generated ownership,
host injection, target equivalence, artifact impact, observations, and
rollback.

The affected crates forbid unsafe code and add no native code or native
dependency. Supply-chain evidence consists of exact Cargo/npm locks, Nix
closure, source-distribution checks, generated symbol/file inventories,
immutable donor hashes, signed+DCO commits, and exact-head target receipts.
Maintenance, release, and security posture remain the existing SDK policies;
this private facade makes no support, publication, stable-API, production, or
certification claim.

## Rejected or deferred candidates

Cardano clients, transaction submission, VDR storage semantics, indexers,
databases, HTTP, ambient fetch, cache/finality/retry policy, timestamps,
private keys, signing, derivation, custody, node composition, SDK-TS mutation,
default activation, React Native, publication, support, certification,
release, and mdoc are rejected or deferred to independent owners.

## Evidence commands

Planning uses:

```text
scripts/factory research-ready add-sdk-ts-prism-did-method
scripts/factory constraints-ready add-sdk-ts-prism-did-method
scripts/factory validate add-sdk-ts-prism-did-method
scripts/factory preflight add-sdk-ts-prism-did-method --issue 610 --write
```

Implementation will run focused PRISM/WASM tests, package/type snapshots,
Browser, Node.js, React SSR/hydration, vectors, mappings, quality, change-ledger,
impact-plan, source-distribution, factory, and complete Nix gates. Exact
commands and unrun gates will be retained at execution time.

## Open questions and blockers

There are no blockers to the private repository-local train. External
activation, accepted budgets, native bindings, publication/support, release,
and downstream mutation remain separate protected decisions.

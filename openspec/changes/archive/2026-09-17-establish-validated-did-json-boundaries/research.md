# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

The current implementation at exact base
`fee94946ca489f88dbc50282b5ee1eca095507f3` has public
`OneOrMany<T>` preserves scalar/array shape but also rejects arrays above the
DID-specific `MAX_DOCUMENT_ITEMS = 128`. Public
`ContextEntry::Object(BTreeMap<String, Value>)` can meanwhile represent JSON
far beyond `MAX_EXTENSION_DEPTH = 32` and `MAX_EXTENSION_NODES = 4096`.
Therefore generic early collection rejection can recursively destroy hostile
JSON before DID validation owns the path.

The repository-wide occurrence audit classified the relevant surfaces:

| Occurrence | Classification | Current failure ownership |
| --- | --- | --- |
| `OneOrMany<T>` and its Serde `Wire<T>` | wire/output representation plus misplaced DID policy | generic function implicitly drops rejected `Vec<T>` |
| `ContextEntry` | public domain type containing unvalidated input | raw map can outlive every intended DID budget |
| `ServiceEndpoint` wire enum | input staging into public domain type | validates after staging; native `Service::new` owns rejection |
| verification/service/document maps | public validated domain candidates | constructor/builder drops rejected recursive values |
| resolution problem/metadata/content maps | public validated domain candidates | constructor/builder drops rejected recursive values |
| query option maps | public validated domain candidates | constructor/builder drops rejected recursive values |
| registration public data | public validated domain candidate | constructor drops rejected recursive values |
| `JsonBudget` and `validate_json_*` | borrowed internal validation | does not own or safely destroy rejection |
| bounded `from_json_slice` scanners | hostile wire/input boundary | rejects depth/nodes before typed materialization |

The complete native owned-JSON family is
`VerificationMethod::new`, `Service::new`, `ContextObject::new`,
`DidDocumentBuilder::build`; `DidResolutionError::new`, operation metadata
constructors, `DidDocumentMetadataBuilder::build`, `DereferencedContent::new`,
`DidUrlContentMetadata::new`; resolution/dereferencing option constructors and
builders; and `RegistrationPublicData::new`. On success each returns values
within existing budgets. On failure current source implicitly drops the owned
candidate; hostile depth may recurse. Bounded wire-slice parsing already owns
the correct pre-materialization boundary and remains preferred for hostile
bytes.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Structural `OneOrMany`, opaque validated `ContextObject`, shared private rejection guard/worklist | sdk-rust base above | `adopt` | Removes the abstraction collision, makes public context JSON bounded by construction, centralizes rejection, and preserves accepted wire. | A future non-JSON representation replaces JSON-LD contexts. |
| Return rejected generic ownership from `OneOrMany::try_many` | local design | `conditional-adopt` | A sound low-level contract, but unnecessary once generic shape no longer owns DID policy and insufficient to make invalid context states unrepresentable. | Another generic fallible policy genuinely requires caller-controlled cleanup. |
| Custom stack-safe `Drop` on public `ContextEntry` | local design | `not-adopt` | Changes field-move semantics and burdens every valid domain value to compensate for the wrong boundary. | A bounded public recursive type later requires special destruction despite valid-by-construction depth. |
| Constructor guards only, as prepared by #297 | sdk-rust commit `1d3f429` | `not-adopt` alone | Correct for most native paths but cannot intercept generic `OneOrMany<ContextEntry>` rejection. Reuse its private mechanism inside the broader design. | Never as the complete #315 answer. |
| `serde_stacker`, `stacker`, or larger thread stacks | current crates ecosystem | `not-adopt` | They target serde recursion or finite stack growth, add dependency/native/unsafe coupling, and do not establish the domain invariant. | A dependency offers guaranteed safe iterative destruction with a smaller audited cone than local code. |
| Intentional leak or unsafe pointer dismantling | not applicable | `not-adopt` | Violates repository safety and resource-ownership policy. | Never. |

## Compatibility and dependency evidence

The accepted API adds one opaque SDK facade type and changes the context enum
payload from a raw map to that type. Direct source callers use
`ContextObject::new(map)?`; borrowed/consuming accessors replace field moves.
The crates are unpublished at version `0.0.0`, so this is an explicit
pre-release migration. Successful serialization remains the exact map and
`OneOrMany` scalar/array shape remains exact. DID owners retain the existing
128-item policy and errors.

No manifest, feature, package, native library, build script, registry source,
or lockfile change is required. `cargo tree -p identus-did --edges normal`
shows the existing `base64`, `bs58`, `fluent-uri`, `identus-core`,
`identus-derive`, `serde`, and `serde_json` cone. Locked `serde_json 1.0.150`
remains the only recursive JSON implementation. Rust 1.98.1 remains the
development compiler and temporary etalon; this change creates no Minimum
Supported Rust Version (MSRV) promise or change. Linux fast plus eligible WASM,
iOS ARM64, and Android ARM64 slow/local targets remain unchanged.

Prior read-only named-consumer searches found no direct affected sdk-rust
constructor use in Oxid, Lace ID Portal, Midnight Identity, or NeoPRISM. The
facade boundary stays SDK-owned; no dependency type becomes policy or error
surface. Rollback is source-only and requires no wire/data migration.

Midnight Identity PR #78 was additionally inspected at exact head
`2dcece66f17614968ce57d0aaa4786966763911a`. Its `DocumentContext` accepts only
one or many strings, so it does not directly construct the changed sdk-rust
context-object variant. The PR independently establishes depth 32, cardinality
128, and aggregate extension budgets for document extensions and service
endpoints, but those public raw recursive values are validated later by owning
domain constructors. This is useful convergence evidence for a future adoption
slice, not authority to mutate that downstream PR or copy its model here.

## Security, privacy and maintenance evidence

The private worklist moves each array/object child exactly once, uses no
recursion or unsafe code, and never logs or formats values. Static errors and
redacted `ContextObject` diagnostics reveal no caller content. Already-owned
breadth can require proportional worklist memory; allocation before SDK entry
remains outer-owned. Every accepted recursive value is bounded, so ordinary
domain destruction becomes bounded rather than dependent on special `Drop`.

License and provenance remain repository Apache-2.0 plus existing locked
dependencies. No donor code or fixture is copied. The direct and resolved
dependency cone, reachable unsafe/native code, SBOM, advisory/license surface,
maintenance owner, release posture, and supply-chain policy are unchanged.
Protocol/draft currency is unchanged: existing pinned W3C DID Core, Resolution,
and Registration profiles remain authoritative; this changes ownership and
model invariants, not semantics.

## Normative sources

- Sponsor-directed issue #315 and durable decision comment
  https://github.com/hyperledger-identus/sdk-rust/issues/315#issuecomment-5710587923.
- Parent issue #297, ADR 0125, `SDK-SEC-003`, and `SDK-LIM-007`.
- User-supplied “Issue #315 — Architecture Analysis and Recommended Direction,”
  received 2026-09-17 and treated as decision research rather than donor code.
- Current source at `fee94946ca489f88dbc50282b5ee1eca095507f3`, especially
  `crates/did/src/{document,resolution,query,registration}.rs`.
- Read-only downstream evidence: Midnight Identity PR #78 at exact head
  `2dcece66f17614968ce57d0aaa4786966763911a`.
- Public primary source URL: https://docs.rs/serde_json/1.0.150/serde_json/value/enum.Value.html.
- Existing internal JWK iterative rejection pattern and #297 implementation
  candidate, both Apache-2.0 sdk-rust source.

## Rejected or deferred candidates

The candidate table records public custom `Drop`, generic ownership return as
the sole fix, dependency stack growth, intentional leaking, unsafe code, and
constructor-only guards. A generic bounded collection is deferred because DID
policy belongs to domain owners and no second identical public contract has
been demonstrated.

## Open questions and blockers

No blocker remains. The durable issue decision authorizes the intentional
pre-release source migration and acceptance reinterpretation. If implementation
finds a public validated DID object that can still directly retain unbounded
recursive JSON, research returns to draft and `SDK-LIM-007` remains broad.

## Evidence commands

Commands run before implementation: `scripts/factory doctor`; `rg` inventory
for all named types, limits, validators, maps, and constructor signatures;
source inspection across the four DID modules and tests; issue/ADR/spec/
constraint review; prior #297 implementation diff inspection; and
`cargo tree -p identus-did --edges normal`. Commands intentionally unrun before
implementation are hostile-depth/policy/wire/API tests, focused and workspace
Rust gates, API/SBOM, portable target/Nix evidence, final factory/OpenSpec
checks, fresh security/architecture review, and protected exact-head CI.

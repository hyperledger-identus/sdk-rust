# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-17
Source retrieval date: 2026-11-17
Research blockers: none

## Problem and existing implementation

The current implementation in `identus-did` already owns the portable
resolution contract. `ResolutionOptions`
and `DereferencingOptions` are immutable bounded open maps; `DidResolver` and
`DidUrlDereferencer` are object-safe `Send + Sync` ports with boxed `Send`
futures; and result values serialize the W3C envelopes without transport or
executor dependencies.

`DidResolutionResult::from_json_str_for` enforces the 512 KiB envelope ceiling,
duplicate-aware raw-wire preflight, depth/node/member/live-name/string budgets,
typed deserialization, success/error/deactivation branch validity, document ID
equality with the requested DID, and same-method canonical/equivalent metadata.
`DidUrlDereferencingResult::from_json_str` applies the same raw-wire and result
budgets and enforces exactly one success or error branch. Options use a 64 KiB
ceiling, typed common fields, reserved-name exclusion, and bounded extension
trees. The current Rust surface therefore owns validation and semantics; it
does not need a second resolver abstraction.

The private `identus-wasm-did` binding API version 2 exposes synchronous DID,
DID URL, and DID Document parsing. `packages/identus-sdk` projects generated
values to frozen capability-oriented TypeScript values, frees generated
ownership exactly once, emits redacted observations, and assembles separate
web and node artifacts reused by browser, Node.js, and React Web fixtures.
There is no resolution host-effect seam yet.

## Normative sources

The primary source URL is <https://www.w3.org/TR/did-resolution/>. The selected
protocol or draft currency is the repository-pinned W3C DID Resolution v1
Candidate Recommendation Draft dated 2026-08-28, together with W3C DID Core
1.0, RFC 9457 problem details, and the repository's accepted ADRs 0010 and
0011. DID URL dereferencing remains separately at risk and SHALL retain its own
versioned operation/result contract.

The canonical implementation source is
`hyperledger-identus/sdk-rust@9b4e463bf37881fd282b5d405fd9272524f34b99`,
Apache-2.0. License and provenance are retained in the repository and donor
records. The MSRV and source/compiler contract is exact Rust 1.99.1. The compatibility
donor is `hyperledger-identus/sdk-ts@4bf86ebf69d5e96616a148e4c973f831f95fa38e`,
`@hyperledger/identus-sdk@8.1.4`, Apache-2.0. The donor is evidence, not
normative authority.

## Donor inventory and provenance

The donor resolver source inventory at the pinned revision is:

| Path | SHA-256 | Disposition |
| --- | --- | --- |
| `packages/lib/sdk/src/castor/index.ts` | `b4de37066994585d2eb7cb35334b19d21373f5c0804f8ecd524dc7ea3d8d4791` | Adapt only async invocation and method selection; reject resolver-loop fallback, generic throws, and logging identifiers. |
| `packages/lib/sdk/src/castor/resolver/DIDResolverHttpProxy.ts` | `701a29ed51029b779b548a196527972b8c22f3ed8bbd1c59f02be04e61663e01` | Adapt the injected async outcome; exclude concatenated URL, fixed headers, global fetch, generic errors, and direct document parsing as host policy/legacy defects. |
| `packages/lib/sdk/src/castor/resolver/LongFormPrismDIDResolver.ts` | `a466e25a4beac125bbcb7ddeca1de8a98abf677e744d355d3da39f7a0fe9a030` | Platform-owned by `did.method.prism`; not generic resolution evidence. |
| `packages/lib/sdk/src/castor/resolver/PeerDIDResolver.ts` | `60dec73c3d010d40c92d7a21537b91b0dd02c5097fbc75c1a438569fe3b04731` | Platform-owned by `did.method.peer`; not generic resolution evidence. |
| `packages/lib/sdk/src/castor/resolver/PrismDIDResolver.ts` | `c4081f8740da0848371d9c0a8b2eb2a62c8de08f40013bd3bfe3985a72043de0` | Platform-owned by `did.method.prism`; not generic resolution evidence. |

The donor test directory contains no transport-neutral W3C result-envelope
suite. `PeerDID.test.ts` (`66a3b70e…ae54`) and `PrismDID.test.ts`
(`18a25ef…330`) mix method construction, cryptography, document projection, and
method-specific resolution. Their consumer outcome—an async resolver yields a
document—is adapted to the generic injected-host journey. Method construction,
signature verification, long-form validation, and external infrastructure are
ledgered to their owning capabilities. No donor production source is copied.
The downstream DIDComm resolver test at
`packages/lib/sdk/tests/mercury/didcomm/DIDResolver.test.ts`
(`e8f4b0ec…9142`) is retained as future consumer rehearsal and remains
platform-owned rather than generic result authority.

## Build-versus-adopt decision

Build the thin adapter over the existing Rust owner. Importing `did-resolver`,
the donor Castor hierarchy, an HTTP client, or an async WASM executor would
duplicate the portable contract or pull host policy inward. The selected seam
uses synchronous WASM functions around an asynchronous TypeScript host effect:

1. Rust validates the DID/DID URL and raw options, then returns a normalized
   generated request view.
2. TypeScript invokes exactly one injected host effect with a frozen plain
   request and awaits raw UTF-8 JSON text.
3. Rust parses the raw response, correlates resolution with the requested DID,
   rejects lossy JavaScript projection, and returns deterministic normalized
   JSON plus closed generated metadata.
4. TypeScript copies to strict frozen values and frees generated ownership.

Returning raw response text is required. `response.json()` would erase
duplicate names and may round unsafe integers before Rust sees them. The host
adapter owns outer response-size, decompression, timeout, cancellation, retry,
URL, header, authentication, SSRF, and network policy; Rust revalidates the
bounded domain bytes it receives.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Implement fetch/HTTP in Rust/WASM | `not-adopt` | Couples portable semantics to runtime, transport, SSRF, credential, and cancellation policy. |
| Return parsed JavaScript objects from the host | `not-adopt` | Loses duplicate-name and integer fidelity before the canonical boundary. |
| Return raw response text to Rust | `adopt` | Preserves exact untrusted wire evidence for bounded validation. |
| Make the generated bridge async | `not-adopt` | Adds lifecycle/executor complexity although only the host invocation is asynchronous. |
| Compose synchronous generated prepare/parse calls around an async TS effect | `adopt` | Keeps allocation and failure domains small and testable. |
| Expose the Rust resolver trait directly to JavaScript | `not-adopt` | Leaks boxed-future/object-safety mechanics and cannot express platform budgets safely. |
| Use one injected `resolve`/`dereference` capability interface | `adopt` | Keeps transport policy explicit and supports deterministic fixtures across targets. |
| Silently retry or fall back after rejection | `not-adopt` | Hides corruption/policy differences and makes rollback non-deterministic. |

The reconsideration trigger is a normative profile change, a proved gap in the Rust
result contract, or measured evidence that the synchronous generated seam
cannot satisfy ownership/resource constraints. A request for a default HTTP
adapter, method resolver, cache, retry, React Native, publication, or support is
a new capability/release decision rather than scope expansion.

## Rejected or deferred candidates

Embedding fetch/HTTP in Rust, accepting parsed JavaScript result objects,
exposing generated async classes, importing the donor Castor hierarchy,
silently retrying, and activating a default provider are `not-adopt` in this
train. Concrete method adapters, cache/coalescing, cancellation, React Native,
consumer activation, and publication are deferred to their separately owned
capabilities and release gates.

## Compatibility and dependency evidence

The exact donor version is 8.1.4 at revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`; the selected feature is the
transport-neutral async resolution outcome, not donor method or fetch policy.
The direct and resolved dependency cone remains unchanged: `identus-wasm-did`
depends inward on `identus-did`, generated web/node artifacts feed the private
TypeScript package, and React reuses those providers. Public Rust API and wire
compatibility remain additive and unchanged; binding API v3 is private and
versioned. The facade boundary is synchronous Rust request/result validation
around one injected asynchronous TypeScript host effect. Rollback selects the
pre-existing provider before initialization and removes the unreleased route.
Target evidence is required independently for browser, Node.js, and React Web.

## Public mapping

The package adds strict immutable resolution/dereferencing options, requests,
results, operation metadata, document/content metadata, problem details, and
bounded JSON values. The provider composes injected `resolve(request)` and
`dereference(request)` effects. Public values contain no generated classes,
WASM memory, initializer, legacy component names, or mutable extension maps.

Stable SDK-owned failures distinguish invalid DID/DID URL/options, host-effect
failure, invalid resolution/dereferencing result, unsupported lossless
projection, binding-contract drift, and unavailable provider. W3C operation
errors remain data inside valid result envelopes, not thrown transport errors.
Messages are constant and observations include only operation, route, result,
mapping/error code, target, and bounded duration.

Existing `DidProvider` remains source-compatible. Resolution and dereferencing
are separate additive capability interfaces because dereferencing is at risk.
W3C problem data is named `DidResolutionProblemValue`, not `DidError`, so it
cannot be confused with thrown local adapter/effect failures. The nine current
standard kinds remain `invalidDid`, `invalidDidDocument`, `notFound`,
`representationNotSupported`, `invalidDidUrl`, `methodNotSupported`,
`invalidOptions`, `internalError`, and `featureNotSupported`.

## Security, privacy and maintenance evidence

Inputs, option extensions, raw result text, problem details, document/content
metadata, dereferenced content, media types, timestamps, version identifiers,
and observers are untrusted. Rust applies existing byte and semantic budgets;
TypeScript applies an outer host-response ceiling before crossing WASM and an
iterative bounded generated-shape validator/freezer afterward. Prototype keys,
unsafe integers, generated API drift, double free, use-after-free, observer
failure, and unknown error codes fail closed.

One provider-neutral assertion body runs unchanged in Node ESM/CommonJS,
Chromium, React SSR, and hydrated React client fixtures. Vectors cover success,
deactivation, all standard problem types, extensions, document correlation,
dereferenced content forms, duplicate names, branch contradictions, exact/over
limits, unsafe integers, redaction, host rejection, observer rejection, and
rollback selection. Property/fuzz evidence belongs to Rust; target evidence
proves only the adapter, artifact, and lifecycle boundary.

The public API accepts structured options only. TypeScript performs iterative
cycle/prototype/number/depth/node/member validation and closed serialization
before Rust applies the canonical typed option validators. Raw option JSON is
not public because the current `ResolutionOptions::from_json_*` path does not
run duplicate-aware raw-wire preflight. Native non-JSON dereferencing bytes are
also excluded because the current portable envelope intentionally represents
bounded JSON content only.

Artifact bytes/hashes, cold/warm load, request preparation, host round-trip
fixture latency, result parse/projection p50/p95, normalized bytes, and reliable
RSS are measurement-only. No support threshold or production claim is created.
The package stays private, unpublished, non-default, and additive.

Unsafe and native-code evidence is unchanged: the affected portable and WASM
crates forbid unsafe code, introduce no native dependency, and keep generated
memory behind reviewed ownership seams. Supply-chain evidence, maintenance,
release and security posture are evaluated through the pinned locks, Nix
closure, source-distribution checks, dependency policy, generated inventory,
and exact revisions. No release or support activation is implied.

## Dependency and supply-chain evidence

Portable crates gain no WASM, npm, TypeScript, React, fetch, HTTP, or runtime
dependency. The expected direct/resolved dependency cone and Cargo/npm locks
are unchanged. Existing web/node `wasm-bindgen` artifacts and the private
package are extended under closed symbol/file inventories. If implementation
requires a new dependency or lock change, this research decision is reopened
before acceptance.

Supply-chain evidence is the exact Cargo/npm lock identity, source-distribution
checks, Nix toolchain closure, generated artifact inventory, immutable donor
revision/hashes, signed+DCO commits, and exact-head target receipts. Consumer
repositories remain read-only.

## Evidence commands

The planned local sequence is:

```text
./scripts/factory research-ready add-sdk-ts-did-resolution
./scripts/factory constraints-ready add-sdk-ts-did-resolution
./scripts/factory validate add-sdk-ts-did-resolution
./scripts/factory preflight add-sdk-ts-did-resolution --issue 599 --write
cargo test -p identus-did
cargo test -p identus-wasm-did
./scripts/check-sdk-ts-did-target.sh all
./scripts/factory check
./scripts/factory ready add-sdk-ts-did-resolution
nix flake check --print-build-logs
```

These are exact commands and unrun checks at planning time. Fuzz, property,
differential, mutation, coverage, resource, artifact, browser,
Node, React, receipt, archive, and fresh-review commands will be recorded
exactly as the implementation exposes them. They are unrun at planning time.

## Open questions and blockers

There are no blockers to the repository-local private train. Concrete host
network policy, method coverage, external consumer activation, accepted
budgets, support/publication, release, React Native, and mdoc remain separate
gates and are not inferred from local success.

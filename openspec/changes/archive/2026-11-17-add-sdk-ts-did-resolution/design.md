# Design

## Dependency and effect direction

```text
identus-did values/options/result validators
    -> identus-wasm-did synchronous binding API v3
        -> private TypeScript request/result projection
            -> injected async host effect
                -> caller-owned browser / Node.js / React transport policy
```

The host returns raw response text back through the TypeScript adapter to a
second synchronous Rust parser call. Portable crates gain no inward WASM, npm,
TypeScript, React, fetch, HTTP, or executor dependency. React owns lifecycle
only and contains no resolution semantics.

## Generated request boundary

`prepareDidResolution(did, optionsJson)` parses the DID and bounded options,
then returns an owned normalized request view. `prepareDidDereferencing`
performs the equivalent DID URL/options validation. The views contain only
copied normalized strings/JSON and binding API identity; TypeScript freezes a
plain request before invoking the host.

The public TypeScript API accepts strict readonly structured options and uses
one iterative cycle/prototype/unsafe-number/depth/node/member validator plus
closed serializer to raw options JSON. Caller-supplied raw option JSON is not
public because the existing Rust option parser does not run the result
envelope's duplicate-aware scanner. The Rust boundary remains authoritative for
reserved fields, value syntax, extension budgets, and deterministic normalized
representation. Generated request ownership is freed before awaiting the host
so no WASM allocation spans an arbitrary promise lifetime.

## Injected asynchronous host effect

The configured host exposes independent `resolve(request)` and
`dereference(request)` capabilities and returns `Promise<string>`. The string
is raw UTF-8 W3C result JSON; adapters using fetch must call `response.text()`.
The package invokes the selected effect once and never retries, caches, or
falls back. Caller policy owns endpoint selection, transport security, headers,
authentication, outer response-size/decompression limits, timeout,
cancellation, retry, and network observability.

Existing `DidProvider` remains source-compatible. Additive
`DidResolutionProvider` and `DidUrlDereferencingProvider` capabilities expose
the async operations independently. An optional operation context may carry an
`AbortSignal`: an already-aborted signal rejects before host invocation; after
invocation the signal is advisory and the host owns cancellation behavior.
The facade does not claim it can stop an effect that ignores the signal.

Host rejection maps to one stable SDK error without forwarding the thrown
message/stack. A valid W3C result containing operation error metadata is not a
host rejection and continues through Rust validation as data.

## Generated result boundary

`parseDidResolutionResult(did, rawJson)` reparses the exact DID and calls
`DidResolutionResult::from_json_str_for`. `parseDidDereferencingResult`
validates the independent dereferencing envelope. Before returning an owned
view, Rust rejects extension numbers that cannot be represented exactly by the
strict TypeScript JSON value contract and serializes the validated result
deterministically.

Binding API version increases to 3. Reviewed generated declarations and closed
symbol/file inventories fail on drift. Generated values expose normalized JSON
and the minimum closed discriminants needed for safe projection; they do not
expose Rust internals or retain host values. Every view/error is copied and
freed exactly once before public completion.

## Typed projection

An iterative bounded validator consumes only Rust-produced normalized JSON and
constructs deeply frozen plain values. It reuses the existing DID Document
projector for a successful resolution document and adds typed operation
metadata, document/content metadata, RFC 9457 problem details, dereferenced
content, and extension maps. Branch tags distinguish `success`, `error`, and
`deactivated` resolution plus `success` and `error` dereferencing.
RFC 9457/W3C error data is projected as `DidResolutionProblemValue`, distinct
from thrown local SDK errors. Native non-JSON dereferencing content is excluded.

The validator checks the reviewed generated schema rather than redefining W3C
domain validity. It rejects prototype keys, unexpected fields/shapes, unsafe
numbers, excessive traversal, and API mismatch as binding-contract failures.
Normalized JSON remains the normative lossless representation and does not
claim original whitespace/order or RFC 8785 canonicalization.

## Stable errors and observation

Closed errors distinguish invalid identifier, invalid options, unavailable
host capability, host execution failure, invalid resolution result, invalid
dereferencing result, unsupported lossless projection, binding-contract drift,
and provider initialization failure. Messages are constant. Raw DIDs, DID
URLs, option/result bytes, endpoints, problem details, thrown values, and
stacks never enter errors or observations.

Observations contain operation, configured route identity, target, mapping ID,
success/error category, stable code, bounded duration, and artifact identity.
Observer failure is terminal after generated ownership is closed and does not
invoke another provider.

## Vectors and donor dispositions

One immutable source-bound packet covers representative resolution success,
error and deactivation branches, correlation mismatch, duplicate names,
contradictory branches, and dereferencing success/error/invalid-state cases.
One provider-neutral TypeScript assertion module runs that packet unchanged in
Node ESM/CommonJS, Chromium, React SSR, and hydrated React client fixtures.
Focused Rust and TypeScript tests separately cover the remaining problem-type,
extension, bound, unsafe-number, redaction, host/observer rejection,
exact-free, and rollback behavior; the packet does not claim to replace those
layer-specific tests.

The donor inventory records immutable revision/path/test/hash and exactly one
disposition. The generic async resolver outcome is adapted. PRISM and peer DID
construction/resolution, cryptography, document mutation, DIDComm, custody,
cloud-agent integration, and live infrastructure remain assigned to their
owning capabilities rather than copied or silently skipped.

## Property, fuzz, differential, and resource evidence

Focused Rust and mutation tests exercise valid/invalid option and result
envelopes and prove deterministic serialize/reparse, branch validity, DID
correlation, bounded cleanup, and redaction. Fuzz targets cover raw
resolution/dereferencing result bytes through the public parsers with exact
byte ceilings and iterative cleanup. Differential evidence compares the
shared packet through canonical Rust and every TypeScript target; the donor is
compatibility evidence only. Generative property and coverage evidence remain
explicit closeout obligations rather than inferred claims.

Measurements cover artifact bytes/hashes, cold/warm initialization, request
preparation, fixture host round trip, ordinary/near-limit result validation and
projection p50/p95, normalized output bytes, and reliable RSS. Measurements do
not activate support budgets.

## Target and CI isolation

Artifact assembly builds independent `web` and `nodejs` outputs, requires API
v3 symbols, and copies a closed inventory. Impact routing includes result and
options owners, WASM bindings, package projection/effect code, vectors,
quality/mapping records, snapshots, fixtures, and target assertions. Rust core
runs first. Selected targets run with isolated caches/logs/artifacts/timeouts
and exact-head receipts; `fail-fast` stays disabled. A target failure preserves
core/sibling evidence but fails the existing terminal `fast` verdict.

## Cohesion and decomposition

The train may exceed 12 files and 1,000 lines because its single reviewable
invariant spans raw host text, canonical Rust validation, generated ownership,
strict TypeScript projection, and identical target behavior. Splitting those
layers would temporarily expose an API-version mismatch, unvalidated async
route, or receipts not bound to the public contract. Signed checkpoints keep
planning, Rust/WASM, TypeScript, conformance, and harness changes reviewable.

Concrete transport/method/cache implementations, downstream adoption,
React Native, support/publication/release, and mdoc have independent ownership,
security, rollback, or release authority and remain separate slices.

## Delivery boundary

This is a local-first issue #599 train. It may be pushed as a reviewable PR only
after complete local evidence. Merge requires protected exact-head checks and
required review. Publication, release, `main` promotion, default activation,
or support claims require separate explicit human authority.

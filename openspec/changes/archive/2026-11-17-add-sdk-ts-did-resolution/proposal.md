# Add the SDK-TS DID resolution train

## Why

The private TypeScript canary now exposes canonical DID, DID URL, and DID
Document values through isolated browser, Node.js, and React Web targets.
SDK-Rust already owns bounded W3C DID Resolution and DID URL Dereferencing
options, result envelopes, metadata, error classification, and runtime-neutral
ports. The missing TS1 capability is a thin asynchronous host-effect adapter
that preserves that Rust contract without importing browser or Node networking
policy into portable crates or reviving historical Castor public names.

This train makes resolution and dereferencing usable through the existing
private capability-oriented package. TypeScript owns invocation of an injected
host effect; Rust owns query normalization, raw-result validation, request/result
correlation, deterministic projection, and stable domain rejection.

## What Changes

- Extend `identus-wasm-did` with synchronous prepare/validate and result-parse
  functions for DID resolution and DID URL dereferencing. The host effect stays
  outside WASM and returns raw response text so duplicate JSON names remain
  observable to Rust.
- Add strict immutable TypeScript query, result, metadata, content, error, and
  host-effect contracts without legacy component names or generated ownership
  in the reviewed public surface.
- Correlate resolution success with the requested DID through
  `DidResolutionResult::from_json_str_for`; retain dereferencing as the separate
  at-risk result contract selected by the repository-pinned W3C profile.
- Reject unsafe JavaScript integer projection explicitly, preserve normalized
  JSON as the lossless representation, and free every generated value/error
  exactly once.
- Inventory and disposition the pinned SDK-TS resolver implementation/tests,
  add shared Rust/TypeScript vectors and Rust-owned negative/resource evidence,
  and extend target receipts, public snapshots, mutation guards, measurements,
  rollback, and isolated CI routing.
- Update only the `did.resolution` parity evidence proved by this local train.

## Capabilities

### New Capabilities

- `sdk-ts-did-resolution`: define the private additive TypeScript resolution
  and dereferencing facade, injected host boundary, strict projections,
  compatibility mapping, conformance evidence, and rollback contract.

### Modified Capabilities

- `did-core`: correct the existing bounded resolution-options requirement to
  list the already implemented `noCache` common member. This is specification
  drift correction only; behavior and public Rust APIs remain unchanged.

Existing DID syntax and DID Document capabilities remain byte-for-byte
compatible and independently reversible.

## Impact

The change affects `identus-wasm-did`, the private `packages/identus-sdk`
facade, generated web/node declarations, browser/Node/React fixtures, DID
resolution vectors/quality evidence, impact routing, and the TS1 capability
record. `identus-did` may receive narrowly scoped parser/property tests or a
resource-bound correction discovered by the binding work, but its public
resolution model and object-safe ports remain authoritative and compatible.

Concrete method implementations, HTTP/fetch policy, cache policy, retries,
timeouts, cancellation, registration, PRISM/Cardano effects, persistence,
trust/authorization, credentials, DIDComm, agent orchestration, React Native,
external SDK-TS mutation/default activation, publication, stable support,
release work, and mdoc remain outside this train.

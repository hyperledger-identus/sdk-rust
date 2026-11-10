# Add the SDK-TS DID Document train

## Why

The private TypeScript DID/DID URL canary proves deterministic target-specific
WASM packaging, strict capability-oriented TypeScript, provider rollback, and
Rust-core-first isolated CI. The next TS1 capability row is `did.document`.
SDK-Rust already owns a bounded immutable W3C DID Document model, while the
pinned SDK-TS donor exposes a permissive class projection that drops fields,
mutates prototypes, admits private key material, and mixes portable document
semantics with historical module and resolver behavior.

This train makes the portable Rust document contract available through the
same private local-first browser, Node.js, and React Web package. It freezes a
versioned mapping and shared vector packet before any consumer activation.

## What Changes

- Extend the private WASM binding to API version 2 with bounded DID Document
  parsing, deterministic normalized JSON, exact ownership, and redacted errors.
- Add strict immutable TypeScript document, verification relationship/method,
  service, endpoint, context, and JSON-extension projections without legacy
  public names or duplicated domain validation.
- Reject JavaScript-unsafe integer projection explicitly rather than silently
  rounding accepted Rust extension values; retain normalized JSON as the
  normative lossless representation.
- Add a versioned mapping, donor-test disposition ledger, immutable document
  vector packet, shared browser/Node/React assertions, rollback evidence,
  document fuzzing, coverage, and resource measurements.
- Reuse the existing target-specific artifact assembly and isolated CI graph,
  extending its impact routing, boundary guards, receipts, and mutation tests.
- Update only the `did.document` parity evidence actually produced locally.

## Capabilities

### New Capabilities

- `sdk-ts-did-document`: define the private additive DID Document parser and
  typed projection, versioned compatibility mapping, shared evidence, resource
  policy, and rollback boundary.

### Modified Capabilities

- `sdk-ts-did-facade`: extend the private provider from DID values to the next
  separately versioned document capability without changing existing value
  semantics or support status.

## Impact

The change affects `identus-wasm-did`, the private `packages/identus-sdk`
facade, deterministic artifact assembly, DID Document vectors/fuzzing/quality
evidence, target fixtures, impact routing, and the TS1 matrix. Existing Rust
public APIs and the external SDK-TS repository remain unchanged.

DID resolution/dereferencing/registration, method execution, PRISM/Cardano
effects, networking, storage, keys/custody, credentials, agent orchestration,
React Native/native bindings, publication, default activation, stable support,
and mdoc remain explicitly outside this train.

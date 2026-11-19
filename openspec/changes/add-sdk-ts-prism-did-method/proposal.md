# Add the SDK-TS PRISM DID method train

## Why

SDK-Rust owns a bounded portable PRISM method kernel and the private
TypeScript package already proves generic DID values, documents, injected
resolution, peer methods, explicit provider selection, and cross-target
loading. TS1 still lacks a capability-named PRISM facade that keeps identifier,
initial-state, authenticated reduction, and projection semantics in Rust while
leaving Cardano, persistence, networking, and service composition downstream.

## What Changes

- Extend `identus-wasm-did` with explicit PRISM canonical/long-form parsing,
  public-material long-form creation, local long-form projection, and bounded
  authenticated history reduction over `identus-prism-did`.
- Add strict immutable TypeScript PRISM inputs, values, stable errors,
  observations, provider capabilities, injected history effects, cancellation,
  explicit selection, and rollback without historical component names.
- Reuse the canonical Rust/Preprod vector packet and ledger donor evidence,
  classify the pinned SDK-TS journeys, and retain NeoPRISM as an immutable
  adoption oracle rather than an implementation dependency.
- Execute shared positive, negative, resource, mapping, ownership, and
  injection assertions through Browser, Node.js, React SSR, and hydrated React
  client targets with isolated receipts.
- Reconcile the TS1 capability table, adapter mappings, quality declaration,
  consumer-change ledger, impact routing, and private package declarations.

## Capabilities

### New Capabilities

- `sdk-ts-prism-did-method`: define the private TypeScript/WASM PRISM DID
  facade, injected portable-history boundary, exact target evidence,
  observability, and rollback.

## Impact

The change affects `identus-wasm-did`, the private `packages/identus-sdk`
facade, generated web/node declarations, Browser/Node/React fixtures, PRISM
vectors and donor evidence, impact routing, adapter/quality/change registries,
and the TS1 capability record. The binding gains one inward workspace
dependency on `identus-prism-did`; it adds no chain client or external engine.

Cardano observation/submission, VDR storage semantics, indexers, persistence,
HTTP, service composition, key generation/custody, signing, SDK-TS mutation,
default activation, React Native, publication, stable support, release work,
and mdoc remain outside this train.

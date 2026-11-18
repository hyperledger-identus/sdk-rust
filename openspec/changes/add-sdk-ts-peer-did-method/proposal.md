# Add the SDK-TS peer DID method train

## Why

The private TypeScript canary now exposes bounded DID values, DID Documents,
resolution, and dereferencing through isolated browser, Node.js, and React Web
targets. SDK-Rust already owns effect-free peer DID numalgo 4 current-profile
and explicit numalgo 2 compatibility creation, parsing, local resolution, DID
Document projection, redacted errors, and finite resource limits. TS1 still
lacks the thin method facade, cross-language equivalence packet, target proof,
and pairwise consumer-shaped evidence needed to use that Rust owner.

Peer and PRISM are intentionally separate. ADR 0174 gives them different
profiles, effect cones, consumers, authority, and rollback. This train advances
only `did.method.peer`; PRISM follows as its own capability train.

## What Changes

- Extend `identus-wasm-did` with explicit numalgo-4 and
  numalgo-2-compatibility operations over `identus-did-peer`, using only public
  input material and the existing DID Document projection boundary.
- Add strict immutable TypeScript peer-profile inputs, results, stable errors,
  provider capabilities, observations, and rollback selection without legacy
  component names or generated ownership in the reviewed public surface.
- Preserve profile separation: new integrations use numalgo 4, while numalgo 2
  remains an explicitly named compatibility route. Numalgos 0, 1, and 3 never
  enter a generic fallback.
- Expand immutable peer vectors and donor dispositions with deterministic
  numalgo-2 identifier/document compatibility, Rust-owned numalgo-4 evidence,
  negative/resource cases, and a pairwise request-shaped proof that does not
  construct or execute DIDComm messages.
- Extend browser, Node.js, React Web, package/type, measurement, mutation,
  source-distribution, impact-routing, quality, mapping, change-ledger, and TS1
  evidence for the exact private facade.

## Capabilities

### New Capabilities

- `sdk-ts-peer-did-method`: define the private additive TypeScript/WASM peer
  DID numalgo-4 and explicit numalgo-2-compatibility facade, strict public
  inputs and projections, conformance evidence, observability, and rollback.

### Modified Capabilities

None. The canonical `peer-did-method`, DID syntax, DID Document, and DID
resolution specifications remain unchanged; this change binds their existing
Rust behavior to a private language facade.

## Impact

The change affects `identus-wasm-did`, the private `packages/identus-sdk`
facade, generated web/node declarations, browser/Node/React fixtures, peer DID
vectors and donor/quality evidence, impact routing, adapter/change registries,
and the TS1 capability record. `identus-wasm-did` gains an inward workspace
dependency on `identus-did-peer`; no new third-party engine is selected.

PRISM, VDR/indexing, Cardano effects, networking, persistence, secret
generation/custody, DIDComm packing or application protocols, agent runtime,
React Native, external SDK-TS mutation/default activation, publication, stable
support, release work, and mdoc remain outside this train.

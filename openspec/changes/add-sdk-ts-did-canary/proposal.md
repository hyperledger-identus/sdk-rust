# Add the SDK-TS DID facade canary

## Why

SDK-Rust already owns bounded DID and DID URL semantics and exposes an
unpublished browser WASM value boundary, but the SDK-TS parity milestone has no
capability-named TypeScript facade, Node.js loader, React web integration, or
isolated target evidence. Issue #492 is the first reversible canary that proves
those migration mechanics without making the Rust-backed route the default.

The canary must be useful locally before hosted CI or publication. It therefore
packages one strict TypeScript contract over private target-specific WASM
artifacts, reuses shared assertions across providers, and produces independent
target receipts while preserving Rust-core evidence.

## What Changes

- Add a private `@hyperledger/identus-sdk` TypeScript workspace whose public DID
  API uses standards/capability names and hides generated WASM ownership.
- Add browser and Node.js loaders over separately generated `web` and `nodejs`
  artifacts, plus a React web lifecycle integration that reuses those loaders.
- Add shared DID/DID URL assertions, exact cross-language vectors, strict type
  checks, redacted error mapping, observability, provider selection, and tested
  rollback behavior.
- Add deterministic package, browser, Node.js ESM/CommonJS, React client,
  SSR/hydration, resource-measurement, and declaration/export evidence.
- Add a fail-closed binding impact planner, independent target receipts, a
  terminal aggregator, and a locally validated CI graph in which Rust core runs
  first and TypeScript failures cannot erase its result.
- Add a proposed consumer-change record and a canary-specific quality evidence
  declaration with fresh property, fuzz, benchmark, and differential routes.
- Update the DID parity row only for evidence actually produced by this slice.

## Capabilities

### New Capabilities

- `sdk-ts-did-facade`: define the private additive DID/DID URL TypeScript
  facade, target loaders, provider/rollback boundary, conformance evidence, and
  explicit non-claims.

### Modified Capabilities

None. Existing browser binding and parity requirements remain valid; this
change adds a target facade and updates their machine evidence without weakening
their requirements.

## Impact

The change affects the unpublished `identus-wasm-did` boundary, a new private
TypeScript workspace, browser/Node/React fixtures, Nix development/check
surfaces, target planning/aggregation scripts, the `fast` workflow structure,
the consumer change ledger, quality evidence declarations, and the TS1 parity
matrix. It introduces exact development dependencies only for TypeScript,
React, Vite, and browser driving; it publishes no package and activates no
runtime support.

Actual SDK-TS repository mutation, a default route, npm publication, React
Native/TurboModule work, mdoc, DID documents, DID methods, resolution, keys,
credentials, storage, networking, and agent orchestration remain out of scope.
Issue #492 stays open until its separate consumer-repository rehearsal and
rollback evidence are complete.

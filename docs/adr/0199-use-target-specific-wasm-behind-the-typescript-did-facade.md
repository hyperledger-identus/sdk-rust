# ADR 0199: use target-specific WASM behind the TypeScript DID facade

- **Status:** Accepted for an experimental unpublished canary
- **Date:** 2026-11-10
- **Decision authority:** issues #492 and #594 under ADRs 0163, 0169, and 0170
- **Related:** ADR 0101; issues #420, #422, #501, #504, #505, #512
- **Constraint impact:** material; broadens local evidence beyond the
  browser-only adapter without activating support or publication

## Context

ADR 0101 selected `wasm-bindgen` for an experimental browser-only DID value
adapter and explicitly made no Node, React, package, or consumer claim. The TS1
contract now requires a capability-named TypeScript facade and independent
browser, Node.js, React web, and React Native evidence. Generated WASM classes
are not a suitable SDK contract because they expose allocation and lifecycle.

## Decision

1. Keep `identus-did` canonical and binding-framework-free and retain
   `identus-wasm-did` as the private generated boundary.
2. Add one strict private TypeScript facade with frozen plain DID/DID URL
   values, stable redacted errors, provider selection, bounded observability,
   and no historical component names.
3. Generate separate private `web` and `nodejs` wasm-pack artifacts. Select
   browser ESM, Node ESM, and Node CommonJS through conditional entries rather
   than runtime guessing.
4. React web owns initialization/lifecycle only: client execution reuses the
   browser provider and SSR/hydration reuse the Node provider.
5. React Native is not part of this WASM canary. ADR 0163's later native New
   Architecture/TurboModule package remains the only selected direction.
6. Rust rejection is terminal. Rollback selects another provider before
   initialization and never retries rejected input through a second parser.
7. Keep the package private, unpublished, opt-in, non-default, and unsupported.
   Measurements do not activate a budget or support tier.
8. Run Rust core first and preserve its independent exact-head result. Target
   failures block the existing terminal `fast` verdict without rewriting core
   evidence or cancelling sibling target evidence.

## Consequences

The repository can prove the reusable TypeScript migration seam and target
loaders locally while keeping portable semantics in Rust. Two generated target
artifacts and ESM/CommonJS entries increase package/test maintenance, so exact
locks, hashes, API snapshots, singleton tests, and independent receipts are
mandatory.

The canary does not prove that SDK-TS or a product adopted the route. That
requires a separate consumer-repository change and keeps issue #492 partial.
Browser, Node.js, React web, React Native, npm, stable API, and production
support remain unactivated.

## Alternatives rejected

- Expose generated classes directly: leaks WASM ownership and generated API.
- Reuse the web artifact in every JavaScript runtime: infers Node and React
  behavior from browser evidence.
- Detect the runtime dynamically: makes bundling and dual initialization
  ambiguous.
- Implement a React-specific parser: duplicates portable semantics.
- Run React Native on browser WASM: conflicts with ADR 0163.
- Modify SDK-TS here: crosses repository ownership and cannot prove consumer
  rollback honestly.

## Verification and rollback

Verification covers strict declarations, API/name snapshots, all canonical DID
vectors, browser engines, Node module formats, React client/SSR/hydration,
disposal and redaction, rollback, deterministic assembly, resource
measurements, dependency direction, impact planning, target receipts, and
terminal aggregation. Rollback removes the private package and target machinery
without changing canonical Rust, generated browser API version 1, SDK-TS, or
published artifacts.

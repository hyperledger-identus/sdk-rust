# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/492
Constraint blockers: none

## Existing entries affected

- `SDK-LIM-002` remains effective: the facade does not activate a supported
  FFI/binding commitment.
- `SDK-LIM-003` remains effective: browser WASM and native targets are not
  runtime-certified.
- `SDK-LIM-009` remains effective: `fast` is the single protected integration
  verdict. The change may expose internal job evidence without adding another
  required status.
- The accepted TS1 contract at issue #594 requires capability-oriented public
  names, target-specific evidence, shared TypeScript assertions, Rust deviation
  evidence, strict types, reusable portable semantics, target measurements,
  rollback, and Rust-core-first isolated CI.

## Introduced or changed constraints

The TypeScript public surface SHALL contain no historical Apollo, Castor,
Mercury, Pluto, Pollux, or Edge Agent names. It SHALL expose plain immutable DID
values, stable redacted errors, and provider creation while generated WASM
types and lifecycle remain private.

Browser, Node.js, and React web SHALL use target-specific loaders and evidence.
React web SHALL reuse the browser provider for client execution and the Node
provider for SSR/hydration. React Native SHALL be explicitly not applicable to
this WASM canary and SHALL NOT install or exercise a native toolchain.

The exact 22-case `did.syntax.v1` packet SHALL execute through the Rust-backed
provider. Shared TypeScript assertions SHALL be provider-neutral. A Rust parse
rejection SHALL NOT fall back to another parser. Rollback SHALL be an explicit
provider selection before initialization.

TypeScript SHALL compile in strict mode without public explicit `any`. Boundary
input SHALL be validated before projection, and Rust SHALL remain the final
domain validator. Every WASM-owned value and error SHALL be freed exactly once,
including exceptional projection and observability paths.

Target evidence SHALL be exact-head and independently terminal. Rust core SHALL
run first and expose its own result. A TypeScript target failure SHALL NOT erase
or rewrite green Rust-core evidence, but every selected target must succeed for
the existing protected `fast` aggregator to pass. Unknown impact SHALL select
all targets; unselected targets SHALL emit bounded inapplicable receipts without
installing their toolchains.

The new package SHALL remain private and unpublished. Package size,
initialization, parse latency, and memory evidence are diagnostic until a later
decision accepts budgets and support. No output may claim supported browser,
Node, React, React Native, npm, stable API, certification, or production use.

## Introduced or changed limitations

This slice is limited to DID and DID URL parsing through a private additive
facade. It does not add DID documents, DID methods, resolution, keys,
credentials, storage, networking, agent orchestration, mdoc, publication, or a
stable TypeScript API. Browser, Node.js, and React web results are local canary
evidence rather than a support declaration. React Native remains deferred to a
separate native binding milestone.

## Consumer and product impact

The repository gains an additive opt-in facade and local consumer fixtures.
Existing Rust APIs, wire values, SDK-TS behavior, and default routes are
unchanged. The separate SDK-TS repository remains the owner of actual legacy
provider wiring and its named consumer rehearsal.

## Activation and rollback

Implementation begins only after strict research and constraint readiness, a
signed/DCO planning-only commit, and exact issue #492 preimplementation receipt.
The local candidate requires focused target gates, complete factory validation,
`nix flake check`, exact-diff review, and archived OpenSpec evidence.

Rollback removes the private facade, loaders, target fixtures, and CI routing
as one unreleased unit while preserving `identus-did`, `identus-wasm-did`, the
canonical vectors/mappings, and existing SDK-TS behavior. Actual consumer
rollback remains a separate SDK-TS change.

## Evidence

Issues #420/#422/#501/#504/#505/#512 are closed; ADRs 0101/0163/0169/0170,
the TS1 parity contract, the canonical mapping/source records, and the
`did.syntax.v1` packet establish the bounded implementation authority. Issue
#594 establishes the modern target and quality criteria without activating
support.

# Design

## Layered boundary

The dependency direction remains one-way:

```text
identus-did
    -> identus-wasm-did
        -> private web/node generated artifacts
            -> strict TypeScript DID facade
                -> browser / Node.js / React web fixtures
```

Portable Rust crates cannot depend on WASM, npm, TypeScript, React, or package
code. The facade copies generated views into frozen plain values and frees the
raw object. It catches only the known generated parse error, copies its closed
code into a normal `DidError`, frees it, and does not expose caller data or
generated diagnostics.

## Public facade

The package exports `DidValue`, `DidUrlValue`, `DidErrorCode`, `DidError`,
`DidProvider`, observer types, and target provider creation. It does not export
generated classes, initializer functions, WASM memory, raw errors, or legacy
component names. Package export snapshots and a case-insensitive name guard
enforce the surface.

Provider options select the route and bounded observer before initialization.
The observer receives only operation, route, result class, mapping ID, stable
code when present, and bounded duration. Observer failure cannot change the
parse result or leak generated objects. Rust rejection is terminal.

## Target artifacts and loaders

The build assembles private `web` and `nodejs` wasm-pack outputs in isolated
scratch directories, verifies their API version and paired hashes, and copies
only the required artifacts into the private package assembly. Conditional
exports select browser ESM, Node ESM, and Node CommonJS entries; there is no
runtime environment guessing.

The browser loader uses an awaited explicit initializer and a singleton promise
that is reset only after failed initialization. The Node loader wraps the
generated node artifact and proves both import and require surfaces. Exact
artifact identity prevents ESM/CJS entries from loading mismatched WASM.

## React web integration

React owns provider initialization and lifecycle only. A provider/context/hook
returns typed loading, ready, and failed states and disposes no plain values
because the facade has already copied/freed WASM ownership. Client rendering
uses the browser loader. SSR uses the Node loader, serializes only the public
plain value, and hydration proves the client can reconstruct the same result
without a second domain implementation.

No React Native source or browser-WASM fallback is added. Its target receipt is
structural and not applicable for this canary.

## Shared conformance and rollback

One provider-neutral TypeScript assertion body consumes all 22 canonical
vectors. It runs against the Rust provider on every applicable target and
against a test-only legacy provider seam for rollback mechanics. The latter is
not evidence that the external SDK-TS implementation has been migrated.

Provider selection is immutable after creation. Rollback selects the legacy
provider before application bootstrap; there is no second-parser retry after a
Rust error. Tests cover valid values, empty-vs-absent query/fragment, exact byte
ceilings, stable errors, redaction, exceptional projection, observer failure,
singleton initialization, and rollback.

## Package and quality evidence

The package is private, exactly locked, strictly typed, and assembled twice for
determinism. Evidence records declaration/export snapshots, uncompressed and
compressed sizes, hashes, cold/warm initialization, ordinary/near-limit parse
latency, and memory where reliably available. Results are measurements rather
than accepted support budgets.

The canary quality declaration owns four pending routes. Implementation makes
property, benchmark, and differential routes source-bound and records a fresh
bounded Rust fuzz receipt; no prior stale receipt is silently promoted.

## Isolated local and CI graph

Local commands expose `package-types`, `browser`, `nodejs`, `react-web`,
`react-native`, and `all` modes. Each mode writes a closed exact-head JSON
receipt to its own directory. React Native writes an inapplicable receipt and
does not install native tools.

The fail-closed impact planner evaluates exported TypeScript/WASM API hashes,
binding schema, transitive Cargo dependency closure, Cargo/npm locks, package
metadata, mappings, vectors, target contracts, and changed paths. Unknown input
selects all targets. Mutation tests cover every selector and stale/malformed
identity.

The hosted workflow is restructured locally as:

```text
core-rust -> binding-impact-plan -> independent selected target jobs -> fast
```

`core-rust` retains today's Rust/factory gates and an independently visible
exact-head receipt. Target jobs use sibling cancellation disabled. The terminal
`fast` job runs regardless of upstream status, validates exact-head receipts,
and passes only when core and every selected target succeeded. A target failure
therefore blocks integration without changing the core result.

## Delivery boundary

This repository-local train may be reviewed and validated as a stacked branch
over the unpushed #594 parity contract. It cannot become an independent PR until
that contract lands or is deliberately included. The archived change uses
deferred issue linkage because the separate SDK-TS rehearsal remains an issue
#492 acceptance gate.

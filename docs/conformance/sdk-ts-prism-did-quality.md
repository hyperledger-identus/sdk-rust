# SDK-TS PRISM DID facade evidence

Issue #610 adds an unpublished, opt-in TypeScript/WASM facade over the
chain-neutral `identus-prism-did` owner. The facade accepts public material for
deterministic long-form creation, parses canonical and long-form identifiers,
projects embedded initial state locally, and resolves canonical history through
one injected effect returning only ordered exact signed-operation base64url.
Rust remains authoritative for correlation, signatures, history linkage,
atomic state transitions, tombstones, and DID Document projection.

The shared assertion body in
`packages/identus-sdk/tests/prism-vector-assertions.mjs` executes unchanged in
Node.js ESM and CommonJS, browser ESM, React SSR, and hydrated React. It checks
the pinned canonical and long-form identifier, local document subject,
immutable projections, a published signed create envelope, the frozen injected
request, and authenticated active-state projection. Provider tests separately
cover hostile/secret-shaped closed input, unavailable or rejected history,
malformed base64url, cancellation before allocation, redaction, bounded input,
observer behavior, exact generated ownership, and API-version drift.

The donor ledger classifies every pinned SDK-TS PRISM source as adapted,
superseded, platform-owned, or conformance-only. Cardano observation and
submission, VDR persistence, indexing, timestamps, networking, retries, cache,
private keys, signing, derivation, custody, agent lifecycle, React Native,
publication, support, release, and mdoc are deliberately excluded.

## Local gates

The implementation boundary is
`155f0f982ec14a70ec22ac9a42a1fb996242125d`. Run the focused gates with the
pinned Rust 1.99.1 and TypeScript environments:

```text
nix develop --command cargo test -p identus-wasm-did
nix develop --command cargo clippy -p identus-wasm-did --all-targets -- -D warnings
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh all
cargo test -p identus-prism-did
cargo test -p identus-conformance --test cross_language_prism_vectors
```

At the implementation boundary, 20 WASM unit tests and 52 package tests pass.
The isolated target harness builds deterministic binding-API-v5 artifacts and
passes package/type checks, 22 DID syntax vectors, 12 DID Document vectors, 9
resolution vectors, 13 peer vectors, and the same PRISM assertion body in each
applicable web runtime. React Native is explicitly inapplicable because it
requires a separately qualified native binding.

The artifact assembly digest is
`a97383525452656aa672ee40cea613cfe6a3038ce4d835d6d909c46419b4caa8`.
The web and Node target digests are respectively
`27a3657c651bee95644102b407e11904cdafb369875317ea07c8d3dc37771284` and
`8c7a785c9f5455c9a3ff76f4a1da914f835330f1c79ed89204dfec5a688e56ab`.
These are deterministic build evidence, not accepted size or latency budgets.

## Security and rollback

TypeScript preflights closed plain objects and arrays, forbidden prototype
keys, accessors, cycles, depth, nodes, collection sizes, finite numeric values,
unsafe integers, UTF-8 bytes, base64url form, operation count, per-operation
decoded bytes, and aggregate history bytes. Rust independently enforces the
canonical method limits and cryptographic/state invariants. Generated owners
are copied and freed exactly once; returned values and host requests are frozen.
Observations contain only operation, route, mapping ID, result category, stable
error code, and bounded duration.

Rollback selects a provider without PRISM capability before initialization.
There is no automatic parser, transport, retry, history, document, or legacy
fallback after a canonical or host rejection. Removing this unpublished
facade leaves canonical Rust and the external SDK-TS repository unchanged.

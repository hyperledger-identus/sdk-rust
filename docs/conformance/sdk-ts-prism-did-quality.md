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
malformed base64url, cancellation before allocation and during the injected
effect, redaction, bounded input, observer behavior, exact generated ownership,
and API-version drift.

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

The retained measurement compares the pre-facade baseline
`bd1981372fcb08772d1196131a0cf26291fad9cb` with candidate
`cac2e4305b060d217aaa51d9391acc5ffcac42b8`; its SHA-256 is
`dab3c455e8fe677ccd345cc3fc4cdaeb1e9e66a90bd9b92bb7e679ef67d3d697`.
The candidate adds the complete peer and PRISM trains, so the directional
artifact delta is 520,356 raw/181,060 deterministic-gzip bytes for web and
518,364 raw/180,992 gzip bytes for Node. On Apple arm64 with Node 24.19.0,
PRISM public-material create p50/p95 was 0.122/0.173 ms for one key and
4.368/4.555 ms for the 50-key profile; authenticated public signed-history
reduction was 0.254/0.304 ms. Process RSS after the complete measurement was
105,316,352 bytes. These single-host numbers are measurement-only and are not
support or regression thresholds.

The deterministic adapter fuzz executes 2,048 cases at seed `0x605f0078`
through the PRISM creation/history serializers and generated DID/state
projection functions. It covers accessors, cycles, exotic prototypes, unsafe
numbers, unknown fields, malformed base64url, bounded arrays, malformed
generated fields, frozen output, stable redaction, and exact-once release. It
found and closed a generated-document callback sanitization gap before this
receipt was recorded.

## Distinct local review

The architecture/security re-review traced public inputs through TypeScript
preflight, generated ownership, Rust parsing/reduction, immutable projection,
observation, and every terminal failure path. It found one evidence gap: the
implementation checked cancellation after the injected history effect, but the
provider suite proved only cancellation before allocation. The review added an
in-effect abort case that returns a host result, then proves the stable abort
category, exact generated-owner cleanup, and zero calls to Rust history
reduction. The 52-test package suite passes with that remediation. No blocking
architecture, security, compatibility, provenance, or evidence finding remains.

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

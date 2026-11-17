# Private SDK-TS DID Document local quality evidence

## Scope and source identity

This record belongs to `did.document.typescript-canary.quality.v1` and issue
502. It covers only the private repository-local DID Document facade over the
canonical Rust parser for browser, Node.js, and React Web. It does not claim
resolution, dereferencing, registration, method effects, React Native,
publication, support, external SDK-TS adoption, production readiness, or mdoc.

The fresh target and full integration validation ran at signed implementation
commit `2422380562521cff2f1fc6794519d8d621ca500b` and source tree
`21f66d01a475af37dcdc891ccb8914769920e234`. After the Rust 1.99.1 base merged,
the tree-preserving rebase maps that exact implementation tree to reachable
signed commit `c5b7d86800dc8887bf5c4c4f18f8037f5bd6582a`. The complete child diff is
byte-identical before and after the rebase. Subsequent signed commits bind the
consumer-change lifecycle, deferred-acceptance sidecar, evidence/spec prose,
and the closed `document-quality-evidence` planner/aggregator contract with its
regression test; they do not change the Rust, WASM, or TypeScript runtime,
vectors, fixtures, or build configuration. Earlier target hashes below remain
historical deterministic packet evidence tied to the identical source tree;
they are not silently relabeled as a new execution.

- packet: `did.document.v1`, 12 cases, SHA-256
  `dcf62523885b42f780a4d57b1c6de7084a95ad12c77ed5be65ddd97d7ebc1ebb`
- deterministic private assembly SHA-256:
  `8dc97984e183d98d3086e8527ac7ec6a1c72d3735c5ba508d01d5a3b3dae014a`
- historical syntax-canary evidence remains independent and is not reused as
  document evidence

## Target and package receipts

The private package ran one provider-neutral document assertion body through
Node ESM, Node CommonJS, Chromium, React SSR, and hydrated React client
fixtures. Generated objects are copied and freed exactly once before any plain
value escapes.

| Target | Applicability | Result | Duration | Bound artifact SHA-256 |
| --- | --- | --- | ---: | --- |
| package and strict types | selected | success | 8 s | `875c86a53052777256ef0a65401db7ac09fb9e4ca5d11e84e52d3ddb0e0f588a` |
| browser | selected | success | 13 s | `dae024788feb865ea43da795c0cccb7a9eff672b1b044bc318c19559c0788f2a` |
| Node.js | selected | success | 14 s | `db3978fd71cf0e7bb38a511ff7325536e2f1cafafa079ab667e3f3d72011b10c` |
| React Web | selected | success | 16 s | `61a3ef9044ed6575ae25c57c600388e52a6dbd97f1f739abd757ad951912300b` |
| React Native | inapplicable to WASM | inapplicable | 16 s | none |

Private receipts are retained under `target/sdk-ts-did-receipts/`; they are
generated evidence, not tracked release artifacts. A stale, missing,
cancelled, skipped, timed-out, failed, malformed, or unexpected selected-target
receipt fails closed while independent Rust-core evidence remains visible.

## Contract, property, and differential evidence

The following local gates passed:

- `cargo test -p identus-wasm-did`: 7 tests;
- `cargo clippy -p identus-wasm-did -p identus-conformance --all-targets -- -D warnings`;
- `cargo test -p identus-conformance --test cross_language_did_vectors`: 2 tests;
- package `npm test`: 24 tests;
- `node scripts/tests/binding-target-isolation.mjs`: 16 mutation/isolation tests;
- `python3 scripts/tests/cross-language-vectors.py`;
- `./scripts/factory check`: 126 OpenSpec/spec records, 69 vectors, and 88
  obligations at the recorded integration revision;
- `nix flake check`: every compatible `aarch64-darwin` check passed, including
  the Rust 1.99.1 source contract, etalon build, 1,200 workspace tests, and the
  isolated binding target contracts.

The operation/branch inventory covers raw JSON entering Rust before TypeScript
parsing; success and both stable document failures; normalized serialization;
scalar/array and absent/present distinctions; reference/embedded relationships;
URI/map/mixed endpoints; bounded extensions; duplicate names and resources;
private or ambiguous verification material; exact wire bounds; unsafe-number
rejection; generated success/error disposal; malformed generated wire;
observer failure; no parser fallback; singleton initialization; explicit
rollback selection; artifact drift; impact routing; and terminal receipt
failure. No percentage is claimed because a source-bound line/branch coverage
artifact was not produced.

## Fuzz evidence

The `did_document` libFuzzer target proves the 256 KiB input ceiling,
deterministic serialize/reparse equality, and public-only verification
material on every accepted input.

- replay: success, 10 runs over 9 seeds including the exact-limit seed; peak
  RSS 45 MiB;
- ASan smoke: success, 4,096 runs; peak RSS 149 MiB;
- 300-second soak: not run and therefore not claimed.

Fuzz success does not substitute for target, property, or differential tests.

## Directional resource measurements

The untracked private measurement record is
`target/sdk-ts-did-measurements.json`. It was captured on Node v24.19.0,
Darwin arm64 and remains `target-not-supported` measurement-only evidence.

- web artifact: 270,511 bytes; Node artifact: 264,683 bytes;
- initialization: 0.095625 ms;
- ordinary document, 59 input/output bytes: p50 0.007208 ms, p95 0.011541 ms;
- near-limit document, 241,123 input/output bytes: p50 0.809000 ms, p95
  0.981250 ms;
- process RSS: 89,849,856 bytes.

Each stable timing used 21 warm-up and 101 measured samples. These values
activate no support or regression budget.

## Limitations and unrun evidence

- The browser package, Playwright Chromium, React SSR, and hydration lanes
  passed. The native `wasm-bindgen-test` browser harness was not run because a
  ChromeDriver executable was unavailable.
- `x86_64-linux` Nix outputs were omitted as incompatible with the Darwin arm64
  host; no cross-host result is inferred.
- The 300-second fuzz soak and source-bound percentage coverage were not run.
- No external SDK-TS consumer, named legacy-provider rollback rehearsal,
  accepted performance budget, support decision, publication, default switch,
  React Native/native binding, or mdoc evidence exists in this train.

No caller document, verification material, secret, credential, or local
absolute path is retained in this record.

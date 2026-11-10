# Private SDK-TS DID Document local quality evidence

## Scope

This record belongs to `did.document.typescript-canary.quality.v1` and issue
502. It covers only the private repository-local DID Document facade over the
canonical Rust parser for browser, Node.js, and React Web. It does not claim
resolution, dereferencing, registration, method effects, React Native,
publication, support, external SDK-TS adoption, production readiness, or mdoc.

The historical syntax canary receipt remains independent and is not reused as
document evidence.

## Evidence protocol

The final source-bound receipt records one committed implementation revision,
the immutable `did.document.v1` packet digest from the vector catalog, and the
private artifact assembly digest. Until that revision exists, results are
local draft evidence and must not be reported as a completed source-bound
receipt.

Focused commands:

```text
cargo test -p identus-wasm-did
node --test scripts/tests/binding-target-isolation.mjs
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh package-types
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh browser
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh nodejs
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh react-web
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh differential
```

The combined target command builds the web and Node artifacts twice, compares
the generated trees, verifies the reviewed binding declaration, synchronizes a
closed package-local artifact inventory, and emits exact-head target receipts.
Browser, Node ESM/CommonJS, React SSR, and hydrated React client execution all
import the same provider-neutral document assertion module.

## Property and isolation evidence

The completed receipt must identify results for:

- raw JSON reaching Rust before any TypeScript JSON parse;
- normalized serialization and the typed immutable projection;
- scalar/array cardinality and absent/present distinctions;
- reference/embedded relationships and every service endpoint form;
- bounded extension preservation and unsupported unsafe-number rejection;
- duplicate-name, private-JWK, malformed, resource, and redaction failures;
- exact-once generated success/error ownership under projection and observer
  failure;
- no parser fallback after Rust or projection rejection;
- singleton initialization, provider rollback selection, and target module
  format;
- target failure retaining independently green Rust-core evidence while the
  terminal verdict fails.

Target receipts bind both the private assembly digest and the shared syntax
and document vector/assertion inputs. A stale, missing, cancelled, skipped,
timed-out, failed, malformed, or unexpected receipt fails closed.

## Resource measurements

Run:

```text
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh measurements
```

The untracked private output is
`target/sdk-ts-did-measurements.json`. It records pinned environment identity,
artifact bytes and assembly SHA-256, initialization time, ordinary and
near-limit DID timings, ordinary and near-limit DID Document timings, document
input/output bytes, and process RSS. Each stable operation uses 21 warm-up and
101 measured samples and reports p50/p95.

These values are directional diagnostics only. They activate no support or
regression budget and contain no document JSON or verification material.

## Coverage and fuzz routes

The wrapper/property inventory covers all public document success and stable
error outcomes, generated-view disposal, projection branches, relationship and
endpoint variants, and each applicable runtime. The final report records the
tool and exact source revision used; a percentage without operation/branch
inventory is insufficient.

Document fuzz evidence remains a separate sanitizer route:

```text
nix develop .#fuzz --command ./scripts/fuzz-did.sh replay did_document
nix develop .#fuzz --command ./scripts/fuzz-did.sh smoke did_document
```

The candidate soak is recorded separately. Fuzz success cannot substitute for
the cross-target property or differential lanes.

## Final receipt fields

The final update must replace this draft section with:

- exact committed source SHA;
- vector packet SHA-256 and case count;
- artifact assembly SHA-256;
- each command, result, duration, and retained private output path;
- ordinary/near-limit document measurement summary;
- coverage operation/branch inventory;
- fuzz replay/smoke/soak state;
- known limitations and every unrun gate.

No secret, caller document, verification material, local absolute path, or
GitHub credential belongs in the published receipt.

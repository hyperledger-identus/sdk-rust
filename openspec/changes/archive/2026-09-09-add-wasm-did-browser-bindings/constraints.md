# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/224
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-001` continues to prohibit authored unsafe Rust.
- `SDK-SEC-002` prohibits raw secret material across FFI; only public
  identifiers cross this boundary.
- `SDK-SEC-003` requires bounded untrusted inputs; the facade preserves the DID
  and DID URL parser ceilings.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005` require Rust 1.98.1.
- `SDK-ARCH-001` keeps product, chain and UI policy out of this generic adapter.
- `SDK-LIM-002` remains effective: this proof does not activate supported FFI.
- `SDK-LIM-003` remains effective: runtime evidence for two engines is not a
  production browser matrix or certification.

## Introduced or changed constraints

The browser leaf owns ABI/API versioning, JavaScript names, exported values,
errors and WASM memory lifecycle. Generic domain crates remain free of
wasm-bindgen. Runtime and test framework versions are exact and aligned; the
package builder is pinned by Nix. No secret, dependency/domain type, browser
authority, callback, future, thread or worker crosses the public surface.

## Introduced or changed limitations

- Only direct browser ESM is proved; Node, React Native and a specific bundler
  integration are unsupported.
- Chromium and Firefox headless tests are evidence, not minimum-version support.
- Consumers own initialization placement, CSP, CORS, caching, asset hosting and
  freeing retained exported values.
- Artifact size is measurement-only; no budget or performance promise exists.
- The package is unpublished and cannot establish npm or production support.

## Consumer and product impact

Rust and downstream repositories are unchanged. A browser consumer gains an
experimental, reproducible API shape suitable for later React adoption. The
machine support policy remains `not-supported`.

## Activation and rollback

Production activation requires a separate versioned release and named-consumer
decision. Rollback removes the isolated adapter and evidence without migration
or persisted-data impact.

## Evidence

Evidence covers exact dependency and license graphs, Rust 1.98.1 host/WASM
builds, bounds/redaction tests, byte-identical package generation, TypeScript
drift, measured artifacts, Chromium/Firefox behavior, full repository gates and
a distinct architecture/security review.

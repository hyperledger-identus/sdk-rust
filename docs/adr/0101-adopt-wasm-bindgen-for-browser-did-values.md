# ADR 0101: adopt wasm-bindgen for experimental browser DID values

- **Status:** Accepted for an experimental unpublished adapter
- **Date:** 2026-09-09
- **Decision authority:** issue #224 under the binding roadmap and effective
  limitations `SDK-LIM-002` and `SDK-LIM-003`
- **Related work:** #163; ADRs 0097–0100

## Context

The SDK compile-checks generic Rust crates for browser WASM and has native
UniFFI DID packages, but neither supplies a browser JavaScript ABI. The existing
bounded `identus-did` parser is a low-risk public-value capability with useful
React/browser consumers and no secret, state or ambient-browser authority.

Browser binding research compared a separate wasm-bindgen leaf with UniFFI,
handwritten JavaScript objects, serde conversion and bundler-specific packages.
wasm-bindgen is the cohesive direct browser boundary; native UniFFI evidence is
not browser evidence, while serde/DOM/bundler dependencies add no value to this
slice.

## Decision

1. Add unpublished `identus-wasm-did` over unchanged `identus-did` and pin
   `wasm-bindgen` 0.2.121 plus dev-only `wasm-bindgen-test` 0.3.71 exactly. Use
   the matching 0.2.121 CLI from the repository-pinned Nix input.
2. Export API version 1, camelCase parse functions, WASM-owned public-value
   classes and a closed error class carrying only stable redacted codes.
3. Preserve the domain parser's input ceilings. No secret, storage, network,
   browser API, callback, future, thread or worker enters the adapter.
4. Use wasm-pack 0.15.0 from the pinned Nix lock to create browser-native `web`
   ESM. Compare two full release outputs and a reviewed TypeScript snapshot,
   then report uncompressed bytes and hashes without a size budget.
5. Execute matching behavior tests in headless Chromium and Firefox in a new
   weekly/manual slow Ubuntu job. Preserve the current Linux fast PR line.
6. Make initialization and lifecycle explicit: applications await the generated
   initializer, host the paired JS/WASM assets, configure CSP/CORS/cache policy,
   and free retained WASM-owned classes.
7. Keep browser/FFI support `not-supported`. The proof does not select Node,
   React Native, a downstream bundler, npm publication, browser versions or a
   certification profile.

## Consequences

The SDK gains a reusable browser-native seam and executable two-engine evidence
without coupling domain code to JavaScript. A React application can import the
ESM package after one asynchronous bootstrap, but the SDK does not own React or
its bundler.

The runtime and test dependencies are MIT OR Apache-2.0 and declare Rust 1.77;
they fit Rust 1.98.1 and the existing license policy. Dependency-owned unsafe
and browser native code remain behind exact pins, supply-chain gates and the
outer facade. Exported classes allocate in WASM and require lifecycle care;
that cost is preferable to adding a serialization cone before a consumer asks
for plain objects.

## Reconsideration and rollback

Reconsider if browser runtime, maintenance, security or a named consumer's
bundle/API requirements fail, or if a narrower standard generator proves the
same contract. A bundler-specific fixture requires its own pinned decision.
Before publication rollback removes the leaf, devshell, slow job and evidence;
generic DID code, native packages, persisted data and downstreams are unchanged.

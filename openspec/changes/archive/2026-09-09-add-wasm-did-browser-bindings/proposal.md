## Why

Issue #224 is the browser-binding child of #163. The SDK compiles selected
domain crates for `wasm32-unknown-unknown`, but that evidence does not provide a
JavaScript package, browser ABI, TypeScript contract, initialization model or
runtime proof. Native UniFFI packages cannot be treated as browser evidence.

The smallest useful browser slice is the same bounded, public-value DID and DID
URL parser already proven for native consumers. A separate adapter can test the
WASM/JavaScript boundary without introducing secrets, state, async behavior or
browser APIs into the generic DID crate.

## What changes

- Add an unpublished `identus-wasm-did` leaf crate over `identus-did` using
  exact matching `wasm-bindgen` runtime/CLI 0.2.121.
- Export a versioned camelCase JavaScript API, SDK-owned values and closed
  redacted error objects while preserving the domain parser's input bounds.
- Produce a browser-native ESM package with TypeScript declarations, compare
  two release builds byte-for-byte and keep a reviewed declaration snapshot.
- Execute the same success and failure families in headless Chromium and
  Firefox in the weekly/manual slow line.
- Record exact Rust, generator, package-builder and browser-tool versions,
  dependency/license evidence, initialization/CSP/worker/memory/cache behavior,
  measured artifact size and rollback.
- Preserve the current unsupported FFI policy and Linux fast line.

## Capabilities

### New capabilities

- `browser-did-bindings`: owns the experimental WASM/JavaScript DID value,
  error, versioning, package and browser-runtime contract.

## Non-goals

- No Node, React Native, native UniFFI, npm publication or downstream mutation.
- No framework runtime, network, storage, signing, keys, secrets, callbacks,
  threads, workers or async API.
- No production browser-support, semantic-version or certification claim.

## Delivery

Issue #224 owns this slice. It starts at
`develop@1b926b881caf996322200f6df3b059e01e0bcdaf` and requires FFI-class
research/constraint readiness, exact dependency and runtime evidence, full
repository gates, distinct review, a signed/DCO PR and green required CI.

## Context and layout

Create `crates/wasm-did` as unpublished package `identus-wasm-did`, built as
`cdylib` and `rlib`. Its direct runtime dependencies are `identus-did` and exact
wasm-bindgen 0.2.121. Generic DID source remains unchanged. Exact
wasm-bindgen-test 0.3.71 is dev-only, and the matching CLI comes from Nix.

## JavaScript contract

Export `bindingApiVersion()`, `parseDid(string)` and `parseDidUrl(string)`.
Successful calls return SDK-owned WASM classes with read-only component
getters. Failures throw an SDK-owned `DidParseError` class whose read-only
`code` is either `did.invalid_did` or `did.invalid_did_url`. No caller input or
domain diagnostic is retained in the error.

All exported classes own their strings and expose generated `free()` lifecycle
methods. The adapter is synchronous, stateless and does not use browser APIs,
workers, threads or async callbacks. ABI/API version `1` covers function names,
class/getter names, error codes and ownership semantics.

## Package and runtime evidence

Use wasm-pack 0.15.0 from a dedicated Nix `wasm` shell. Build target `web` emits
browser ESM, `.wasm` and TypeScript declarations. A script builds twice into
separate directories, compares the complete trees and the committed declaration
snapshot, and emits hashes and byte measurements. Generated package files stay
untracked.

The same script runs official wasm-bindgen tests in headless Chromium and
Firefox. Linux-only browser packages and WebDrivers live in the slow Nix shell;
the manual/scheduled slow Ubuntu workflow runs both engines. Ordinary host unit
tests cover the facade without adding browser execution to fast CI.

The generated default export must be awaited before calls. A React application
does that once in its bootstrap/provider layer. Consumers host/hash the paired
JS/WASM resources, configure CSP/CORS and free retained class instances. Those
application policies remain outside the SDK.

## Compatibility and rollback

Breaking API or TypeScript changes require an ADR, version bump and consumer
migration after activation. Before publication, rollback removes this leaf and
its slow evidence. The domain model and existing native binding are unaffected.

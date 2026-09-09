# Experimental browser DID binding

**Issue:** [#224](https://github.com/hyperledger-identus/sdk-rust/issues/224)

**Decision:** [ADR 0101](../adr/0101-adopt-wasm-bindgen-for-browser-did-values.md)

**Status:** unpublished evidence; browser FFI remains unsupported

`identus-wasm-did` is a narrow browser-native ESM adapter over the existing
bounded `identus-did` parser. It exports API version 1, DID/DID URL component
classes and `DidParseError`. The only error codes are `did.invalid_did` and
`did.invalid_did_url`; caller input and parser detail are not returned.

## Consumer call shape

A React application can await initialization once in its bootstrap/provider
layer and keep the SDK package independent from React:

```typescript
import init, {
  DidParseError,
  bindingApiVersion,
  parseDid,
} from "./identus_wasm_did.js";

const ready = init({
  module_or_path: new URL("./identus_wasm_did_bg.wasm", import.meta.url),
});

export async function inspectDid(input: string): Promise<string> {
  await ready;
  if (bindingApiVersion() !== 1) throw new Error("unsupported DID binding API");

  try {
    const did = parseDid(input);
    try {
      return `${did.method}:${did.methodSpecificId}`;
    } finally {
      did.free();
    }
  } catch (error) {
    if (error instanceof DidParseError) {
      const code = error.code;
      error.free();
      throw new Error(code);
    }
    throw error;
  }
}
```

The generated initializer accepts a URL, Request, Response, byte buffer or
compiled WebAssembly module. Passing the named `{ module_or_path }` form is the
durable shape; direct positional input is currently marked deprecated upstream.

## Exact inputs and dependency cone

| Input | Exact value | Role |
| --- | --- | --- |
| Rust | 1.98.1 | workspace compiler and etalon |
| wasm-bindgen | 0.2.121 runtime and Nix CLI | runtime/generator schema; MIT OR Apache-2.0; declared Rust 1.77 |
| wasm-bindgen-test | 0.3.71 | dev-only official browser harness; MIT OR Apache-2.0; declared Rust 1.77 |
| wasm-pack | 0.15.0 from `flake.lock` | slow-line package/test tool |
| target | `wasm32-unknown-unknown` / wasm-pack `web` | browser-native ESM with explicit initialization |

The resolved normal/build cone contains 35 unique rendered tree lines including
the SDK/DID graph. Including the browser test harness contains 56. Direct
dependencies are `identus-did` and wasm-bindgen; wasm-bindgen-test is dev-only.
No direct `web-sys`, `js-sys`, serde-to-JS or JavaScript framework dependency is
introduced.

## Reproducibility and measurements

Two exact release builds produced byte-identical complete package trees. The
reviewed TypeScript surface is
[`crates/wasm-did/api/typescript-api.d.ts`](../../crates/wasm-did/api/typescript-api.d.ts).
Measurements are uncompressed and are evidence, not budgets:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `identus_wasm_did.js` | 14,896 | `ad91b42cb1d2f2d7d1fd396b7a6019c5cf4d55d2bc563dee77f88a4c1ebb9c13` |
| `identus_wasm_did.d.ts` | 4,360 | `6e2b997da95670828d196bcb4c6de5b67e95650043f39273e3b4fd17223e3ae0` |
| `identus_wasm_did_bg.wasm` | 33,833 | `100b1d176093b9bfbbe87d8c7b58e5b2c761394ff0e2bc0dba002aff8baf575e` |
| `identus_wasm_did_bg.wasm.d.ts` | 1,594 | `e87404b51a37f0ddfaa3cee7bc816126e8a4e82c2d2b69b861d3094463f5adc3` |
| `package.json` | 319 | `d1c441370dc33e7febe04290a4a4e06951dd5f58e7cff9dfb43c8b492240cca3` |

Local Chrome 152.0.7977.83 with ChromeDriver 152.0.7977.65 passed the two
browser behavior families. The weekly/manual Ubuntu gate supplies pinned Nix
Chromium/ChromeDriver and Firefox/GeckoDriver and requires both independently.

## Integration and limitation contract

- The default initializer must resolve before any exported call.
- The consumer hosts the paired `.js`/`.wasm`, keeps their revisions together,
  and configures same-origin or CORS delivery and cache invalidation.
- The consumer owns CSP. WebAssembly compilation may require the applicable
  `wasm-unsafe-eval` or browser policy allowance; this SDK does not weaken CSP.
- Returned values and errors are WASM-owned. Explicit `free()` is required for
  retained/high-volume values; generated disposal/finalization is convenience,
  not the lifecycle contract.
- The API is synchronous and single-threaded. It enables no workers,
  `SharedArrayBuffer`, blocking main-thread work, storage or network authority.
- Node, React Native, npm publication, a named bundler and production browser
  versions remain unsupported and need their own adoption evidence.

Run package-only or browser evidence through:

```bash
nix develop .#wasm --command ./scripts/check-wasm-did-browser.sh --package-only
nix develop .#wasm --command ./scripts/check-wasm-did-browser.sh --all-browsers
```

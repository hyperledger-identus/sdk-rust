# Identus SDK for Rust

Rust SDK for building decentralized identity solutions with the [Identus](https://github.com/hyperledger-identus) ecosystem.

## Getting Started

> **⚠️ Work in Progress** — This SDK is under active development.

### Prerequisites

- Rust (latest stable)
- `cargo`

### Development

```bash
# Build
cargo build

# Test
cargo test
```

## Cross-compilation to WASM

The SDK supports cross-compilation to `wasm32-unknown-unknown` for Web/browser targets.

### WASM Prerequisites

- A C compiler (`clang`) is required at build time — the dependency graph includes
  crates with C/Rust FFI (elliptic curve operations via `blst`).
- The `wasm32-unknown-unknown` target is pre-installed in the Nix devshell.

### Building

From the Nix devshell:

```bash
cargo build --target wasm32-unknown-unknown -p identus-crypto
```

`CC_wasm32_unknown_unknown` is automatically set to the unwrapped clang in the
devshell — no manual configuration is needed.

### Important notes

- **Runtime randomness (`getrandom`):** The `wasm_js` feature is enabled on
  `getrandom` for WASM compilation. On the `wasm32-unknown-unknown` target,
  `getrandom` delegates to `Math.random()` via the JS environment at runtime.
  This is transparent to the caller but means the WASM binary requires a JS
  host to execute.
- **No JS interop:** This target does **not** include `wasm-bindgen`, `web-sys`,
  or any WASM-to-JS bindings. The compiled `.wasm` binary contains Rust types
  only. WASM bindings are a separate concern.

## Resources

- [Identus Documentation](https://hyperledger-identus.github.io/)
- [Hyperledger Identus GitHub](https://github.com/hyperledger-identus)

# Identus Crypto — WASM Web App Demo

An interactive browser-based demo of Identus SDK cryptography using WebAssembly.
Generate Schnorr key pairs over the Jubjub curve, sign messages, and verify
signatures — all running client-side in your browser.

## Architecture

```
lib/identus-crypto-wasm/     ← Rust crate exposing JS bindings via wasm-bindgen
  └── src/lib.rs              ←   generate_key(), sign(), verify()
examples/wasm-app/            ← Static web frontend (NOT a Rust crate)
  ├── index.html              ←   Demo UI
  ├── index.js                ←   ES module that imports the WASM glue
  ├── style.css               ←   Dark-theme styling
  └── README.md               ←   This file
nix/packages/
  ├── default.nix             ← Flake-parts module wiring the packages
  ├── demo-dir.nix            ← Nix derivation: builds WASM + stages web files
  ├── wasm-bindgen-cli.nix    ← wasm-bindgen CLI built from source
  └── uniffi-bindgen.nix      ← UniFFI bindgen for foreign-language bindings
nix/apps/
  ├── default.nix             ← Flake-parts module wiring the example-web app
  └── example-web.nix         ← Serve script (python3 http.server)
```

### How it works

1. The **WASM binding crate** (`lib/identus-crypto-wasm/`) is a Rust library
   compiled to `wasm32-unknown-unknown` with `#[wasm_bindgen]` annotations. It
   exposes three functions:
   - `generate_key()` → `{ secret_key_hex, public_key_hex }`
   - `sign(message, secret_key_hex)` → signature hex
   - `verify(message, signature_hex, public_key_hex)` → boolean

2. The **static HTML/JS/CSS** in `examples/wasm-app/` is a plain ES module page
   that dynamically imports the WASM glue code and calls the functions above.

3. The **Nix app** (`nix run .#example-web`) wraps everything: it builds the
   WASM crate with hermetic dependencies (no network at build time), runs
   `wasm-bindgen --target web` to produce JavaScript bindings, co-locates the
   static web files, and serves the result via Python's HTTP server.

## Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- A modern browser that supports ES modules and WebAssembly

## Quick Start (Nix)

Run the demo from the `sdk-rust/` directory:

```bash
nix run .#example-web
```

This builds the WASM module (first run may take a while to fetch dependencies)
and starts a local HTTP server at **http://localhost:8080**.

Open http://localhost:8080 in a browser to use the demo.

**Note:** The Nix derivation pre-fetches all Cargo dependencies at evaluation
time via `cargoLock.lockFile`, so no network access is needed during the build.
This works in any Nix sandbox without `__noChroot`.

## Manual Build (without Nix)

If you prefer to build manually with `wasm-pack`:

```bash
# Build the WASM module (from sdk-rust/)
just build-wasm

# Copy the generated files alongside the demo HTML
cp lib/identus-crypto-wasm/pkg/* examples/wasm-app/

# Serve from the examples directory
python3 -m http.server 8080 --directory examples/wasm-app/
```

Open http://localhost:8080 in a browser.

## Usage

1. Click **Generate Key** to create a new random Schnorr key pair.
2. Type or edit a message in the text area.
3. Click **Sign** to produce a signature for the message with the generated key.
4. Click **Verify** to check the signature against the original public key.
5. Click **Verify (wrong key)** to confirm that a different public key correctly
   rejects the signature.

## WASM Binary Size

The `identus-crypto` dependency chain includes elliptic curve arithmetic (Jubjub
curve via `midnight-transient-crypto`), which results in a WASM binary of
several hundred kilobytes. This is acceptable for a developer demo but worth
noting for production use.

## File Overview

| File | Purpose |
|------|---------|
| `index.html` | Demo page with message input, buttons, and result displays |
| `index.js` | ES module that imports WASM glue and wires up UI interactions |
| `style.css` | Dark-theme styling (GitHub-inspired) |
| `README.md` | This documentation |

## Related Tasks

- **TASK-7** — WASM binding crate (`lib/identus-crypto-wasm/`)
- **TASK-3** — WASM cross-compilation target (`wasm32-unknown-unknown`)

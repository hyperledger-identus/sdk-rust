# Identus Crypto — WASM Web App Demo

An interactive browser-based demo of Identus SDK cryptography using WebAssembly.
Generate Schnorr key pairs over the Jubjub curve, sign messages, and verify
signatures — all running client-side in your browser.

## Architecture

```text
lib/identus-crypto-wasm/     ← Rust crate exposing JS bindings via wasm-bindgen
  └── src/lib.rs              ←   generate_key(), sign(), verify()
examples/wasm-app/            ← Static web frontend (NOT a Rust crate)
  ├── index.html              ←   Tab-based demo UI (3 tabs)
  ├── index.js                ←   ES module that imports the WASM glue
  ├── style.css               ←   Dark-theme styling
  └── README.md               ←   This file
```

### How it works

1. The **WASM binding crate** (`lib/identus-crypto-wasm/`) is a Rust library
   compiled to `wasm32-unknown-unknown` with `#[wasm_bindgen]` annotations. It
   exposes three functions:
   - `generate_key()` → `{ secret_key_hex, public_key_hex }`
   - `sign(message, secret_key_hex)` → signature hex
   - `verify(message, signature_hex, public_key_hex)` → boolean

2. The **static HTML/JS/CSS** in `examples/wasm-app/` is a plain ES module page
   with a 3-tab interface. Each tab corresponds to one WASM function:
   - **Generate Key** tab calls `wasm.generate_key()` and displays the key material.
   - **Sign** tab reads a message and secret key, calls `wasm.sign()`, and shows the signature.
   - **Verify** tab reads a message, signature, and public key, calls `wasm.verify()`, and shows ✅/❌.
   Tabs are independent — users copy values between them manually, mirroring real API usage.

3. The **justfile workflow** (`just build-wasm` → `just build-web-example` → `just run-web-example`)
   compiles the WASM crate with `wasm-pack`, stages the WASM bindings alongside the static web
   files into `target/web-demo/`, and serves them via Python's built-in HTTP server.

## Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
  (`nix develop -c ...` provides `wasm-pack`, `python3`, and all toolchain dependencies)
- A modern browser that supports ES modules and WebAssembly

## Quick Start

Run the demo from the `sdk-rust/` directory inside the Nix devshell:

```bash
# Enter the devshell
nix develop -c $SHELL

# Build and run the WASM web demo
just run-web-example
```

This will:

1. Build the WASM module via `wasm-pack` (`build-wasm`)
2. Stage all files into `target/web-demo/` (`build-web-example`)
3. Start a local HTTP server at **`http://localhost:8080`**

Open `http://localhost:8080` in a browser to use the demo.

To build without serving:

```bash
just build-web-example
```

The staged output will be in `target/web-demo/`.

## Manual Build (without Nix)

If you have `wasm-pack` and `python3` installed locally:

```bash
# Build the WASM module (from sdk-rust/)
just build-wasm

# Stage the files manually
mkdir -p target/web-demo
cp lib/identus-crypto-wasm/pkg/*.wasm target/web-demo/
cp lib/identus-crypto-wasm/pkg/*.js target/web-demo/
cp examples/wasm-app/index.html target/web-demo/
cp examples/wasm-app/index.js target/web-demo/
cp examples/wasm-app/style.css target/web-demo/

# Serve from the staged directory
python3 -m http.server 8080 --directory target/web-demo/
```

## Usage

The demo is organized into three independent tabs:

### 1. Generate Key

Click the **Generate Key** tab and press **Generate** to create a new random
Schnorr key pair. The secret key and public key are displayed in read-only
text fields — select and copy the values you need.

### 2. Sign

Switch to the **Sign** tab. Paste the secret key (from Generate Key) into the
"Secret Key (hex)" field and type a message. Click **Sign** to produce a
signature. The signature hex is displayed in a read-only text field — copy it
for verification.

### 3. Verify

Switch to the **Verify** tab. Paste the original message, the signature hex,
and the public key hex into the three input fields. Click **Verify** to check
the signature. A ✅ Valid or ❌ Invalid badge is shown.

## WASM Binary Size

The `identus-crypto` dependency chain includes elliptic curve arithmetic (Jubjub
curve via `midnight-transient-crypto`), which results in a WASM binary of
several hundred kilobytes. This is acceptable for a developer demo but worth
noting for production use.

## File Overview

- `index.html` — Tab-based demo page with Generate Key, Sign, and Verify tabs
- `index.js` — ES module that imports WASM glue and wires up tab interactions
- `style.css` — Dark-theme styling (GitHub-inspired)
- `README.md` — This documentation

## Justfile Recipes

| Recipe | Description |
| ------ | ----------- |
| `just build-wasm` | Compile the WASM binding crate (`lib/identus-crypto-wasm/`) |
| `just build-web-example` | Build WASM + stage all files into `target/web-demo/` |
| `just run-web-example` | Build + stage + serve at `http://localhost:8080` |

## Related Tasks

- **TASK-7** — WASM binding crate (`lib/identus-crypto-wasm/`)
- **TASK-3** — WASM cross-compilation target (`wasm32-unknown-unknown`)
- **TASK-12** — Tab-based UI refactor
- **TASK-17** — Justfile dev workflow for WASM web demo

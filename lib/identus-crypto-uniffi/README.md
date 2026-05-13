# identus-crypto-uniffi

UniFFI bindings for Identus cryptographic operations (JubJub Schnorr signatures).

This crate wraps [`identus-crypto`](../identus-crypto) functions using Mozilla's
[UniFFI](https://github.com/mozilla/uniffi-rs) framework, producing a shared
library (`cdylib`) with automatically-generated foreign-language bindings for
**Kotlin** (Android/JVM) and **Swift** (iOS/macOS).

## API

Three exported functions mirror the existing [WASM binding](../identus-crypto-wasm):

| Function | Returns | Description |
| --- | --- | --- |
| `generate_key()` → `KeyInfo` | `KeyInfo { secret_key: String, public_key: String }` | Generate a random Schnorr keypair over the Jubjub embedded curve. Both keys are hex-encoded (64 hex chars each). |
| `sign(secret_key: String, message: Vec<u8>)` → `String` | Hex-encoded 64-byte Schnorr signature | Sign a message. Internally hashes `message` using the Blake2b-512 → wide reduction pipeline. Returns an error if the secret key is not valid hex or not a valid Jubjub scalar. |
| `verify(public_key: String, message: Vec<u8>, signature: String)` → `bool` | `true` if valid, `false` otherwise | Verify a Schnorr signature. Returns `false` on invalid input (no panic). |

### Generated language bindings

| Language | `message` type | File location |
| --- | --- | --- |
| Kotlin | `kotlin.ByteArray` | `bindings/kotlin/` |
| Swift | `Data` | `bindings/swift/` |

## Message hashing pipeline

To match the behavior of `identus-crypto-wasm`, the `sign()` and `verify()`
functions hash messages using this pipeline before calling the underlying
`schnorr::sign` / `schnorr::verify`:

1. **Blake2b-512** hash of raw message bytes → 64-byte digest
2. **Jubjub wide reduction**: `embedded::Scalar::from_bytes_wide(&[u8; 64])`
   → `EmbeddedFr` (Jubjub embedded curve scalar)
3. **Conversion**: `Fr::try_from(EmbeddedFr)` → BLS12-381 outer scalar

This ensures **signature compatibility** between the WASM binding and UniFFI
bindings — a message signed via `identus-crypto-wasm` can be verified via
`identus-crypto-uniffi` and vice versa.

## Building

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- Enter the devshell: `nix develop` (from `sdk-rust/`)
- The devshell provides the nightly Rust toolchain and `uniffi-bindgen`
  (v0.31.1) — see [TASK-11](../../nix/packages/uniffi-bindgen.nix)

### Build the crate

```bash
cargo build -p identus-crypto-uniffi
```

This produces:

- `target/debug/libidentus_crypto_uniffi.so` (Linux) — the shared library
  with UniFFI metadata
- `target/debug/libidentus_crypto_uniffi.rlib` — the Rust static library

> **Note:** Use debug builds for `uniffi-bindgen generate --library`.
> Release builds with LTO may strip the UniFFI metadata ELF section.

### Run tests

```bash
cargo test -p identus-crypto-uniffi
```

### Generate bindings

Use the provided `just` commands:

```bash
# Build + generate all bindings
just build-uniffi

# Generate Kotlin bindings only (from existing build)
just generate-kotlin-bindings

# Generate Swift bindings only (from existing build)
just generate-swift-bindings
```

Or run `uniffi-bindgen` directly (using `find` to locate the library,
which works on both Linux (.so) and macOS (.dylib)):

```bash
# Locate the built library (platform-agnostic)
LIB_FILE=$(find target/debug -maxdepth 1 \
  -name "libidentus_crypto_uniffi.*" \
  ! -name "*.rlib" ! -name "*.d" \
  2>/dev/null | head -1)

# Kotlin
uniffi-bindgen generate --library "$LIB_FILE" \
  --language kotlin \
  --out-dir lib/identus-crypto-uniffi/bindings/kotlin/

# Swift
uniffi-bindgen generate --library "$LIB_FILE" \
  --language swift \
  --out-dir lib/identus-crypto-uniffi/bindings/swift/
```

## Consuming the bindings

### Android (Kotlin)

The generated Kotlin bindings (`bindings/kotlin/`) depend on the
[`uniffi_jni`](https://mvnrepository.com/artifact/mozilla.uniffi/uniffi-jni)
runtime and [JNA](https://github.com/java-native-access/jna). To use them
in an Android app:

1. Add the generated `.kt` files to your project
2. Add dependencies on `uniffi-jni` and `com.sun.jna:jna`
3. Load the native library: `System.loadLibrary("identus_crypto_uniffi")`
4. Call `generateKey()`, `sign()`, `verify()` as top-level functions

See `examples/android-app/` for a complete example (TASK-8).

### iOS (Swift)

The generated Swift bindings (`bindings/swift/`) use a C-FFI shim. To use
them in an iOS app:

1. Add the generated `.swift` file and the `.dylib` (or `.a` for static
   linking) to your Xcode project
2. The `identus_crypto_uniffi` Swift module exposes the same API:
   `generateKey()`, `sign(secretKey:message:)`, `verify(publicKey:message:signature:)`

## Versioning

| Component | Version |
| --- | --- |
| `uniffi` crate (workspace dep) | 0.31.1 |
| `uniffi-bindgen` CLI | 0.31.1 |

The crate dependency and CLI tool **must always match** to ensure ABI
compatibility. The version is pinned in:

- `sdk-rust/Cargo.toml` — workspace dependency
- `sdk-rust/nix/packages/uniffi-bindgen.nix` — Nix derivation

## Implementation notes

- Uses the UniFFI **proc-macro** approach (`#[uniffi::export]`), not the
  older UDL file approach
- Uses `--library` mode for `uniffi-bindgen generate` — reads metadata
  directly from the compiled `.so` rather than requiring a `.udl` file
- UniFFI metadata is stored in a special ELF section — debug builds are
  recommended for bindgen to ensure the section is not stripped
- `sign()` uses `OsRng` internally for cryptographically secure randomness
- `sign()` returns an error (thrown exception in Kotlin/Swift) on invalid input
  (bad hex, wrong key size) rather than panicking — safe for foreign-language callers
- `verify()` returns `false` on invalid input (bad hex, wrong key size)
  rather than panicking — safe for foreign-language callers

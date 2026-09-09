# UniFFI DID research spike

This unpublished nested workspace proves issue #215 without adding UniFFI to
the SDK workspace or changing `identus-bindings`. It exposes only owned public
identifier records and a closed redacted error enum. `identus-did` remains
binding-framework-free.

Run on an Apple Silicon macOS host with Rust 1.98.1, Swift 6.3, an arm64 JDK 17
and Gradle 8.12.1:

```bash
./scripts/verify.sh
```

The script builds the Rust `cdylib`, generates Swift and Kotlin twice from
UniFFI 0.32.0 library metadata, compares the complete generated trees, checks
the normalized API snapshots, then compiles and executes consumer-shaped
native tests. Generated sources and build products stay under `target/`.

Passing this host proof does not establish iOS/Android device, packaging,
React Native, browser, certification or production-ABI support.

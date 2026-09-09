# ADR 0099: prove a local UniFFI DID Apple package

- **Status:** Accepted
- **Date:** 2026-09-09
- **Decision authority:** issues #163, #222 and #228 under standing SDK mandate
- **Builds on:** ADR 0098
- **Related work:** issue #228

## Context

ADR 0098 establishes an ABI-versioned DID/DID URL UniFFI boundary and proves
generated Swift/Kotlin behavior on macOS hosts. It does not prove that Rust can
produce distinct iOS device and Simulator static libraries, that SwiftPM can
select and import an XCFramework slice, or that the resulting code runs in an
iOS runtime.

Apple requires alternate build systems to keep device and Simulator libraries
separate inside an XCFramework. UniFFI 0.32 generates the Swift source and C
header required by the accepted ABI. Its XCFramework module-map template,
however, declares a framework module and explicitly imports Darwin plus builtin
modules. Exact Xcode 26.4 experiments show that shape cannot be imported by a
SwiftPM static-library binary target. Removing only the builtin imports is
insufficient; a plain static-library module declaration is also required.

## Decision

1. Extend only `identus-uniffi-did` with a `staticlib` artifact. Retain its
   existing host `cdylib` and Rust `rlib` products and ABI version 1.
2. Build separate arm64 `aarch64-apple-ios` and
   `aarch64-apple-ios-sim` release archives from the same source, lock and Rust
   1.98.1 toolchain with iOS 15.0 as the explicit deployment floor. Never
   combine device and Simulator code with `lipo`.
3. Use the separately locked exact UniFFI 0.32.0 Swift generator for source,
   header and module-map input. Keep the generated Swift source and C header
   byte-for-byte unchanged.
4. Apply a fail-closed static-library module-map adapter. It accepts only the
   complete known UniFFI 0.32.0 template, removes its `framework` qualifier and
   three explicit Darwin/builtin `use` lines, and emits a plain module over the
   generated header. Generator drift is an error. No unchecked or unsafe Swift
   compiler flag is permitted.
5. Assemble both archives and identical headers with
   `xcodebuild -create-xcframework`. Expose that local binary target beneath a
   generated Swift source target and test only the public `IdentusDid` product.
6. Build/package independently twice. Compare both static archives, all
   generated content and complete normalized package trees; normalize only the
   semantically unordered XCFramework library list.
7. Inspect architecture, Apple platform/minimum version, required FFI symbols,
   module-map shape, absolute-path absence and artifact sizes. Execute API
   version, valid identifier, invalid identifier, oversized input and redacted
   error behavior on an available arm64 iOS Simulator through Xcode.
8. Keep every generated source and binary under ignored `target/` paths. Track
   only the package/test template and gate. Run the proof in the weekly/manual
   slow macOS lane, leaving fast Linux PR CI unchanged.

## Consequences

The repository gains reproducible evidence that one isolated Rust static
library can be consumed and executed through a local Swift package on iOS
Simulator. The module-map adapter is intentionally coupled to exact UniFFI
0.32.0 output so an upstream template change requires a conscious review rather
than an accidental textual rewrite.

This is not a supported Apple SDK. The package is unsigned, unpublished and not
run on a physical device or older-runtime matrix. Xcode and Apple SDK versions
are observed host inputs rather than Nix-fetched artifacts. Linking multiple
independently packaged Rust static libraries may duplicate Rust runtime symbols;
a future native SDK should aggregate modules into one Rust static library or
prove another composition model before distribution. Android evidence remains
separate.

No generic DID or crypto crate gains Apple or UniFFI coupling. No secret,
signing, storage, networking, callback, async or handle surface is exported.
The root and generator dependency cones do not change, and authored unsafe Rust
remains forbidden.

## Compatibility and rollback

The public Rust and cross-language ABI remains version 1. This local package
shape is experimental and does not activate a URL, checksum, signing identity,
semantic version or support promise. Rollback removes `staticlib`, the dedicated
generator entry point, Apple package template/gate, slow-lane invocation and
this evidence. Existing host bindings, persisted identifiers and downstream
repositories remain unchanged. `SDK-LIM-002` and `SDK-LIM-003` remain effective.

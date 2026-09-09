## Why

Issue #226 is the first production implementation child of native-binding issue
#222. The accepted research in #215 proved that bounded DID and DID URL parsing
is useful and viable through UniFFI 0.32, but the proof is intentionally outside
the workspace and cannot be consumed as an SDK component.

The next smallest useful slice is the native binding foundation itself: an
isolated, unpublished crate with a versioned value/error ABI, deterministic
Swift and Kotlin generation, and executable host-language tests. Mobile
packaging and device support remain separate so they cannot silently broaden
this change's support claim.

## What changes

- Add `identus-uniffi-did`, an isolated workspace crate that depends on
  `identus-did` and exact UniFFI 0.32.0 while keeping domain crates UniFFI-free.
- Export only DID and DID URL parsing/component records, a binding API version,
  and closed errors with stable redacted codes.
- Contain unexpected Rust panics before the authored wrapper returns to generated
  scaffolding, without exposing panic payloads.
- Add an exact-version bindgen tool outside the runtime workspace dependency
  cone, reproducible complete-tree/API drift checks, and executable Swift and
  Kotlin/JVM host smoke tests.
- Record the new experimental component in the bootstrap inventory and clarify
  that `SDK-LIM-002` remains effective until #222 supplies mobile package/runtime
  evidence.
- Accept the production host foundation in a new ADR without claiming release,
  publication, mobile, React Native, browser, Node or certification support.

## Capabilities

### New capabilities

- `native-did-bindings`: owns the bounded cross-language DID value, error,
  versioning, generation and host-runtime contract.

## Non-goals

- No XCFramework, SwiftPM, AAR, NDK, iOS simulator/device or Android
  emulator/device package or runtime claim.
- No secret, key, signing, storage, networking, callback, future, opaque handle
  or thread-affine object crosses the ABI.
- No React Native, browser, Node, publication, release or downstream mutation.
- No replacement or activation of the quarantined `identus-bindings`
  placeholder in this slice.

## Delivery

Issue #226 owns this slice under #222/#163. It starts at
`develop@808119d7f3b6d52c8fd89cfae68f0d61cb68d402` and requires FFI-class
research/constraint readiness, architecture/security review, exact host runtime
evidence, full repository gates, a signed/DCO PR and green required CI.

## Why

Issue #228 is the next packaging child of native-binding issue #222. The host
foundation merged through #227 proves the versioned DID/DID URL boundary and
generated Swift behavior, but it deliberately stops before an Apple-consumable
artifact. A static library built for macOS is not evidence that SwiftPM can
select, import, link and execute separate iOS device and Simulator variants.

The smallest honest next slice is a deterministic local XCFramework and Swift
package proof. It closes the build-system gap without publishing an artifact or
removing `SDK-LIM-002`.

## What changes

- Add `staticlib` to the isolated `identus-uniffi-did` crate while retaining its
  existing `rlib` and host `cdylib` products.
- Add an exact UniFFI 0.32 Swift generator entry point for XCFramework-compatible
  `module.modulemap` output, without adding a dependency.
- Build arm64 iOS-device and arm64 iOS-Simulator archives with Rust 1.98.1 and an
  explicit iOS 15.0 minimum.
- Assemble the two distinct archives and generated header into a local
  XCFramework, then compose the generated Swift source above it as a SwiftPM
  source target depending on a local binary target.
- Generate/package twice and compare normalized complete trees, architecture,
  minimum-platform, public symbol, manifest and size evidence.
- Compile and execute the package's behavior tests on an available iOS
  Simulator through Xcode 26.4.
- Run the Apple package gate only in the existing weekly/manual slow macOS lane
  and keep the default shell plus fast PR line unchanged.
- Record an Apple packaging ADR and retain the existing unsupported-mobile
  limitation.

## Capabilities

### Modified capabilities

- `native-did-bindings`: adds deterministic Apple binary/package construction
  and iOS Simulator execution evidence to the existing host foundation.

## Non-goals

- No publication, remote binary URL/checksum contract, code signing, App Store,
  physical-device execution or production support activation.
- No Android, React Native, browser, Node, macOS product, Catalyst, watchOS,
  tvOS or visionOS package.
- No secret, signing, storage, networking, async, callback or handle surface.
- No downstream repository mutation and no generic-domain dependency change.

## Delivery

Issue #228 owns this slice under #222/#163. It starts at
`develop@f941c73f1e9f038f6795a14828b341138ee40430` and requires FFI-class
research/constraint readiness, a specification commit before implementation,
an exact-diff architecture/security review, all local gates, a signed/DCO PR and
green required hosted CI.

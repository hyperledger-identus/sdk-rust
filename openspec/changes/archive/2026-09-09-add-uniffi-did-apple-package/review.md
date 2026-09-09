# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Evidence head: 0555a2827b8a66e07a76b6a5ecbb7aa8ccf24d05
Specification commits: c18243680e86c3225fc2e427febaa8cca8ddfac7 and e4f78431d8d0ad8deb3dad45f72e25e556fc4280
Unresolved blockers: none

## Scope reviewed

The review inspected the complete
`develop@f941c73f1e9f038f6795a14828b341138ee40430...0555a282` diff,
issue #228, ADR 0099, the Apple package delta contract, both generated platform
artifacts, the SwiftPM wrapper and test, the dedicated Nix shell, the weekly
workflow change, the architecture/support ledgers and all local verification
receipts.

## Findings

1. **Architecture and cohesion — accepted.** Apple and UniFFI mechanics remain
   in the outer `identus-uniffi-did` boundary, its isolated bindgen tool and
   verification assets. `identus-core`, `identus-did`, `identus-crypto` and
   `identus-jose` are byte-for-byte unchanged from the base.
2. **Artifact model — accepted.** `staticlib` is additive to the existing
   `cdylib` and `rlib`. Device and Simulator are independently compiled from
   one source and lock, remain separate XCFramework variants and are never
   combined with `lipo`.
3. **Platform identity — accepted.** `lipo`, XCFramework plist and `vtool`
   checks jointly prove arm64 iOS, arm64 iOS Simulator and minimum iOS 15.0.
   Required FFI symbols are inspected with the pinned Rust `llvm-nm`.
4. **Generator boundary — accepted after empirical correction.** Exact Xcode
   26.4 tests showed UniFFI 0.32's framework module map cannot be imported as a
   static SwiftPM binary target. The adapter matches the complete seven-line
   upstream output before emitting the four-line plain static module; any
   generator drift fails. Generated Swift and C header content stays
   unmodified, and no unsafe or unchecked Swift flag is used.
5. **Determinism and hygiene — accepted.** Two clean target trees produce
   byte-identical device archives, Simulator archives, generator output and
   complete normalized package trees. Only semantically unordered plist entry
   order is normalized. Generated content stays below ignored `target/`, and
   the final package contains no absolute repository path.
6. **Behavior and diagnostics — accepted.** Xcode compiles, links and executes
   the public Swift product on an arm64 iOS Simulator. Tests cover API version,
   valid DID/DID URL projections, malformed input, oversized input and absence
   of caller text in errors. No secret, key, callback, handle, async, storage or
   network surface is added.
7. **Dependency and unsafe posture — accepted.** Neither root nor bindgen lock
   changes. No package, feature or license enters either resolved graph. The
   only changed first-party Rust source is a safe three-line generator entry
   point; workspace unsafe guards pass.
8. **CI and support accuracy — accepted.** The dedicated bindings shell pins
   Rust 1.98.1 plus only the Apple targets and LLVM tools it needs. The fast
   Linux line and primary shell remain unchanged. The proof runs in the
   weekly/manual macOS lane, while `[ffi].status`, `SDK-LIM-002` and
   `SDK-LIM-003` remain not-supported/effective.
9. **Static-library composition risk — accepted as a limitation.** This proof
   packages exactly one Rust static library. It does not imply that multiple
   independently packaged Rust XCFrameworks can link without duplicate runtime
   symbols. Aggregate native SDK design remains a future decision.
10. **Delivery boundary — accepted.** No binary, generated bridge, signature,
    remote SwiftPM URL, checksum or release artifact is committed or published.
    Apollo, NeoPRISM, midnight-identity, Lace and Oxid are unchanged.

## Residual limitations

- Only arm64 iOS device and arm64 iOS Simulator variants are constructed.
- One available Simulator is runtime evidence; physical devices, signing,
  installation, older runtimes and downstream applications remain unproven.
- Host Xcode and Apple SDKs are observed and receipted inputs, not Nix-fetched
  artifacts.
- The package remains local, unsigned, unpublished and unsupported.

## Review decision

The change is bounded, deterministic, reversible and consistent with the
existing ABI and support policy. No unresolved correctness, architecture,
security, privacy, dependency, licensing, portability or delivery blocker
remains for hosted review.

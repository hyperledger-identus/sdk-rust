# Verification

## Local evidence

- Exact generator graph, ban/license/source policy and current RustSec audit:
  passed in the Nix `bindings` shell. Only RUSTSEC-2026-0374 was scoped out;
  the current advisory database loaded 1,381 entries.
- Guard fixtures: 4 passed, covering success, malformed/expired clocks, graph
  drift, deny failure and unexpected audit failure.
- Swift and Kotlin/JVM host consumers: passed after correcting their stale
  4,096-byte fixture to exercise the current independent 8,192-byte DID URL
  suffix bound.
- Apple evidence: deterministic device/Simulator archives and XCFramework,
  Swift package import, and iOS Simulator test passed with Rust 1.98.1, Xcode
  26.4, Swift 6.3 and iOS deployment target 15.0.
- Android ARM64 package evidence: two deterministic AAR builds, consumer APK
  assembly, lock comparison and receipt passed with Rust 1.98.1, Gradle 8.14.4,
  JDK 17.0.19 and NDK 27.0.12077973.
- Factory and strict OpenSpec validation: 121 items passed.
- Claude CLI reviewed exact implementation head
  `b2aad0cbbb131118bd9c5530ee3ec1a220c152e4` and reported no blocker, high or
  medium findings. The review covered fail-closed ordering, exact graph and
  advisory scope, fixtures, CI registration, receipt integrity and host bounds.

## Findings corrected during reproduction

The supply-chain failure had masked stale host fixtures left from the former
4,096-byte DID URL ceiling. The canonical SDK contract now permits 131,072-byte
DIDs and 139,264-byte DID URLs while retaining an 8,192-byte URL suffix bound.
All four host fixtures now test that stable suffix boundary without changing
production behavior.

The Android receipt also invoked `gradle --version` without its selected
`JAVA_HOME`, which produced a green package check with an empty Gradle version.
The script now binds the version command to the same JDK, rejects empty evidence
and records Gradle 8.14.4.

## Remaining evidence

Protected hosted CI remains pending. The next natural scheduled slow run, not a
rerun or manual dispatch, will be the authoritative post-merge weekly receipt.

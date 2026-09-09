## ADDED Requirements

### Requirement: Android AAR carries one explicit SDK ABI

The experimental Android package SHALL build `identus-uniffi-did` from the
same source, Cargo lock and Rust 1.98.1 toolchain for
`aarch64-linux-android`, using exact NDK 27.0.12077973 and minimum Android API
21. The SDK AAR SHALL contain the generated Kotlin API and exactly one SDK
native library at `jni/arm64-v8a/libidentus_uniffi_did.so`; it SHALL NOT
contain another SDK ABI or a copied/shadowed JNA implementation.

#### Scenario: packaged ABI is inspected

- **WHEN** the package gate opens the AAR
- **THEN** its native payload SHALL be an API-21 AArch64 ELF with the required
  ABI-version, checksum, DID and DID URL symbols and reviewed runtime needs

### Requirement: JNA remains an explicit exact dependency

Android consumers SHALL resolve exact `net.java.dev.jna:jna:5.18.1@aar`
separately from the local SDK AAR. The gate SHALL lock and inventory that
dependency and SHALL execute a consumer using only the SDK AAR plus its
declared exact runtime dependency.

#### Scenario: local consumer assembles

- **WHEN** the consumer application depends on the local SDK AAR
- **THEN** adding the exact JNA AAR dependency SHALL package both native
  libraries without requiring caller-owned JNI code or copied resources

### Requirement: Android construction is deterministic and ephemeral

Two independent builds SHALL produce byte-identical Rust libraries, generated
Kotlin and normalized complete AAR trees. The gate SHALL reject unexpected
archive paths, absolute checkout paths, ABI/API/ELF drift and missing symbols;
it SHALL record tool versions, dependency closure and artifact sizes. Generated
outputs, Gradle state and AVD state SHALL remain under ignored `target/` paths.

#### Scenario: package generation is repeated

- **WHEN** two independent package trees are constructed from the same locks
- **THEN** normalized content comparison SHALL pass or identify the exact
  differing path before runtime execution

### Requirement: Emulator execution does not activate Android support

An isolated arm64 API-35 emulator consumer SHALL observe binding API version
`1` and the existing valid, invalid, oversized and redacted-error behavior
families. The evidence SHALL run only in the weekly/manual slow macOS lane and
SHALL NOT mutate a user AVD. `SDK-LIM-002` and `SDK-LIM-003` SHALL remain
effective.

#### Scenario: one ephemeral emulator passes

- **WHEN** the generated local AAR executes on the selected emulator
- **THEN** the receipt SHALL name its SDK/image/API/ABI inputs while making no
  physical-device, other-ABI, publication, compatibility-matrix or support
  claim

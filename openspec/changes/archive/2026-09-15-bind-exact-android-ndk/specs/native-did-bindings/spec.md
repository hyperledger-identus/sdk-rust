## MODIFIED Requirements

### Requirement: Android AAR carries one explicit SDK ABI

The experimental Android package SHALL build `identus-uniffi-did` from the
same source, Cargo lock and Rust 1.98.1 toolchain for
`aarch64-linux-android`, using exact NDK 27.0.12077973 and minimum Android API
21. The SDK AAR SHALL contain the generated Kotlin API and exactly one SDK
native library at `jni/arm64-v8a/libidentus_uniffi_did.so`; it SHALL NOT
contain another SDK ABI or a copied/shadowed JNA implementation.

The verifier SHALL select NDK 27.0.12077973 from the declared Android SDK's
side-by-side directory without giving inherited NDK aliases precedence. It
SHALL bind child native tools to that path and independently verify exact NDK
source metadata and packaged ELF build identity.

#### Scenario: packaged ABI is inspected

- **WHEN** the package gate opens the AAR
- **THEN** its native payload SHALL be an API-21 AArch64 ELF with the required
  ABI-version, checksum, DID and DID URL symbols and reviewed runtime needs

#### Scenario: hosted runner declares another default NDK

- **WHEN** ambient NDK aliases point to a version other than 27.0.12077973
- **THEN** the verifier selects the exact SDK-relative package, rebinds child
  aliases, and requires both metadata and ELF evidence for 27.0.12077973

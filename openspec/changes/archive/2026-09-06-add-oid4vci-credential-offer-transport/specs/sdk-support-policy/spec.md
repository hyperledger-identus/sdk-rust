## ADDED Requirements

### Requirement: OID4VCI transport joins portable compile evidence

The machine-readable support policy SHALL include `identus-oid4vci` in the
existing browser-WASM, Android ARM64, and iOS ARM64 compile-checked package
lists. The crate SHALL also remain covered by workspace-default host tests,
MSRV build, strict Clippy, docs, deny, and audit gates. These entries SHALL
claim compile evidence only and SHALL NOT claim browser/mobile runtime,
network, deep-link registration, FFI, packaging, or certification support.

#### Scenario: portable target lists include the new package

- **WHEN** support-policy validation compares target package lists with
  generated Nix gates
- **THEN** each existing portable target compiles `identus-oid4vci` under the
  same tier and limitations as the other portable domain/protocol crates

#### Scenario: compile evidence is not runtime certification

- **WHEN** compatibility documentation describes the new transport surface
- **THEN** it states that URI parsing is compile-checked and leaves application
  deep-link registration, QR ingress, HTTP retrieval, and device certification
  downstream

# release-candidate-trains

## ADDED Requirements

### Requirement: A candidate train has package-specific compiler and target evidence

Before final approval, a candidate train SHALL declare a closed package,
profile, compiler, native-host, portable-target, operation, tier, and limitation
matrix. Execution SHALL use the same staged release-shaped sources and exact
internal requirements as archive evidence, not canonical unpublished manifest
identity. Candidate policy MAY narrow but SHALL NOT broaden the effective
global support policy.

Native lane receipts SHALL bind exact clean source SHA, staged version,
generated lock identity, detected host, exact Rust/Cargo version, compiler
class, commands, entries, outcomes, and limitations. One aggregate receipt
SHALL reject missing/duplicate lanes, source/lock drift, failed supported
entries, extra packages/targets, and an unsupported package represented as
portable. Receipts SHALL be closed, bounded, atomic, credential-free, and
contain no raw command output or environment dump.

#### Scenario: Staged DID candidate qualifies

- **WHEN** Linux x86_64 and macOS ARM64 each produce primary and MSRV lane
  receipts for the DID candidate
- **THEN** the aggregate receipt proves both packages on both hosts and proves
  only `identus-did` for WASM, Android ARM64, and iOS ARM64 compilation

#### Scenario: Canonical manifest substitutes for candidate source

- **WHEN** a lane reports `0.0.0`, path-only internal identity, a dirty source,
  or another revision
- **THEN** qualification fails even if the canonical workspace gates are green

#### Scenario: HTTP adapter is overclaimed

- **WHEN** a receipt reports `identus-did-resolver-http` as compile-checked or
  supported for WASM, Android ARM64, or iOS ARM64
- **THEN** aggregation fails rather than inferring support from Cargo success

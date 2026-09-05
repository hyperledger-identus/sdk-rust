## MODIFIED Requirements

### Requirement: Crate stubs at minimal code depth

The workspace SHALL classify `identus-agent`, `identus-bindings`,
`identus-messaging`, `identus-openid4vc` and `identus-trust` as quarantined
runtime placeholders at minimal code depth. Each placeholder's `src/lib.rs`
SHALL contain only a module doc-comment, an `identus_core::Component` import
and a public `COMPONENT` constant with stable current `name` and `summary`
metadata. The name and layer membership preserve seed evidence only; neither
is a release, namespace or future capability commitment.

`identus-core`, `identus-derive`, `identus-crypto`, `identus-did`,
`identus-credentials`, `identus-presentations`, `identus-wallet` and
`identus-adapters-entropy` contain implemented experimental foundations,
semantics or orchestration ports and SHALL NOT be described as stubs.
`identus-conformance` contains verification-only guards and SHALL NOT be
described as a runtime stub. The wallet activation SHALL remain limited to the
storage-port surface accepted under issue #89 and SHALL NOT claim a wallet
product, custody or persistence implementation.

#### Scenario: Each remaining placeholder self-describes via COMPONENT

- **WHEN** `COMPONENT.name` is inspected for each quarantined placeholder
- **THEN** it equals the package name recorded in the bootstrap inventory

#### Scenario: Remaining placeholder source stays at marker depth

- **WHEN** `cargo build --workspace` and inventory validation run
- **THEN** every remaining placeholder compiles with only its documentation,
  import and `COMPONENT` marker while implemented and verification crates
  retain their real code

#### Scenario: Wallet storage contract does not imply a wallet product

- **WHEN** `identus-wallet` is inspected after issue #89
- **THEN** it contains only the accepted policy-neutral storage vocabulary and
  ports and remains explicitly free of custody, persistence adapters and
  product behavior

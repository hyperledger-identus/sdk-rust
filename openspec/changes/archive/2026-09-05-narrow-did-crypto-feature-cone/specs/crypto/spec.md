## ADDED Requirements

### Requirement: Crypto integration tests declare feature prerequisites

The SDK SHALL declare the minimum complete `required-features` set in the
`identus-crypto` manifest for every integration-test target that imports an
optional public module. Tests for always-available behavior SHALL remain
runnable with no default features. The machine support policy SHALL execute an
isolated no-default crypto test gate so unrelated workspace feature
unification cannot make an incomplete target declaration pass.

#### Scenario: Minimal crypto tests run in isolation

- **WHEN** `identus-crypto` tests run with `--no-default-features`
- **THEN** Cargo skips integration targets whose declared optional APIs are
  unavailable and executes the always-available unit and error-contract tests

#### Scenario: Complete feature sets execute their targets

- **WHEN** default or all crypto features are enabled
- **THEN** curve, derivation, JWK, COSE and secp256k1 compatibility integration
  targets compile and execute as before

#### Scenario: A test relies on accidental feature unification

- **WHEN** an integration target imports a feature-gated crypto API without a
  matching complete `required-features` declaration
- **THEN** the isolated minimal-crypto gate fails instead of being masked by an
  unrelated workspace dependency

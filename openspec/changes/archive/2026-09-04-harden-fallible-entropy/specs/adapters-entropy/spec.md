## MODIFIED Requirements

### Requirement: getrandom-backed system RNG adapter (all targets)

The crate SHALL provide a `GetrandomSystemRandomAdapter` implementing
`identus_crypto::SecureRandom`, gated behind a `getrandom` cargo feature
(`default = []`). It SHALL fill caller-owned slices via `getrandom` configured
with the `js` feature and SHALL map any backend failure to
`Error::SecureRandomFailure` without panic or backend detail. It SHALL compile
and produce entropy on Kotlin/JVM, Android, native, and
`wasm32-unknown-unknown` (browser WASM). WASI remains out of scope.

#### Scenario: getrandom feature enables the adapter and the js backend

- **WHEN** `crates/adapters-entropy/Cargo.toml` is inspected with the
  `getrandom` feature
- **THEN** it SHALL enable `dep:getrandom` with the `js` feature and expose
  `GetrandomSystemRandomAdapter`

#### Scenario: getrandom adapter fills the requested slice

- **WHEN** `GetrandomSystemRandomAdapter.fill_bytes` is called with a 32-byte
  slice on a working provider
- **THEN** it SHALL fill the slice and return `Ok(())`

#### Scenario: getrandom failure is recoverable

- **WHEN** `getrandom` reports a provider failure
- **THEN** the adapter SHALL return `Error::SecureRandomFailure` without panic
  or backend detail

#### Scenario: getrandom adapter builds on browser WASM

- **WHEN** the crate is built with `--features getrandom --target
  wasm32-unknown-unknown`
- **THEN** it SHALL succeed using `crypto.getRandomValues()`

#### Scenario: getrandom adapter builds on native targets

- **WHEN** the crate is built with `--features getrandom` on a native host
- **THEN** it SHALL succeed and resolve entropy to the host OS CSPRNG

### Requirement: Deterministic test-only adapter (unchanged)

The crate SHALL provide a `DeterministicRandomAdapter` implementing
`identus_crypto::SecureRandom`, gated behind a `deterministic` cargo feature
(`default = []`). It SHALL fill every element of a caller-owned slice from a
fixed, repeatable sequence and return `Ok(())`, so random constructors are
reproducible across crates. It SHALL be documented as never for production.

#### Scenario: deterministic adapter is reproducible

- **WHEN** separate default adapter instances fill equal-length slices
- **THEN** both calls SHALL return `Ok(())` and byte-identical contents

#### Scenario: deterministic adapter accepts an empty slice

- **WHEN** it fills an empty slice
- **THEN** it SHALL return `Ok(())`

#### Scenario: deterministic feature is off by default

- **WHEN** the crate is built with default features
- **THEN** neither deterministic nor getrandom adapter SHALL be compiled

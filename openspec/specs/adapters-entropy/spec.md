## Purpose

`identus-adapters-entropy` is the outer-boundary crate that owns concrete
backends for the `identus_crypto::SecureRandom` port (the only infrastructure
port in the domain layer). It depends only inward (`identus-core`,
`identus-crypto`) and is wired into composition roots / bindings via dependency
injection — no production domain crate depends on it. Its enduring rules are:

- **Port-adapter boundary.** The crate realizes concrete entropy adapters
  behind cargo features; the port trait lives in `identus-crypto`.
- **Clean-cut ring disposition.** The `ring` adapter is removed (no remaining
  consumer); `getrandom` is the canonical cross-platform backend, building on
  native and `wasm32-unknown-unknown` (browser WASM, via the `js` feature).
- **Test-only determinism.** A `deterministic` feature exposes a reproducible
  adapter for cross-crate test parity, never for production entropy.
## Requirements
### Requirement: Entropy-port adapter family in the outer-boundary layer

`identus-adapters-entropy` SHALL be the outer-boundary crate that owns concrete
backends for the `identus_crypto::SecureRandom` port (the only infrastructure
port in the domain layer). It SHALL depend only inward
(`identus-core`, `identus-crypto`) and SHALL NOT be depended on by any
production (`core`, `crypto`, `did`, `trust`, `credentials`, `presentations`,
`messaging`, `openid4vc`, `wallet`, `agent`) crate — composition roots and
bindings wire a concrete adapter in via dependency injection. The crate SHALL
expose a `pub const COMPONENT: identus_core::Component` with
`name = "identus-adapters-entropy"`.

#### Scenario: adapters-entropy depends only inward

- **WHEN** `crates/adapters-entropy/Cargo.toml` is inspected
- **THEN** its `[dependencies]` SHALL list only `identus-core`,
  `identus-crypto`, and feature-gated external entropy backends (`getrandom`,
  and, if retained, `ring`) — and SHALL NOT list any outward crate
  (`identus-wallet`, `identus-agent`, `identus-bindings`, etc.)

#### Scenario: production crates do not depend on adapters-entropy

- **WHEN** the dependency manifests of `core`, `crypto`, `did`, `trust`,
  `credentials`, `presentations`, `messaging`, `openid4vc`, `wallet`, and
  `agent` are inspected
- **THEN** none SHALL list `identus-adapters-entropy` as a dependency (the
  composition root / bindings inject it, not domain crates)

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

### Requirement: Removed ring adapter (clean-cut disposition)

Per the design's recommended Q3 disposition (clean cut), the crate SHALL NOT
provide `RingSystemRandomAdapter` and SHALL NOT declare a `ring` cargo feature
or a `ring` dependency. The workspace `[workspace.dependencies]` SHALL NOT map
`ring` (no remaining consumer exists).

> **Gate note:** This requirement reflects the recommended outcome of the Q3
> decision gate (task 1). If task 1 finds real published consumers of the
> `ring` feature with a stability commitment, this requirement is amended (by
> task 1's output) to instead retain a `RingSystemRandomAdapter` requirement
> marked `#[deprecated]`, and the workspace `ring` dep line is kept.

#### Scenario: adapters-entropy declares no ring dependency (clean cut)

- **WHEN** `crates/adapters-entropy/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `ring` as a dependency and SHALL NOT define a
  `ring` feature

#### Scenario: workspace drops the ring dependency line (clean cut)

- **WHEN** the root `Cargo.toml` `[workspace.dependencies]` is inspected
- **THEN** it SHALL NOT contain a `ring` entry (the only consumer was this
  crate, now removed)

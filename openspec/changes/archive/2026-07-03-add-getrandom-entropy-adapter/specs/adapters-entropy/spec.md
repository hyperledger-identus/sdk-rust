## ADDED Requirements

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
(`default = []`). It SHALL draw bytes from the `getrandom` crate configured
with the `js` feature (so browser WASM resolves to
`crypto.getRandomValues()` and native targets resolve to the OS CSPRNG). It
SHALL compile and produce entropy on Kotlin/JVM, Android, native, and
`wasm32-unknown-unknown` (browser WASM). It SHALL NOT be required to compile on
`wasm32-wasi` / `wasip1` (WASI is out of scope).

#### Scenario: getrandom feature enables the adapter and the js backend

- **WHEN** `crates/adapters-entropy/Cargo.toml` is inspected with the
  `getrandom` feature
- **THEN** it SHALL enable `dep:getrandom` with the `js` feature, and the
  crate SHALL expose `GetrandomSystemRandomAdapter`

#### Scenario: getrandom adapter produces the requested length

- **WHEN** `GetrandomSystemRandomAdapter.generate_seed(32)` is called
- **THEN** it SHALL return exactly 32 bytes

#### Scenario: getrandom adapter builds on browser WASM

- **WHEN** `cargo build -p identus-adapters-entropy --features getrandom
  --target wasm32-unknown-unknown` is run
- **THEN** it SHALL succeed (the `js` feature resolves entropy to
  `crypto.getRandomValues()`)

#### Scenario: getrandom adapter builds on native targets

- **WHEN** `cargo build -p identus-adapters-entropy --features getrandom` is
  run on a native host
- **THEN** it SHALL succeed and resolve entropy to the host OS CSPRNG

### Requirement: Deterministic test-only adapter (unchanged)

The crate SHALL provide a `DeterministicRandomAdapter` implementing
`identus_crypto::SecureRandom`, gated behind a `deterministic` cargo feature
(`default = []`). It SHALL return bytes drawn from a fixed, repeatable
sequence so that `generate` / `create_random_mnemonics` calls are reproducible
across crates. It SHALL be documented as **never for production entropy**.
This requirement is unchanged by this change and is restated here only because
no prior spec captured the `adapters-entropy` capability.

#### Scenario: deterministic adapter is reproducible

- **WHEN** `DeterministicRandomAdapter.generate_seed(N)` is called twice with
  the same `N`
- **THEN** both calls SHALL return byte-identical vectors

#### Scenario: deterministic feature is off by default

- **WHEN** the crate is built with default features
- **THEN** neither `DeterministicRandomAdapter` nor `GetrandomSystemRandomAdapter`
  SHALL be compiled (both are opt-in)

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
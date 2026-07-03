## MODIFIED Requirements

### Requirement: SecureRandom — the single infrastructure port

The crate SHALL provide a `SecureRandom` trait (the only infrastructure port in the crate) with `generate_seed(num_bytes) -> Vec<u8>`, as the canonical entropy source for key generation and mnemonic creation. The crate SHALL define the **port only**; it SHALL NOT ship any concrete `SecureRandom` implementation in this crate. `SecureRandom` is a port because a known second backend exists: the concrete backends live in the outer-boundary `identus-adapters-entropy` crate — a `getrandom`-backed adapter (`GetrandomSystemRandomAdapter`, the cross-platform backend that builds on `wasm32-unknown-unknown` and is the canonical entropy source for the Kotlin/WASM bindings) and, where retained, a `ring`-backed adapter. (The `getrandom` adapter was previously described as "deferred to that future binding change"; it is now realized by the consumed `add-getrandom-entropy-adapter` change.) Key generation (`*KeyPair::generate`) and mnemonic creation (`MnemonicHelper::create_random_mnemonics`) SHALL take an injected `&mut impl SecureRandom` (dependency injection), since the domain crate cannot reach an outer-boundary adapter. No other primitive operation is exposed as a port: curve sign/verify, hashing, derivation, and conversion are concrete (see the "Concrete where backends don't vary" Purpose rule).

#### Scenario: SecureRandom generates the requested length

- **WHEN** `generate_seed(32)` is called on an adapter implementing the port
- **THEN** it SHALL return 32 bytes

#### Scenario: crypto declares no ring dependency

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `ring` as a dependency (concrete entropy adapters live in `identus-adapters-entropy`, never in `identus-crypto`)

#### Scenario: the getrandom adapter lives in identus-adapters-entropy

- **WHEN** `crates/adapters-entropy/` (behind its `getrandom` feature) is inspected
- **THEN** it SHALL provide the `GetrandomSystemRandomAdapter` implementing `identus_crypto::SecureRandom` via `getrandom`

### Requirement: Wasm-clean by default

The crate SHALL build on `wasm32-unknown-unknown` with default features,
because it has no `ring` dependency. Its external cryptographic dependencies
are wasm-safe. The domain primitive crate is wasm-clean; no infrastructure
it depends on introduces a `wasm32`-incompatible backend.

#### Scenario: crypto builds on wasm32 with default features

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is
  run with default features
- **THEN** the build SHALL succeed
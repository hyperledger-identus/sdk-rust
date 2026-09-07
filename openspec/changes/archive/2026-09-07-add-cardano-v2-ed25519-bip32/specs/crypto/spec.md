## ADDED Requirements

### Requirement: Cardano V2 Ed25519-BIP32 hierarchical derivation

When the `cardano-bip32` feature is enabled, the crate SHALL provide
`CardanoV2ExtendedPrivateKey` and `CardanoV2ExtendedPublicKey` SDK-owned types
implementing Cardano/IOG Ed25519-BIP32 scheme V2. The private type SHALL support
hardened and soft child derivation. The public type SHALL support soft child
derivation and SHALL reject hardened derivation with `Error::DerivationFailed`.
Deriving a soft public child SHALL equal converting the corresponding private
child to its public representation.

The private type SHALL own and zeroize its 96-byte representation, provide a
redacted `Debug`, provide no `Display` or serde implementation, and expose raw
private bytes only through an explicitly named `expose_secret_bytes` method.
The public type's `Debug` SHALL omit its public key and chain code. Neither
`ed25519-bip32`, `cryptoxide`, `XPrv`, `XPub` nor dependency errors SHALL appear
in the public API. The existing hardened-only SLIP-0010 `EdHDKey` SHALL remain
unchanged.

#### Scenario: Apollo hardened private derivation matches

- **WHEN** the Apollo donor's D1 extended private key derives child `0'`
- **THEN** the complete 96-byte child SHALL equal Apollo's D1-H0 vector

#### Scenario: soft private and public derivation agree

- **WHEN** a valid extended private key and its extended public key each derive
  the same non-hardened axis
- **THEN** the public key converted from the private child SHALL equal the
  directly derived public child byte-for-byte

#### Scenario: hardened public derivation is rejected safely

- **WHEN** an extended public key derives a hardened axis
- **THEN** the operation SHALL return `Error::DerivationFailed` without panic,
  dependency error exposure, or key bytes in the rendered error

#### Scenario: invalid private material is rejected

- **WHEN** a 96-byte extended private-key representation violates the required
  Ed25519 scalar bit shape
- **THEN** construction SHALL return `Error::DerivationFailed` without retaining
  or rendering the rejected bytes

#### Scenario: private formatting is redacted

- **WHEN** a Cardano V2 extended private key is formatted with `Debug`
- **THEN** the result SHALL identify the SDK type but contain none of its
  extended private-key or chain-code bytes

#### Scenario: owned private material is zeroizing

- **WHEN** the SDK private type's security traits and explicit `zeroize` result
  are tested
- **THEN** it SHALL implement `Zeroize` and `ZeroizeOnDrop`, and explicit
  zeroization SHALL clear all 96 owned bytes

#### Scenario: dependency remains an implementation detail

- **WHEN** the generated public API and feature-disabled dependency graph are
  inspected
- **THEN** no `ed25519-bip32` or `cryptoxide` type SHALL be public and neither
  package SHALL resolve under `--no-default-features`

## MODIFIED Requirements

### Requirement: Feature-gated with all-on default

The crate SHALL expose cargo features `ed25519`, `x25519`, `secp256k1`,
`secp256r1`, `hash`, `hex`, `base64`, `jwk`, `jwk-thumbprint`, `cose`,
`derivation`, `cardano-bip32`, and `kmp-compat`, with `default` enabling all of
them except `kmp-compat`. The `cardano-bip32` feature SHALL enable only the
private `ed25519-bip32` dependency and the existing `zeroize` dependency. The
`cose` feature SHALL enable only optional, default-feature-disabled `coset`.
The `jwk` feature SHALL enable `base64`, `serde`, and `serde_json`; curve
features SHALL continue to imply `jwk`. `jwk-thumbprint` SHALL compose `jwk`
and `hash`, while `jwk` SHALL remain independently usable without SHA-2. Curve
`EncodeCose` implementations SHALL be available when the curve and `cose`
features are both enabled; JWK conversion SHALL additionally require `jwk`. A
minimal COSE build SHALL not require serde, JSON, base64 or a curve backend.
There SHALL be no `securerandom` feature because the zero-dependency
`SecureRandom` port is always available, and no `wasm` feature because concrete
entropy adapters belong to `identus-adapters-entropy`. `kmp-compat` SHALL remain
opt-in, gate only the KMP legacy mnemonic surface, and introduce no dependency
outside the default dependency set. The full default and supported minimal
feature combinations SHALL remain wasm-safe.

#### Scenario: Default features compile the full surface

- **WHEN** `cargo build -p identus-crypto` is run with default features
- **THEN** all curve, hashing, derivation, Cardano V2 derivation, JWK, JWK
  thumbprint, COSE, and `SecureRandom`-port modules SHALL compile and be
  available, and the KMP-interop surface SHALL NOT be present

#### Scenario: Default features compile on wasm32

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is
  run with default features
- **THEN** the build SHALL succeed

#### Scenario: minimal JWK feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features jwk`
  is run
- **THEN** the validated JWK and JSON wire surface SHALL compile without
  SHA-2 or a curve backend

#### Scenario: minimal JWK thumbprint feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features
  jwk-thumbprint` is run
- **THEN** validated JWK, SHA-256 thumbprint and base64url output SHALL compile
  without a curve backend

#### Scenario: a minimal curve feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519`
  is run
- **THEN** Ed25519 and its validated JWK wire surface SHALL compile

#### Scenario: minimal Cardano V2 derivation feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features
  cardano-bip32` is run
- **THEN** the SDK-owned Cardano V2 derivation surface SHALL compile without
  unrelated curve, JWK, COSE, mnemonic or protocol dependencies

#### Scenario: minimal COSE feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features cose`
  is run
- **THEN** the validated COSE Key and CBOR wire surface SHALL compile without
  JSON, base64 or a curve backend

#### Scenario: curve plus COSE feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519,cose`
  is run
- **THEN** Ed25519 and its `EncodeCose` implementation SHALL compile

#### Scenario: The kmp-compat feature is opt-in

- **WHEN** `cargo build -p identus-crypto --features kmp-compat` is run
- **THEN** the build SHALL succeed and `create_seed_kmp` SHALL be present on
  `MnemonicHelper`

#### Scenario: kmp-compat introduces no new external dependency

- **WHEN** the `kmp-compat` feature is enabled with its required base features
- **THEN** the build SHALL succeed without pulling a crate not already
  required by the default feature set

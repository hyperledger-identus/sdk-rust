## ADDED Requirements

### Requirement: DID domain dependency cone is crypto-free

`identus-did` SHALL own representation-neutral DID syntax, document,
resolution, dereferencing, registration and cache contracts without depending
on `identus-crypto` or activating an algorithm feature. Its normal internal
dependency cone SHALL contain only `identus-core` and the `identus-derive`
proc macro. The crate SHALL declare no Cargo feature until optional DID
behavior is accepted by a focused component contract.

Recognized `publicKeyJwk` material SHALL remain structurally validated,
bounded and public-only. DID construction SHALL NOT claim curve-point,
algorithm, signature or authorization validity. A consumer performing a
cryptographic operation SHALL select and bind its required crypto capability
at that operation boundary.

#### Scenario: DID-only consumer does not compile crypto algorithms

- **WHEN** the default or `--no-default-features` dependency tree for
  `identus-did` is inspected
- **THEN** it contains no `identus-crypto`, Ed25519, X25519, secp256k1, P-256,
  derivation, hashing or COSE dependency introduced by the DID crate

#### Scenario: structural JWK behavior remains unchanged

- **WHEN** a DID verification method contains bounded public JWK material
- **THEN** the DID model preserves its open members, rejects registered private
  material and multiple recognized key representations, and makes no
  cryptographic-validity claim

#### Scenario: algorithm-aware consumer owns key binding

- **WHEN** JOSE or another consumer dereferences a verification method and
  needs to verify a signature
- **THEN** that consumer requests its exact crypto features and converts the
  structural public material into an algorithm-bound key without widening the
  DID dependency cone

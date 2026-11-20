# sdk-ts-hd-derivation Specification

## Purpose
TBD - created by archiving change add-sdk-ts-hd-derivation. Update Purpose after archive.
## Requirements
### Requirement: every derivation selects one named Rust profile

The private TypeScript facade SHALL route derivation only to an explicitly
named `identus-crypto` profile. TypeScript SHALL NOT implement, infer, alias, or
fallback between derivation algorithms.

#### Scenario: selected profile remains stable

- **WHEN** a caller imports seed material or derives a child
- **THEN** master construction, hardened/soft rules, key shape, and path
  semantics remain those of the selected immutable profile

### Requirement: supported profiles remain distinct

The facade SHALL expose hardened-only secp256k1 BIP-32, hardened-only Ed25519
SLIP-0010, and Cardano/IOG Ed25519-BIP32 V2 as separate profile values.

#### Scenario: identical seed and path do not imply interchangeable keys

- **WHEN** the same profile-valid seed and path are supplied to two different
  profiles
- **THEN** each result retains its profile identity and cannot be consumed or
  represented as the other profile

### Requirement: seed custody remains host-owned at ingress

The facade SHALL accept only bounded mutable seed buffers, copy them into an
opaque Rust owner, and synchronously overwrite the supplied buffer before
returning or throwing. It SHALL expose no mnemonic or seed-generation API.

#### Scenario: caller seed is overwritten

- **WHEN** root construction succeeds or fails after receiving a mutable seed
- **THEN** the supplied buffer is overwritten and no seed byte appears in a
  result, observation, error, or retained TypeScript DTO

### Requirement: paths are canonical and bounded before work

Rust SHALL parse path text using the canonical `m[/axis...]` grammar with a
4,096-byte and 255-axis ceiling. Each decimal axis SHALL be below 2^31 and MAY
carry exactly one trailing apostrophe for hardening.

#### Scenario: malformed or excessive paths fail closed

- **WHEN** a path has an empty or partially numeric axis, sign, fraction,
  internal whitespace, overflow, excessive bytes, or a 256th axis
- **THEN** parsing fails with a stable redacted path error before child work

### Requirement: profile-specific private derivation is enforced

Secp256k1 BIP-32 and Ed25519 SLIP-0010 private handles SHALL accept hardened
axes only. Cardano V2 private handles SHALL accept hard or soft axes.

#### Scenario: forbidden soft axis is rejected

- **WHEN** the hardened-only BIP-32 or SLIP-0010 profile receives a soft axis
- **THEN** derivation fails before child-key work and the parent remains usable

#### Scenario: Cardano private derivation accepts both classes

- **WHEN** a Cardano V2 private handle receives a bounded mixed hard/soft path
- **THEN** it returns a new opaque child at the exact resulting depth

### Requirement: public derivation is closed to supported behavior

Only Cardano V2 public handles SHALL support child derivation, and they SHALL
accept soft axes only. The other profiles SHALL expose public projection but no
public child-derivation operation.

#### Scenario: hardened Cardano public derivation fails

- **WHEN** a Cardano V2 public handle receives a hardened axis
- **THEN** derivation fails with a stable profile error and no child result

### Requirement: derived secrets remain opaque and disposable

Root and child private state SHALL remain in Rust-owned WASM owners. The facade
SHALL expose no private or chain-code export and SHALL free each owner exactly
once through an idempotent explicit lifecycle.

#### Scenario: disposed handles fail closed

- **WHEN** a disposed derivation handle is used or disposed again
- **THEN** repeated disposal is harmless and every operation fails with a
  stable lifecycle error without invoking derivation work

### Requirement: public projections are immutable and profile-specific

The facade SHALL return copied immutable public values: 33-byte compressed
SEC1 for secp256k1, 32-byte verification key for SLIP-0010, and 64-byte
public-key-plus-chain-code for Cardano V2.

#### Scenario: caller mutation cannot alter retained identity

- **WHEN** a caller mutates input or returned typed arrays
- **THEN** existing handles and public values retain their original semantics

### Requirement: errors and observations reveal no secret material

Stable errors and bounded observations SHALL contain only operation, profile,
route, mapping ID, result category, stable code, depth, and duration. They
SHALL omit seed, private key, chain code, public key, path text, stacks, and
generated error objects.

#### Scenario: every failure remains redacted

- **WHEN** seed, path, derivation, projection, observer, binding, or lifecycle
  failure occurs
- **THEN** public diagnostics contain no caller byte or path encoding

### Requirement: applicable targets execute one contract

Node ESM/CommonJS, browser ESM, React SSR, and hydrated React client SHALL run
one neutral assertion body over the selected vectors. React SHALL contain no
derivation logic. React Native SHALL remain explicitly inapplicable.

#### Scenario: target results are equivalent

- **WHEN** every applicable target runs the same fixed vectors
- **THEN** public projections, depths, errors, lifecycle, and rollback outcomes
  are equivalent

### Requirement: artifact and CI isolation remain fail closed

Rust core SHALL finish before selected TypeScript jobs. A target failure SHALL
NOT erase core evidence, and the protected aggregator SHALL fail unless every
selected exact-head receipt succeeds.

#### Scenario: derivation impact selects crypto jobs

- **WHEN** derivation binding, facade, vector, mapping, or evidence paths change
- **THEN** crypto package/types, browser, Node.js, and React Web jobs run
  independently without changing DID or native binding success claims

### Requirement: rollback is explicit before initialization

Provider selection SHALL occur before generated module initialization. A
selected provider failure SHALL be terminal and SHALL NOT invoke another
profile, provider, engine, retry, or secret migration automatically.

#### Scenario: legacy selection is explicit

- **WHEN** a host selects its legacy derivation route before initialization
- **THEN** the WASM HD capability is not initialized or invoked


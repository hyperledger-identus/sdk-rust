## ADDED Requirements

### Requirement: Rust owns the selected key-operation semantics

The private TypeScript facade SHALL expose capability-named Ed25519 and X25519
operations backed only by `identus-crypto`. TypeScript SHALL NOT implement or
fallback to another cryptographic engine.

#### Scenario: selected profiles remain closed

- **WHEN** a caller creates or imports a key
- **THEN** it selects Ed25519 signing/verification or X25519 shared-secret
  purpose explicitly, with no generic algorithm string or implicit conversion

### Requirement: private keys remain opaque and disposable

Generated or imported private keys SHALL remain in Rust-owned WASM owners.
The facade SHALL expose no private export and SHALL free each owner exactly
once through an idempotent explicit lifecycle.

#### Scenario: disposed handles fail closed

- **WHEN** a disposed private handle is used or disposed again
- **THEN** repeated disposal is harmless and every operation fails with a
  stable lifecycle error without invoking cryptographic work

#### Scenario: imported caller bytes are overwritten

- **WHEN** private import succeeds or fails after receiving an exact mutable
  32-byte input
- **THEN** the adapter overwrites that supplied buffer synchronously and never
  includes its content in a result, observation, or error

### Requirement: Ed25519 operations are exact and bounded

The facade SHALL support system generation, exact private/public import,
public export, public derivation, signing, and strict verification with fixed
key/signature lengths and a bounded message ceiling.

#### Scenario: generated and imported keys agree

- **WHEN** an Ed25519 private handle signs a bounded message
- **THEN** its public value verifies the exact message/signature, tampering
  returns false, and malformed lengths fail before verification

### Requirement: X25519 shared secrets are contributory

The facade SHALL support system generation, exact private/public import,
public export, public derivation, and shared-secret derivation. Peer public
values SHALL be canonical and contributory and an all-zero result SHALL fail.

#### Scenario: two parties derive the same secret

- **WHEN** two valid X25519 private handles derive against each other's public
  values
- **THEN** both return equal 32-byte secrets while neither private key is
  exported

#### Scenario: weak peer input fails closed

- **WHEN** a low-order, noncanonical, or malformed peer is supplied
- **THEN** derivation fails with a stable redacted input error and no secret
  result

### Requirement: public values are immutable and strictly projected

Public key and operation results SHALL be copied into immutable
capability-named values. Generated classes and unknown fields SHALL NOT cross
the package API.

#### Scenario: caller mutation cannot alter retained public identity

- **WHEN** the caller mutates an input or returned typed array
- **THEN** previously constructed public values and private handles retain
  their original semantics

### Requirement: errors and observations reveal no sensitive material

Stable errors and bounded observations SHALL contain only operation, curve,
route, mapping ID, result category, stable code, and duration. They SHALL omit
private/public key bytes, messages, signatures, shared secrets, stacks, and
generated error objects.

#### Scenario: every failure remains redacted

- **WHEN** import, entropy, sign, verify, derive, projection, observer, binding,
  or lifecycle failure occurs
- **THEN** the public error and observation contain no caller byte encoding or
  dependency-generated text

### Requirement: applicable targets execute one contract

Node ESM/CommonJS, browser ESM, React SSR, and hydrated React client SHALL run
one neutral assertion body over the selected vectors. React SHALL contain no
cryptographic logic. React Native SHALL emit an explicit inapplicable receipt.

#### Scenario: target results are equivalent

- **WHEN** every applicable target runs the same fixed vectors
- **THEN** public keys, signatures, verification results, shared secrets,
  errors, lifecycle, and rollback outcomes are equivalent

### Requirement: artifact and CI isolation remain fail closed

Crypto and DID WASM artifacts SHALL have independent deterministic identities.
Rust core SHALL finish before selected TypeScript jobs, selected target failure
SHALL NOT erase core evidence, and the protected aggregator SHALL fail unless
every selected exact-head receipt succeeds.

#### Scenario: crypto-only impact selects the right jobs

- **WHEN** crypto binding, facade, vector, mapping, or evidence paths change
- **THEN** package/types, browser, Node.js, and React Web run independently,
  React Native records inapplicability, and unrelated native binding jobs are
  not inferred as successful

### Requirement: rollback is explicit before initialization

Provider selection SHALL occur before generated module initialization. A
selected provider failure SHALL be terminal and SHALL NOT invoke another
provider, engine, retry, or private-key migration automatically.

#### Scenario: legacy selection is explicit

- **WHEN** a host selects its legacy provider before initialization
- **THEN** the WASM key capability is not initialized or invoked

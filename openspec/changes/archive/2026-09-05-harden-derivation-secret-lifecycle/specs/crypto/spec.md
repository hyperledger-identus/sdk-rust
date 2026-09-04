## ADDED Requirements

### Requirement: Best-effort lifecycle protection for owned secret buffers

The crypto capability SHALL prevent raw secret material from entering safe
`Debug`, `Display`, error, serialization, or FFI surfaces. `HDKey` and
`EdHDKey` SHALL implement `Zeroize` and `ZeroizeOnDrop`; their formatting SHALL
show only public derivation metadata and SHALL omit private key and chain-code
bytes. SDK-owned temporary entropy, internally consumed mnemonic word strings,
mnemonic entropy, HD HMAC input/output, and PBKDF2 output buffers SHALL use
zeroizing storage and preserve every existing algorithm and byte output.

This is a best-effort owned-buffer contract. It SHALL NOT claim to erase
caller-created or compiler-created copies, allocator state, swap, crash dumps,
or hardware state. Raw secret-returning APIs SHALL remain explicit and their
returned values SHALL be caller-owned.

#### Scenario: HD debug output is redacted

- **WHEN** an `HDKey` or `EdHDKey` containing known private key and chain-code
  bytes is formatted with `Debug`
- **THEN** the output SHALL identify the type and public derivation metadata
  but SHALL NOT contain either secret byte sequence

#### Scenario: HD keys expose an enforceable erasure contract

- **WHEN** compile-time assertions inspect `HDKey` and `EdHDKey`
- **THEN** both SHALL implement `Zeroize` and `ZeroizeOnDrop`

#### Scenario: explicit erasure clears owned state

- **WHEN** `Zeroize::zeroize` is invoked on an HD key
- **THEN** its private key, chain code, depth, and child index SHALL be reset to
  zero without panic

#### Scenario: random construction intermediates are scoped for erasure

- **WHEN** Ed25519, X25519, secp256k1, P-256, or BIP-39 random construction
  succeeds or returns an error
- **THEN** the SDK-owned entropy buffer SHALL be guarded by drop-time
  zeroization and SHALL NOT be included in formatting or errors

#### Scenario: internally consumed random mnemonic words are scoped for erasure

- **WHEN** random seed convenience creation converts SDK-owned entropy into
  mnemonic word strings and consumes them internally
- **THEN** the word vector and its strings SHALL be guarded by drop-time
  zeroization while caller-returned mnemonic words remain caller-owned

#### Scenario: derivation outputs remain compatible

- **WHEN** the existing BIP32, SLIP-0010, BIP39, and KMP compatibility vectors
  are evaluated after lifecycle hardening
- **THEN** every derived byte sequence and public error result SHALL remain
  unchanged

#### Scenario: erasure limits are documented truthfully

- **WHEN** a consumer reads the secret lifecycle documentation
- **THEN** it SHALL distinguish SDK-owned buffers from caller or platform
  copies and SHALL NOT describe best-effort memory erasure as custody or secure
  storage

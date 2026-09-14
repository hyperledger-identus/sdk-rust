## MODIFIED Requirements

### Requirement: Best-effort lifecycle protection for owned secret buffers

The crypto capability SHALL prevent raw secret material from entering safe
`Debug`, `Display`, error, serialization, or FFI surfaces. `HDKey` and
`EdHDKey` SHALL keep private-key and chain-code storage private and SHALL
implement `Zeroize` and `ZeroizeOnDrop`; their formatting SHALL show only
public derivation metadata and SHALL omit private key and chain-code bytes.
SDK-owned temporary entropy, internally consumed mnemonic word strings,
mnemonic entropy, HD HMAC input/output, and PBKDF2 output buffers SHALL use
zeroizing storage and preserve every existing algorithm and byte output.

Raw HD secret access SHALL be available only through explicitly named methods
that return an SDK-owned fixed-size exposure value. That value SHALL own and
zeroize the exported copy, redact `Debug`, implement neither `Clone`, `Copy`,
`Display`, nor Serde, and lend its raw array only through an explicitly named
borrow tied to the exposure owner's lifetime. Neither the HD key types nor the
exposure value SHALL enter generated binding surfaces.

This capability SHALL NOT provide public struct-literal construction or a
`from_parts`/raw-import constructor for private-key, chain-code and derivation
metadata. Callers that retain the original seed and supported derivation path
MAY reconstruct through `init_from_seed` and derivation. Raw extended-state
rehydration without those inputs SHALL remain unsupported until a separate
security and API decision authorizes it.

This is a best-effort owned-buffer contract. It SHALL NOT claim to erase
caller-created or compiler-created copies, registers, allocator state, swap,
crash dumps, or hardware state. A caller that deliberately copies bytes from
the named borrowed view owns and must erase that further copy.

#### Scenario: HD debug output is redacted

- **WHEN** an `HDKey` or `EdHDKey` containing known private key and chain-code
  bytes is formatted with `Debug`
- **THEN** the output SHALL identify the type and public derivation metadata
  but SHALL NOT contain either secret byte sequence

#### Scenario: HD key fields are opaque to external code

- **WHEN** an external crate tries to read `private_key` or `chain_code`
  directly from `HDKey` or `EdHDKey`
- **THEN** compilation SHALL fail because all four fields are private

#### Scenario: raw extended-state rehydration remains unsupported

- **WHEN** external code attempts to recreate an HD key from private-key,
  chain-code, depth, and child metadata without the original seed and path
- **THEN** no public struct literal or `from_parts`/raw-import constructor SHALL
  exist, and this use case SHALL remain deferred

#### Scenario: explicit exposure owns and erases one copy

- **WHEN** a caller invokes a named private-key or chain-code exposure method
- **THEN** it SHALL receive a non-cloneable, non-copyable, redacted owner whose
  explicitly borrowed 32-byte view matches the derived vector and whose owned
  bytes are zeroized by explicit erasure and on drop

#### Scenario: exposure does not gain ambient output surfaces

- **WHEN** external code tries to format the exposure owner with `Display` or
  serialize it through Serde
- **THEN** compilation SHALL fail, while `Debug` SHALL reveal no bytes

#### Scenario: HD keys expose an enforceable erasure contract

- **WHEN** compile-time assertions inspect `HDKey`, `EdHDKey`, and the exposure
  owner
- **THEN** all SHALL implement `Zeroize` and `ZeroizeOnDrop`

#### Scenario: explicit erasure clears owned state

- **WHEN** `Zeroize::zeroize` is invoked on an HD key or exposure owner
- **THEN** its owned secret bytes SHALL be reset to zero without panic, and HD
  metadata SHALL retain the existing reset behavior

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

- **WHEN** the existing BIP32, SLIP-0010, BIP39, KMP compatibility and Apollo
  overlap vectors are evaluated after opacity hardening
- **THEN** every derived byte sequence and public error result SHALL remain
  unchanged through the explicit exposure boundary

#### Scenario: unpublished baseline records intentional source break

- **WHEN** public-API evidence compares the new candidate with the committed
  unpublished `0.1.0-rc.1` baseline
- **THEN** the four public secret fields SHALL be absent, the named exposure
  API SHALL be present, and the ADR SHALL classify the pre-release source break
  without asserting a released SemVer commitment

#### Scenario: erasure limits are documented truthfully

- **WHEN** a consumer reads the secret lifecycle documentation
- **THEN** it SHALL distinguish SDK-owned buffers from caller or platform
  copies and SHALL NOT describe best-effort memory erasure as custody or secure
  storage

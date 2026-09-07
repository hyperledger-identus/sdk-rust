## MODIFIED Requirements

### Requirement: BIP32 secp256k1 hierarchical derivation

The crate SHALL provide an `HDKey` for secp256k1 with `derive(path)`,
`derive_child(index)`, hardened derivation, and `init_from_seed(seed)`, ported
from the KMP `derivation.HDKey`. The public facade and Identus error boundary
SHALL remain SDK-owned and SHALL NOT expose a BIP-32 framework type.

Master derivation SHALL accept every seed length from 16 through 64 bytes
inclusive and reject every other length. Its HMAC left half SHALL be a valid
nonzero secp256k1 scalar. Child derivation SHALL reject non-hardened axes,
`IL >= n`, a zero resulting child key, and depth overflow. It SHALL accept
`IL = 0` when the resulting child is nonzero. Invalid values SHALL NOT be
reduced modulo the curve order and all failures SHALL map to the stable,
redacted `Error::DerivationFailed` boundary.

Seed, HMAC, scalar, private-key and chain-code temporaries SHALL remain bounded
by zeroizing ownership. The implementation SHALL reuse the existing `k256`
scalar implementation and SHALL NOT add xprv/xpub, Base58Check, RIPEMD,
non-hardened/public derivation, account, chain or wallet policy.

#### Scenario: BIP32 derivation matches known vectors

- **WHEN** a known seed is derived along a supported hardened path
- **THEN** the resulting key SHALL match the published BIP32 test vector and
  Apollo's overlapping result byte-for-byte

#### Scenario: every normative seed length is accepted

- **WHEN** master derivation receives any seed length from 16 through 64 bytes
- **THEN** it SHALL accept the length and produce a validated nonzero master
  unless the HMAC's synthetic scalar is invalid

#### Scenario: invalid master material fails closed

- **WHEN** the seed length is outside 16 through 64 bytes, or a synthetic
  master HMAC left half is zero or at least the secp256k1 order
- **THEN** master derivation SHALL return `Error::DerivationFailed` without
  returning or formatting key material

#### Scenario: invalid child scalars are not reduced

- **WHEN** a synthetic child HMAC produces `IL >= n`, or adding a valid `IL`
  to the parent produces zero
- **THEN** child derivation SHALL return `Error::DerivationFailed` rather than
  reducing the value or selecting another index

#### Scenario: a zero child tweak is standards-valid

- **WHEN** a synthetic child HMAC produces `IL = 0` for a valid nonzero parent
- **THEN** the child private key SHALL equal the parent private key, the new
  chain code and metadata SHALL be retained, and derivation SHALL succeed

#### Scenario: hardened and depth boundaries fail closed

- **WHEN** child derivation receives a non-hardened axis or a parent at maximum
  depth
- **THEN** it SHALL return `Error::DerivationFailed` without deriving a child

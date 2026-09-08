## ADDED Requirements

### Requirement: bounded hierarchical derivation inputs and work

The crate SHALL expose a 4,096 UTF-8-byte maximum for textual derivation paths,
a 255-axis maximum for generic BIP-32-shaped paths, and 16-through-64-byte seed
bounds for BIP-32 and SLIP-0010 master derivation. Text parsing SHALL check the
byte ceiling before splitting or syntax inspection and SHALL parse without an
eager input-proportional intermediate collection.

All cryptographic path consumers SHALL reject a path above 255 axes before the
first child operation. `HDKey` and `EdHDKey` SHALL reject a child beyond depth
255 before HMAC or curve work. Failures SHALL use the existing redacted
`Error::DerivationFailed` boundary. Valid in-budget derivation vectors and
public/wire key representations SHALL remain unchanged.

The source-compatible infallible programmatic path append API MAY construct an
oversized path, but SHALL be documented as caller-budgeted; cryptographic
consumers SHALL still fail before proportional key-derivation work.

#### Scenario: path text is bounded before parse work

- **WHEN** a caller provides path text of 4,097 or more UTF-8 bytes
- **THEN** parsing SHALL return `Error::DerivationFailed` before splitting,
  root inspection or axis syntax parsing

#### Scenario: path work is bounded at 255 axes

- **WHEN** a textual or programmatically constructed path contains 256 axes
- **THEN** parsing or cryptographic consumption SHALL return
  `Error::DerivationFailed` before processing the excess axis or deriving the
  first child respectively

#### Scenario: normative seed boundaries are enforced

- **WHEN** BIP-32 or SLIP-0010 master derivation receives a seed from 16
  through 64 bytes
- **THEN** it SHALL accept that length, while 15- and 65-byte seeds SHALL fail
  before HMAC with `Error::DerivationFailed`

#### Scenario: maximum depth fails closed

- **WHEN** `HDKey` or `EdHDKey` at depth 255 is asked to derive a child
- **THEN** it SHALL return `Error::DerivationFailed` before HMAC or curve work

#### Scenario: accepted vectors remain compatible

- **WHEN** existing BIP-32, SLIP-0010, Apollo-overlap or Cardano V2 vectors are
  derived within the budgets
- **THEN** all result bytes and public metadata SHALL remain unchanged

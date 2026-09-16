# SDK input resource governance delta

## ADDED Requirements

### Requirement: Crypto codec retention is SDK-enforced

The input-boundary inventory SHALL classify public byte-to-`HexStr` and
byte-to-`Base64UrlStrNoPad` construction as SDK-enforced only after no public
unbounded infallible encoding path remains and exact boundary evidence passes.

#### Scenario: Codec limitation is narrowed

- **WHEN** the blanket public `From<AsRef<[u8]>>` implementations are absent
  and every remaining public byte constructor enforces the encoded-text ceiling
- **THEN** the codec clause SHALL be removed from `SDK-LIM-007`
- **AND** unrelated residual limitations SHALL remain unchanged

#### Scenario: Public bypass returns

- **WHEN** a future public conversion can retain codec text above the declared
  ceiling
- **THEN** the inventory checker or review SHALL fail and the limitation SHALL
  be restored immediately

## MODIFIED Requirements

### Requirement: Known unbounded compatibility retention remains explicit

An implemented SDK type SHALL have a distinct
`known-unbounded-compatibility` inventory row when it retains unbounded caller
input for public compatibility. The row SHALL name the exact public surface,
source evidence, consumer guard, migration trigger, and effective limitation.
It SHALL NOT be
misrepresented as bounded, non-retaining, or protected by an outer allocator.

#### Scenario: Historical Multihash placeholder is inspected

- **WHEN** an agent audits `identus_did::Multihash`
- **THEN** the inventory and `SDK-LIM-007` SHALL disclose its unbounded opaque
  byte retention and require consumers to bound input before construction or
  serde until an explicit migration replaces the compatibility contract

#### Scenario: Direct JOSE retained enums are inspected

- **WHEN** an agent audits direct `JwsKeyReference::KeyId`/`X5c` or
  `Oid4vciProofJwtClient::Identified` construction
- **THEN** the inventory and `SDK-LIM-007` SHALL disclose arbitrary retained
  strings or collection cardinality and require caller validation until issue
  #299 makes those retained values opaque and validated

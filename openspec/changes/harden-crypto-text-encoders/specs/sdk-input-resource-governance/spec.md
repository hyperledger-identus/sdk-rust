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

## REMOVED Requirements

### Requirement: Infallible crypto encoders remain caller-bounded

**Reason:** Issue #298 removes the blanket public encoding bypass and replaces
it with checked construction.

**Migration:** Replace `Type::from(bytes)` with
`Type::try_from_bytes(bytes)?` or an applicable `TryFrom` conversion.

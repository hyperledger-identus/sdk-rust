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

## ADDED Requirements

### Requirement: JOSE retained alternatives are SDK-enforced

The input-boundary inventory SHALL classify `JwsKeyReference::KeyId`/`X5c` and
`Oid4vciProofJwtClient::Identified` as SDK-enforced only after every public
variant payload is opaque and validated and no raw retained construction path
remains.

#### Scenario: JOSE compatibility limitation is narrowed

- **WHEN** named fallible constructors, parser equivalence, tighter-limit
  revalidation, and public API closure evidence pass
- **THEN** the direct JOSE retained-enum clause SHALL be removed from
  `SDK-LIM-007`
- **AND** outer preallocation and every unrelated residual limitation SHALL
  remain unchanged

#### Scenario: A raw retained variant returns

- **WHEN** a future public alternative can retain an unvalidated raw string or
  collection
- **THEN** the inventory checker or review SHALL fail and the compatibility
  limitation SHALL be restored immediately

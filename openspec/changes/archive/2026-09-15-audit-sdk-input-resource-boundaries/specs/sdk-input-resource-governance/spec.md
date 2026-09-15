## ADDED Requirements

### Requirement: Distinct DID registry and cache ownership remains visible

The boundary inventory SHALL record the bounded DID method registry separately
from dispatched adapter work. It SHALL record portable DID resolution cache
key, declared-capacity, and TTL limits separately from caller-budgeted cache and
clock adapter allocation and QoS. Existing package-level coverage SHALL NOT be
treated as evidence that these distinct families are present.

#### Scenario: DID method registry is inspected

- **WHEN** an agent searches the inventory for retained method bindings
- **THEN** it SHALL find the exact 64-entry limit and registry source evidence

#### Scenario: DID resolution cache ownership is inspected

- **WHEN** an agent searches for portable cache policy and operational adapter
  work
- **THEN** it SHALL find SDK-enforced key/capacity/TTL limits and a separate
  caller-budgeted cache/clock adapter row

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

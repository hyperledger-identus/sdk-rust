## ADDED Requirements

### Requirement: transaction data is exact, bounded, and linked

The SDK SHALL parse each exact base64url-encoded transaction-data object under
finite byte/JSON/item limits, require a supported type and non-empty unique
DCQL credential references, and retain the exact encoded representation.

#### Scenario: supported transaction data references a query

- **WHEN** one registered type capability accepts the object and every credential ID resolves
- **THEN** authorization state retains redacted bounded linkage and the exact encoded input

#### Scenario: mandatory semantics are unknown or malformed

- **WHEN** encoding, JSON, required fields, type capability, duplicates, references, or limits fail
- **THEN** validation terminates with a static transaction-data category

### Requirement: credential formats use SDK-owned capability ports

The SDK SHALL represent known Final credential formats and bounded extensions
with SDK-owned values and SHALL validate non-empty format metadata through one
exact injected capability without exposing implementation types or errors.

#### Scenario: format metadata is supported

- **WHEN** the DCQL format and metadata are accepted by exactly one capability
- **THEN** the query retains SDK-owned format evidence for later selection linkage

#### Scenario: format semantics are absent or ambiguous

- **WHEN** non-empty metadata has no capability, registrations conflict, or a capability rejects it
- **THEN** query validation fails before candidate evaluation

### Requirement: selected inputs preserve transaction binding

The SDK SHALL validate that each transaction reference is satisfied by one
selected credential query and that its selected format accepts the transaction
type before returning proof-engine input.

#### Scenario: selected credential can carry the transaction

- **WHEN** the selected query ID and format satisfy the retained linkage
- **THEN** the SDK returns a redacted binding input with an explicit sensitive accessor for the exact encoding

#### Scenario: selected input cannot carry the transaction

- **WHEN** no referenced query is selected or the selected format rejects the type
- **THEN** the SDK returns a static failure before proof creation

### Requirement: format proof engines remain outside OID4VP

The SDK SHALL NOT create or verify SD-JWT VC, mdoc, W3C VC, or extension proofs
inside the OID4VP protocol crate.

#### Scenario: a caller receives binding input

- **WHEN** protocol linkage succeeds
- **THEN** the owning credential-format crate remains responsible for cryptographic binding

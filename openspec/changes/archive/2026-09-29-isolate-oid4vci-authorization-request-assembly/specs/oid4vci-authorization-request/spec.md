# OID4VCI Authorization Request assembly ownership

## ADDED Requirements

### Requirement: Authorization Request assembly has one private owner

The SDK MUST keep endpoint/query preparation, bounded Authorization Details,
checked complete sizing, and exact form rendering behind the unchanged
consuming public method in one cohesive private assembly boundary.

#### Scenario: A bounded request is assembled

- **WHEN** validated input lineage and the selected server satisfy existing
  endpoint, query, Authorization Details, and final URI limits
- **THEN** the same exact zeroizing request URI is returned with identical
  managed parameter order, conditional values, predecessor lineage, and
  redacted diagnostics.

#### Scenario: Multiple assembly phases are invalid

- **WHEN** endpoint query, Authorization Details, and final URI size contain
  two or more simultaneous failures
- **THEN** the same existing error is returned from the earliest current phase
- **AND** no request secret or caller-controlled value is exposed or usable.

#### Scenario: Assembly remains bounded and exact

- **WHEN** the private owner prepares and renders a request
- **THEN** existing count/name/value/details/final ceilings, strict decoded
  duplicate and reserved-name rejection, checked arithmetic, fixed parameter
  order, one exact zeroizing allocation, and byte-preserved existing query
  remain authoritative
- **AND** no new dependency, allocation class, ambient I/O, dynamic dispatch,
  synchronization, metric waiver, or forwarding-only helper chain is added.

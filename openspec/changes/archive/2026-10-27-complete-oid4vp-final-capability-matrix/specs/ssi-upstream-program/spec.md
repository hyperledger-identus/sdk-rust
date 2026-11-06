# SSI upstream program evidence

## ADDED Requirements

### Requirement: OpenID4VP completion is evidence based

IDR-024 SHALL become delivered only from a reviewed, machine-checked Final
matrix whose pinned required capability set contains no non-implemented row.
Official OpenID certification remains a separate release and product gate.

#### Scenario: a required capability is missing

- **WHEN** the matrix finds absent Request Object security, client scheme,
  response-mode, transaction-data, or format-metadata behavior
- **THEN** one focused issue owns each independent gap
- **AND** IDR-024 remains in progress

### Requirement: delivered rows do not remain stale

The roadmap SHALL reconcile an implementation owner after its reviewed change
is merged and its issue is closed while retaining explicit protocol maturity
limitations.

#### Scenario: the SIOPv2 draft core is delivered

- **WHEN** issue #514 is merged and closed with canonical specification and
  executable evidence
- **THEN** IDR-031 is marked delivered
- **AND** its draft-13 and excluded-profile limitations remain explicit

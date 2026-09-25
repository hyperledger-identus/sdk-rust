## ADDED Requirements

### Requirement: IDR-023 completion is evidence based

IDR-023 and M4 SHALL close only from a reviewed, machine-checked OpenID4VCI
Final wallet-core matrix that reconciles issue #7, current public APIs,
canonical specifications, executable tests, consumer-vector provenance and
residual limitations.

#### Scenario: closeout finds a required gap

- **WHEN** the matrix identifies missing required wallet-core behavior or
  unresolved fixture provenance
- **THEN** the program creates one focused owner for each independent gap
- **AND** keeps issue #7 or M4 open as required by the reviewed recommendation

#### Scenario: historical acceptance wording is stale

- **WHEN** issue #7 wording predates the bounded architecture or current source
  evidence
- **THEN** the report reconciles it explicitly instead of inferring completion
  from elapsed time or child issue count

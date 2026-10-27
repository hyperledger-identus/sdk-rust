# oid4vp-authorization-request-routing delta

## ADDED Requirements

### Requirement: Authorization state retains bounded response lineage

The SDK SHALL retain optional bounded `state` and only the private DCQL
response-shape evidence needed by a consuming response transition while it
constructs the existing composed authorization state.

#### Scenario: bounded state is present

- **WHEN** a supported request contains optional bounded state
- **THEN** the composed result retains it for exact return by the consuming
  response transition without exposing it in diagnostics

#### Scenario: optional state is outside policy

- **WHEN** `state` is empty, not a string, contains non-visible ASCII, or
  exceeds the configured bound
- **THEN** request validation fails with a stable redacted category before DCQL
  response construction can occur

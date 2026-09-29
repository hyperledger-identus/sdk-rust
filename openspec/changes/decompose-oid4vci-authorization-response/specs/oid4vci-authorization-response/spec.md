# OID4VCI Authorization Response ownership

## ADDED Requirements

### Requirement: Authorization Response decoding and correlation have distinct private owners

The SDK MUST keep complete bounded query decoding separate from request-bound
state, selected-server issuer, and exclusive outcome correlation behind the
unchanged consuming public method.

#### Scenario: A bounded response is accepted

- **WHEN** a routed query satisfies structural/resource limits and exactly
  correlates state and effective RFC 9207 issuer policy
- **THEN** exactly one success or error branch is validated and returns the same
  redacted owned outcome with identical request lineage and issuer evidence.

#### Scenario: Multiple phases are invalid

- **WHEN** query structure or resources, state, issuer, branch shape, and field
  grammar contain two or more simultaneous failures
- **THEN** the same existing error is returned from the earliest current phase
- **AND** no authorization code, remote developer value, or request secret is
  exposed or made usable.

#### Scenario: Query decoding remains bounded and exact

- **WHEN** the private decoder scans encoded parameters
- **THEN** existing byte/count/name/role ceilings, single strict-form decoding,
  decoded duplicate rejection, input order, known-role retention, unknown-field
  discard, and zeroizing ownership remain authoritative
- **AND** no new decoding path, dependency, allocation class, ambient I/O,
  dynamic dispatch, synchronization, metric waiver, or forwarding-only helper
  chain is added.

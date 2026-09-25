# oid4vci-error-contracts Specification

## ADDED Requirements

### Requirement: Authorization Response diagnostics are static append-only contracts

The Authorization Response capability SHALL append unique public error codes
and static messages for invalid limits, encoded size/count/components,
malformed form/structure, duplicate parameters, state correlation, issuer
correlation, invalid branch and invalid/oversized code/error roles. A focused
private catalogue SHALL own these contracts and the central router SHALL make
every mapping explicit without a wildcard.

#### Scenario: remote values never enter diagnostics

- **WHEN** a response is rejected for any new reason
- **THEN** Debug, Display and bridged public errors contain no query, code,
  state, issuer, description, URI or extension value

#### Scenario: existing contracts remain immutable

- **WHEN** the new catalogue is appended
- **THEN** every baseline and prior live error keeps its exact order, code,
  kind, capability and message

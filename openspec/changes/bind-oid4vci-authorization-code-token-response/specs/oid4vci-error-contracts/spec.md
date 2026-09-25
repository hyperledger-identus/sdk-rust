# oid4vci-error-contracts Specification

## ADDED Requirements

### Requirement: Authorization Code Token HTTP diagnostics are append-only

The response-binding capability SHALL append unique fieldless public
diagnostics for invalid non-positive limits, unsupported HTTP status, a `401`
status/error mismatch,
oversized/invalid Content-Type, oversized/invalid Cache-Control, and
oversized/invalid Pragma. One focused private catalogue SHALL own the records
and the central wildcard-free router SHALL map every variant explicitly.

Every prior baseline and live error SHALL preserve its exact order,
discriminant, code, kind, capability, message, Display output and help URL. No
diagnostic SHALL retain or display status-associated content, a header, body,
token, request secret, endpoint, lineage or remote value.

#### Scenario: response validation stays static and redacted

- **WHEN** limits, status or headers reject a response
- **THEN** the error is an append-only static contract containing no rejected
  value

#### Scenario: historical inventory stays exact

- **WHEN** the nine response diagnostics are appended
- **THEN** every prior exhaustive inventory row remains semantically exact and
  the router has no wildcard fallback

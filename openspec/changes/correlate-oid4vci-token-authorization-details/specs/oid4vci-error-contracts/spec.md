# oid4vci-error-contracts Specification

## ADDED Requirements

### Requirement: Authorization Code Token correlation diagnostics are append-only

The correlation capability SHALL append unique fieldless public diagnostics
for a recognized configuration mismatch and ambiguous repeated matching
Authorization Details. One focused private catalogue SHALL own the records and
the central wildcard-free router SHALL map both variants explicitly.

Every prior baseline and live error SHALL preserve its exact order,
discriminant, code, kind, capability, message, Display output and help URL. No
diagnostic SHALL retain or display a token, JSON body, dataset identifier,
configuration, issuer/server/endpoint, lineage or remote value.

#### Scenario: correlation errors remain static and redacted

- **WHEN** configuration authority or entry cardinality rejects a response
- **THEN** the returned error is an append-only static contract containing no
  rejected value

#### Scenario: historical inventory stays exact

- **WHEN** the two correlation diagnostics are appended
- **THEN** every prior exhaustive inventory row remains semantically exact and
  the router has no wildcard fallback

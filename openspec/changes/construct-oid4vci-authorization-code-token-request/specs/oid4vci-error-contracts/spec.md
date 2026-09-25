# oid4vci-error-contracts Specification

## ADDED Requirements

### Requirement: Authorization Code Token Request diagnostics are append-only

The Authorization Code Token Request capability SHALL append unique fieldless
public diagnostics for invalid non-positive limits, an oversized request Token
Endpoint, and an oversized encoded request body. One focused private catalogue
SHALL own the records and the central wildcard-free router SHALL map every
variant explicitly.

Every prior baseline and live error SHALL preserve its exact order,
discriminant, code, kind, capability, message, Display output and help URL.
No diagnostic SHALL retain or display an endpoint, body, code, verifier,
redirect URI, client identifier or remote value.

#### Scenario: new errors remain static and redacted

- **WHEN** limits or request construction reject input
- **THEN** the returned error is an append-only static contract containing no
  rejected value

#### Scenario: the historical inventory stays exact

- **WHEN** the three request diagnostics are appended
- **THEN** every prior exhaustive inventory row remains semantically exact and
  the router has no wildcard fallback

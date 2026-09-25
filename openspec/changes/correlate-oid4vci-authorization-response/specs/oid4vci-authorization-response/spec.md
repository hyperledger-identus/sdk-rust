# oid4vci-authorization-response Specification

## ADDED Requirements

### Requirement: Authorization Response input is a bounded routed query

The SDK SHALL accept only an already-extracted query-form payload from the
request's registered callback adapter. It SHALL apply positive limits to the
complete encoded payload, decoded parameter count, names, values and each
retained role. It SHALL strictly form-decode UTF-8, reject empty names,
malformed bytes and duplicate decoded names, and validate then discard unique
unknown fields. It SHALL NOT accept or claim validation of a full callback
URI, fragment, form-post body, JARM object or transport event.

#### Scenario: unknown unique fields remain compatible

- **WHEN** a bounded response contains valid unique extension fields
- **THEN** correlation ignores them without retaining or interpreting them

#### Scenario: alternate encodings cannot bypass duplicate defense

- **WHEN** literal and percent-encoded names decode to the same field
- **THEN** the response fails with a static duplicate error

### Requirement: Every response is correlated to exact state first

Every request produced by the predecessor contains state. The SDK SHALL
require one non-empty returned state and compare its decoded value exactly to
the retained request state before returning a usable success or error outcome.
The request SHALL be consumed so the API cannot correlate it twice.

#### Scenario: matching state permits branch validation

- **WHEN** exactly one returned state equals the request state
- **THEN** issuer and success/error validation continue

#### Scenario: missing or substituted state fails closed

- **WHEN** returned state is absent, empty or differs by any byte
- **THEN** no code or remote error becomes usable

### Requirement: Issuer identification follows exact selected-server metadata

The SDK SHALL compare `iss`, when required, with the exact selected
Authorization Server Metadata issuer rather than the Credential Issuer. If
RFC 9207 support is effectively true, `iss` SHALL be present and equal by
simple string comparison. If support is effectively false, `iss` SHALL be
absent. Outcomes SHALL distinguish verified RFC 9207 evidence from not
advertised and SHALL NOT describe the latter as mix-up protection.

#### Scenario: delegated server matches its own issuer

- **WHEN** an explicitly delegated server advertises RFC 9207 and returns its
  exact issuer
- **THEN** the outcome records verified RFC 9207 identification

#### Scenario: unadvertised issuer is rejected

- **WHEN** support is false or omitted and a response supplies `iss`
- **THEN** correlation fails rather than silently changing policy

### Requirement: Success and OAuth error branches are exclusive and redacted

Success SHALL contain exactly one bounded non-empty RFC 6749 visible-ASCII
authorization code and no error fields. Error SHALL contain exactly one
bounded NQSCHAR error code, no code, and may contain independently bounded
NQSCHAR developer description and valid URI-reference. Standard Authorization
Endpoint errors SHALL have a closed classification and extensions SHALL remain
exact and untrusted. All retained fields and aggregate outcomes SHALL use
redacted Debug; sensitive code and untrusted text SHALL require explicit
accessors.

#### Scenario: exact success preserves code-exchange lineage

- **WHEN** state/issuer correlate and one valid code is present
- **THEN** the success retains the complete request and code for later
  bounded code-exchange construction

#### Scenario: ambiguous branches fail closed

- **WHEN** code and error coexist, neither exists, or developer fields appear
  without error
- **THEN** no outcome is constructed

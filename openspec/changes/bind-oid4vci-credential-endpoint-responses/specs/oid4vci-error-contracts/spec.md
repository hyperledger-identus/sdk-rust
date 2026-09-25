## ADDED Requirements

### Requirement: Credential Endpoint classifier status diagnostic is append-only

The Credential Endpoint response classifier SHALL append one unique fieldless
public diagnostic for a status other than `200`, `202`, or `400`. The focused
deferred/immediate issuance catalogue SHALL own the static contract and the
central wildcard-free router SHALL map it explicitly.

Every prior baseline and live error SHALL preserve its exact order,
discriminant, code, kind, capability, message, Display output and help URL. The
new diagnostic SHALL NOT retain or display status-associated content, a media
type, body, bearer token, proof, endpoint, transaction, credential or remote
value.

#### Scenario: unsupported status remains static and redacted

- **WHEN** status-first classification rejects a response before media/body parsing
- **THEN** the returned error is the append-only static
  `oid4vci.invalid_credential_endpoint_http_status` contract

#### Scenario: historical inventory stays exact

- **WHEN** the classifier diagnostic is appended
- **THEN** every prior exhaustive inventory entry remains semantically exact
  and the router has no wildcard fallback

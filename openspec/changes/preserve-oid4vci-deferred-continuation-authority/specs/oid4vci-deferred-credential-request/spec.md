## ADDED Requirements

### Requirement: request-bound continuation preserves exact authority

The SDK SHALL consume a request-bound initial HTTP 202 response into a distinct
authorized Deferred Credential Request. Construction SHALL accept only the
existing request-limit policy and SHALL preserve the exact issuer, advertised
Deferred Credential Endpoint, bearer Authorization, response transaction and
originating proof count. It SHALL accept no replacement authority value.

#### Scenario: exact authority constructs the continuation

- **WHEN** a request built from matched metadata and bearer authority receives
  a valid request-bound HTTP 202 response
- **THEN** the resulting authorized request exposes that exact issuer, endpoint,
  Authorization, serialized transaction and proof count

#### Scenario: advertised endpoint is absent

- **WHEN** the originating matched metadata omitted a Deferred Credential Endpoint
- **THEN** bound construction fails with `DeferredCredentialEndpointRequired`
- **AND** no detached metadata or endpoint can be supplied

### Requirement: authorized continuation is one-shot and redacted

The request-bound response and authorized request SHALL NOT implement Clone or
generic Serde. Construction SHALL move rather than duplicate bearer authority,
reuse the existing bounded transaction serializer, and erase retained authority
on error or drop. Debug and errors SHALL NOT expose issuer, endpoint, bearer,
transaction, proof or request-body values.

#### Scenario: diagnostics remain value-free

- **WHEN** issuer, endpoint, bearer and transaction contain unique canaries
- **THEN** Debug and direct/bridged errors contain none of those canaries

#### Scenario: transport and lifecycle remain outside the type

- **WHEN** an authorized continuation request is constructed
- **THEN** it performs no HTTP, token validation, scheduling, retry, polling,
  credential processing, persistence or product policy

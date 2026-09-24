# oid4vci-deferred-credential-error-http-response Specification

## ADDED Requirements

### Requirement: A deferred request validates the inherited Final payload-error envelope

The SDK SHALL validate an unencrypted Deferred Credential Endpoint
payload-error response through the originating `DeferredCredentialRequest`.
Validation SHALL delegate status, bounded Content-Type, body limits, JSON
syntax, duplicate handling, error-code grammar, descriptions, extensions and
generic `invalid_request` rejection to the existing
`CredentialErrorResponseCore::parse_http_response` contract with caller-chosen
`CredentialErrorHttpResponseLimits`.

Success SHALL prove only bounded structural attachment to a request object. It
SHALL NOT prove response origin, issuer identity, transaction correlation,
truth, authorization, retry safety, terminal state or any side effect.

#### Scenario: exact deferred payload error succeeds

- **WHEN** a request validates status 400, bounded `application/json`, and a
  valid Final section 9.3 payload body
- **THEN** validation returns a deferred error response retaining the existing
  bounded core

#### Scenario: inherited envelope failures remain exact

- **WHEN** status, media type or body violates the existing Credential Error
  HTTP response contract
- **THEN** the existing static error is returned with unchanged precedence

### Requirement: Deferred error classification is additive and endpoint-specific

The result SHALL classify exact `invalid_transaction_id` as
`InvalidTransactionId`, exact `credential_request_denied` as
`CredentialRequestDenied`, and every other accepted code as
`Inherited(CredentialEndpointErrorKind)`. The existing public generic
classification SHALL NOT be widened or reinterpreted.

The result SHALL expose the retained core by shared reference so callers may
read the exact bounded code and deliberately untrusted description. It SHALL
NOT copy those strings or expose them through Debug, Display or errors.

#### Scenario: additional Final code is recognized exactly

- **WHEN** the payload code is exact `invalid_transaction_id`
- **THEN** deferred classification is `InvalidTransactionId` while the core
  retains the exact bounded code

#### Scenario: inherited known and extension codes remain interoperable

- **WHEN** the payload contains another accepted known or extension code
- **THEN** deferred classification wraps its existing generic classification

### Requirement: Stop-polling guidance is pure and narrowly scoped

The result SHALL expose section 9.3 stop-polling guidance as true only for
exact `credential_request_denied`. It SHALL perform no timer, retry,
cancellation, invalidation, deletion, storage or other lifecycle action.

Repeated validation of the same request and input SHALL remain structurally
equivalent. No new dependency, feature, lockfile, unsafe/native, target,
consumer, chain or product behavior SHALL be added.

#### Scenario: denied request carries explicit guidance

- **WHEN** deferred classification is `CredentialRequestDenied`
- **THEN** stop-polling guidance is true and no request state is mutated

#### Scenario: other errors gain no policy

- **WHEN** deferred classification is any other branch
- **THEN** stop-polling guidance is false and retry/remediation remains caller
  policy

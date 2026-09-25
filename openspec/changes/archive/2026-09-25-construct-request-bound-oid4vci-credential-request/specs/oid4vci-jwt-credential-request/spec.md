# oid4vci-jwt-credential-request Specification

## ADDED Requirements

### Requirement: Correlated Authorization Code authority constructs one request

The SDK SHALL consume one correlated Authorization Code Token success, select
one authorized Credential Dataset identifier by checked source-order index,
and construct the existing bounded JWT-proof Credential Request. The
Credential Endpoint, access token, selected configuration and dataset
identifier SHALL derive only from the consumed state; callers SHALL NOT supply
replacement metadata, token or selector text.

#### Scenario: exact authorized dataset constructs the existing wire shape

- **WHEN** a valid index selects one correlated identifier and valid bounded
  proofs are supplied
- **THEN** the existing request type contains the retained Credential Endpoint,
  exact Bearer token, exact `credential_identifier` and proofs in caller order

#### Scenario: missing selection fails before output

- **WHEN** the index is outside the authorized identifier collection
- **THEN** construction fails with the existing static missing-identifier error
  and returns no Authorization value or JSON body

#### Scenario: correlated authority is one-shot

- **WHEN** construction succeeds or fails
- **THEN** the correlated token state is consumed and cannot authorize a
  second request through the typed API

### Requirement: Existing Credential Request behavior remains exact

The new transition SHALL reuse the existing Bearer, proof, authorization and
body validation plus deterministic serializer. Every existing public
constructor, validation precedence, limit, error and output byte SHALL remain
unchanged.

#### Scenario: existing routes remain compatible

- **WHEN** callers use either prior configuration-ID or detached authorized
  dataset constructor
- **THEN** source compatibility and exact successful/error behavior are unchanged

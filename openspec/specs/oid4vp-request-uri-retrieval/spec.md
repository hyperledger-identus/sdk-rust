# oid4vp-request-uri-retrieval Specification

## Purpose
Define bounded runtime-neutral GET/POST Request URI construction and exact
response binding before signed Request Object processing.
## Requirements
### Requirement: Retrieval requests are bounded runtime-neutral data

The SDK SHALL consume a referenced request into a bounded HTTP request
description without performing transport, generating entropy, or selecting
runtime policy.

#### Scenario: GET retrieval is prepared

- **WHEN** a GET/default reference is consumed
- **THEN** the request targets the retained HTTPS URI, carries the JAR Accept media type, and has no content type or body

#### Scenario: POST retrieval is prepared

- **WHEN** a POST reference is consumed with bounded optional metadata and nonce
- **THEN** the request carries the JAR Accept value and a deterministic UTF-8 form body with the form content type

#### Scenario: outbound content exceeds a limit

- **WHEN** metadata, nonce, or the encoded body exceeds its independent maximum
- **THEN** construction fails with a static resource category

### Requirement: Request URI responses are bound before JAR parsing

The SDK SHALL consume the prepared request and accept only one successful,
correctly typed, non-empty, bounded compact signed Request Object response.

#### Scenario: a valid response arrives

- **WHEN** a 2xx response has the exact JAR media type and a bounded three-segment compact body
- **THEN** response binding produces an explicitly unverified Request Object state

#### Scenario: HTTP or media evidence is invalid

- **WHEN** status is not 2xx or the bounded content type is absent or wrong
- **THEN** binding terminates with a static response category

#### Scenario: an encrypted object arrives

- **WHEN** the bounded response has JWE compact shape
- **THEN** binding returns the explicit unsupported-encrypted-object category

#### Scenario: response content exceeds a limit

- **WHEN** content type or body exceeds its independent maximum
- **THEN** binding fails before parsing or retaining the object

### Requirement: Retrieval diagnostics are redacted

The SDK SHALL keep endpoint, client id, metadata, nonce, body, and response
content out of public diagnostics.

#### Scenario: hostile transport values fail

- **WHEN** verifier-controlled canaries appear in a failing request or response
- **THEN** `Debug`, `Display`, errors, and shared metadata expose no canary

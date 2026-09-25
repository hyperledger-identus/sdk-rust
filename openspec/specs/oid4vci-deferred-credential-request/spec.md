# oid4vci-deferred-credential-request Specification

## Purpose
TBD - created by archiving change add-oid4vci-deferred-credential-request. Update Purpose after archive.
## Requirements
### Requirement: The SDK constructs the unencrypted Final Deferred Credential Request

The SDK SHALL construct a `DeferredCredentialRequest` from a validated
`DeferredCredentialResponseCore` and `CredentialIssuerMetadata` that advertises
a `DeferredCredentialEndpoint`. The request SHALL own that exact typed endpoint
and a deterministic JSON object containing exactly the response's decoded
`transaction_id` string.

The request SHALL report HTTP POST, media type `application/json`, and that an
OAuth access token is required. It SHALL NOT own a token, construct an
Authorization header or execute HTTP.

#### Scenario: Final example becomes a typed request

- **WHEN** a deferred response contains transaction ID `8xLOxBtZp8` and matched
  metadata advertises `https://server.example.com/deferred_credential`
- **THEN** construction returns that endpoint and exact body
  `{"transaction_id":"8xLOxBtZp8"}` with POST and `application/json`

#### Scenario: endpoint omission fails before transport

- **WHEN** otherwise valid metadata does not advertise a Deferred Credential
  Endpoint
- **THEN** construction fails with the static
  `DeferredCredentialEndpointRequired` error

### Requirement: Request serialization is independently bounded and standards-correct

`DeferredCredentialRequestLimits` SHALL accept exactly one positive complete
JSON body byte ceiling and SHALL default to 16,384 bytes. Construction SHALL
serialize the transaction string through the existing JSON serializer into a
writer that checks the complete body bound before retaining each write.

An invalid zero limit SHALL return `InvalidDeferredCredentialRequestLimits`.
An encoded body above the ceiling or checked-length overflow SHALL return
`DeferredCredentialRequestTooLarge`. Exact-limit output SHALL succeed.
Caller-configured response limits SHALL NOT bypass this independent request
body policy.

#### Scenario: escaped transaction remains exact JSON

- **WHEN** a valid decoded transaction contains quotes, backslashes, control
  escapes or non-ASCII text
- **THEN** the request body is valid JSON that decodes to the exact original
  transaction string without an unbounded encoded intermediate

#### Scenario: exact and one-under body policies diverge deterministically

- **WHEN** the complete encoded request body exactly matches its configured
  ceiling
- **THEN** construction succeeds
- **AND WHEN** the ceiling is one byte smaller
- **THEN** construction returns the static request-too-large error

### Requirement: Deferred request state is redacted and policy-neutral

The request body SHALL use zeroizing ownership, SHALL expose bytes only through
`expose_sensitive_json_body`, and SHALL expose its length separately. The
request, endpoint, transaction and errors SHALL NOT expose endpoint or
transaction content through Debug or Display. The public request SHALL NOT
implement Clone or generic Serde.

Construction SHALL add no dependency, feature, unsafe, runtime, network,
storage, timer, target, chain, downstream or product-policy authority. It SHALL
NOT establish endpoint provenance, issuer trust/control, reachability, TLS or
redirect safety, token/transaction validity, interval compliance,
retry/replay/invalidation, encryption, response correlation, credential
verification/storage, publication, release or certification.

#### Scenario: diagnostics contain no sensitive request material

- **WHEN** a transaction and endpoint contain unique canary strings and callers
  inspect request/metadata/response Debug plus direct and bridged errors
- **THEN** no canary content appears

#### Scenario: repeated construction does not imply replay policy

- **WHEN** callers construct more than one request from the same still-deferred
  response
- **THEN** each structural request is identical
- **AND** the SDK makes no timing, retry, terminal-use or invalidation claim

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


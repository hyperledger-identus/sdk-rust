## ADDED Requirements

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

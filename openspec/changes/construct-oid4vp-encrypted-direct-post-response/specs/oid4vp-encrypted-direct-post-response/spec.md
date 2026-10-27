# oid4vp-encrypted-direct-post-response delta

## ADDED Requirements

### Requirement: Final protected response metadata is bounded and explicit

The SDK SHALL accept `direct_post.jwt` only from a verified Request Object whose
bounded verifier metadata yields exactly one supported public recipient key and
one supported content-encryption algorithm. The first profile SHALL support
P-256 `ECDH-ES` and `A128GCM` only.

#### Scenario: supported metadata selects one key

- **WHEN** one public P-256 JWK has `alg` equal to `ECDH-ES`, compatible
  encryption use/operations and optional bounded `kid`
- **THEN** the validated request retains an opaque SDK-owned encryption plan

#### Scenario: metadata is unsafe or ambiguous

- **WHEN** keys are missing, multiple keys qualify, a private member is present,
  algorithms or operations conflict, or a resource limit is exceeded
- **THEN** validation fails before any encryption provider is called

### Requirement: the JWE capability is an SDK-owned bounded port

The SDK SHALL expose protocol-neutral P-256 ECDH-ES/A128GCM request and compact
result types without exposing an implementation dependency. Plaintext, key ID,
public key and output SHALL have independent finite limits and redacted
diagnostics.

#### Scenario: an adapter encrypts accepted material

- **WHEN** a conforming adapter receives the validated SDK request
- **THEN** it returns one bounded opaque five-segment compact JWE or one static
  failure category

### Requirement: protected response construction consumes validated lineage

The SDK SHALL consume one validated `direct_post.jwt` request and matching
bounded presentation results, construct the Final top-level Authorization
Response JSON, invoke the encrypter at most once, and emit deterministic form
material containing exactly one `response` parameter.

#### Scenario: protected success completes

- **WHEN** DCQL/cardinality checks pass and the adapter returns valid bounded
  compact JWE material
- **THEN** the result is bound to the validated response URI and contains no
  plain `vp_token`, state, key or presentation form field

#### Scenario: encryption cannot complete

- **WHEN** the adapter fails or returns malformed/oversized compact material
- **THEN** construction returns a stable redacted local failure and does not
  automatically select a wire error

### Requirement: effects and policy remain outside the protocol core

The protected response transition SHALL perform no HTTP, trust, consent,
credential verification, persistence, telemetry or product policy. Encryption
SHALL NOT be described as wallet authentication.

#### Scenario: a caller delivers protected material

- **WHEN** the caller reveals the inert target and form body
- **THEN** its adapter retains responsibility for transport and delivery outcome


# oid4vp-direct-post-response Specification

## Purpose
TBD - created by archiving change construct-oid4vp-direct-post-response. Update Purpose after archive.
## Requirements
### Requirement: response construction consumes validated request lineage

The SDK SHALL construct a plain direct-post success response only by consuming
one `ValidatedAuthorizationRequest` and caller-supplied bounded presentation
results. It SHALL derive the destination, optional state, query identifiers,
credential-set coverage and cardinality from that validated state.

The plain constructor SHALL additionally require the validated request's exact
mode to be `direct_post` and SHALL reject a protected request before serializing
plain response material.

#### Scenario: one supported request receives matching results

- **WHEN** every required query has a conforming caller-selected presentation
- **THEN** the SDK returns one inert direct-post envelope bound to the request's
  validated response URI

#### Scenario: caller result lineage does not match

- **WHEN** a result identifier is unknown or duplicated, required
  credential-set coverage is unsatisfied, or a non-multiple query contains
  more than one presentation
- **THEN** construction fails with a stable redacted category before response
  serialization

#### Scenario: protected mode reaches the plain constructor

- **WHEN** a validated `direct_post.jwt` request is supplied to the plain
  response constructor
- **THEN** construction fails with a static mode-mismatch category before
  returning any `vp_token` or state form material

### Requirement: vp_token follows the Final DCQL response shape

The SDK SHALL serialize `vp_token` as one JSON object keyed by validated DCQL
credential-query identifier. Every present value SHALL be a non-empty array of
bounded presentation strings or objects. When `credential_sets` is absent,
every query SHALL be present. When it is present, every required set SHALL have
at least one option whose query identifiers are all present; optional sets MAY
remain unsatisfied.

#### Scenario: one required alternative is satisfied

- **WHEN** a required credential set contains alternatives `[["pid"],
  ["mdl"]]` and the response includes only the bounded `pid` result
- **THEN** credential-set coverage is satisfied without requiring `mdl`

#### Scenario: no required alternative is satisfied

- **WHEN** no option of one required credential set is fully represented
- **THEN** construction fails before response serialization

#### Scenario: string and object presentations are selected

- **WHEN** bounded Final presentation values satisfy their corresponding query
- **THEN** they are serialized without changing their JSON type and in the
  validated query order

#### Scenario: a presentation is malformed or outside policy

- **WHEN** a value is neither a JSON string nor object, its JSON work is
  invalid, or any value/count/aggregate limit is exceeded
- **THEN** construction fails before an unbounded response buffer is produced

### Requirement: direct-post output is deterministic and bounded

The SDK SHALL emit a bounded UTF-8 `application/x-www-form-urlencoded` body
containing `vp_token` and the request's exact `state` when present. Encoding
SHALL be deterministic and SHALL apply output limits during construction.

#### Scenario: the request contains state

- **WHEN** response construction succeeds for a request with bounded state
- **THEN** the form contains `vp_token` followed by exactly one returned
  `state` parameter with correct form encoding

#### Scenario: encoded output exceeds policy

- **WHEN** JSON or percent encoding would exceed its configured output bound
- **THEN** construction fails with a static limit category before returning an
  envelope

### Requirement: an envelope does not own transport or proof semantics

The SDK SHALL keep direct-post construction synchronous and effect-free. The
result SHALL NOT claim HTTP execution, destination trust, credential or
presentation verification, consent, signing, encryption, replay protection,
or successful delivery.

#### Scenario: caller chooses to deliver the envelope

- **WHEN** a caller explicitly reveals the target and body
- **THEN** its adapter remains responsible for DNS, address policy, TLS,
  redirects, timeout, retry, SSRF defense, and HTTP outcome handling

### Requirement: protocol-error construction consumes validated request lineage

The SDK SHALL construct a plain direct-post Authorization Error Response only
by consuming one `ValidatedAuthorizationRequest` and an explicit SDK-owned
protocol error code. It SHALL derive the destination and optional state from
that request and SHALL NOT automatically translate SDK or transport errors.

#### Scenario: caller selects a Final protocol error

- **WHEN** a caller explicitly selects a supported error after applying its
  privacy and consent policy
- **THEN** the SDK returns one inert error envelope bound to the validated
  response URI and exact request state

#### Scenario: internal processing fails

- **WHEN** request parsing, validation, transport or SDK processing returns an
  internal error
- **THEN** no implicit conversion produces a protocol error response

### Requirement: the protocol vocabulary and optional fields are bounded

The SDK SHALL expose the RFC 6749 authorization errors and OpenID4VP 1.0 Final
errors named in sections 8.5 and Appendix E as a closed enum. Optional
`error_description` SHALL be non-empty RFC NQSCHAR ASCII under an explicit
limit. Optional `error_uri` SHALL be a bounded syntactically valid URI-reference.
All variable values SHALL have redacted diagnostics and SHALL NOT imply display,
resolution, provenance, privacy or consent safety.

#### Scenario: bounded developer information is supplied

- **WHEN** description and URI-reference satisfy their exact syntax and limits
- **THEN** the SDK retains them as untrusted redacted protocol values

#### Scenario: optional information is malformed or oversized

- **WHEN** a value is empty, violates RFC grammar, fails URI-reference parsing,
  or exceeds policy
- **THEN** construction fails with a static redacted category

### Requirement: error form output is deterministic, exclusive and bounded

The SDK SHALL emit bounded UTF-8 `application/x-www-form-urlencoded` material
ordered as `error`, optional `error_description`, optional `error_uri`, and
exact request `state` when present. Error output SHALL never contain `vp_token`.
The final encoded body SHALL be checked with overflow-safe arithmetic before
allocation and returned in a redaction-safe inert envelope.

#### Scenario: Final example is constructed

- **WHEN** `invalid_request`, description `unsupported client_id_prefix`, and
  request state are supplied
- **THEN** the form matches the Final field order and encoding

#### Scenario: encoded output exceeds policy

- **WHEN** optional values or percent encoding exceed the configured form bound
- **THEN** construction fails without returning a partial envelope

### Requirement: error construction owns no effects or privacy decision

The SDK SHALL NOT perform HTTP, resolve an error URI, authorize disclosure,
claim End-User interaction, expose holder data, or infer successful delivery.

#### Scenario: a caller delivers or displays error material

- **WHEN** a caller reveals the inert target/body or optional developer fields
- **THEN** its adapter remains responsible for transport, UI, localization,
  user interaction, consent and privacy policy


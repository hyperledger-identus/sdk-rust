# oid4vp-direct-post-response Specification

## Purpose
TBD - created by archiving change construct-oid4vp-direct-post-response. Update Purpose after archive.
## Requirements
### Requirement: response construction consumes validated request lineage

The SDK SHALL construct a plain direct-post success response only by consuming
one `ValidatedAuthorizationRequest` and caller-supplied bounded presentation
results. It SHALL derive the destination, optional state, query identifiers,
credential-set coverage and cardinality from that validated state.

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


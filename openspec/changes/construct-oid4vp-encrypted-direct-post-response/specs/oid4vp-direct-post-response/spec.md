# oid4vp-direct-post-response delta

## MODIFIED Requirements

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

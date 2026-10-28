## ADDED Requirements

### Requirement: redirect responses consume authorized request lineage

The SDK SHALL construct `fragment`, `query`, or `form_post` response material
only by consuming a verifier-authorized request carrying the matching exact
bounded redirect destination.

#### Scenario: a supported redirect mode succeeds

- **WHEN** DCQL results satisfy the request and its signed redirect URI is valid
- **THEN** deterministic inert material contains the exact VP token and optional exact state in the requested encoding

#### Scenario: mode and destination conflict

- **WHEN** a redirect mode has `response_uri`, a direct-post mode has `redirect_uri`, or both are present
- **THEN** routing fails before response construction

### Requirement: redirect encodings are bounded and unambiguous

The SDK SHALL bound every destination, parameter, encoded component, and final
output, preserve only non-conflicting existing URI components, and reject
duplicate or colliding generated response names.

#### Scenario: destination already has safe data

- **WHEN** existing query or fragment fields cannot be confused with generated response fields
- **THEN** they are preserved and generated fields use the response mode's exact component

#### Scenario: output is ambiguous or oversized

- **WHEN** parsing, escaping, collisions, component count, field size, or complete output violates policy
- **THEN** construction fails with a static redacted category

### Requirement: no redirect material authorizes effects

The SDK SHALL expose inert sensitive accessors and SHALL NOT navigate, render
HTML, execute HTTP, select product privacy policy, or claim that the destination
is safe beyond the accepted verifier policy and syntax evidence.

#### Scenario: caller receives response material

- **WHEN** construction succeeds
- **THEN** browser, transport, leakage, consent, and session policy remain caller-owned

### Requirement: direct-post acknowledgement is request-bound

The SDK SHALL consume sent direct-post material before accepting exact status
200, bounded JSON media type, strict no-store cache evidence, and a bounded
duplicate-safe acknowledgement object.

#### Scenario: acknowledgement omits redirect

- **WHEN** the response endpoint returns a valid empty object
- **THEN** the result is terminal and requires no navigation

#### Scenario: acknowledgement returns a redirect

- **WHEN** a valid object contains one bounded absolute redirect URI
- **THEN** the result exposes an inert navigation candidate and optional unique opaque response code

#### Scenario: acknowledgement is detached or malformed

- **WHEN** status, media type, cache policy, JSON, URI, duplicate member, response-code ambiguity, or resource policy fails
- **THEN** no navigation-capable successor is returned

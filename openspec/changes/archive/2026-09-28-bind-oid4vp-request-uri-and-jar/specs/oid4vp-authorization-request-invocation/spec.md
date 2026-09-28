## MODIFIED Requirements

### Requirement: Reference invocation is exact and bounded

The SDK SHALL parse one bounded `openid4vp:?…` Authorization Request reference
at the Final static `openid4vp:` authorization endpoint containing exactly one
decoded non-empty `client_id`, exactly one decoded HTTPS `request_uri`, and at
most one `request_uri_method` whose supported case-sensitive explicit values
are `get` and `post`.

#### Scenario: Final by-reference shape arrives

- **WHEN** a bounded invocation carries `client_id` and `request_uri` without a method
- **THEN** the result retains redacted owned values and reports GET retrieval

#### Scenario: GET is requested explicitly

- **WHEN** `request_uri_method=get` occurs once
- **THEN** the result reports GET retrieval

#### Scenario: POST capability negotiation is requested

- **WHEN** `request_uri_method=post` occurs once
- **THEN** the result reports POST without performing transport

#### Scenario: a configurable boundary is exceeded

- **WHEN** invocation bytes, pairs, names, values, client id or URI exceed the configured maximum
- **THEN** parsing fails before retaining a partial result

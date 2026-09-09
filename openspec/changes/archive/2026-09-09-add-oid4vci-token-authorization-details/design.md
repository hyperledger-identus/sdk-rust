# Design

## State transition

`TokenResponseCore::try_validate_authorization_details(limits)` consumes the
partial core and returns `TokenResponseWithAuthorizationDetails`. Consuming the
state prevents an application from accidentally treating the same response as
both presence-only and semantically validated. The validated wrapper retains
the core and an ordered vector of recognized credential entries.

## Parser

The existing custom scanner receives a second entry point over the retained
JSON. It reuses the complete-response depth/node budget and validates the
`authorization_details` value in place. Each array element must be an object
with unique members and a bounded string `type`.

Unknown types are traversed and counted but not retained. A recognized
`openid_credential` entry requires a bounded
`credential_configuration_id` and a non-empty bounded array of credential
identifier strings. Unknown members are traversed and discarded. Duplicate
identifiers, including reuse across recognized entries, are rejected.

## Public model

`CredentialAuthorizationDetail` exposes borrowed configuration and credential
identifier strings and count accessors. Debug output contains counts only.
`TokenResponseWithAuthorizationDetails` exposes the underlying OAuth core,
recognized entries and unknown-type count. The transition rejects a missing or
empty Authorization Details member and a response with no recognized
credential entry; a caller needing only OAuth core keeps the original state.

## Risks and mitigations

- Reparse cost is bounded and avoids changing the original parser contract.
- Mixed authorization types can hide a malformed credential entry; every
  recognized entry is strictly validated, while unrelated types are ignored.
- Identifier collisions are ambiguous; reject them before exposing selection.
- Caller data could leak through derived formatting; implement explicit
  redacted Debug and fieldless errors.

## Rollback

Remove the additive model, parser entry point, limits, errors, tests and
canonical requirement. The original partial Token Response behavior remains.

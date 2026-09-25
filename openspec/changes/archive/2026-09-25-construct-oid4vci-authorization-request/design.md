# Design

## Owned request transition

Add `AuthorizationRequest` and `AuthorizationRequestLimits` in a focused
module. `CredentialOfferWithAuthorizationRequestInput::try_into_authorization_request`
consumes the predecessor, constructs one exact request URI and retains the
predecessor so the verifier/state and complete typed lineage remain available
for later response correlation and code exchange.

The public request exposes the static GET method, byte length, whether
`issuer_state` is present, the predecessor by shared reference, and an
explicitly sensitive exact-URI accessor. Debug reports only byte length and
issuer-state presence.

## Credential intent

Use one minified `authorization_details` array with one
`openid_credential` object. Its selected `credential_configuration_id` is
borrowed from the validated offered list. If Credential Issuer Metadata
explicitly advertises `authorization_servers`, insert one `locations` array
containing the exact validated Credential Issuer identifier. Otherwise omit
`locations`. Do not emit scope or RFC 8707 resource.

## Endpoint query contract

The already validated HTTPS Authorization Endpoint may contain a form query.
Before construction, split it into a positively bounded number of fields and
strictly decode each name and value under independent byte ceilings. Reject
malformed percent escapes, raw non-ASCII, decoded invalid UTF-8/NUL, empty
names, duplicate decoded names, and collisions with:

`response_type`, `client_id`, `redirect_uri`, `state`, `code_challenge`,
`code_challenge_method`, `authorization_details`, `issuer_state`, `scope`,
`resource`, `request`, and `request_uri`.

Accepted existing query bytes are retained exactly. The managed query is then
appended with `?` when no query exists or `&` when one exists. Empty existing
queries are invalid rather than producing ambiguous separators.

## Serialization and limits

Factor the already accepted strict local form serializer into a crate-private
helper used by both pre-authorized token and authorization requests. It keeps
checked encoded sizing, literal/space/%HH rules and no public type. Endpoint
query validation reuses a generalized strict decoder without changing existing
Credential Offer transport behavior.

Construct Authorization Details JSON with a byte-bounded writer, fixed member
order and `serde_json` string escaping. Compute the final URI length with
checked arithmetic before allocation. The final fixed parameter order matches
the research contract.

## Error and compatibility surface

Append fieldless static errors for invalid request limits, oversized
Authorization Details, too many endpoint query parameters, oversized or
malformed endpoint query components, duplicate/reserved endpoint query names,
and oversized final request URI. Extend the focused authorization-code
catalogue while keeping it below the 39-record ceiling. No prior error,
discriminant, message, API or wire behavior changes.

## Risks and mitigations

- Credential-intent ambiguity: emit Authorization Details only.
- Endpoint override: decode names before retaining bytes and reject duplicates
  plus a conservative reserved set.
- Form drift: reuse the existing accepted local codec and prove exact bytes.
- Request/history leakage: zeroize ownership, redact Debug/errors and name the
  accessor sensitive; PAR/browser policy remains outside the type.
- Authorization overclaim: retain the typed predecessor but document that
  response correlation, mix-up defense and code exchange are still absent.

## Rollback

Remove the additive module/exports, private form-helper factoring,
append-only errors, tests, ADR and capability. The predecessor and all prior
OID4VCI behavior remain intact.

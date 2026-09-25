# Design

## Consuming correlation transition

Add `RequestBoundAuthorizationCodeTokenResponse::try_correlate_authorization_details`,
taking the existing positive `TokenAuthorizationDetailsLimits`. The method
consumes the request-bound success, moves out its exact lineage and
`TokenResponseCore`, and invokes the existing consuming response-local parser.
The OAuth error branch has no such method.

Add a crate-private decomposition method to
`TokenResponseWithAuthorizationDetails` and its detail objects so validated
zeroizing values move without copying. Preserve every existing public method.

## Correlated state

Add `CorrelatedAuthorizationCodeTokenResponse` containing:

- exact `AuthorizationCodeTokenResponseLineage`;
- the existing secret-bearing `TokenResponseCore`;
- the one matched entry's source-ordered unique zeroizing Credential Dataset
  identifiers; and
- the bounded count of ignored unknown authorization-detail types.

Expose borrowed lineage and token-core access, identifier count and iterator,
and unknown-type count. Do not expose raw parts, Clone, Display, Serde, a
configuration replacement, or an independently constructible state. Debug
contains counts and the already-redacted lineage only.

## Deterministic authority checks

After full response-local validation:

1. Inspect every recognized detail for exact selected-configuration equality.
   Any mismatch returns a static configuration-mismatch diagnostic.
2. Require exactly one recognized detail. More than one exact match returns a
   static ambiguity diagnostic.
3. Move the one detail's identifiers into the correlated state.

The parser already rejects missing details, an empty recognized set, empty
identifier lists, duplicate identifiers, oversized values, excessive entries,
invalid JSON and structural ambiguity. Unknown types are counted only.

## Error and compatibility surface

Append two fieldless diagnostics after every existing live row:
configuration mismatch and ambiguous recognized Authorization Details. Keep
them in one focused private correlation catalogue and map them explicitly in
the wildcard-free router.

The feature is additive and unpublished. It changes no current parser,
constructor, serializer, dependency, feature, target or baseline error row.

## Risks and mitigations

- Authority expansion: reject every recognized configuration not selected in
  the originating request.
- Positional ambiguity: require one recognized entry rather than choosing the
  first match.
- Token reuse after failure: consume the bound success; failed correlation
  drops the zeroizing token/JSON owner.
- Secret/identifier leakage: retain zeroizing owners and expose only borrowed
  deliberate accessors; Debug and errors contain counts/static text only.
- Extension coupling: ignore bounded unknown types and fields without granting
  typed authority.
- Resource exhaustion: reuse the original response byte/depth/node budgets and
  positive Authorization Details limits before correlation work.

## Rollback

Remove the additive correlation module/export/errors/tests/ADR and private
decomposition helpers. Existing request-bound HTTP response behavior and
response-local Authorization Details parsing remain unchanged.

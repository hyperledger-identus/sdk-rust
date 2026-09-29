# Context

`VerifiedRequestObject` proves signature and correlation, while
`ValidatedDcqlQuery` proves only query structure and bounded selection. The
current consuming DCQL transition discards signature evidence and makes later
routing validation require a retained or reparsed payload copy. A broader safe
state must compose these proofs without overstating verifier trust or transport
authority.

# Goals

- Validate one exact Final authorization and response-routing profile.
- Preserve JAR algorithm/header/client lineage and one valid DCQL query.
- Deserialize request semantics once after bounded duplicate-safe scanning.
- Bound every new retained field and keep diagnostics static.
- Preserve the narrower DCQL-only transition for compatibility.

# Decisions

## The composed state owns routing and DCQL evidence

`VerifiedRequestObject::into_authorization_request(routing_limits,
dcql_limits)` consumes the verified envelope and returns
`ValidatedAuthorizationRequest`. The result owns safe response enums, the
signature algorithm/header, correlated client id, optional wallet nonce,
authorization nonce, HTTPS response URI, and `ValidatedDcqlQuery`. It does not
retain the raw payload after validation.

The public state exposes safe enums/counts normally and verifier-controlled
strings only through explicitly sensitive accessors. It lends the validated
query for evaluation; it does not provide a path back to raw request JSON.

## One bounded semantic parse feeds both validators

`VerifiedRequestObject` transfers private parts to the new module. The new
transition deserializes the already completely scanned and bounded payload into
one object map. Routing validation reads that map and the DCQL facade removes
and validates the query from the same map. A private DCQL constructor accepts
the map/query so the composed path performs no second semantic parse.

The existing `into_dcql_query` remains source- and behavior-compatible: it
uses the same private constructor after its own single deserialize but does not
gain routing requirements.

## Supported routing is deliberately narrow

The first enums have one supported value each: `VpToken` and `DirectPost`.
Exact strings are required. `direct_post` requires exactly one string-valued
`response_uri` and forbids any `redirect_uri`. The URI must be bounded,
absolute HTTPS, have a non-empty host, and contain neither user information nor
a fragment. These checks do not imply network safety or endpoint trust.

The request must have one non-empty bounded nonce composed only of ASCII
unreserved characters. The prior scanner guarantees top-level name uniqueness;
missing, non-string, invalid, and oversized values map to static categories.

## First-error order is contractual

After JAR verification, checks run in this order: response type, response mode,
nonce, destination conflict/presence/syntax, then scope/DCQL extraction and
DCQL semantics. This keeps negative vectors deterministic without exposing
input values.

## Limits compose rather than merge ownership

`AuthorizationRequestValidationLimits` owns positive nonce and response-URI
byte ceilings. Existing `RequestObjectValidationLimits` continues to bound the
payload scanner and `DcqlLimits` continues to own query/evaluation work. The
transition accepts the two later policies explicitly so changing routing does
not silently alter DCQL costs.

# Alternatives rejected

- Extending `ValidatedDcqlQuery` with routing would discard signature lineage
  and give a query engine responsibility for OAuth routing.
- Requiring callers to retain raw payload duplicates sensitive state and lets
  request semantics diverge from evaluated DCQL.
- Changing `into_dcql_query` to require routing would break its documented
  narrow evidence contract.
- Supporting redirect and encrypted modes in one enum now would imply
  unimplemented trust, crypto, browser, and transport behavior.
- Performing HTTP or SSRF resolution would couple runtime and platform policy.

# Rollback

Remove the additive authorization module, limits, error variants, ADR, and
delta spec. The existing JAR and DCQL-only transitions remain unchanged and no
consumer or stored state requires migration.

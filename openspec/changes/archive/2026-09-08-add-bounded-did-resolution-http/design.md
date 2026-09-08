# Context

The transport-neutral DID crate returns one bounded `DidResolutionResult`
from an object-safe asynchronous `DidResolver`. The HTTP binding must project
that result into either the complete W3C envelope or the resolved DID document
without teaching DID Core about HTTP and without accepting unbounded header
input. NeoPRISM supplies a useful adapter shape but not the required failure
and negotiation policy.

# Goals

- Provide one composable, state-closed Axum router over any shared resolver.
- Preserve the SDK's validated DID/options/result types and redacted errors.
- Implement deterministic RFC-shaped negotiation for three representations.
- Map all standard resolver states to explicit W3C statuses and media types.
- Keep server execution and deployment policy outside the library.

# Decisions

## Keep one small public surface

The crate exports media-type constants, the two header resource ceilings and
one constructor:

`did_resolver_http_router(Arc<dyn DidResolver>) -> axum::Router`.

The router owns its state and has fixed route `GET /{did}`. All handlers,
extractor rejection conversion, negotiation values and response helpers are
private. Consumers nest it under their desired prefix, avoiding a caller-
supplied route grammar and construction panic.

## Bound and validate negotiation before selection

Repeated `Accept` fields are joined only after aggregate byte and 32-range
prechecks. A quote-aware private scanner validates range separation and every
quality parameter with the exact RFC 9110 form: `0` or `1`, optionally a dot
and at most three decimal digits, with only zero digits after `1`. Duplicate
`q`, empty ranges and invalid syntax fail closed.

After preflight, private `headers_accept::Accept` selects from server choices
ordered `application/did`, `application/json`, then
`application/did-resolution`. This order makes absent or wildcard input choose
the default document representation while still honoring client quality,
specificity and order. A valid list with no match is 406.

## Keep option and projection semantics explicit

Full-result selection calls the resolver with empty options because
`application/did-resolution` is a transport envelope rather than a DID
document representation. Document selection passes the exact selected media
type through `ResolutionOptions.accept`.

Successful full-result responses serialize the result envelope. Successful
document projections serialize only `didDocument` and use the resolver's
metadata content type after verifying it matches the negotiated representation
ASCII-case-insensitively. Missing document/content type or mismatch becomes a
standard internal-error result.

## Centralize W3C response mapping

Standard error kinds map to 400/404/406/500/501 as required by the binding.
Unknown extension error URIs map to 500. Document metadata
`deactivated: true` maps to 410. All failure and deactivation bodies are the
complete bounded result with `application/did-resolution`. Invalid local
inputs create a standard error-only result; no caller text enters the error.
All responses carry `Vary: Accept`.

Serialization of already validated bounded SDK values should be infallible,
but helpers still convert a failure into a minimal static 500 response rather
than panic. No production `unwrap`, `expect` or dynamic router path is needed.

## Preserve host ownership

Axum is compiled with defaults disabled and `json` only. Tokio runtime,
listener, Hyper protocol selection, tower middleware and deployment controls
are absent from the normal crate graph. Tests add only the minimal executor,
request and service utilities needed to exercise the router in memory.

# Compatibility matrix

| Input/result | Status | Content-Type | Body |
| --- | ---: | --- | --- |
| Missing/wildcard `Accept`, resolver success | 200 | negotiated document type | DID document |
| `application/did-resolution`, resolver success | 200 | `application/did-resolution` | full result |
| Invalid path/DID or malformed/oversized `Accept` | 400 | `application/did-resolution` | standard error result |
| Non-empty query | 400 | `application/did-resolution` | `invalidOptions` result |
| Valid unsupported `Accept` | 406 | `application/did-resolution` | `representationNotSupported` result |
| Resolver not found | 404 | `application/did-resolution` | resolver result |
| Deactivated result | 410 | `application/did-resolution` | resolver result |
| Unsupported method/feature | 501 | `application/did-resolution` | resolver result |
| Invalid document/internal/extension or invalid success projection | 500 | `application/did-resolution` | result or standard internal result |

# Rollback

Revert the focused delivery commit/PR. No existing crate or public type changes,
so rollback removes only the outer adapter and its dependency/inventory edges.

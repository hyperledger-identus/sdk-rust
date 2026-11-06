# Design

## Routing lineage

`VerifierAuthorizationEvidence -> ValidatedAuthorizationRequest` remains the
only routing entry. The state owns a closed destination enum:
`ResponseUri(https)` for direct post or `RedirectUri(absolute)` for
fragment/query/form-post. It never stores both. The trust input exposes the
exact signed destination before it accepts the verifier/key/request binding.

## Response material

One private parameter builder validates DCQL results once and produces ordered
`vp_token` then optional `state`. Mode-specific encoders return inert types:
an exact URI for query/fragment, or target plus form body for form post. Existing
direct-post types retain their API and reuse the same bounded parameter core.
Errors follow the same mode and destination as the request.

URI assembly parses the destination, rejects collisions with generated names,
preserves non-conflicting existing components, and applies one final output
byte bound before constructing the public state. No output type has a generic
`Display` implementation.

## Direct-post acknowledgement

Plain and encrypted direct-post response material can be consumed with caller-
supplied status, content type, cache control, and body. The parser classifies
status and headers before bounded duplicate-safe JSON. Absence of
`redirect_uri` yields a terminal acknowledgement; presence yields an inert
bounded navigation candidate. A unique `response_code` in its query or
fragment is retained as opaque zeroizing text. Unknown JSON members are bounded
and ignored; conflicting query/fragment response codes fail.

## Evidence architecture

Clean-room E2E covers signed JAR, security, verifier authorization, DCQL,
redirect output, and acknowledgement. Negative cases cover mode/destination
cross-products, collisions, duplicates, malformed URI/media/cache/JSON,
oversized values, response-code ambiguity, and diagnostic canaries. Property
tests sweep encodings and exact bounds; fuzz targets acknowledgement parsing
and mode construction; benchmarks compare ordinary and near-limit paths.

# Context

The SDK owns a validated Request URI but must not own a network runtime. It also
owns a reviewed JOSE verifier but not verifier trust or client-prefix policy.
The next transition must preserve those boundaries while preventing an
untrusted response or merely parsed JWT from appearing verified.

# Goals

- Describe exact Final GET/POST retrieval under explicit bounds.
- Bind only successful, correctly typed, bounded compact signed responses.
- Reuse `identus-jose` for algorithm/key-bound signature verification.
- Enforce Final `typ`, client-id equality, and optional wallet nonce.
- Expose only redacted, consuming state transitions.

# Decisions

## Transport is data, not execution

`ReferencedAuthorizationRequest` is consumed into a
`RequestUriRetrievalRequest`. The value exposes an HTTPS endpoint, method,
static Accept value, optional static content type, and an explicitly sensitive
bounded body. GET has no body. POST encodes optional caller-supplied metadata
and nonce in deterministic UTF-8 form order. The SDK never generates the nonce.

The request retains the outer client id and sent nonce privately. Binding a
response consumes it, validates a 2xx status, a bounded exact media type, and a
bounded non-empty body. Five-segment JWE is a distinct unsupported result;
other malformed compact values are invalid.

## Parse and verify are separate consuming states

Response binding produces an `UnverifiedRequestObject` backed by
`UnverifiedCompactJws`. It requires the protected Final `typ`, but its public
surface explicitly labels algorithm/key-reference header evidence untrusted so
a caller can resolve a candidate key. It cannot expose authorization claims.

`verify` consumes that state, a caller-owned `SignatureSuiteRegistry`, and an
algorithm-bound `JwsVerificationKey`. `identus-jose` performs exact algorithm
matching and cryptographic verification. Only after success does OID4VP parse
the payload and enforce client-id and optional wallet-nonce correlation.

## Bounded JSON stays protocol-local

A small iterative/scanner-style parser validates the complete payload under
byte, depth, node, member, and decoded-string limits and extracts only
top-level `client_id` and optional `wallet_nonce`. It rejects duplicates of all
top-level names so later request semantics cannot observe ambiguous input.
Unknown nested authorization members remain intact but are not interpreted.

The verified result owns the exact bounded payload plus correlated fields and
safe algorithm/header-shape evidence. It does not claim audience, freshness,
OAuth validity, DCQL validity, key authorization, verifier trust, or consent.

## Errors remain one redacted catalogue

Retrieval, response, compact, signature, header, JSON, and correlation failures
map to additive `Oid4vpError` variants with stable static messages. No nested
JOSE error rendering or verifier input crosses the public boundary.

# Alternatives rejected

- Performing HTTP in the crate couples TLS/runtime/platform policy.
- Returning raw response bytes lets callers skip media-type and size checks.
- Returning parsed claims before signature verification creates type confusion.
- Treating a caller-supplied key as client-authorized overclaims trust.
- Supporting JWE now expands key management and algorithm surfaces prematurely.
- Reusing private OID4VCI JSON helpers creates sideways protocol/error coupling.

# Rollback

Remove the additive retrieval/JAR modules, dependency edge, ADR, and delta
specifications. Restore the previous ingress documentation. No released or
downstream migration exists.

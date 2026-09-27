# Context

The first OID4VP slice must accept enough authority to locate a Request Object
without confusing an untrusted deep link with a verified request. Existing
generic presentation types intentionally contain no protocol wire data.

# Goals

- Establish a cohesive unpublished `identus-oid4vp` owner crate.
- Classify one Final by-reference invocation under explicit limits.
- Preserve least authority, redacted diagnostics and later extensibility.
- Avoid code sharing that couples OID4VP errors to OID4VCI.

# Decisions

## One closed transport result

`AuthorizationRequestInvocation::parse` accepts the custom-scheme invocation
and currently returns one `ReferencedAuthorizationRequest`. Unsupported inline
or by-value shapes receive a distinct static error. The enum leaves room for
later transports without exposing unbounded maps.

## Decode once, detect duplicates after decoding

The parser scans at most the configured pair count, strictly decodes each name
and value, then rejects any duplicate decoded name before interpretation.
Recognized fields cannot be shadowed by percent-encoding variants. Unknown
bounded fields are dropped; `transaction_data` is explicitly unsupported.

## Retain only least authority

The result owns only the exact decoded `client_id`, HTTPS Request URI and
GET/default versus POST retrieval signal. Values use zeroizing storage and
only explicit sensitive accessors reveal them. `Debug` reports safe lengths
and the method, never contents.

## Validate structure, not trust

The custom scheme is case-insensitive but host/path/query shape is exact,
without userinfo, port or fragment. The Request URI must be absolute HTTPS
with a non-empty host, no userinfo and no fragment; query is allowed. No
network safety, redirect, TLS, signature, `typ`, audience or prefix claim is
made.

## Error ownership

`Oid4vpError` implements the common Identus error contract with stable codes,
static messages and no source data. A focused golden fixture freezes the first
catalogue while allowing additive future catalogues under a later decision.

# Rollback

Delete the unpublished crate and additive governance/spec records. No consumer,
registry, data or compatibility migration is required.

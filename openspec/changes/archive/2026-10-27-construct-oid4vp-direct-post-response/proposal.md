# Construct bounded OID4VP direct-post responses

## Why

`identus-oid4vp` can prove a signed, bounded Final Authorization Request and
its DCQL selection semantics, but callers must still assemble `vp_token` and
`state` by hand. That loses the query cardinality and request-lineage evidence
already established by the SDK and invites mismatched query identifiers,
missing state, malformed JSON, or unbounded form bodies.

## What changes

- Retain an optional bounded `state` value in `ValidatedAuthorizationRequest`.
- Retain the exact DCQL response rules needed to validate query identifiers,
  credential-set alternatives and the `multiple` cardinality flag.
- Add SDK-owned presentation values and query-result inputs that accept only
  the Final string-or-object presentation shapes.
- Consume a validated request and caller-selected results into one bounded,
  deterministic `application/x-www-form-urlencoded` POST envelope.
- Keep response construction synchronous, target-neutral, redaction-safe, and
  independent from HTTP, JOSE response protection, credential formats, and UI.

## Capabilities

### Added capabilities

- `oid4vp-direct-post-response`: bounded construction of the OpenID4VP 1.0
  Final direct-post success response.

### Modified capabilities

- `oid4vp-authorization-request-routing`: retain optional state and the narrow
  DCQL response-shape evidence needed by its consuming successor.

## Non-goals

No HTTP/DNS/TLS/redirect execution, `direct_post.jwt`, JWE, signing, encryption,
presentation generation, credential verification, consent UI, verifier trust,
SIOPv2, Digital Credentials API, HAIP, persistence, or consumer mutation.

## Delivery

Issue #487 owns this additive SDK-only slice under IDR-024. The unpublished
crate remains experimental. A later issue must separately authorize any
protected response mode or transport adapter.

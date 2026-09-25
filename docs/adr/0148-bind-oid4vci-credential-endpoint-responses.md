# ADR 0148: bind OID4VCI Credential Endpoint responses

- **Status:** Accepted; HTTP 202 secret lifetime amended by ADR 0149
- **Date:** 2026-09-25
- **Issue:** [#366](https://github.com/hyperledger-identus/sdk-rust/issues/366)
- **Decision authority:** ADR 0119, ADR 0137, ADR 0147 and issue #366

## Context

`JwtCredentialRequest` owns the exact Credential Endpoint, bearer field,
serialized proofs and proof count produced by the preceding bounded issuance
flow. Separate response parsers already validate immediate success (`200`),
deferred success (`202`) and Credential payload errors (`400`), but a caller
can invoke them without consuming the request and must choose the parser itself.
That permits replay of secret-bearing request state and accidental parsing of a
response body under the wrong status branch.

OpenID4VCI 1.0 Final assigns distinct status and body semantics to these three
branches. RFC 6750 authorization errors such as `401` are not Credential
payload errors. Encrypted success responses use a different media type and are
outside the current unencrypted profile.

## Decision

1. Add a consuming `JwtCredentialRequest` transition that accepts one status,
   effective Content-Type, borrowed body and composed response limits.
2. Copy only the local proof count, then drop the request before parsing remote
   input so its zeroizing bearer field and proof body are erased first. ADR
   0149 later narrows this rule for exact HTTP 202 only by retaining minimal
   continuation authority while still erasing the proof body first.
3. Select exactly one parser by status before inspecting media type or body:
   `200` immediate success, `202` deferred success, and `400` Credential
   payload error. Reject every other status with one static diagnostic.
4. Reuse the existing parsers and their branch-specific media/body errors. Do
   not duplicate JSON parsing or loosen existing limits.
5. Bind every accepted result to the originating proof count. Continue to
   reject an immediate credential count above that exact count.
6. Keep the existing borrowed immediate-only validator behavior-compatible and
   share its implementation through one crate-private helper.
7. Keep Debug and errors redacted. Expose sensitive response values only
   through the already explicit accessors on bounded response cores.
8. Add no dependency, feature, HTTP execution, response provenance, retry,
   polling, trust, validation, storage, encryption or product policy.
9. Defer retaining the authority needed to construct a Deferred Credential
   Request to focused issue
   [#368](https://github.com/hyperledger-identus/sdk-rust/issues/368).

## Consequences

Consumers gain one status-first, one-shot response boundary and cannot reuse
the same `JwtCredentialRequest` after classification. Existing response cores,
limits and compatibility methods remain available. The new result proves only
structural continuity and the request proof-count bound; it does not prove HTTP
origin, issuer authorization, credential validity, trust, freshness, or safe
recovery.

Dropping the request before parsing intentionally avoids extending the bearer
token lifetime on terminal, payload-error and invalid-status paths. ADR 0149
records the explicit different lifetime decision required for exact HTTP 202
continuation.

## Reconsideration and rollback

Reconsider the accepted status set only when the pinned protocol profile adds a
new interoperable branch. Rollback removes the additive composed limits,
outcome/wrapper types and consuming method, then restores the old validator's
small private body without changing its public behavior.

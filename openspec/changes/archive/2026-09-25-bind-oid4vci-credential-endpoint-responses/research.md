# Request-bound Credential Endpoint response research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

Issue #364 constructs `JwtCredentialRequest` from exact correlated
Authorization Code authority. The current
`JwtCredentialRequest::validate_immediate_response` borrows a reusable request
and recognizes only HTTP 200. Existing parsers separately cover immediate
response bodies, deferred response bodies and HTTP 400 Credential payload
errors, but no consuming initial-endpoint classifier composes them.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 8.3, 8.3.1 and 10, retrieved 2026-09-25.
- [RFC 6750](https://www.rfc-editor.org/rfc/rfc6750.html), section 3,
  retrieved 2026-09-25.

Final unencrypted immediate issuance uses HTTP 200, initial deferred issuance
uses HTTP 202 with `transaction_id`, and Credential payload errors use HTTP 400
with JSON. Access-token authorization errors use the distinct RFC 6750
challenge grammar. Encrypted success uses `application/jwt` and is outside the
current unencrypted request profile.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Consuming method on `JwtCredentialRequest` | `adopt-and-consume` | Ownership prevents typed replay and accepts no replacement request authority. |
| Existing immediate, deferred-core and Credential Error parsers | `adopt-and-compose` | They already own the bounded Final body semantics and stable errors. |
| Composed existing limit types | `adopt` | Preserves branch-specific policies without a second resource vocabulary. |
| Closed request-bound outcome states | `adopt` | Preserves proof-count evidence and prevents branch ambiguity. |
| Mutate borrowed immediate method | `retain-for-unpublished-compatibility` | Existing callers remain exact; both 200 paths share one private binder. |
| Generic HTTP/OAuth dependency | `not-adopt` | It cannot encode SDK ownership or Final OID4VCI branch authority without coupling. |

## Compatibility and dependency evidence

The API is additive and unpublished. The borrowed immediate method, existing
body parsers, output shapes, limits and errors remain exact. One invalid-status
diagnostic appends to the non-exhaustive error catalogue. No dependency,
feature, manifest, lockfile, unsafe/native, stored-data or network change is
needed; portable compile remains applicable.

## Security, privacy and maintenance evidence

Status selects the parser before media/body inspection. The request is
consumed; its proof count is copied and its zeroizing Authorization/body are
dropped before untrusted response parsing. Bound states retain only the parsed
redaction-safe core plus proof count. Immediate credentials cannot exceed the
exact proof count. Errors remain fieldless/static and no remote content enters
Debug or Display.

## Rejected or deferred candidates

Body-first inference, 401-as-payload-error and encrypted response guessing are
rejected. HTTP execution/origin, TLS, token validation/storage, proof
generation, retry/recovery, polling, credential verification/storage and
product policy remain downstream. Preserving issuer/token authority into a
Deferred Credential Request is separated to issue #368 after an explicit
secret-lifetime decision.

## Open questions and blockers

None for the bounded initial response classifier.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-credential-endpoint-responses
scripts/factory constraints-ready bind-oid4vci-credential-endpoint-responses
cargo test -p identus-oid4vci --test credential_endpoint_http_response
cargo test -p identus-oid4vci
scripts/factory check
```

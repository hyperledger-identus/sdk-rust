# Deferred Credential payload-error research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`DeferredCredentialRequest` already validates bounded `200` issued and
correlated `202` pending responses. Separately,
`CredentialErrorResponseCore::parse_http_response` validates exact status 400,
bounded `application/json`, the Final section 8.3.1.2 payload shape, specific
known error codes, open extension codes, NQSCHAR descriptions, and static
redacted diagnostics. It deliberately excludes RFC 6750 `invalid_request`.

The missing behavior is endpoint-specific interpretation. The generic core
classifies `invalid_transaction_id` as an extension and does not expose section
9.3's guidance that a wallet should stop polling a transaction after
`credential_request_denied`.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 8.3.1 and 9.3, retrieved 2026-09-25.

Section 9.3 says an invalid Deferred Credential Request uses the section 8.3.1
error response. It adds exact `invalid_transaction_id` when the transaction was
not issued by the respective issuer or was already used. When credentials can
no longer be issued, `credential_request_denied` should be returned and the
wallet should stop requesting that transaction. The non-normative example uses
status 400, `application/json`, and `Cache-Control: no-store`; the inherited
section 8.3.1.2 contract makes status 400 and JSON normative for payload errors,
while Cache-Control remains example-only. Authorization errors remain RFC 6750
responses and are outside this JSON payload-error type.

## Compatibility and dependency evidence

The existing response parser and `CredentialErrorHttpResponseLimits` already
provide the required syntax, bounds, media-type behavior and diagnostics. A
small wrapper can own deferred-specific classification while retaining the
core for exact code and untrusted-description access. No parser, JSON model,
limit type, error variant, dependency, feature, lockfile or target change is
justified.

Extending the existing public closed `CredentialEndpointErrorKind` enum would
be unnecessarily coupled and could break exhaustive downstream matches. A new
deferred-specific enum is additive and keeps generic section 8.3.1 semantics
orthogonal to endpoint-specific lifecycle guidance.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing Credential Error body and HTTP parser | `adopt` | Already implements the inherited Final payload-error contract and limits. |
| Deferred-specific wrapper and classification | `adopt` | Adds section 9.3 semantics without widening the generic closed enum. |
| Request-bound validation method | `adopt` | Keeps successful and error transitions attached to the same bounded request state. |
| Boolean stop-polling guidance | `adopt` | Exposes the exact normative recommendation as data without side effects. |
| Add `InvalidTransactionId` to `CredentialEndpointErrorKind` | `not-adopt` | Couples generic and deferred semantics and may break exhaustive matches. |
| Duplicate the Credential Error parser or limits | `not-adopt` | Creates drift without new protocol behavior. |
| Require Cache-Control input | `not-adopt` | `no-store` appears in a non-normative example, not the inherited payload contract. |
| Treat every error as terminal | `not-adopt` | Final only explicitly directs stop-polling for `credential_request_denied`. |
| RFC 6750 authorization challenge parsing | `defer` | Different wire grammar, status/header inputs and security boundary. |
| Automatic retry/invalidation | `defer` | Belongs to caller-owned lifecycle policy, not structural parsing. |

## Security, privacy and maintenance evidence

The wrapper stores only the already-bounded core and a value-only
classification. It does not copy the request transaction identifier, response
body, media type, status, error code or description. Debug exposes no remote
content beyond the core's existing safe shape. The request method does not
prove server origin or transaction correlation because the error body carries
no transaction identifier.

No network, redirects, DNS, TLS, time, entropy, storage, native or ambient
operation is introduced. Maintenance and portable target posture are
unchanged.

## Rejected or deferred candidates

HTTP execution, Authorization headers, token lifecycle, timers, automatic
retry, terminal mutation, RFC 6750 errors, response encryption, credential
verification/storage, downstream adoption, publication and release remain
excluded.

## Open questions and blockers

None. The method may be called repeatedly because parsing exposes protocol
guidance but does not mutate request or transaction state.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-deferred-error-http-response
scripts/factory constraints-ready bind-oid4vci-deferred-error-http-response
cargo test -p identus-oid4vci
nix flake check
```

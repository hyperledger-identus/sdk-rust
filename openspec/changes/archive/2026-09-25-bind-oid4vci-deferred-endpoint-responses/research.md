# OID4VCI deferred endpoint response research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`RequestBoundDeferredCredentialRequest` owns the exact Deferred Credential
Endpoint, zeroizing Authorization field, zeroizing request body, retained
transaction and originating proof count. `DeferredCredentialRequest` already
has independent borrowed validators for 200/202 success and 400 payload errors.
The success validator correlates 202 transactions but deliberately makes no
proof-count claim for 200. Neither borrowed API consumes authorization.

The initial Credential Endpoint classifier already supplies the desired model:
status chooses one parser, terminal paths erase the request first, immediate
credentials are capped to request proofs, and only deferred state retains the
minimum authority. The deferred endpoint needs the same closed ownership
property with exact transaction correlation on its continuing branch.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 8.3, 9.2 and 9.3, retrieved 2026-09-25.

The Final text assigns HTTP 200 to issued credentials, HTTP 202 to a still
pending transaction whose identifier is the same as the request, and HTTP 400
to the inherited Credential Error Response. The number of credentials may not
exceed the number of keys supplied by the originating proofs. Exact
`credential_request_denied` carries stop-request guidance; lifecycle effects
remain outside parsing.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Keep only the separate borrowed validators | `not-adopt` | They cannot enforce one-shot bearer lifetime or 200 proof-count binding. |
| Add a consuming 200/202/400 classifier | `adopt` | It makes terminal versus continuing authority explicit in the type transition. |
| Reuse existing response cores and parsers | `adopt-and-reuse` | They already enforce bounded syntax, media rules, duplicate policy and static errors. |
| Return the existing request-bound immediate response for 200 | `adopt-and-reuse` | It already carries proof count and redacted access to the parsed issued response. |
| Return the existing request-bound deferred response for 202 | `adopt-and-reuse` | It already consumes into the next authorized request without replacement inputs. |
| Retain bearer authority for 200, 400 or any parse failure | `reject` | Those paths are terminal or invalid and have no justified continuation authority. |
| Accept replacement transaction, endpoint, bearer, issuer or proof count | `reject` | Replacement breaks the exact request lineage. |
| Add transport, timers or automatic polling | `reject` | Those effects require separate ownership, budgets and policy. |

## Compatibility and dependency evidence

The surface is additive and unpublished. Existing borrowed validators, wire
models, response cores, errors and limits remain behavior-compatible. The new
composite limits type contains only existing validated policies. No dependency,
feature, manifest, lockfile, unsafe/native, stored-data or network change is
needed; supported target compilation remains applicable.

## Security, privacy and maintenance evidence

Status is selected before media/body inspection. For 200, 400 and unsupported
statuses, the complete request and bearer allocation are dropped before remote
parsing. For 202, only the exact issuer, endpoint, bearer and proof count move
forward; the serialized request body is dropped first, the retained request
transaction is used only for constant semantic equality, and every mismatch or
parse failure drops all authority. Bound results are non-Clone, redacted and
accept no replacement authority.

## Rejected or deferred candidates

A new wire parser, full metadata/token retention, detached reconstruction,
generic OAuth/HTTP dependencies and polling machinery are rejected. Encrypted
responses, RFC 6750 challenges, transport/origin, token lifecycle, interval and
retry behavior, credential processing and consumers remain deferred.

## Open questions and blockers

None for this bounded response-transition slice.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-deferred-endpoint-responses
scripts/factory constraints-ready bind-oid4vci-deferred-endpoint-responses
cargo test -p identus-oid4vci --test deferred_credential_endpoint_response
cargo test -p identus-oid4vci --all-features
scripts/factory check
```

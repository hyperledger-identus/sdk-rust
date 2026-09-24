# Deferred Credential HTTP response research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The crate already constructs a bounded `DeferredCredentialRequest` and parses
bounded `ImmediateCredentialResponseCore` and `DeferredCredentialResponseCore`
values. The request does not retain its transaction identifier and therefore
cannot enforce the Final response correlation rule. Consumers must also
reimplement status and media-type branching around otherwise reusable parsers.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  section 9.2, retrieved 2026-09-25.

Section 9.2 requires an unencrypted issued response to use status `200` and an
unencrypted pending response to use status `202`. A pending response contains
an `interval` and `transaction_id`, and the returned transaction identifier
must equal the request identifier. Unencrypted responses use
`application/json`. Section 9.3 defines error responses separately, so they are
not collapsed into this success transition.

## Compatibility and dependency evidence

The public change is additive. It reuses the current JSON Content-Type helper,
immediate/deferred response parsers, `zeroize`, and existing request state. No
crate, feature, lockfile, target or dependency change is justified. Retaining a
second zeroizing copy of the decoded transaction identifier is bounded by the
existing deferred response parser and is necessary for correlation after the
request body has been constructed.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Request-owned zeroizing transaction identifier | `adopt` | Preserves the exact correlation input without reparsing the serialized body. |
| Existing immediate/deferred response parsers | `adopt` | They already own strict JSON shape and resource bounds. |
| Existing JSON Content-Type helper | `adopt` | Keeps media-type behavior coherent with immediate response handling. |
| Request-bound issued/pending enum | `adopt` | Makes the two successful Final outcomes exhaustive without adding transport. |
| Reparse the sensitive request body | `not-adopt` | Duplicates parsing and couples validation to serialization details. |
| Return an uncorrelated deferred core | `not-adopt` | Would permit transaction substitution contrary to Final section 9.2. |
| Constant-time transaction comparison | `not-adopt` | This local response-correlation check is not an authentication oracle; ordinary exact equality is clearer and adds no dependency. |
| Infer immediate proof cardinality | `not-adopt` | A deferred request does not retain the originating proof count, so such a claim would be unsound. |
| Parse protocol error responses | `defer` | Section 9.3 has distinct status and lifecycle semantics requiring its own slice. |
| Encrypted responses | `defer` | JWE, metadata and key-agreement policy require separate research and types. |

## Security, privacy and maintenance evidence

The transaction identifier is bearer-adjacent correlating state. It remains
zeroizing, has no new public accessor, and is absent from Debug, Display and
errors. The response body and Content-Type are bounded before parsing. A
mismatch produces a fieldless static error and returns neither identifier.

The validator performs no networking, redirects, DNS, TLS, time, entropy,
storage, native or ambient operation. Maintenance and portable target posture
are unchanged.

## Rejected or deferred candidates

HTTP execution, Authorization headers, token lifecycle, interval scheduling,
automatic retry, terminal invalidation, error response parsing, response
encryption, extensions, credential verification/storage, downstream adoption,
publication and release remain excluded.

## Open questions and blockers

None. The request may validate more than one response because timing, retries
and terminal transaction invalidation remain caller-owned policy.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-deferred-http-response
scripts/factory constraints-ready bind-oid4vci-deferred-http-response
cargo test -p identus-oid4vci
nix flake check
```

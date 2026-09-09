# Token Response authorization-details research

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`TokenResponseCore` already bounds complete JSON bytes, depth, nodes and OAuth
core strings. Its custom duplicate-safe scanner deliberately traverses but
does not interpret `authorization_details`; it retains the exact response and
exposes only a presence bit. The JWT Credential Request constructor therefore
fails closed whenever that member is present.

The next least-authority seam is validation and typed extraction only. A later
issue can consume one returned identifier into a Credential Request without
mixing parsing, authorization policy and request construction here.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  approved 2025-09-16, especially sections 3.3.4 and 6.2.
- [RFC 9396](https://www.rfc-editor.org/rfc/rfc9396), especially the non-empty
  array and authorization-detail object requirements in sections 2 and 7.

The Final profile requires each returned `openid_credential` object to contain
`credential_configuration_id` and a non-empty `credential_identifiers` array.
It requires wallets to ignore unknown fields in those objects. The SDK treats
other bounded authorization-detail `type` values as unrelated extension
entries rather than falsely rejecting a mixed OAuth response.

## Compatibility and dependency evidence

The immediate consumer is the existing unpublished `identus-oid4vci` state
machine. Oxid revision `183664aeca500c25d6d27a22fa402b4d40c649d3`
and Lace ID Portal revision
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` were inspected read-only; neither
contains a reusable Rust Token Response authorization-details model. No donor
source or fixture is copied.

`TokenResponseCore::parse`, `authorization_details_present`, the existing
limits constructor and all wire acceptance remain unchanged. The new
transition is additive. Exact Final examples are independently reconstructed.
Rollback removes only additive types, methods, errors, tests and spec text.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Extend the existing bounded scanner | `adopt` | Preserves duplicate detection, global depth/node budgets, exact integer handling and the current dependency cone. |
| Deserialize through `serde_json::Value` | `not-adopt` | Duplicates are lost and semantic allocations occur before the SDK can apply per-field limits. |
| Adopt an OAuth/RAR crate | `not-adopt` | The required seam is small and profile-specific; available general models do not provide this SDK's bounds, redaction or typed state transition. |
| Validate during `TokenResponseCore::parse` | `not-adopt` | Would change the documented partial-core compatibility and prevent callers from inspecting otherwise valid OAuth core responses. |
| Build Credential Requests in this change | `defer` | Requires identifier selection, offer/metadata correlation and a separate transition contract. |

## Security, privacy and maintenance evidence

The response may contain bearer and refresh tokens plus dataset identifiers.
The retained JSON is already zeroizing and redacted. New identifiers use
zeroizing owned strings and expose borrowed accessors only; public formatting
reports counts. Independent positive limits bound entry count, identifier
count, and decoded string bytes in addition to the existing complete JSON,
depth and node limits. Duplicate member names and duplicate credential
identifiers fail closed.

The work is linear in bounded JSON size plus small bounded duplicate scans. No
network, clock, random source, native code, unsafe code, feature, dependency or
target change occurs. Rust remains 1.98.1. Existing fast and weekly target
gates remain sufficient; no performance threshold is justified for a bounded
parser transition.

## Rejected or deferred candidates

General RAR ownership, Authorization Request construction, Authorization Code
and PKCE execution, token validation, metadata matching, Credential Request
construction, HTTP adapters, downstream adoption, publication and release are
rejected from this change. Request-by-identifier is deferred to a separate
issue after this parsed state is accepted.

## Open questions and blockers

None. This state validates syntax and uniqueness, not authorization or trust.
Cross-entry identifier reuse is rejected because a wallet cannot select an
unambiguous credential dataset from two configurations. Unknown types are
counted but not retained.

## Evidence commands

```text
scripts/factory research-ready add-oid4vci-token-authorization-details
scripts/factory constraints-ready add-oid4vci-token-authorization-details
cargo test -p identus-oid4vci
nix flake check
```

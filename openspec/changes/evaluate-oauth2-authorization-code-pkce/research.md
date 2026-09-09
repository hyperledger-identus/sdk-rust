# OAuth2 authorization-code and PKCE research

Research class: protocol
Research status: draft
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: executable candidate and target evidence pending

## Problem and existing implementation

`identus-oid4vci` has bounded OID4VCI Final metadata and partial token models,
but no authorization-request or authorization-code token-exchange API. The
current implementation therefore needs a reuse decision before adding those
mechanics. Exact `oauth2 5.0.0` is mature and typed, but its URL, HTTP, secret,
clock and response ownership may conflict with cohesive transport-neutral SDK
facades. Current consumer evidence is the authorization-code flow required by
the OID4VCI roadmap; no downstream application integration is claimed.

## Normative sources

- OpenID for Verifiable Credential Issuance 1.0 Final, authorization flow and
  authorization request sections: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html
- RFC 6749 authorization code grant and token request:
  https://www.rfc-editor.org/rfc/rfc6749
- RFC 7636 PKCE S256:
  https://www.rfc-editor.org/rfc/rfc7636
- OAuth 2.0 Security Best Current Practice, RFC 9700:
  https://www.rfc-editor.org/rfc/rfc9700
- oauth2 5.0.0 release/source commit and upgrade notes:
  https://github.com/ramosbugs/oauth2-rs/tree/f3424b4

The exact immutable source revision, packaged checksum, license and provenance,
and current maintenance evidence will be completed from the resolved release
before this record becomes ready.

## Candidate decisions

| Candidate/mechanic | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| oauth2 authorization URL | 5.0.0 / pending exact SHA | `spike` | Typed construction and extension parameters may be reusable; endpoint and scope policy fit is unproved. | Executable mismatch matrix and dependency review complete. |
| oauth2 PKCE S256 | 5.0.0 / pending exact SHA | `spike` | Closed RFC 7636 challenge mechanic should not be reimplemented if caller-owned secret flow and targets fit. | Deterministic RFC vector and diagnostics evidence complete. |
| oauth2 token request | 5.0.0 / pending exact SHA | `spike` | Correct form/auth construction is valuable; HTTP and client-auth policy must remain injected. | Recording-transport proof and OID4VC profile mapping complete. |
| oauth2 token response model | 5.0.0 / pending exact SHA | `spike` | Parser is mature, but chrono/extension/duplicate/resource behavior may conflict with bounded SDK states. | Positive/negative and resource evidence complete. |
| SDK-local implementation | current develop | `retain-local` | Current bounded SDK types remain authoritative until external mechanics are admitted individually. | A focused candidate passes every production gate. |

## Compatibility and dependency evidence

Pending exact minimal features, packaged metadata, MSRV, direct and resolved
dependency cone, public and wire compatibility, facade boundary, host/WASM/iOS/
Android target commands and rollback confirmation. Root runtime and lock graphs
must remain unchanged.

## Security, privacy and maintenance evidence

Pending source scan for unsafe/native/build code, dependency advisories and
licenses, secret Debug/Display/error behavior, caller-owned randomness, input
bounds, endpoint policy, redirects, time, duplicate fields, maintenance and
release posture, and protocol/draft currency.

Pending supply-chain evidence includes the exact lock, `cargo deny`, pinned
advisory audit and registry checksum.

## Rejected or deferred candidates

Production adoption, default HTTP clients, external URL/HTTP/chrono types,
browser launch, redirect listeners, persistence and downstream use are deferred
until the executable evidence and final per-mechanic disposition are complete.

## Open questions and blockers

The exact release source, cone, target behavior and semantic mismatch matrix
remain blockers. No production implementation may begin while this record is
draft.

## Evidence commands

Planned commands include exact locked Cargo metadata/tree/deny/audit, fixture
tests, source scans and Rust 1.98.1 host/WASM/iOS/Android compile checks. Every
unrun check will be recorded truthfully; no result is inferred from a plan.

# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation is assessed at pinned source revision
`a5a54d6fac55912cd79d1f6112311375c96ccac3`.

`identus-oid4vp` owns bounded by-reference ingress, runtime-neutral Request URI
retrieval, signed compact JAR verification, exact client/wallet-nonce
correlation, and bounded DCQL selection. `VerifiedRequestObject` owns the
protected header, accepted algorithm, correlated client id, optional wallet
nonce, and exact bounded payload. Its `into_dcql_query` transition consumes the
payload and deserializes it into a request map before extracting DCQL, so a
caller cannot subsequently prove response routing without retaining or
reparsing a second copy.

The JAR verifier's iterative scanner already validates the complete payload,
rejects duplicate object members, and enforces byte/depth/node/member/string
limits. One later semantic `serde_json` deserialization is therefore safe under
the established payload ceiling. The composed transition can retain the
signature evidence, validate routing from that one request map, and pass the
same map to the existing strict DCQL facade.

## Normative sources

- OpenID for Verifiable Presentations 1.0 Final with current errata, retrieved
  from `https://openid.net/specs/openid-4-verifiable-presentations-1_0.html`.
  Sections 5 and 8 define Authorization Request parameters, nonce processing,
  DCQL selection, response modes, response destinations, and security
  considerations. The publication identifies itself as Final and incorporates
  current errata; its frozen Final publication is
  `https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html`.
- RFC 6749 and RFC 9101 provide the Authorization Request and signed Request
  Object envelope inherited by the existing JAR transition.
- ADRs 0157, 0158, and 0165 fix the existing least-authority ingress, JAR, and
  private DCQL ownership boundaries.

For the supported profile, `client_id`, `response_type`, `response_mode`, and
`nonce` are required. The SDK accepts exact `response_type=vp_token` and exact
`response_mode=direct_post`. That mode requires `response_uri`, forbids a
simultaneous `redirect_uri`, and sends the later response as form data to the
response URI. The nonce is limited to non-empty ASCII unreserved characters
(`ALPHA / DIGIT / "-" / "." / "_" / "~"`). The initial SDK capability
requires an object-valued `dcql_query` and retains the existing rejection of
scope-based query selection.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| SDK-owned composed request typestate | `retain-local` | Preserves exact JAR evidence, deterministic limits/errors, and a future safe response-construction prerequisite. | A focused dependency supplies the same bounded state without leaking transport or framework types. |
| Existing bounded JSON scanner plus one semantic deserialize | `adopt` | The scanner already proves complete structure, uniqueness, and resource bounds; one map avoids duplicate semantic parsing. | The payload ceiling no longer bounds deserializer allocation or cleanup safely. |
| Existing `fluent-uri` | `adopt` | Already owns URI syntax in this crate and can validate an absolute HTTPS destination without a new cone. | Final routing needs URL semantics the existing parser cannot express safely. |
| Existing private `siros-dcql 0.3.0` facade | `adopt` | ADR 0165 already owns strict Final query validation and selection behind SDK types. | Its private seam cannot accept an already parsed request/query without duplicated semantics. |
| General OAuth/OIDC/OID4VC framework | `not-adopt` | A framework adds HTTP/runtime, public-model, policy, and dependency coupling while this transition is a narrow validation state. | A candidate deletes more SDK boundary code than its adapter adds and passes all adoption gates. |
| New URL, JOSE, or JWT dependency | `not-adopt` | Existing `fluent-uri` and `identus-jose` already own these cohesive boundaries. | A required Final behavior is absent from the current owner. |
| `direct_post.jwt`, redirects, SIOPv2, or DC API | `not-adopt` for this slice | Each adds encryption, redirect, subject-identity, origin, or platform policy beyond one response route. | A focused successor issue defines its trust inputs and conformance vectors. |

## Compatibility and dependency evidence

No external package or internal dependency edge is added. The public API is
additive and SDK-owned. Existing `VerifiedRequestObject::into_dcql_query`
behavior remains available; its implementation may share a private
map-to-query constructor, but it does not acquire the new routing preconditions.
The new composed transition requires routing validity and is intentionally a
separate type. `identus-oid4vp` remains version `0.0.0`, unpublished, and
outside release trains. Rust 1.89.0 remains the source floor and Rust 1.98.1
the primary/etalon toolchain.

The exact version and features decision is no-change: workspace manifests,
exact `siros-dcql =0.3.0`, existing `fluent-uri`, and all enabled features stay
as locked at the pinned revision. The direct and resolved dependency cone for
`identus-oid4vp` is unchanged; implementation evidence will compare Cargo
metadata/tree output against the base rather than infer this from prose. Public
and wire compatibility is additive: no accepted wire spelling changes, the
existing query-only API remains, and only callers choosing the new API acquire
the narrow routing requirements. The facade boundary continues to expose only
Identus-owned request, query, limit, error, and result types.

Current target evidence from the immediately preceding #429 delivery records
green Rust 1.89 MSRV plus `wasm32-unknown-unknown`, `aarch64-apple-ios`, and
`aarch64-linux-android` compile gates for this crate. The new exact head must
repeat applicable target checks; inherited evidence is context, not a result
for this change.

## Security, privacy and maintenance evidence

Independent positive byte limits cover the authorization nonce and response
URI in addition to the established payload/DCQL limits. URI validation
requires an absolute HTTPS URI, non-empty host, no user information, and no
fragment. This is syntax and routing evidence only: adapters still own DNS,
private-address policy, TLS, redirects, HTTP execution, and SSRF defense.

Validation order is stable: response type, response mode, nonce, destination,
then DCQL selection/semantics. Earlier JAR validation has already proven JSON
uniqueness and client-id correlation. Unsupported values are distinguished
from absent/malformed values without reflecting them. Retained client id,
nonce, response URI, wallet nonce, and query identifiers are zeroizing or
available only through explicitly sensitive accessors. `Debug`, `Display`,
errors, component metadata, and metrics expose only safe enum/count/length
evidence.

The composed state proves cryptographic provenance, correlated client id,
narrow request semantics, one HTTPS response route, and valid DCQL. It does
not prove key authorization, verifier trust, endpoint safety/reachability,
credential authenticity, consent, or response safety. No donor code or vector
is copied; clean-room vectors are generated with the existing JOSE helpers.

License and provenance remain unchanged: SDK additions are Apache-2.0,
`siros-dcql` remains the pinned BSD-2-Clause private engine, and no donor source
or fixture is copied. Unsafe and native-code evidence is likewise no-change:
the crate forbids authored unsafe Rust, the private engine is pure Rust with no
I/O/native code, and the final dependency/source guards must confirm that
post-implementation. Existing Cargo-deny, RustSec, exact-lock, source,
dependency, unsafe, and portable-build gates remain the supply-chain evidence.
The maintenance, release, and security posture stays experimental and
unpublished; protocol or draft currency is fixed to OpenID4VP 1.0 Final plus
the current errata publication retrieved on the date above.

## Open questions and blockers

None. Exact `vp_token` plus `direct_post` is the first bounded route authorized
by issue #447 and IDR-024. Broader response modes would create materially
different trust and transport outcomes and remain successors rather than
implicit extensibility promises.

Consumer evidence is deliberately read-only and architectural: Oxid and Lace
are named future consumers in IDR-024, while issue #447 authorizes no downstream
mutation or conformance claim. The SDK boundary is therefore derived from the
Final protocol and existing headless ports rather than copied consumer code.
Rollback deletes the additive unpublished state and restores the existing JAR
and DCQL-only APIs without registry, data, or downstream migration.

## Evidence commands

Planning evidence is `scripts/factory doctor`, `scripts/factory validate
validate-oid4vp-final-routing`, `scripts/factory check`, `scripts/factory
research-ready validate-oid4vp-final-routing`, and `scripts/factory
constraints-ready validate-oid4vp-final-routing`. Implementation evidence will
include focused positive/negative/resource/redaction tests, strict Clippy and
rustdoc, error/public API contracts, workspace/factory gates, stable/MSRV, and
applicable compile-only WASM/iOS/Android checks. Hosted CI and runtime network
interop are explicit unrun checks until implementation; no transport or
downstream test is claimed. Exact commands and their outcomes will be recorded
in `verification.md`; unrun checks will remain named rather than implied.

## Rejected or deferred candidates

Broad OAuth/OIDC/OID4VC frameworks, a second URI or JOSE implementation,
redirect routing, encrypted responses, SIOPv2, DC API, HAIP, and HTTP execution
are rejected or deferred because they duplicate an existing owner or expand
runtime, trust, platform, key-management, and dependency boundaries.

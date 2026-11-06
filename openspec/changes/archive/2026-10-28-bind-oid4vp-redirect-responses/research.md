# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

Current implementation: the exact base provides a signed, security-gated,
verifier-authorized direct-post-only routing core.

At exact base `3a2e25fa0699dda29be7ab71c9176aecc3fe2413`, routing accepts only
`direct_post` and `direct_post.jwt`, requires an HTTPS `response_uri`, rejects
`redirect_uri`, and consumes verifier authorization before DCQL routing. Plain
and encrypted constructors share bounded presentation/state semantics but do
not parse the response endpoint acknowledgement.

## Normative sources

- OpenID4VP 1.0 Final, published 2025-07-09, sections 3.1, 5.6, 8.1,
  8.2, 13.3, 14.2, and 14.3:
  https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html.
- OAuth 2.0 Multiple Response Type Encoding Practices 1.0 for `query`,
  `fragment`, and `form_post` response modes.
- RFC 3986, RFC 6749, and RFC 9700 for URI syntax, Authorization Responses,
  and redirect-security guidance.

Final is normative. Foreign SDK behavior is only an oracle and does not
dictate Rust DTOs or errors.

## Protocol or draft currency

OpenID4VP 1.0 Final published 2025-07-09 is pinned. Superseded drafts are not
normative, and later errata require explicit compatibility review.

## Profile decision

| Mode | Decision | SDK output | Effect owner |
| --- | --- | --- | --- |
| `fragment` | adopt | exact bounded redirect URI with encoded parameters in fragment | caller navigation |
| `query` | adopt | exact bounded redirect URI with encoded parameters in query | caller navigation and leakage policy |
| `form_post` | adopt | target plus deterministic bounded form fields | caller HTML/user-agent adapter |
| `direct_post` | retain | current HTTPS POST material | caller HTTP adapter |
| `direct_post.jwt` | retain | current encrypted POST material | caller JWE and HTTP adapters |

All modes consume verifier-authorized request lineage. Redirect modes accept
only a signed `redirect_uri`; the trust adapter receives the exact destination
before routing. `response_uri` and `redirect_uri` are mutually exclusive.

The response endpoint acknowledgement is not an Authorization Request redirect
mode. It consumes a sent direct-post response, accepts exact status 200,
bounded `application/json`, strict `no-store`, and a bounded duplicate-safe
object. Its optional `redirect_uri` is absolute and redacted. A unique
`response_code` query or fragment parameter may be exposed only as opaque data;
the SDK cannot prove the Final recommendation of fresh 128-bit randomness.

## Candidate decisions

| Candidate | Revision/version | Decision | Reason |
| --- | --- | --- | --- |
| existing OID4VP/DCQL/form codec | exact base SHA | `adopt` | Preserves request lineage, ordering, bounds, and errors. |
| `fluent-uri 0.3.2`, `serde_json 1.0.149` | exact Cargo.lock | `adopt` | Existing private engines suffice; no feature/dependency change. |
| general OAuth/OpenID crates | current registry | `not-adopt` | No protocol-specific consumed-state transition or SDK bounds. |
| browser/HTTP frameworks | current ecosystem | `not-adopt` | Effects remain consumer-owned. |
| foreign Identus SDKs | current heads | `oracle` | Behavior evidence only, not normative contracts. |

## Compatibility and dependency evidence

The direct and resolved production cone is inspected with `cargo tree -p
identus-oid4vp --edges normal --depth 2`; it does not grow. The SDK-owned
facade keeps both engines private. Exact versions and features stay locked.
No donor code or fixture is copied; clean-room cases are Apache-2.0.
Cargo deny, source-distribution checks, Nix, and RustSec own license,
provenance, supply-chain, and advisory evidence. Rust 1.89 remains MSRV and
Rust 1.98.1 the etalon. The crate remains unpublished.

The response-mode enum and constructors expand before publication. Wire output
follows Final/OAuth. Rollback removes new variants and successors without
weakening direct-post security. The 2025-07-09 Final is pinned; later errata
are triggers, not silent updates.

## Security, privacy and maintenance evidence

Destinations, encoded output, response body, media/cache headers, JSON work,
member counts, parameter counts, names, values, and optional response code are
independently bounded. Duplicate parameters, mixed destinations, existing
query/fragment collisions, userinfo, malformed escapes, and control bytes fail
closed. Debug, Display, errors, and metrics expose only static categories and
counts/lengths.

The response-mode enum, routing validation, constructors, and errors are an
intentional pre-release source/behavior expansion. Wire output follows Final
and the referenced OAuth specifications. Rollback removes all new variants and
successors before publication without weakening existing direct-post behavior.
Retained text is zeroizing and the exact destination reaches verifier trust
policy before routing. No value authorizes a network or browser effect. No
cryptography, unsafe code, runtime, filesystem, native dependency, dynamic
loading, or background work is introduced. OID4VP maintainers retain ownership.

## Rejected or deferred candidates

Browser navigation, HTML generation, HTTP clients, response-code redemption,
session storage, product policy, and foreign DTO/error facades are deferred or
rejected. A generic OAuth codec is rejected because it would not preserve
SDK-owned consumed evidence and bounds.

## Quality evidence

Pending declaration `oid4vp.redirect-response.quality.v1` will own property,
fuzz, benchmark, and differential obligations. Implementation evidence must
include a clean-room full invocation-to-output E2E, acknowledgement binding,
strict Clippy/rustdoc, all/no-default tests, portable WASM/iOS/Android builds,
factory/Nix gates, and exact-head Claude-or-Codex CLI review.

## Open questions and blockers

None. Navigation policy, URL disclosure policy, HTML rendering, HTTP, response
code redemption, and verifier session storage are explicitly downstream.

## Reconsideration triggers

Reconsider if Final errata or a named profile forbids a mode, if two adopters
need a shared browser adapter, or if interoperability evidence finds a wire
mismatch. An external codec must satisfy MSRV, portable targets, bounds,
dependency cone, license, and SDK-facade rules.

## Evidence commands

Planning uses repository inspection, the pinned Final source, `cargo tree -p
identus-oid4vp --edges normal --depth 2`, `scripts/factory research-ready`,
`constraints-ready`, `check`, and preflight. Tests, portable builds,
property/fuzz/benchmark/differential evidence, Nix, hosted CI, and CLI review
are unrun at planning time.

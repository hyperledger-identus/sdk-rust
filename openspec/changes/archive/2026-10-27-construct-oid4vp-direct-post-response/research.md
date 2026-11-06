# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation owns bounded invocation parsing, runtime-neutral
Request URI retrieval envelopes, JAR verification, strict DCQL evaluation, and
composed `vp_token`/`direct_post` routing. It intentionally stops before
response construction. `ValidatedDcqlQuery` currently retains its private
`siros-dcql 0.3.0` engine and aggregate counts, but not a response-facing list
of query identifiers and `multiple` flags. `ValidatedAuthorizationRequest`
currently drops optional OAuth `state` after validating the same request map.

The response successor therefore belongs inside `identus-oid4vp`: it can
consume proven request lineage and enforce exact DCQL response shape without
exporting the private engine or trusting caller-supplied booleans. A generic
JOSE or OAuth dependency does not solve this protocol-specific transition.

## Normative sources

- OpenID for Verifiable Presentations 1.0 Final, published 2025-07-09, current
  errata text retrieved 2026-10-27:
  https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.
  Sections 8.1 and 8.2 require a JSON-object `vp_token`, query-id keys,
non-empty presentation arrays, exact cardinality when `multiple` is false,
credential-set satisfaction, returned `state`, UTF-8 form encoding, and POST
to the already validated `response_uri`.
- RFC 6749 state and form-encoding rules as incorporated by the Final
  specification: https://www.rfc-editor.org/rfc/rfc6749.
- Repository authority: ADR 0168, ADR 0165, IDR-024, issue #487, and the
  existing `oid4vp-authorization-request-routing` specification.

Normative provenance is the OpenID Foundation publication and the IETF RFC;
both specifications are publicly distributed under their publisher terms.
No source code or fixture is copied, so third-party source-code license intake
is not applicable. Existing dependency licenses remain governed by the locked
workspace supply-chain checks.

The protocol/draft currency is Final plus published errata. A superseding OpenID4VP
revision or erratum changing response shape is a reconsideration trigger, not
an automatic upgrade.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| SDK-owned response transition | current `identus-oid4vp` | `retain-local` | It alone owns validated request/DCQL lineage and can remain runtime-neutral. | A narrow maintained crate accepts SDK evidence types without public-model or runtime coupling. |
| Existing `serde_json` | workspace 1.x lock | `adopt` | Needed for Final JSON values and already in the direct dependency cone. | A no-std or canonical serialization requirement is accepted. |
| Private `siros-dcql` engine | exact 0.3.0 | `conditional-adopt` | Retain for selection; copy only response-shape facts into SDK-owned private metadata. | Upstream exposes a stable bounded response validator matching Final semantics. |
| `openid4vp`/wallet framework crates | versions assessed in repository reuse research | `oracle` | Useful behavior references, but their DTO, transport, format, or policy ownership is broader than this transition. | A candidate declares compatible MSRV/targets and becomes a strict dependency subset of this contract. |
| New JOSE/OAuth framework | not applicable | `not-adopt` | Plain `direct_post` uses neither a protected response nor an OAuth client runtime. | `direct_post.jwt` or another protected profile receives its own accepted issue. |
| Reuse OID4VCI form codec by crate dependency | current workspace | `not-adopt` | A sideways protocol dependency would couple unrelated release surfaces. | A separately governed protocol-neutral form crate has two real consumers. |

No donor becomes a production dependency. No fixture is copied in this slice;
official examples are adapted as attributed clean-room tests.

## Compatibility and dependency evidence

The direct dependency set remains `identus-core`, `identus-jose`,
`fluent-uri`, `serde_json`, `siros-dcql` and `zeroize` as already locked. The
direct and resolved dependency cone does not grow. The public facade uses only
SDK-owned Rust types; the wire output is Final JSON inside UTF-8 form encoding.
The private DCQL engine remains unreachable from the public API.

MSRV remains Rust 1.89 and the primary compiler remains 1.98.1. The design is
synchronous and allocation-bounded, with no native library, executor, clock,
filesystem, network, or target-specific feature. Existing native,
`wasm32-unknown-unknown`, iOS, and Android library lanes remain applicable.
Reachable unsafe code and supply-chain posture are unchanged because no package
is added. Public compatibility is additive on unpublished version `0.0.0`;
wire compatibility is fixed to OpenID4VP 1.0 Final plain `direct_post`.

Rollback removes the additive transition, retained state/rules, limits,
errors, tests, ADR and delta specification before publication. The existing
request/JAR/DCQL APIs continue unchanged except for private retained evidence.

## Security, privacy and maintenance evidence

Limits cover state, query results, presentations, presentation bytes, JSON
depth/nodes, encoded `vp_token`, and final form-body bytes. Cheap count, key,
and cardinality checks precede presentation parsing and serialization. Query
identifiers must come from the validated request. Without `credential_sets`,
every query must be answered. With sets, every required set must have one
fully answered option while optional sets impose no coverage requirement.
`multiple=false` requires exactly one presentation. Duplicate, unknown, empty,
or out-of-policy results fail closed.

Presentation and state bytes are sensitive SDK-owned values with explicit
accessors and redacted diagnostics. The completed envelope exposes its target
and body only through explicitly sensitive accessors and confers no authority
to execute network I/O. Errors remain static categories. The security posture
adds no secret generation, key custody, signing, encryption, logging, global
state, unsafe block, or native code.

Maintenance and release ownership stay with `identus-oid4vp` maintainers.
Property, fuzz, benchmark, official-vector, redaction, target, dependency, and
independent review evidence must be recorded before activation. Claude CLI is
preferred for review; any missing, unauthenticated, quota-limited, or
session-limited Claude invocation immediately falls back to Codex CLI against
the same exact commit and never blocks delivery.

## Rejected or deferred candidates

Broad response-mode enums, generic OAuth clients, HTTP execution, response
JWS/JWE, credential-format validation, consent and presentation generation are
deferred. Accepting an arbitrary prebuilt `vp_token` object is rejected because
it would discard validated query/cardinality lineage. Exporting `serde_json::Value`
as the primary contract is rejected because it weakens the bounded SDK facade.

## Open questions and blockers

No planning blocker remains. Implementation must keep object-valued
presentations bounded without retaining unredacted diagnostics, and must stop
if Final examples cannot be reproduced without broadening scope. The public
representation may own a validated encoded object rather than a general JSON
tree; that is an implementation detail so long as both Final string and object
values are supported.

## Evidence commands

Planning inspection used `rg`/`sed` over `crates/oid4vp`, ADRs 0165/0168,
IDR-024, the OpenSpec baseline, and the pinned official sources. The executed
factory baseline was `./scripts/factory doctor`. Planning gates are
`scripts/factory research-ready construct-oid4vp-direct-post-response`,
`scripts/factory constraints-ready construct-oid4vp-direct-post-response`,
`scripts/factory validate construct-oid4vp-direct-post-response`, and
`scripts/factory preflight construct-oid4vp-direct-post-response --issue 487
--write`.

Unrun at planning time are Rust tests, Clippy, rustdoc, fuzzing, benchmarks,
portable target builds, dependency receipts, exact-head independent CLI review,
hosted CI, and consumer tests. They cannot be claimed until implementation.

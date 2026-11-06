# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation consumes `ValidatedAuthorizationRequest` and
emits one bounded success envelope. It already owns deterministic form encoding,
validated `response_uri`, bounded exact `state`, zeroizing buffers and redacted
diagnostics. Error construction belongs at that same cohesion boundary, but it
must remain a distinct branch because error choice is product/privacy policy and
must not be inferred from an `Oid4vpError`.

## Normative sources

- OpenID for Verifiable Presentations 1.0 Final, published 2025-07-09,
  incorporating errata, retrieved 2026-10-27:
  https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.
  Sections 8.2 and 8.5 define the plain direct-post carrier and clarify the
  protocol vocabulary. Section 15.9 requires error responses to avoid sensitive
  detail and warns that protocol errors can reveal holder data.
- The immutable approved Final text remains available at
  https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html.
- RFC 6749 sections 4.1.2.1/4.2.2.1 and Appendix B define authorization-error
  fields, NQSCHAR grammar, URI-reference syntax, exact state return, and UTF-8
  form encoding: https://www.rfc-editor.org/rfc/rfc6749.
- Repository authority: issue #532, ADR 0179, IDR-024, and the existing
  `oid4vp-direct-post-response` specification.

The pinned vocabulary is the seven RFC authorization-endpoint errors
(`invalid_request`, `unauthorized_client`, `access_denied`,
`unsupported_response_type`, `invalid_scope`, `server_error`, and
`temporarily_unavailable`), the Final clarification `invalid_client`, and the
four Final extension errors (`vp_formats_not_supported`,
`invalid_request_uri_method`, `invalid_transaction_data`, and
`wallet_unavailable`). Arbitrary extension codes are not accepted in this first
construction slice because the SDK can add a governed escape hatch later without
weakening the closed baseline.

No source code or fixture is copied. Official examples are adapted as
attributed clean-room vectors. A superseding OpenID revision or erratum changing
the vocabulary or carrier is a reconsideration trigger, not an automatic
upgrade.

License and provenance are the OpenID Foundation Final publication and the IETF
RFC under their publisher terms; no donor source-code license enters the crate.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| SDK-owned error constructor | current `identus-oid4vp` | `retain-local` | It alone owns validated response target/state lineage and the Final vocabulary. | A narrow maintained crate accepts SDK evidence types without DTO/runtime coupling. |
| Existing direct-post form codec | current in-crate implementation | `adopt` | Same wire grammar, limits and redaction boundary; avoids divergent encoders. | Two crates need a separately governed protocol-neutral encoder. |
| Existing `fluent-uri` | workspace 0.4 lock | `adopt` | Already direct and portable; validates RFC URI-reference syntax without effects. | Dependency posture or URI-reference behavior changes. |
| OID4VCI OAuth response types | current sibling crate | `oracle` | Useful grammar precedent, but a sideways protocol dependency would couple release surfaces and still lacks OID4VP codes. | A governed protocol-neutral OAuth crate has two real consumers. |
| Broad OAuth/OID4VC framework | assessed repository portfolio | `not-adopt` | Adds DTO, transport and policy ownership beyond this construction transition. | A candidate is a strict MSRV/target/dependency subset of this contract. |

## Compatibility and dependency evidence

The exact candidate version and features are the existing `fluent-uri 0.4`
workspace lock with its current feature set and the in-crate form encoder. The
direct and resolved dependency cone does not grow. Supply-chain evidence remains
the locked workspace license/advisory/duplicate-source receipt. MSRV remains Rust
1.89 and primary remains 1.98.1. The synchronous implementation has no executor,
network, filesystem, clock, randomness, native code, unsafe code or target-only
feature, so current native, WASM, iOS and Android target evidence applies.

Public compatibility is additive on unpublished `identus-oid4vp 0.0.0`; wire
compatibility is pinned to OpenID4VP 1.0 Final plain `direct_post` and RFC 6749.
The facade boundary exports only SDK-owned types; `fluent-uri` and the private
encoder are not exposed. Consumer evidence is the headless `oxid` and
`lace-id-portal` adoption intent named by IDR-024, not a claim of current use.
Protocol and draft currency is Final plus its published errata; draft behavior
does not silently alter this contract.

Rollback removes the additive types, constructor, limits, errors and evidence
before publication. Existing invocation, JAR, DCQL and success APIs remain.

## Security, privacy and maintenance evidence

The code is a closed enum rather than caller text. Optional description and URI
are explicit bounded types. Description accepts only non-empty RFC NQSCHAR
ASCII; URI accepts only the RFC-visible character set and a parsed URI-reference.
All retained variable text uses zeroizing ownership and redacted `Debug`.
The final form has an independent positive byte limit and deterministic field
order: `error`, optional `error_description`, optional `error_uri`, then request
`state` when present.

The primitive cannot determine whether returning an error is privacy-safe.
Callers must obtain required user interaction/consent and avoid holder or
credential details. No internal `Oid4vpError` conversion is supplied. This
prevents accidental exposure of implementation diagnostics and preserves a
deliberate policy boundary.

Maintenance stays with `identus-oid4vp`. Official/negative/boundary vectors,
property tests, fuzzing, benchmarks, portable targets, dependency evidence,
redaction tests and exact-head independent CLI review are required. Claude CLI
is preferred; availability, authentication, quota or session failure falls back
immediately to Codex CLI and does not block delivery.

The maintenance, release and security posture remains owned by the OID4VP crate
maintainers and guarded by OpenSpec, the platform ledger and protected CI.

## Rejected or deferred candidates

Free-form extension codes, automatic mapping from SDK errors, localized/end-user
messages, HTTP execution, encrypted responses, and Digital Credentials API
errors are deferred. Omitting optional developer fields entirely is rejected
because RFC 6749 defines them and the Final example uses a description; bounded
explicit types preserve interoperability without claiming display safety.

## Open questions and blockers

No planning blocker remains. Implementation must reuse one form codec without
making success and error fields co-exist, and must preserve exact state after
the request is consumed.

## Evidence commands

Planning inspection covers the predecessor OpenSpec/ADR, current OID4VP and
OID4VCI OAuth code, official Final/errata text, RFC 6749 and IDR-024. Planning
gates are `scripts/factory research-ready`, `constraints-ready`, `validate`, and
`preflight ... --issue 532 --write`. Rust, fuzz, benchmark, portable, dependency,
independent-review and hosted evidence remain unrun until implementation.

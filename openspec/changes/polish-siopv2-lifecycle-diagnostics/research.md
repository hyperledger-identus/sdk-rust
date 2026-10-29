# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

The current implementation of `SiopFlow` retains only the event identity
belonging to its current state.
Replaying that event while the same state is current is idempotent; reusing a
different identity conflicts; and replaying an earlier transition after the
flow advances is an invalid transition. This is safe and bounded, but the
canonical text currently says only that duplicate events are idempotent.

The request path maps both client binding and retrieved wallet-nonce correlation
to `ClientBindingMismatch`. The response path maps every unrecognized JOSE
failure to `ResponseTooLarge`, even when encoding or invariant construction is
the failure. The compact request is parsed twice because the JOSE parser returns
an owned compact string, payload, and signature whose buffers are not zeroized
on drop. The public unverified state instead retains one zeroized compact value
and only the bounded public protected-header evidence until verification.

## Normative sources

- Self-Issued OpenID Provider v2 draft 13, dated 2023-11-28, defines the pinned
  signed-request and `direct_post` profile and requires replay mitigations for
  short-lived requests: https://openid.net/specs/openid-connect-self-issued-v2-1_0-13.html.
- RFC 3986 defines URI components, user information, fragments, and percent
  encoding used by the profile's HTTPS and form boundaries:
  https://www.rfc-editor.org/rfc/rfc3986.html.
- OAuth 2.0 Appendix B defines UTF-8 form encoding for
  `application/x-www-form-urlencoded`:
  https://www.rfc-editor.org/rfc/rfc6749.html#appendix-B.

The pinned profile remains SIOPv2 draft 13. No later draft, product fixture, or
foreign crate becomes normative through this maintenance slice.

License and provenance remain the OpenID Foundation draft and IETF RFCs under
their publisher terms plus Apache-2.0 repository code; no donor implementation
or fixture enters the crate.

## Candidate decisions

| Candidate | Exact version/features or revision | Decision | Reason |
| --- | --- | --- | --- |
| Current-state lifecycle receipts | repository revision `b9c2be2ed7877172d8461266a6522ff4a9b72187` | `retain-local` | Finite, deterministic, effect-free, and consistent with caller-owned durable replay policy. |
| Retained in-core transition history | new state/history collection | `not-adopt` | Adds storage/retention policy, sensitive response lifetime, and unbounded workflow coupling to a pure protocol core. |
| Protocol-private URI/form helpers | current `request.rs` and `response.rs` functions | `retain-local` | Semantics and limits belong to each protocol; small duplication is safer than a public bypass seam. |
| New public shared helper crate | generic HTTPS/form API | `not-adopt` | No cohesive typed owner or common invariant contract exists across protocols. |
| Ephemeral owned JOSE parse plus zeroized compact | current two-pass parse | `retain-local` | Avoids retaining a second non-zeroizing compact/payload/signature copy and preserves explicit verification state. |

## Compatibility and dependency evidence

No dependency, feature, MSRV, unsafe code, native library, wire field, request
profile, or target changes. The public non-exhaustive `Siopv2Error` gains two
stable categories and previously coarse failures become more accurate. This is
a consumer-visible pre-release diagnostic refinement recorded against the
existing capability ledger entry. Existing successful flows and form/JWS bytes
remain unchanged.

Rust 1.89 remains MSRV and Rust 1.98.1 remains primary. Native,
`wasm32-unknown-unknown`, iOS, and Android library targets remain required. The
normal dependency cone stays `identus-openid4vc`, `identus-jose`, `identus-did`,
`fluent-uri`, `serde`, `serde_json`, `sha2`, and `zeroize` under workspace pins.
The direct and resolved dependency cone therefore does not grow. Existing
supply-chain license, advisory, duplicate-source, and lockfile evidence remains
authoritative. The facade boundary exports only SDK-owned SIOPv2 types and does
not expose `fluent-uri`, JOSE errors, or serde DTOs.

Rollback before publication removes the two categories and documentation as one
unreleased unit. It does not require a wire, persistence, or migration rollback.

## Security, privacy and maintenance evidence

All categories and `Debug` output remain static and redacted. The SDK stores no
event history, transport result, identifier, token, or caller policy beyond the
current pure state. Resource limits and required-before-optional validation
ordering remain unchanged. `OutcomeUnknown` remains terminal and cannot be
approved, delivered, or automatically retried.

Maintenance ownership stays with SDK-Rust OpenID maintainers. Durable event
deduplication, retention, audit trails, distributed consistency, and delivery
retry policy remain adopter responsibilities.

## Rejected or deferred candidates

A generic OpenID utility crate, public URI validator, public form encoder,
long-lived duplicate parsed-JWS buffers, event-history collection, and product
replay store are rejected for this slice. Reconsider a
shared helper only when at least two protocols share the same typed invariants,
bounds, error ownership, and a non-bypass public contract.

## Open questions and blockers

No planning blocker remains. Cross-process or durable lifecycle idempotency is
intentionally outside this core and requires a separately specified caller port
if a generic adopter need is later demonstrated.

## Reconsideration triggers

Reconsider state-local idempotency only if a normative SIOPv2 profile requires
the protocol engine itself to retain cross-transition receipts. Reconsider the
two-pass parse if `identus-jose` later offers an owned parsed state whose compact,
payload, and signature buffers are zeroized and which preserves finite bounds
and explicit unverified/verified lineage without widening the public facade.

## Evidence commands

Planning evidence is `scripts/factory research-ready
polish-siopv2-lifecycle-diagnostics`, `scripts/factory constraints-ready
polish-siopv2-lifecycle-diagnostics`, strict validation, and issue-bound
preflight. Implementation evidence requires focused/all-feature/no-default
tests, SIOPv2 conformance, property, E2E, fuzz and benchmark evidence, Clippy,
rustdoc, portable targets, factory checks, and an exact-head Claude-or-Codex CLI
review. Implementation commands and hosted CI are unrun at planning time.

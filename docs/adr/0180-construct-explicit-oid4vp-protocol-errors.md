# ADR 0180: Construct explicit bounded OID4VP protocol-error responses

- **Status:** Accepted under the IDR-024 roadmap mandate
- **Date:** 2026-10-27
- **Decision authority:** issue #532, OpenID4VP 1.0 Final, RFC 6749, ADR 0179

## Context

`identus-oid4vp` can consume a validated request and caller-selected
presentations to build a bounded plain `direct_post` success response. A wallet
also needs the mutually exclusive Authorization Error Response described by
OpenID4VP Final section 8.5 and RFC 6749. If callers assemble it themselves,
they can detach state or target lineage, misspell protocol codes, accept invalid
developer fields, grow output without limits, or leak internal and holder data.

Protocol errors are not equivalent to SDK errors. Choosing to send one is a
wallet policy decision with privacy and End-User interaction consequences.
OpenID4VP section 15.9 specifically warns against detailed contextual errors and
against returning protocol errors without suitable interaction or consent.

The workspace already contains an OID4VCI authorization-response parser that
is useful as an RFC grammar oracle. Importing it would create a sideways
protocol dependency and would not supply the OID4VP-specific vocabulary or
validated request lineage.

## Decision

1. Implement error-response construction inside `identus-oid4vp`, consuming
   `ValidatedAuthorizationRequest` to preserve the validated response URI and
   exact optional state.
2. Expose a closed `DirectPostErrorCode` enum for the seven RFC authorization
   errors, Final's `invalid_client` clarification, and the four Final extension
   errors. Do not accept arbitrary strings in this first slice.
3. Support optional bounded `error_description` and `error_uri` through
   redaction-safe owned types. Enforce RFC NQSCHAR and URI-reference grammar,
   but make no truth, display, localization, provenance or dereference claim.
4. Reuse the private deterministic form encoder. Order fields as `error`,
   optional description, optional URI, then exact request state and enforce an
   independent final-body limit before allocation.
5. Return an inert envelope with explicit sensitive accessors and no HTTP or
   network authority. Keep success and error construction mutually exclusive.
6. Provide no `From<Oid4vpError>` or equivalent automatic mapping. The caller
   owns error selection, End-User interaction, consent and privacy policy.
7. Add no dependency. Treat the OID4VCI implementation as an oracle until a
   separately governed protocol-neutral OAuth component has two real consumers.
8. Require official/negative/boundary, property, fuzz, benchmark, portable,
   dependency, E2E, redaction and exact-head independent review evidence.
   Fall back immediately from unavailable Claude CLI to Codex CLI.

## Consequences

Headless wallets get an interoperable, bounded Final error branch without
pulling transport or product policy into the protocol crate. Callers must make
an explicit privacy decision and cannot accidentally serialize SDK diagnostics.
The public surface grows additively while the resolved dependency and target
cones remain unchanged.

Closed error codes intentionally postpone arbitrary extension support. A later
governed enum variant or explicit extension type can widen the vocabulary
without weakening this baseline.

## Alternatives rejected

- **Map every SDK error to `invalid_request`:** conflates internal and protocol
  semantics and risks disclosure without policy review.
- **Accept arbitrary error strings:** weakens spelling and conformance evidence
  before an extension consumer exists.
- **Omit optional RFC fields:** loses a standard interoperability surface used
  by the Final example.
- **Depend on `identus-oid4vci`:** violates protocol cohesion and dependency
  direction for a small grammar overlap.
- **Add a generic OAuth crate now:** premature abstraction without an accepted
  second consumer contract.
- **Deliver HTTP or `direct_post.jwt` together:** materially expands effects,
  key ownership and threat model.

## Verification and rollback

Verification follows the issue #532 OpenSpec change and must cover every code,
the official form example, exact state, invalid/oversized optional values,
redaction, deterministic order, overflow boundaries, mutually exclusive success
and error E2E paths, property/fuzz/benchmark evidence, portable targets,
dependency receipts, independent CLI review and protected CI.

Before publication, rollback removes the additive types, constructor, limits,
errors, evidence and knowledge records. There is no persisted-state migration.

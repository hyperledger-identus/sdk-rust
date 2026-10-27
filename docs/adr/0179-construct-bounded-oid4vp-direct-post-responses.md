# ADR 0179: construct bounded OID4VP direct-post responses in-tree

- **Status:** Accepted under the IDR-024 roadmap mandate
- **Date:** 2026-10-27
- **Issue:** [#487](https://github.com/hyperledger-identus/sdk-rust/issues/487)
- **Decision authority:** issue #487 under ADR 0004 standing authority
- **Extends:** ADRs 0165 and 0168

## Context

The SDK proves signed request lineage, direct-post routing and bounded DCQL
selection, but it does not construct the corresponding Authorization Response.
Leaving final assembly to callers would duplicate Final rules and allow query
identifiers, cardinality, destination and state to diverge from validated
request evidence.

Plain `direct_post` is a small protocol-specific serialization transition. It
does not need a JOSE implementation, OAuth client runtime, credential-format
engine, HTTP client, or product policy framework.

## Decision

1. Implement the transition inside `identus-oid4vp` with SDK-owned Rust types
   and no new dependency.
2. Consume `ValidatedAuthorizationRequest` so destination, optional state and
   DCQL response rules cannot be independently substituted.
3. Retain bounded optional state, ordered private query/cardinality rules, and
   the required/optional credential-set option matrix during the existing
   request/DCQL validation pass.
4. Accept only bounded presentation strings or encoded JSON objects and
   bounded query-result collections.
5. Validate exact identifiers, duplicates, absent-set all-query coverage,
   required-set alternatives, non-empty arrays and `multiple` cardinality
   before serialization.
6. Emit deterministic `application/x-www-form-urlencoded` with a Final
   JSON-object `vp_token` and exact state when present.
7. Return an inert, redaction-safe envelope. Callers own HTTP and every network
   security policy.
8. Defer protocol-error responses, `direct_post.jwt`, JWE, SIOPv2, DC API,
   HAIP, presentation generation, credential verification, trust, consent,
   replay and consumer adapters.
9. Prefer Claude CLI for exact-head review, but immediately use Codex CLI on
   any Claude availability, authentication, quota, or session-limit failure.

## Consequences

Headless consumers receive a small portable response boundary whose wire shape
is tied to already-proven request evidence. The dependency and target cones do
not grow. Retaining state and query rules adds bounded private memory to the
validated request; diagnostics and explicit accessors preserve least authority.

The crate remains unpublished and experimental. Any protected response mode
requires a separate ADR because it introduces materially different key,
algorithm, encryption and interoperability obligations.

## Alternatives rejected

- **Caller-built `vp_token`:** loses DCQL and request-lineage guarantees.
- **Broad OAuth/OID4VC framework:** imports unrelated models, runtime and policy.
- **A sideways dependency on OID4VCI form code:** couples independent protocol
  packages for a small internal encoder.
- **A new shared form crate now:** premature with no separately governed second
  consumer.
- **Add `direct_post.jwt` simultaneously:** expands the threat model and key
  ownership beyond this bounded slice.

## Verification and rollback

Official, negative, limit, cardinality, optionality, encoding and redaction
vectors are required with property, fuzz, benchmark, dependency, MSRV,
portable-target, independent CLI and protected-CI evidence. Rollback deletes
the additive response transition and private retained metadata before release;
no persisted state or consumer migration is involved.

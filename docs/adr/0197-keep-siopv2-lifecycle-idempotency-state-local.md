# ADR 0197: keep SIOPv2 lifecycle idempotency state-local

- **Status:** Accepted for issue #543 implementation
- **Date:** 2026-10-29
- **Decision authority:** issue #543 under ADRs 0003, 0004, and 0182
- **Applies to:** `identus-siopv2` lifecycle, diagnostics, and private helpers

## Context

The initial SIOPv2 core stores one bounded event identity in each resulting
lifecycle state. The specification says duplicate events are idempotent but
does not define whether the core must retain identities after later transitions.
Adding such history would introduce retention, persistence, and sensitive-state
lifetime policy into an otherwise pure bounded protocol core.

The same review found two coarse diagnostics, repeated private string checks,
small URI/form duplication across protocols, and deliberate reparsing of a
compact JWS whose owned parsed representation retains non-zeroizing compact,
payload, and signature buffers.

## Decision

1. Idempotency is local to the current lifecycle state. Repeating its event
   identity is idempotent; a conflicting identity fails closed; and replay of a
   superseded transition is invalid.
2. Durable and cross-transition event history, audit, distributed consistency,
   and retry policy remain caller-owned. The core adds no history collection or
   persistence port.
3. `OutcomeUnknown` remains terminal and never authorizes automatic retry.
4. Wallet-nonce correlation and response encoding receive distinct stable,
   redacted error categories. Explicit JOSE size failures remain size errors.
5. URI and form helpers remain private to their protocols. Small duplication is
   preferable to a public generic seam that does not own common typed invariants.
6. The compact request continues to be parsed twice. The public unverified state
   retains one zeroized compact value and bounded public header evidence rather
   than extending the lifetime of duplicate non-zeroizing parsed buffers.
7. Repeated string validation may use one private predicate only if the existing
   bounds and required-before-optional validation order remain unchanged.

## Consequences

Lifecycle storage stays finite, runtime-neutral, and easy to port. Adopters that
need durable idempotency must persist their own bounded event receipts, which is
already part of caller-owned product policy. Error recovery becomes more precise
without exposing sensitive values or dependency diagnostics. Successful API and
wire behavior, dependencies, MSRV, portable targets, and the pinned draft-13
profile remain unchanged.

The crate deliberately retains a small amount of private duplication and one
extra bounded JWS parse. These costs are explicit and preferable to broad public
abstractions or longer-lived duplicate sensitive buffers.

## Alternatives rejected

An in-core event history couples retention and sensitive-state lifetime to the
protocol. A generic OpenID helper crate lacks one shared invariant/error owner.
Retaining the current parsed JWS extends the lifetime of duplicate non-zeroizing
compact, payload, and signature buffers. Returning raw JOSE errors would leak
dependency policy and make the SDK contract unstable.

## Verification and reconsideration

Verification requires state-local replay/conflict/state-preservation tests,
precise redacted error tests, unchanged conformance bytes, E2E coverage, property
and fuzz evidence, benchmark direction, portable targets, factory gates, and
independent exact-head CLI review. Reconsider only if a normative profile
requires engine-owned history or a safe owned JOSE type/common typed helper
proves a cohesive contract across multiple protocols.

# ADR 0030: model truthful presentation protocol lifecycle state

- **Status:** Accepted for implementation
- **Date:** 2026-09-05
- **Decision authority:** standing product mandate and `IDR-008d` issue #85
- **Normative evidence:** format-neutral SDK contract; Oxid conceptual state
  evidence at `bfe3b481568dc738f0732c2b27548fab8721fd95`
- **Related work:** issues #20, #79, #81, #83 and #85

## Context

The SDK can now describe and validate presentation inputs, selections and
outputs, but not progress or terminality. Oxid already needs cancellation,
timeout, refusal and failure state. Directly porting its enum would conflate
product consent, proof generation, artifact readiness, delivery and an
ambiguous notion of success.

A headless SDK also needs to be honest when cancellation races irreversible
work. Treating a cancellation request as confirmed rollback can lead wallets
to retry a presentation that was already delivered.

## Decision

1. Separate six active lifecycle phases from five terminal outcomes and wrap
   them in one `PresentationProtocolState`.
2. Use `awaiting_authorization` as an external prerequisite, not a claim that
   the SDK captures or decides consent.
3. Split generation, ready output and delivery so cancellation semantics match
   actual side-effect boundaries.
4. Define one conservative transition table. Reject self-transitions,
   backward movement, phase skips and every transition from terminal state.
5. Permit `cancellation_requested` to become either `cancelled` or `completed`
   because irreversible completion may win the race.
6. Define `completed` as protocol-adapter terminality only. Keep verifier
   acceptance, proof validity, credential trust and persistence evidence
   separate.
7. Expose strict stable spellings and parsing without serde or a wire-format
   promise.
8. Keep the state allocation-free and free of identifiers, timestamps, error
   detail, evidence and external effects.

## Consequences

- OID4VP, Midnight and future adapters can share lifecycle truth without
  sharing protocol wire data or product policy.
- Storage adapters can later persist an explicit vocabulary while owning
  concurrency, versioning and migration.
- Protocol-specific substates remain adapter-owned.
- Applications that need consent, acknowledgement or audit evidence must store
  those facts separately rather than infer them from generic state.

## Deferred work

- protocol-specific state mapping and engines;
- durable state repositories, compare-and-swap and migration;
- consent/authorization, retry, timeout clocks and receipt policy;
- downstream adoption, FFI and publication.

## Rollback

Revert issue #85's implementation PR. The additive experimental values own no
state and perform no external effects.

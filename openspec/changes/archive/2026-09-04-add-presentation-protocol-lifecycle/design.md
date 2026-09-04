# Design: presentation protocol lifecycle state

## Context

Issue #85 completes the type-level `IDR-008` outcome from
`develop@8fb533562d5b006151372e214a38ef8a7e3fa5bd`. The first three slices
established request/candidate semantics, validated disclosure plans, opaque
generated artifacts and value-free receipt inputs. None can express whether a
coordinator is still preparing, generating, handing off, cancelling or
terminal.

Oxid provides Apache-2.0 conceptual evidence through
`CredentialPresentationState`, including the important race where a
cancellation request may arrive after irreversible work. Its
`AwaitingConsent`, `Presenting` and `Succeeded` names combine product policy,
several technical phases and an ambiguous success claim. Copying that enum
would therefore preserve coupling rather than crystallize a reusable SDK
contract.

## Provenance and isolation

No donor code or fixture is copied. Exact revisions and repository states are
recorded in issue #85. Oxid, midnight-identity, Lace ID Portal, NeoPRISM and
Apollo remain read-only.

## Decisions

### D1 — Separate active phases from terminal outcomes

`PresentationLifecyclePhase` contains `requested`, `awaiting_authorization`,
`generating`, `ready`, `delivering`, `generation_cancellation_requested` and
`delivery_cancellation_requested`.
`PresentationTerminalOutcome` contains `completed`, `refused`, `cancelled`,
`expired` and `failed`. `PresentationProtocolState` wraps exactly one phase or
outcome and makes terminality explicit.

`awaiting_authorization` describes an external prerequisite only. The SDK does
not say who authorizes, how consent is captured or whether authorization is a
user, agent, custody or enterprise-policy decision. `completed` says only that
the selected protocol adapter reports terminal completion; it does not mean a
proof is valid, a verifier accepted it or a credential is trusted.

### D2 — Keep preparation coarse and output boundaries precise

`requested` covers protocol parsing, candidate discovery and disclosure-plan
preparation owned by outer layers. Those activities differ by protocol and
product and do not justify persistent generic substates. `generating`, `ready`
and `delivering` are distinct because proof work, bounded generated artifacts
and an irreversible handoff have materially different cancellation behavior.

### D3 — Enforce one conservative transition table

The allowed edges are:

| From | Allowed destinations |
| --- | --- |
| requested | awaiting_authorization, refused, cancelled, expired, failed |
| awaiting_authorization | generating, refused, cancelled, expired, failed |
| generating | ready, generation_cancellation_requested, cancelled, expired, failed |
| ready | delivering, cancelled, expired, failed |
| delivering | delivery_cancellation_requested, completed, cancelled, expired, failed |
| generation_cancellation_requested | cancelled, expired, failed |
| delivery_cancellation_requested | completed, cancelled, expired, failed |
| any terminal outcome | none |

The guard rejects self-transitions, backward transitions and phase skips. A
caller that receives an idempotent replay may compare states before requesting
a transition. Refusal is restricted to pre-generation phases. Cancellation
origin remains explicit: generation cancellation cannot become completion,
while completion from `delivery_cancellation_requested` is permitted because
cancellation is a request, not a rollback guarantee and irreversible delivery
may have won the race.

### D4 — Stable text is an adapter seam, not a wire format

Each phase, outcome and state exposes an exact lowercase snake-case spelling
and implements strict `FromStr`. Unknown, padded or differently cased input is
rejected. No serde, schema, protocol message or compatibility promise is added.
Storage and wire adapters may explicitly map the vocabulary and own versioning
or migration.

### D5 — State is data-free and errors stay static

Lifecycle values contain no request, verifier, credential, query, artifact,
timestamp, transport identifier, error detail or audit context. Invalid phase,
outcome, state and transition failures use zero-data `PresentationError`
variants mapped to stable static `presentation.*` invalid-input errors.

### D6 — Constant-time validation is measured, not thresholded

Transition validation is a match over two copyable values with no allocation.
An ignored release diagnostic cycles through the complete state-pair matrix and
reports throughput without encoding a machine-dependent correctness threshold.

## Risks and trade-offs

- A conservative table may require an adapter to represent protocol-specific
  substates outside the SDK. This is preferable to freezing one protocol's
  state machine in the core.
- Strict parsing rejects future states until an adapter and SDK upgrade agree.
  The API is experimental and additive enum evolution remains a pre-1.0 design
  concern.
- `completed` is deliberately weak. Applications must store separate delivery,
  acknowledgement, verifier, proof and trust evidence when those distinctions
  matter.
- Two cancellation phases add vocabulary, but prevent an origin-free state
  from fabricating completion before delivery begins.
- The state guard does not provide compare-and-swap, durability or concurrency
  control; storage ports and application services own those responsibilities.

## Migration and rollback

The API is additive and unreleased. There is no persisted or wire migration.
A focused revert removes it. OID4VP/Midnight mappings, storage ports,
downstream adoption and publication remain separate issues.

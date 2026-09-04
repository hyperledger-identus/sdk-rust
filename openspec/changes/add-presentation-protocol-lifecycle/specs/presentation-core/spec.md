# Presentation protocol lifecycle delta specification

## ADDED Requirements

### Requirement: Protocol state separates active phases and terminal outcomes

The SDK SHALL provide a `PresentationLifecyclePhase` vocabulary containing
`requested`, `awaiting_authorization`, `generating`, `ready`, `delivering` and
`cancellation_requested`, and a distinct `PresentationTerminalOutcome`
vocabulary containing `completed`, `refused`, `cancelled`, `expired` and
`failed`. A `PresentationProtocolState` SHALL contain exactly one active phase
or terminal outcome and SHALL report terminality without allocation.

`awaiting_authorization` SHALL express only an unsatisfied external
prerequisite. `completed` SHALL express only that the protocol adapter reports
terminal completion; neither value SHALL assert user consent, proof validity,
verifier acceptance, credential trust or successful persistence.

#### Scenario: unrelated adapters share truthful progress

- **WHEN** OID4VP-, Midnight- and dummy-protocol coordinators prepare,
  authorize, generate, expose and deliver presentations
- **THEN** each SHALL use the same active/terminal vocabulary while retaining
  its protocol-specific substate and evidence outside the generic core

#### Scenario: completion does not become verifier evidence

- **WHEN** an adapter reaches `completed`
- **THEN** the state SHALL make no claim about proof verification, verifier
  acknowledgement, credential trust or receipt persistence

### Requirement: Presentation lifecycle transitions are conservative and exact

The SDK SHALL accept only these directed state transitions:

- `requested` to `awaiting_authorization`, `refused`, `cancelled`, `expired`
  or `failed`;
- `awaiting_authorization` to `generating`, `refused`, `cancelled`, `expired`
  or `failed`;
- `generating` to `ready`, `cancellation_requested`, `cancelled`, `expired` or
  `failed`;
- `ready` to `delivering`, `cancelled`, `expired` or `failed`;
- `delivering` to `cancellation_requested`, `completed`, `cancelled`,
  `expired` or `failed`; and
- `cancellation_requested` to `completed`, `cancelled`, `expired` or `failed`.

Every other pair, including self-transition, backward progress, a skipped
generation/delivery boundary and any transition from a terminal outcome,
SHALL fail with one static typed error.

#### Scenario: irreversible completion wins a cancellation race

- **WHEN** cancellation is requested while delivery may already be
  irreversible and the adapter subsequently observes terminal completion
- **THEN** `cancellation_requested` SHALL be allowed to become `completed`
  rather than fabricate rollback

#### Scenario: terminal truth cannot be rewritten

- **WHEN** a caller attempts to transition a completed, refused, cancelled,
  expired or failed state
- **THEN** the guard SHALL reject the attempt without changing either value

### Requirement: Lifecycle spellings are stable, strict and dependency-free

Every phase, outcome and composed state SHALL expose its exact lowercase
snake-case spelling and SHALL round-trip through strict `FromStr` parsing.
Unknown, padded or differently cased spellings SHALL return static redacted
invalid-input errors. The spellings SHALL be an adapter mapping seam, not a
serde schema or protocol wire compatibility claim.

#### Scenario: storage and protocol adapters map explicitly

- **WHEN** an adapter maps every accepted lifecycle spelling to its own
  versioned representation
- **THEN** the generic values SHALL round-trip exactly without adding serde,
  storage or protocol dependencies to the presentation crate

#### Scenario: unrecognized state fails closed

- **WHEN** an adapter supplies an unknown, padded or differently cased phase,
  outcome or state
- **THEN** parsing SHALL reject it without retaining or rendering the input

### Requirement: Lifecycle diagnostics and execution remain narrow

Lifecycle values SHALL contain no verifier, purpose, challenge, query,
credential, claim, artifact, transport identifier, timestamp, error detail or
audit evidence. Invalid phase, outcome, state and transition failures SHALL map
to `IdentusError` with capability `presentation`, kind `InvalidInput`, a stable
static `presentation.*` code and static public text. Transition validation
SHALL allocate no memory and access no network, clock, randomness, runtime,
storage, chain or product service. An ignored release diagnostic SHALL report
complete transition-matrix throughput without a machine-dependent threshold.

#### Scenario: rejected input cannot enter diagnostics

- **WHEN** state parsing rejects a caller canary and errors are formatted or
  bridged
- **THEN** no caller input SHALL appear in Debug, Display, error code or public
  text

#### Scenario: orchestration stays downstream

- **WHEN** a consumer needs consent, authorization, compare-and-swap,
  persistence, retry, timestamps, transport acknowledgement or audit policy
- **THEN** it SHALL layer those behaviors around the allocation-free state
  contract

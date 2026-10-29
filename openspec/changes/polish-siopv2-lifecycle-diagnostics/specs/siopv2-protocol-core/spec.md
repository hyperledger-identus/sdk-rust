# siopv2-protocol-core Delta Specification

## MODIFIED Requirements

### Requirement: lifecycle transitions are deterministic and effect-bounded

The SDK SHALL model awaiting decision, response ready, refusal, cancellation,
timeout, delivery accepted and delivery unknown as pure transitions driven by
explicit time and bounded event identity. Repeating the identical event SHALL
be idempotent only while its resulting state remains current; conflicting reuse
in that state SHALL fail closed; and an event for a superseded transition SHALL
be invalid. Durable or cross-transition idempotency SHALL remain caller-owned.

#### Scenario: the End-User refuses or the request expires

- **WHEN** refusal, cancellation or timeout is applied before approval
- **THEN** the flow reaches the corresponding terminal state without invoking a signer or transport

#### Scenario: a current terminal event is repeated

- **WHEN** the same event identity is applied again while its terminal state remains current
- **THEN** the transition succeeds idempotently without changing state

#### Scenario: an earlier transition is repeated after the flow advances

- **WHEN** an event belonging to a superseded state is applied to the current state
- **THEN** the flow returns a stable invalid-transition category and does not mutate state

#### Scenario: delivery outcome cannot be established

- **WHEN** a response was released but the caller cannot establish acceptance
- **THEN** the flow records terminal `outcome_unknown` and does not authorize automatic retransmission

## ADDED Requirements

### Requirement: correlation and response failures have accurate redacted categories

The SDK SHALL distinguish signed wallet-nonce correlation failure from client
binding failure and SHALL distinguish response encoding/invariant failure from
bounded response-size failure. All public diagnostics SHALL remain stable,
static, and free of identifiers, nonces, tokens, payloads, and dependency text.

#### Scenario: retrieved wallet nonce differs from the signed claim

- **WHEN** the bounded outer wallet nonce is not exactly equal to the signed request claim
- **THEN** verification fails with the stable wallet-nonce-mismatch category

#### Scenario: JOSE response construction fails without exceeding a size bound

- **WHEN** the injected JOSE boundary reports an encoding or invariant failure that is not algorithm or signer policy
- **THEN** response creation fails with the stable response-encoding category

#### Scenario: a response size bound is exceeded

- **WHEN** an explicit header, payload, signing-input, signature, compact, or form-body limit is exceeded
- **THEN** response creation fails with the stable response-too-large category

### Requirement: private helper reuse cannot bypass protocol invariants

The SDK SHALL keep SIOPv2 HTTPS validation, form encoding, and bounded string
validation behind the protocol's typed request and response transitions. It
SHALL NOT expose a generic public helper solely to deduplicate implementation.

#### Scenario: another protocol needs URI or form behavior

- **WHEN** another protocol has different error ownership, bounds, or semantic invariants
- **THEN** it cannot bypass those invariants through a public SIOPv2 helper

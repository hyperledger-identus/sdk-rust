## Why

The SDK is about to implement several standards-heavy capabilities while
already carrying hand-written crypto and parsing mechanics. Agents need a
repeatable, evidence-backed way to decide what to adopt, wrap, test as an
oracle or reject before implementation begins.

## What Changes

- Establish a durable Rust dependency reuse portfolio and negative-decision
  ledger with objective reconsideration triggers.
- Adopt the architectural rule that focused standards engines remain private
  behind cohesive Identus-owned facades.
- Define a rolling near-current MSRV policy and a separate implementation path
  from the current Rust 1.85 operational floor.
- Add a proportionate `research.md` contract and a machine-checked
  `factory research-ready` gate before qualifying implementation.
- Create focused follow-up issues for approved integrations and spikes without
  adding runtime dependencies in this change.

## Capabilities

### New Capabilities

- `dependency-research-readiness`: Records and validates build-versus-adopt
  evidence, explicit candidate dispositions, negative decisions and the
  pre-implementation research gate.

### Modified Capabilities

None.

## Impact

The change affects repository governance, factory scripts/tests, agent and Pi
adapters, issue/PR intake, two ADRs and research documentation. It adds no Rust
runtime dependency, public API, wire behavior, target claim or downstream
mutation. Follow-up issues own every dependency and MSRV implementation.

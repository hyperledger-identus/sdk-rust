# Correct platform capability ownership

## Why

The SDK-TS 8.1.4 inventory correctly treats the newest language SDK as
discovery evidence, but several preliminary dispositions still let the donor
architecture dictate the target. In particular, the report asks Rust behavior
to preserve TypeScript DTOs and errors, leaves peer DID and the portable agent
runtime outside Rust, treats browser/Node networking as one indivisible
TypeScript responsibility, narrows the first DID binding so far that the full
DID migration direction is obscured, and labels required quality methods as
merely exploratory.

Those conclusions conflict with the platform-core mandate. SDK-Rust must own
canonical reusable contracts. Language SDKs may translate those contracts into
temporary compatibility shapes, but donor DTOs, package boundaries, private
workspace packages, and runtime frameworks are not the Rust architecture.

## What changes

- Make SDK-Rust DTOs and errors canonical and define versioned language
  compatibility adapters as optional migration surfaces.
- Treat the SDK-TS protobuf workspace package as Prism DID evidence already
  anticipated by the SDK-Rust toolchain; require every other portable private
  workspace responsibility to move to Rust or a qualified Rust dependency.
- Promote peer DID and portable DID/domain/service behavior into the SDK-Rust
  target while leaving concrete ledger and host I/O effects behind ports.
- Separate RFC 9901 base SD-JWT, the current SD-JWT VC profile, and evidenced
  legacy `vc+sd-jwt` compatibility.
- Make DIDComm Messaging v2.1 a core capability and require an independently
  versioned application-protocol catalog.
- Define a small, idiomatic Rust agent runtime with deterministic protocol
  state, typed effects, explicit scheduling, cancellation, persistence, and
  event boundaries.
- Decide browser/Node networking through ports and target adapters rather than
  assigning protocol behavior to TypeScript.
- Establish existing Cloud Agent/Mediator E2E interoperability as phase-one
  evidence and composable Rust services as the long-term replacement path.
- Require property, fuzz, benchmark, and differential evidence proportionate
  to each capability's risks.
- Publish an evidence-gated adoption roadmap whose milestones each deliver a
  usable Rust capability, target adapter, conformance packet, and bounded
  consumer or service proof rather than broad repository parity.

## Capability

### Modified capability

- `identus-platform-core-migration`: correct canonical ownership, language
  adapter, DID, DIDComm, agent-runtime, networking, service-composition, and
  quality-evidence rules.

## Non-goals

This change does not implement peer DID, SD-JWT, DIDComm, an agent executor,
network adapters, Cloud Agent, or Mediator. It does not select a production
dependency, mutate SDK-TS or another consumer, promise a supported binding,
or deprecate a deployed service. Each implementation remains issue-first and
requires its own exact standards, dependency, target, security, and consumer
evidence.

## Delivery

Issue #497 owns this correction. The planning-only contract and preflight
precede changes to ADRs, inventories, checkers, reports, and GitHub backlog.
The result is a corrected, enforceable roadmap rather than feature delivery.

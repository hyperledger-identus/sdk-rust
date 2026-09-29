# SDK-Rust platform-core adoption milestones

**Status:** target roadmap; only completed exit evidence is a delivery claim

**Machine record:**
[`platform-core-adoption-milestones.toml`](platform-core-adoption-milestones.toml)

**Architecture:** ADRs [0170](../adr/0170-make-rust-contracts-canonical-and-language-adapters-transitional.md),
[0171](../adr/0171-own-a-portable-effect-driven-agent-runtime.md), and
[0172](../adr/0172-version-didcomm-engine-and-application-protocols-separately.md)

## Why milestones are capability proofs

Repository-wide parity is too coarse to manage. It hides which standard,
engine, target, consumer, or compatibility decision is blocking progress and
encourages broad ports. This roadmap instead advances through independently
useful capability proofs. Each milestone has a bounded Rust deliverable, a
binding/host path, a conformance packet, and one consumer-shaped or service
interoperability proof.

A milestone is not closed because code exists, a unit suite passes, or another
SDK has the feature. It closes only when all of these are present:

1. exact standard/profile and accepted responsibility boundary;
2. owned Rust contract plus build/adopt decision;
3. implementation with bounded inputs, redacted errors, and security/privacy
   review proportional to risk;
4. property, fuzz, benchmark, and differential evidence required or justified
   not applicable under issue #501;
5. selected target/binding evidence;
6. immutable conformance/compatibility packet;
7. one named adoption or E2E proof with observability and rollback; and
8. candidate release/migration evidence without implying protected publication.

Research for a later milestone may overlap. Its implementation/adoption claim
cannot skip a predecessor contract or proof.

## Milestone map

| ID | Outcome | Primary issues | Exit adoption proof |
| --- | --- | --- | --- |
| A0 | Correct canonical ownership, adapters, DID/DIDComm/runtime boundaries, quality rules, and this roadmap | #497 | offline inventories reject donor-shaped ownership and incomplete milestone records |
| A1 | Shared compatibility foundation: vector catalog, change ledger, adapter/error mapping rules, and risk-routed evidence plan | #420, #501 | first SDK-TS DID adapter consumes the same versioned packet as Rust tests |
| A2 | Portable DID platform: complete generic DID domain/service binding sequence, peer DID, and separately bounded Prism portable semantics | #492, #493 | SDK-TS DID facade and one native binding use Rust without duplicated DID semantics |
| A3 | Credential format engines: RFC 9901 base, pinned SD-JWT VC profile, bounded legacy adapter, and independently qualified AnonCreds 1.0 facade | #489, #490 | shared cross-language issuance/presentation vectors pass for each delivered format |
| A4 | DIDComm v2.1 core: qualified engine facade, DID/secret ports, pack/unpack/routing, attachment/resource/error contracts | #491 | immutable existing agent/Mediator endpoint interoperates with exact engine capabilities |
| A5 | Portable agent kernel: deterministic events/transitions/effects/checkpoints and multi-executor contract | #498 | one fixture produces identical results in reference, native-async, and WASM-shaped executors |
| A6 | Versioned DIDComm application protocols delivered one bounded state machine at a time | #500, #419 | selected existing Cloud Agent/Mediator issue, present, revoke, and mediation/pickup E2E flows pass at pinned versions |
| A7 | Host/language adapters: browser/Node networking decision, WASM/UniFFI surfaces, and thin TypeScript/Swift hosts | #499, #492 | named browser, Node, iOS, and Android canaries pass only for explicitly selected targets |
| A8 | Language SDK adoption: SDK-TS first, SDK-Swift deviation/parity second, SDK-KMP compatibility last | #502, #421, #416 | released-language APIs route selected capabilities through Rust with measured fallback and deprecation evidence |
| A9 | Composable reference services from public SDK primitives | #418, #419 | operational, persistence/migration, tenancy, QoS, consumer, and rollback parity supports a separate deprecation decision |

## Dependency flow

```text
A0 contract correction
        │
        ▼
A1 shared evidence ───────────────┐
        │                         │
        ├────────► A2 DID         ├────────► A3 credential formats
        │              │          │
        │              ▼          │
        └────────► A4 DIDComm core│
                       │          │
                       ▼          │
                 A5 agent kernel  │
                       │          │
                       ▼          │
                 A6 app protocols │
                       └──────┬────┘
                              ▼
                        A7 host adapters
                              │
                              ▼
                        A8 SDK adoption
                              │
                              ▼
                     A9 reference services
```

The diagram shows adoption dependencies, not a ban on parallel research. A3
can progress beside A2/A4. A7 adapter research can start early, but a target
support claim waits for the capabilities it exposes.

## Deliverable decomposition

### A0 — contract correction

Deliver ADRs 0170–0172, the corrected SDK-TS inventory, portable runtime
design, machine milestone registry, child issues, and validators. No runtime or
protocol implementation belongs here.

### A1 — shared compatibility foundation

Complete the language-neutral vector/provenance catalog under #420. Add a
versioned adapter mapping record for Rust-to-language DTO/error compatibility,
the consumer-visible change ledger, and the four-class evidence declaration
under #501. Seed it with the DID canary; do not wait for every future protocol.

### A2 — portable DID platform

Keep #492's value-only path as the first canary, then create separate slices
for documents/verification relationships/services, resolver/dereferencer/
registrar ports, peer DID, and method adapters. #493 decides exact peer DID
numalgos and whether `did-peer` is a dependency or differential oracle. Prism
portable codec/method work remains separate from Cardano/indexer/node effects.

### A3 — credential format engines

Run SD-JWT and AnonCreds as separate trains. SD-JWT has an RFC 9901 base,
separately pinned VC profile, and evidence-driven legacy `vc+sd-jwt` adapter.
AnonCreds targets current 1.0 behavior and a maintained Rust engine with VDR,
link-secret, revocation, native/mobile/WASM, and supply-chain boundaries. One
format does not block the other's research or delivery.

### A4 — DIDComm v2.1 core

Select the exact engine/facade under #491. Deliver message values, pack/unpack,
routing/forward, attachments, `from_prior`, DID and secret resolver ports,
redacted errors, resource limits, and host/WASM/mobile evidence. Do not include
issue-credential or agent scheduling in the engine milestone.

### A5 — portable agent kernel

Use two unrelated consumer protocols to prove the kernel boundary before
creating a stable crate. Deliver the deterministic reference executor and
target-shaped proofs, not a full product agent. Product workflow, consent,
custody, storage implementation, and service lifecycle remain outside.

### A6 — DIDComm application protocols

Create the catalog first. Suggested incremental order is basic message/problem
report, discover features/out-of-band, issue credential, present proof,
revocation notification, then coordinate mediation/routing/message pickup.
Actual order follows named consumers and immutable existing-service E2E
evidence. Each protocol/version/role receives its own issue and release unit.

### A7 — host and language adapters

Issue #499 decides browser/Node transports per target. Bind only consumer-
required Rust APIs. Use thin TypeScript and Swift adapters for host integration
and compatibility mappings; keep protocol state in Rust. React Native and KMP
receive explicit decisions rather than implicit support through another target.

### A8 — language SDK adoption

Migrate normalized capabilities, not historical Apollo/Castor/Pollux/Mercury/
Pluto modules. SDK-TS is first because it is current and supplies the initial
consumer contract. SDK-Swift follows to expose meaningful deviations. SDK-KMP
comes last to recover required compatibility without making its stale design
normative.

### A9 — composable services

First keep Cloud Agent and Mediator as pinned black-box interop targets. Then
build reference services from public SDK crates and explicit adapters. Service
deprecation needs a separate decision after protocol, persistence/migration,
tenancy, deployment, operations, security, QoS, consumer, and rollback parity.

## Immediate sequence after A0

The next implementation milestone should be A1, not peer DID or the agent
runtime directly. It creates the shared vector and adapter evidence every
later adoption slice needs. Within A1, use #492's existing bounded DID values
as the first concrete packet. In parallel, research-only work may continue on
#489, #491, #493, #498, and #499 without activating dependencies or APIs.

## Non-claims

This roadmap is not a release date, support matrix, certification claim,
dependency approval, SDK/service deprecation, or authorization to mutate a
consumer. An open issue, catalog row, compiled target, or passing donor suite
does not close a milestone. Protected publication and releases remain human
maintainer decisions.

# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/493
Constraint blockers: none

The maintainer's exact scope confirmation is retained in
https://github.com/hyperledger-identus/sdk-rust/issues/493#issuecomment-6420422983.

## Existing entries affected

ADR 0110's reusable-module hard gates remain mandatory. ADR 0174 supersedes
only its initial source disposition for portable PRISM method, VDR, and
deterministic indexing behavior. Generic layering, provenance, cohesion,
effect-isolation, safety, target, dependency, consumer-evidence, and release
independence gates remain unchanged.

SDK-ARCH-001 is clarified rather than waived: a standards-named method crate
is not a chain dependency when its dependency cone remains chain-neutral and
its behavior is deterministic and effect-isolated. Concrete Cardano,
persistence, network, service, and node effects remain downstream.

## Introduced or changed constraints

- `identus-did` remains generic and cannot depend on a method crate.
- Peer numalgo 4 is the current profile and numalgo 2 is explicit opt-in
  compatibility; profiles do not silently parse or emit each other.
- PRISM method, VDR, and indexer responsibilities are separate selectable
  release units with inward-only dependencies.
- Encoded/decoded bytes, elements, nesting, ancestry, pages, batches, channels,
  retries, and expensive work are bounded before or at their narrowest safe
  boundary.
- PRISM long-form support cannot silently inherit the generic 2,048-byte DID
  ceiling or the unrelated 4,096-byte crypto text ceiling. Issue #524 must
  derive compatible finite budgets before #521 starts.
- Public errors redact caller-controlled key, operation, service, and VDR
  payloads; secret generation and custody remain caller-owned.
- Donor revisions, paths, licenses, transformations, and fixture provenance
  are recorded exactly; normative sources outrank donor behavior.
- Downstream deletion requires an immutable SDK release and measured
  same-snapshot differential adoption.

## Introduced or changed limitations

- Peer numalgos 0, 1, and 3 are outside the first implementation.
- The assessed Rust peer crates are differential oracles, not public SDK
  dependencies.
- Concrete Blockfrost, db-sync, Oura, Cardano wallet, SQL, HTTP, CLI,
  deployment, and node-runtime adapters stay in NeoPRISM initially.
- Planning does not establish implementation, conformance, target support,
  publication, consumer adoption, Cardano connectivity, or code reduction.

## Consumer and product impact

No current consumer API or runtime behavior changes. The decision gives
SDK-TS a future thin peer/PRISM adapter path and gives NeoPRISM an immutable,
measurable reduction path without making either donor authoritative. Oxid can
later select method crates without importing a node or language-SDK runtime.

## Activation and rollback

The ownership plan activates when issue #493's behavior-neutral PR passes
local and protected hosted gates and merges to `develop`. Production work is
separately authorized by #521, #522, #523, and prerequisite #524. Before publication, reverting
the planning PR restores the earlier roadmap. After publication, a superseding
ADR and explicit migration are required. NeoPRISM removal remains separately
authorized and independently reversible.

## Evidence

The research record pins normative, SDK-TS, NeoPRISM, and Rust candidate
revisions; compares profiles, ownership, dependency coupling, MSRV, maintenance,
security, effects, and candidate sizes; defines bounds and stop conditions;
and records local validation commands. The child issues provide durable
implementation scope and dependency order.

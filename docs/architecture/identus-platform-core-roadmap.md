# Identus platform-core migration roadmap

**Program owner:** sdk-rust issue #415

**Architecture:** ADR 0162, ADR 0163, ADR 0164, ADR 0169

**Discussions:** [roadmap #424](https://github.com/hyperledger-identus/sdk-rust/discussions/424), [normalization #423](https://github.com/hyperledger-identus/sdk-rust/discussions/423)

**Status:** bootstrap; no language SDK retirement is authorized

## Outcome

`sdk-rust` becomes the reusable semantic core of Identus. TypeScript, Swift,
Kotlin, and React Native consumers retain idiomatic APIs and platform adapters
while duplicate portable implementations are removed capability by capability.
Success is fewer authoritative implementations with equal or better behavior,
not the number of repositories archived or lines moved to Rust.

## Program phases

| Phase | Exit evidence |
|---|---|
| P0 — governance and inventory | accepted ADRs; immutable snapshots; registry/ledger schemas; per-SDK inventory issues and discussions |
| P1 — binding contract hardening | stable Identus-owned DTO/error/version rules; target matrices; async/ownership/cancellation policy; package versioning and rollback |
| P2 — non-secret canaries | DID/DID URL path consumed by TS/browser, Swift, and Kotlin with shared vectors and package/runtime evidence |
| P3 — foundation waves | crypto and generic DID behavior moved behind facades; secrets/custody deliberately isolated; duplicate implementations deprecated |
| P4 — credential/protocol waves | credential formats, presentations, OpenID4VC, DIDComm and reusable workflow state normalized and migrated independently |
| P5 — storage and agent waves | portable storage/backup contracts and protocol state machines converge; platform databases and runtime orchestration remain adapters |
| P6 — defaults and retirement | Rust-backed paths default; migrations rehearsed; compatibility windows close; duplicate code or whole repos retired only after gates |
| P7 — service discovery | separate cloud-agent and mediator decomposition roadmaps; no implicit Scala/service rewrite |

Phases overlap across SDKs only when their binding and evidence dependencies are
independent. A failed canary blocks that surface, not the whole initiative.

Discovery order is SDK-TS, SDK-Swift, then SDK-KMP. This order reflects release
currency and reduces missed capabilities; it does not make an implementation
normative. Each deviation follows ADR 0169's standard/profile/upstream/
compatibility precedence. Historical Apollo, Castor, Pollux, Mercury, Pluto,
and Edge Agent names remain source aliases only and cannot become SDK-Rust
component boundaries.

## sdk-ts milestones

| Milestone | Deliverable | Stop/go gate |
|---|---|---|
| TS0 inventory | exact 8.1.4 report and machine-readable capability map, runtime/package/plugin/dependency/test evidence, deviation rules | every capability has a preliminary disposition and unresolved decision issue |
| TS1 browser DID canary | optional facade route to `identus-wasm-did`; shared DID vectors; npm/bundler/browser/Node decision | no public/wire/error regression; explicit unsupported runtimes |
| TS2 facade contract | TypeScript DTO/error/versioning and WASM ownership/resource rules | generated/manual API is reviewed and rollback works |
| TS3 crypto and DID wave | Rust-backed portable crypto/DID semantics; JS custody/browser adapters stay outside | authoritative vectors plus real consumer rehearsal |
| TS4 credential/protocol wave | normalized credential formats and OID4VC behavior; plugin responsibilities split | exact draft/final and error compatibility recorded |
| TS5 agent/storage wave | reusable workflow state and portable storage contracts; browser/Node adapters retained | persistence/backup migration and cancellation proven |
| TS6 retirement review | delete duplicate implementations; archive packages/repo only if no unique role remains | support window closed, consumers migrated, release ledger complete |

## sdk-swift milestones

| Milestone | Deliverable | Stop/go gate |
|---|---|---|
| SW0 inventory | products/modules, public symbols, Apple platform responsibilities, dependencies and test authority | every capability has a preliminary disposition |
| SW1 native DID canary | Swift facade uses experimental UniFFI DID package in a reversible path | simulator/device/package and shared-vector evidence |
| SW2 facade contract | Swift DTO/error/async/ownership/version rules; SwiftPM/XCFramework strategy | ABI/package compatibility and rollback proven |
| SW3 crypto and DID wave | Rust-backed portable semantics; Keychain/Secure Enclave stay Swift adapters | secret handles never expose raw material across FFI |
| SW4 credential/protocol wave | normalized formats, presentations, OID4VC and DIDComm slices | authoritative vectors and consumer rehearsal |
| SW5 agent/storage wave | reusable workflow state; CoreData/network/background adapters retained | schema/backup/lifecycle migration proven |
| SW6 retirement review | remove duplicate Swift semantics, preserving a thin Apple SDK as needed | supported responsibilities and consumers accounted for |

## sdk-kmp milestones

| Milestone | Deliverable | Stop/go gate |
|---|---|---|
| KMP0 inventory | Android/JVM public API, source-set/platform map, dependencies and test authority | every capability has a preliminary disposition |
| KMP1 native DID canary | Kotlin facade uses experimental UniFFI DID package | Android device/JVM/package and shared-vector evidence |
| KMP2 facade contract | Kotlin DTO/error/coroutine/ownership/version rules; Maven/AAR/JVM strategy | runtime and package compatibility plus rollback |
| KMP3 crypto and DID wave | Rust-backed portable semantics; keystore and lifecycle stay adapters | secrets, threading and cancellation proven |
| KMP4 credential/protocol wave | normalized formats, presentations, OID4VC and DIDComm slices | authoritative vectors and consumer rehearsal |
| KMP5 agent/storage wave | reusable workflow state; Ktor/SQLDelight/platform adapters retained | persistence/backup and concurrency migration proven |
| KMP6 retirement review | remove duplicate Kotlin semantics; retain thin Android/JVM packages if needed | support window and consumer migration complete |

## React Native milestones

| Milestone | Deliverable | Stop/go gate |
|---|---|---|
| RN0 generator/runtime qualification | refresh issue #223 against exact released versions; choose generator/runtime strategy | exact UniFFI compatibility, supply-chain and New Architecture design |
| RN1 DID canary | versioned TurboModule surface with shared DID vectors | iOS and Android runtime/package evidence; fast refresh/reload safety |
| RN2 binding contract | async/cancellation/threading/ownership/errors/version negotiation | no raw secrets or unconstrained buffers; deterministic generation |
| RN3 mobile foundations | selected crypto/DID/credential primitives with platform custody ports | device evidence, performance/resource baseline, migration guide |
| RN4 supported package | reproducible package, example app, compatibility matrix and release train | support policy updated explicitly; independent consumer canary |

The latest assessed `uniffi-bindgen-react-native` release pins UniFFI 0.31 and
does not match the sdk-rust 0.32 native line. This roadmap does not force an
upgrade or mixed-version experiment.

## Capability delivery packet

Every implementation issue contains:

1. one registry row or a justified cohesive set;
2. source and target owners at immutable revisions;
3. disposition and alternatives;
4. normative/profile sources and test-authority report;
5. public, wire, persistence, ABI, target, resource, and security impact;
6. change-ledger entry and deprecation/legacy-bug decision;
7. binding/package and consumer canary plan;
8. positive, negative, boundary, mutation and differential evidence;
9. rollback and observability; and
10. release-note and migration-guide text.

Implementation starts only after OpenSpec research/constraints and factory
preflight. Protected PRs remain small and stackable.

## Deprecation and retirement gate

A feature can be removed only when:

- its disposition is reviewed;
- a supported replacement exists or deliberate removal is approved;
- authoritative tests and compatibility differences are known;
- affected consumers and persisted/wire data are accounted for;
- warnings, docs, migration instructions, and release versions are recorded;
- replacement and rollback have been rehearsed; and
- security/privacy policy does not require earlier fail-closed removal.

A repository can be archived only after all capability rows are `removed`,
`moved`, or deliberately owned elsewhere and no unique packaging/platform
responsibility remains.

## First execution backlog

1. Complete exact sdk-ts inventory and select its DID canary ([#417](https://github.com/hyperledger-identus/sdk-rust/issues/417)).
2. Qualify SDK-TS deviations independently: SD-JWT (#489), AnonCreds 1.0
   (#490), DIDComm (#491), DID method adapters (#493), Presentation Exchange
   (#494), and backup format (#495).
3. Execute the opt-in SDK-TS DID/DID URL canary (#492) after its binding and
   shared-vector prerequisites pass.
4. Complete exact sdk-swift inventory and select its UniFFI DID canary ([#421](https://github.com/hyperledger-identus/sdk-rust/issues/421)).
5. Complete exact sdk-kmp inventory and select its UniFFI DID canary ([#416](https://github.com/hyperledger-identus/sdk-rust/issues/416)).
6. Build the shared cross-language test-vector catalog and provenance rules ([#420](https://github.com/hyperledger-identus/sdk-rust/issues/420)).
7. Define binding DTO/error/version and async/ownership contracts under #163.
8. Refresh React Native qualification under #223 without mixing UniFFI lines.
9. Enforce the capability/change ledgers and render release/migration evidence ([#422](https://github.com/hyperledger-identus/sdk-rust/issues/422)).
10. Discover cloud-agent reusable capabilities as a separate service program ([#418](https://github.com/hyperledger-identus/sdk-rust/issues/418)).
11. Discover mediator reusable capabilities as a separate service program ([#419](https://github.com/hyperledger-identus/sdk-rust/issues/419)).

The inventory issues may run in parallel. Canary implementation waits for the
relevant inventory, binding contract, and test-authority output.

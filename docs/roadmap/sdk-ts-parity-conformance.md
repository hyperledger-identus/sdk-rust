# TS1 — SDK-Rust/SDK-TS capability parity and conformance

**Status:** baseline and success contract defined; no SDK-TS route is claimed
delivered

**GitHub milestone:**
[TS1 — SDK-Rust/SDK-TS capability parity and conformance](https://github.com/hyperledger-identus/sdk-rust/milestone/7)

**Machine record:**
[`sdk-ts-parity-conformance.toml`](sdk-ts-parity-conformance.toml)

**Source baseline:** `@hyperledger/identus-sdk@8.1.4` at
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`

## Outcome

TS1 makes cross-language progress reviewable without pretending that Rust and
TypeScript should have identical repositories, modules, APIs, or runtime
responsibilities. It classifies every normalized SDK-TS capability twice:

1. a **relation** says what parity should mean for that capability; and
2. a **state** says what the current immutable evidence actually proves.

The counts below are an inventory check, not a parity score. There is no single
repository-wide percentage because `platform-owned`, intentionally different,
blocked, and deferred work have different meanings.

<!-- target-contract:start -->
## Target contract

Target support is proved independently. Browser and Node.js consume portable
Rust through target-specific WASM package entries. React web reuses those
artifacts through a thin lifecycle integration, while SSR and hydration also
cross the Node.js contract. React Native does **not** reuse browser WASM: ADR
0163 requires a separate New Architecture/TurboModule package over a qualified
native Rust binding. None of these target records is a current support claim.

| Target | Current target state | Artifact and loader | Consumer proof | CI job | Governing limits |
| --- | --- | --- | --- | --- | --- |
| Browser | `target` | Browser ESM WASM package through bundler and direct-module exports | Plain browser and client React applications in real Chromium and Firefox | `target-browser` | `SDK-LIM-002`, `SDK-LIM-003` |
| Node.js | `target` | Node-compatible WASM entry with ESM and CommonJS compatibility exports | Plain Node.js application, including worker, filesystem, and WebCrypto policy | `target-nodejs` | `SDK-LIM-002` |
| React web | `target` | Shared browser package for client execution; Node.js contract for SSR | Client plus SSR/hydration React fixtures without duplicated domain logic | `target-react-web` | `SDK-LIM-002`, `SDK-LIM-003` |
| React Native | `blocked` | Separate iOS and Android native artifacts behind a generated TurboModule interface | Minimal iOS and Android New Architecture applications | `target-react-native` | `SDK-LIM-002`, ADR 0163, issue #223 |

`target` means a planned delivery target, not a supported runtime. A capability
may be called `delivered` only when every applicable target is delivered; an
omitted target is recorded explicitly as `not-applicable` with ownership and a
reconsideration trigger.
<!-- target-contract:end -->

<!-- ci-contract:start -->
## Rust-core-first isolated CI contract

The existing protected `fast` status remains the single branch-protection
signal required by `SDK-LIM-009`, but its internal evidence is layered:

1. `core-rust` always runs first and publishes an independently visible result.
2. `binding-impact-plan` evaluates exported API and binding-schema
   fingerprints, transitively bound crates, lockfiles, generated mappings,
   package metadata, target contracts, and changed paths. Unknown impact fails
   closed by selecting every target job.
3. Selected `target-browser`, `target-nodejs`, `target-react-web`,
   `target-react-native`, and `target-package-types` jobs run at the same exact
   head with sibling cancellation disabled (`fail-fast: false`).
4. Unselected jobs emit machine-readable inapplicable receipts without
   installing target toolchains. The final `fast` aggregator fails for a
   selected failure while preserving the independent Rust-core result.

Bindings and platform adapters depend inward on portable Rust crates only.
Target failures therefore block the target claim without erasing valid core
evidence, and path-only routing cannot suppress relevant binding work.
<!-- ci-contract:end -->

<!-- success-criteria:start -->
## Success criteria

| ID | Criterion | Passing evidence |
| --- | --- | --- |
| `SC-01` | Capability-oriented TypeScript contract | Public API and package-export snapshots, a case-insensitive guard for `apollo`, `castor`, `mercury`, `pluto`, `pollux`, and `edge-agent`, and a migration-only exception inventory. |
| `SC-02` | Target-specific build and runtime proof | Exact build artifact, loader, package entry, consumer E2E, resource/budget evidence, and rollback rehearsal for each claimed runtime matrix. |
| `SC-03` | Applicable SDK-TS E2E reuse | Every discovered donor test is ledgered as `shared`, `adapted`, `platform-owned`, `superseded`, or `excluded`; shared journeys run one TypeScript assertion body against legacy and Rust-backed private providers. |
| `SC-04` | Rust evidence for additions and deviations | Every `rust-addition` or `rust-deviation` has its selected profile plus normative, negative, boundary/resource, and Rust E2E or conformance evidence, with a TypeScript boundary smoke test where applicable. |
| `SC-05` | Coverage and public-operation exercise | Every owned public operation maps to contract or E2E evidence; changed owned code reaches at least 80% line and 70% branch coverage where reliable, never below a higher established package baseline, with pinned reproducible measurements. |
| `SC-06` | Type-safe and validated boundary | Strict TypeScript declarations contain no explicit `any`; ingress `unknown` is narrowed, wire values are validated, cross-language values/errors/lifecycle/cancellation are mapped deterministically, and Rust revalidates domain invariants. |
| `SC-07` | Shared portable semantics and thin adapters | Dependency-direction and duplication guards prove that portable core has no inward binding/platform dependency and TypeScript does not reimplement cryptography, canonical parsing, formats, or protocol state. |
| `SC-08` | Operable target with accepted budgets | Before support, each target has accepted size, startup, latency, throughput, and memory budgets as applicable, plus bounded diagnostics, observability, fallback, and rollback. Until acceptance, performance is measurement-only. |
| `SC-09` | Rust-core-first isolated CI | Exact-head core, fail-closed impact plan, isolated non-cancelling target receipts, and the single protected `fast` aggregation receipt prove core and target outcomes independently. |

Coverage is supporting evidence, not conformance authority. Any unavailable
measure or not-applicable evidence class needs an owner, rationale, strongest
alternative evidence, and review/expiry or reconsideration trigger; invented
percentages and silent exclusions are forbidden.
<!-- success-criteria:end -->

## Relation vocabulary

| Relation | Meaning |
| --- | --- |
| `semantic-equivalent` | The selected profile must produce the same bounded domain or protocol result in both SDKs. |
| `compatible-adapter` | Rust owns portable semantics; TypeScript may keep language DTOs, errors, async conventions, custody, networking, lifecycle, or packaging behind a versioned adapter. |
| `platform-owned` | The responsibility is inherently browser, Node, npm, plugin-host, UI, permission, or product lifecycle behavior. Rust exposes only the necessary port. |
| `intentionally-different` | Rust follows an accepted newer or safer profile; the migration and compatibility behavior are explicit. |
| `deferred` | Product, profile, dependency, or consumer evidence is not sufficient to select a parity target. |
| `retired` | A donor capability has no continuing target and needs an explicit removal path. |

Delivery state is independent: `delivered`, `partial`, `blocked`, `excluded`,
or `not-applicable`. A mature Rust implementation remains `partial` until the
SDK-TS adapter, consumer, observability, and rollback evidence is complete.

## Current baseline

### By conformance state

| State | Count | Capability IDs |
| --- | ---: | --- |
| `partial` | 14 | `crypto.key-operations`, `crypto.hd-derivation`, `did.syntax`, `did.document`, `did.method.peer`, `did.method.prism`, `did.resolution`, `credentials.jwt-vc`, `credentials.sd-jwt`, `credentials.status`, `openid4vc.issuance-wallet`, `wallet.storage-contracts`, `platform.browser-node-networking`, `bindings.wasm-did` |
| `blocked` | 7 | `credentials.anoncreds-v1`, `presentations.exchange`, `messaging.didcomm-v2`, `messaging.didcomm-application-protocols`, `wallet.backup-format`, `agent.protocol-state`, `agent.runtime-orchestration` |
| `not-applicable` | 3 | `wallet.storage-browser`, `platform.typescript-plugin-host`, `platform.npm-packaging` |
| `delivered` | 0 | none |
| `excluded` | 0 | none |

### By parity relation

| Relation | Count | Capability IDs |
| --- | ---: | --- |
| `semantic-equivalent` | 3 | `did.method.peer`, `messaging.didcomm-application-protocols`, `agent.protocol-state` |
| `compatible-adapter` | 15 | `crypto.key-operations`, `crypto.hd-derivation`, `did.syntax`, `did.document`, `did.method.prism`, `did.resolution`, `credentials.jwt-vc`, `credentials.anoncreds-v1`, `credentials.status`, `openid4vc.issuance-wallet`, `messaging.didcomm-v2`, `wallet.storage-contracts`, `agent.runtime-orchestration`, `platform.browser-node-networking`, `bindings.wasm-did` |
| `platform-owned` | 3 | `wallet.storage-browser`, `platform.typescript-plugin-host`, `platform.npm-packaging` |
| `intentionally-different` | 1 | `credentials.sd-jwt` |
| `deferred` | 2 | `presentations.exchange`, `wallet.backup-format` |
| `retired` | 0 | none |

<!-- feature-comparison:start -->
### Rust / SDK-TS feature comparison

The SDK-TS column is joined from the canonical inventory at the pinned donor
revision; it is not a second behavioral source of truth. Target cells report
the full machine state in fixed browser, Node.js, React web, and React Native
order. They describe evidence per runtime, not a global implementation state:
a capability is not delivered while any applicable cell is incomplete.

| Capability | Rust today | SDK-TS today | Target | B/N/RW/RN | Deviation / missing proof |
| --- | --- | --- | --- | --- | --- |
| `crypto.key-operations` | Mature portable key generation, import/export, signing, verification, and shared-secret operations. | Generate, import, export, sign, verify, and derive shared secrets with supported key types. | Rust-owned cryptographic operations behind a thin typed TypeScript key facade; secret custody remains host-owned. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Secret custody and key-object ergonomics remain TypeScript adapter responsibilities.; Missing proof: Cross-SDK vectors, error mapping, secret-handling review, and browser, Node.js, React web, and React Native target E2E evidence. |
| `crypto.hd-derivation` | Mature deterministic derivation and path semantics for the selected profiles. | Derive deterministic keys and paths from mnemonic or seed material. | Rust-owned derivation behind a thin TypeScript facade; mnemonic sources and custody remain host-owned. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | CIP-0003 applies only where an explicit profile selects it.; Missing proof: Frozen cross-language vectors, profile-specific negative tests, adapter E2E coverage, and target-runtime evidence. |
| `did.syntax` | Mature DID and DID URL parsing, validation, component access, and serialization with an existing WASM value surface. | Parse and serialize DID and DID URL values with stable component semantics. | Capability-named DID and DID URL TypeScript values backed by Rust semantics on supported targets. | browser=partial; nodejs=blocked; react-web=blocked; react-native=blocked | TypeScript constructors and error classes remain compatibility adapters.; Missing proof: Equivalent TypeScript E2E tests, constructor and error mapping, package loading, bundler evidence, and rollback proof. |
| `did.document` | Substantial portable DID document, verification method, service, and relationship model. | Represent DID documents, verification methods, services, and relationships. | Rust-owned DID document semantics exposed through generated or versioned typed TypeScript DTO mappings. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Method-specific extensions remain versioned adapters.; Missing proof: Round-trip vectors, stable field and error mapping, extension policy, and resolver-consumer E2E evidence. |
| `did.method.peer` | No production SDK-Rust implementation for the selected peer DID profiles. | Create and resolve peer DIDs required by pairwise messaging relationships. | Rust-owned deterministic creation and resolution for explicitly selected peer DID profiles with a thin TypeScript facade. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Only explicitly selected numalgo profiles are in scope.; Missing proof: Implementation, selected-profile vectors, document equivalence, pairwise messaging E2E tests, and runtime-target evidence. |
| `did.method.prism` | Portable PRISM method semantics are consumer-proven downstream but are not yet a complete SDK-Rust core and binding surface. | Construct long-form Prism DIDs and resolve Prism identifiers through configured infrastructure. | Rust-owned portable PRISM identifier and document semantics; network, ledger, persistence, and service composition stay injected. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Cardano observation, submission, persistence, and service composition are not SDK-Rust parity targets.; Missing proof: SDK-Rust integration boundary, cross-SDK vectors, typed binding surface, resolver adapter E2E tests, and rollback evidence. |
| `did.resolution` | Substantial resolver, cancellation, cache, and HTTP contract support with injected network policy. | Resolve identifiers through composable method and HTTP resolver contracts. | Rust-owned resolution semantics behind injected browser, Node.js, React web, and React Native transport adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Browser and Node fetch policy remains platform-owned.; Missing proof: Named resolver canary, cache, cancellation, and error equivalence, bounded-network tests, and per-target E2E evidence. |
| `credentials.jwt-vc` | Partial credential support; the exact normative JWT VC profile is not yet fully pinned and delivered. | Parse, represent, issue, and verify JWT-encoded credentials and presentations. | Rust-owned selected JWT VC and JOSE semantics with thin TypeScript workflow and ergonomic adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | No generic historical JWT VC behavior is normative without a selected profile.; Missing proof: Normative profile decision, issuance and verification vectors, negative cases, workflow E2E tests, and target-runtime proof. |
| `credentials.sd-jwt` | Partial RFC 9901 and draft-22 implementation behind an Identus-owned direction. | Issue, present, and verify selectively disclosed JWT claims and the selected VC profile. | Rust-owned RFC 9901 and selected VC-profile semantics with a capability-named TypeScript facade. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | dc+sd-jwt is canonical; vc+sd-jwt is only a named-consumer-gated compatibility adapter.; Missing proof: Complete base and profile train, media-type compatibility decision, cross-SDK vectors, security negatives, and workflow E2E evidence. |
| `credentials.anoncreds-v1` | No approved SDK-Rust engine or complete facade. | Issue, store, present, and verify AnonCreds 1.0 credentials with revocation support. | Identus-owned Rust facade over a qualified engine; ledger, secret, packaging, and platform adapters remain injected. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | The SDK-TS embedded engine revision is evidence, not an approved SDK-Rust dependency.; Missing proof: Engine qualification, dependency and security review, target matrix, VDR and secret-port contracts, interoperability vectors, and E2E flows. |
| `credentials.status` | Partial portable status and revocation validation support. | Evaluate credential status and revocation evidence with explicit network policy. | Rust-owned selected status-profile validation with explicit bounded TypeScript retrieval and cache adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Network retrieval is injected and bounded rather than embedded in the core.; Missing proof: Exact status-profile selection, normative and negative vectors, privacy and cache policy tests, and presentation E2E evidence. |
| `presentations.exchange` | Partial presentation functionality without an accepted Presentation Exchange versus DCQL boundary. | Construct and evaluate presentation definitions, submissions, and format constraints. | Versioned Rust presentation-query semantics after the protocol boundary is selected, with TypeScript UI and query composition retained. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | No parity target is selected until Presentation Exchange versus DCQL ownership is decided.; Missing proof: Protocol and version decision, ownership boundary, normative vectors, format coverage, and end-to-end presentation evidence. |
| `openid4vc.issuance-wallet` | Substantial OpenID4VCI 1.0 Final wallet-side protocol core. | Execute wallet-side credential offer, authorization, token, proof, and credential flows. | Rust-owned Final-spec protocol state and validation with thin TypeScript redirect, callback, fetch, consent, and lifecycle adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Rust Final-spec behavior supersedes older donor-flow semantics.; Missing proof: Applicable donor E2E inventory, differential results, browser, Node.js, React web, and React Native target tests, error mapping, and rollback evidence. |
| `messaging.didcomm-v2` | Placeholder support; no qualified production DIDComm v2.1 engine is selected. | Pack, unpack, route, and resolve secrets for DIDComm v2 messages. | Identus-owned DIDComm v2.1 Rust facade over a qualified engine with injected resolver, secret, and transport ports. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | The SDK-TS embedded fork is not an approved SDK-Rust dependency.; Missing proof: Engine qualification, JOSE and attachment coverage, interoperability vectors, secret review, runtime matrix, and mediator E2E evidence. |
| `messaging.didcomm-application-protocols` | Placeholder protocol support without a complete versioned protocol catalog or production state machines. | Model and execute mediation, pickup, issue-credential, presentation, out-of-band, and basic-message interactions. | Versioned Rust messages and deterministic state machines per selected PIURI, role, and protocol version; product workflow remains host-owned. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Product workflow, consent, and lifecycle remain host effects.; Missing proof: Protocol catalog, message and state vectors, role and negative coverage, engine prerequisites, and Cloud Agent and Mediator E2E evidence. |
| `wallet.storage-contracts` | Substantial portable repository and domain-contract support, without ownership of a physical browser database. | Persist and query credentials, DIDs, keys, messages, settings, links, and issuance metadata through portable repository contracts. | Rust-owned repository semantics and typed ports with TypeScript database, reactive-query, and runtime adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Rust contracts do not mandate RIDB or a browser database.; Missing proof: DTO and query mapping, migration contracts, transaction and error semantics, credential repository canary, and multi-runtime E2E evidence. |
| `wallet.storage-browser` | Outside the portable Rust core except for storage ports and shared contracts. | Provide browser and Node persistence, query reactivity, encryption integration, and runtime lifecycle. | Platform-owned storage implementations conforming to Rust-defined ports and shared data contracts. | browser=blocked; nodejs=not-applicable; react-web=blocked; react-native=not-applicable | No duplicate Rust browser database implementation is planned.; Missing proof: Port-conformance suite, schema and migration tests, encryption and lifecycle evidence, and browser and React web runtime coverage. |
| `wallet.backup-format` | Partial backup-related support without an accepted versioned format and recovery contract. | Export and restore versioned wallet data while preserving relationships and cryptographic protection. | Versioned Rust-owned backup wire and cryptographic semantics with host-owned filesystem, download, storage, and recovery UX. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | No parity target exists until format and recovery ownership are accepted.; Missing proof: Threat model, format decision, compatibility policy, cross-SDK fixtures, corruption negatives, and full restore E2E evidence. |
| `agent.protocol-state` | Placeholder support; reusable production protocol state machines are not yet established. | Provide reusable connection and credential workflow state transitions independent of runtime scheduling. | Capability-specific deterministic Rust state machines reusable across native and WASM-shaped executors. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Host effects and scheduling are outside deterministic transition semantics.; Missing proof: Two unrelated protocol implementations, shared transition vectors, replay and error tests, host-effect separation, and E2E canaries. |
| `agent.runtime-orchestration` | Placeholder effect-driven runtime concepts without a completed bounded reference executor. | Coordinate jobs, events, fetch, lifecycle, user interaction, and plugin workflows in browser and Node runtimes. | Rust-owned deterministic events, transitions, correlation, checkpoints, and typed effects with thin TypeScript host executors. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Product lifecycle, consent, telemetry, and scheduling remain platform effects.; Missing proof: Reference executor, native and WASM-shaped fixtures, cancellation and checkpoint tests, effect-port mapping, and two consumer E2E proofs. |
| `platform.typescript-plugin-host` | Outside the Rust core by design. | Register, compose, and expose optional capability plugins through TypeScript package subpaths. | TypeScript-owned thin plugin host that composes Rust-backed capabilities without duplicating portable semantics. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Portable protocol semantics may move below the host, but the plugin host itself does not.; Missing proof: Architecture checks against duplicated domain logic, plugin compatibility tests, package-subpath tests, and consumer E2E evidence. |
| `platform.browser-node-networking` | Partial portable request, response, correlation, bounds, cancellation, and transport-port contracts. | Provide fetch, WebSocket, callback, redirect, and environment-specific network integration. | Rust-owned protocol and network contracts with isolated browser, Node.js, React web, and React Native transport adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Fetch, redirects, proxies, cancellation wiring, and credential policy differ by host.; Missing proof: Target-specific binding decisions, transport contract tests, cancellation, error, redirect, and proxy coverage, security review, and isolated target CI. |
| `platform.npm-packaging` | Outside Cargo distribution ownership; Rust and WASM artifacts are package inputs. | Publish ESM, CommonJS, declarations, plugin subpaths, and bundled private workspace packages for supported runtimes. | A capability-named TypeScript package that loads Rust artifacts per target without exposing historical component names. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Rust artifacts may be inputs, but npm distribution is not reproduced in Cargo.; Missing proof: Install and import tests, ESM, CommonJS, and declaration checks, bundler matrix, artifact provenance, target entry points, and legacy-name export guard. |
| `bindings.wasm-did` | Substantial experimental WASM DID value binding backed by mature Rust DID semantics. | Expose the portable DID domain and service capabilities to browser and Node TypeScript consumers through phased WASM facades. | Thin DID TypeScript facade using WASM in browser, Node.js, and React web; React Native uses a separate native binding under the same capability contract. | browser=partial; nodejs=blocked; react-web=blocked; react-native=not-applicable | WASM DTOs and JavaScript errors are adapters, not the canonical domain model.; Missing proof: Integrated package canary, shared TypeScript E2E suite, browser, Node.js, and React web matrix, React Native native-binding proof at the capability level, bundle metrics, and rollback proof. |

<!-- feature-comparison:end -->

The first activation remains issue #492: an opt-in DID and DID URL facade over
the existing WASM value surface. It proves the migration mechanism and rollback
boundary; it does not imply parity for DID documents, methods, credentials,
messaging, storage, or agent orchestration.

## Local-first capability trains

TS1 implementation should normally be delivered as one cohesive vertical train
per capability or tightly coupled capability cluster. The planning target is
**5,000–10,000 meaningful changed lines** carrying:

- the canonical Rust implementation and applicable public/binding adapter;
- normative, Identus-contract, compatibility, and negative vectors;
- migration, observability, fallback, and rollback behavior;
- documentation and generated-surface updates; and
- focused, mutation, differential, target, and integration tests.

This is not a merge gate or a line-count objective. Generated sources, vendored
code, lockfiles, and evidence artifacts do not count. Unrelated work must not
be bundled to reach the range. A train is split when work has independent
ownership, security or publication authority, rollback, or cannot be reviewed
safely as one unit. A naturally bounded change may be smaller; a larger train
needs an explicit cohesion and reviewability justification.

Signed internal commits are review checkpoints. Targeted local checks run while
the train is assembled; the complete applicable local gate qualifies the
candidate before push. Protected hosted CI remains mandatory final confirmation
and is never weakened or bypassed, but it is not the primary edit/test loop.

## Exit rule

TS1 exits only when each of the 24 records is one of:

- `delivered`, with every required conformance dimension, named consumer,
  observability, fallback, and rollback proved;
- `excluded` or `not-applicable`, with accepted ownership and non-duplication
  evidence; or
- moved to an explicitly accepted later milestone with a concrete blocker and
  no misleading parity claim.

Every delivered capability identifies its selected standard/profile, exact
Rust and SDK-TS revisions, shared vectors, DTO/error mapping, deviations,
resource and security bounds, selected targets, named consumer, and rollback.

## Explicit non-claims

TS1 does not authorize mdoc work, SDK-TS mutation by this repository, a default
binding switch, package publication, stable API status, or service/SDK
deprecation. It does not require historical Apollo, Castor, Mercury, Pluto,
Pollux, or Edge Agent module shapes in Rust. Browser storage, plugin hosting,
and npm packaging remain TypeScript responsibilities unless a later reviewed
decision changes their ownership.

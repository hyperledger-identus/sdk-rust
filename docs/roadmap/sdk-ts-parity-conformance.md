# TS1 — SDK-Rust/SDK-TS capability parity and conformance

**Status:** matrix status = `baseline`; delivered capabilities = 0; no runtime support is claimed

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

| Target | State | Runtime matrix | Binding | Artifact | Module format | Loader | Host ports | Consumer fixture | Support tier | CI job | Success evidence | Limits and non-claims |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `browser` | `target` | real Chromium; real Firefox; supported browser versions remain unselected | wasm-bindgen WASM over portable Rust crates | browser ESM WASM package | ESM | browser package export with bundler and direct-module fixtures | fetch; storage; time; entropy; custody; lifecycle | plain browser and client React application | target-not-supported | `target-browser` | wasm build; package import; real-browser E2E; bundle budget; rollback | limitations: SDK-LIM-002; SDK-LIM-003; non-claims: current runtime certification; publication; default route |
| `nodejs` | `target` | supported Node.js versions remain unselected; ESM and CommonJS consumers | Node-compatible WASM over portable Rust crates | Node.js WASM package entry | ESM and CommonJS compatibility entries | Node package exports with worker, filesystem, and WebCrypto policy | fetch; filesystem; storage; time; entropy; custody; workers; lifecycle | plain Node.js application | target-not-supported | `target-nodejs` | wasm build; package import; Node E2E; module-format matrix; rollback | limitations: SDK-LIM-002; non-claims: browser ESM equivalence; current runtime certification; publication |
| `react-web` | `target` | client React over browser target; SSR and hydration additionally require the Node.js target | browser WASM for client execution; Node.js contract for SSR | shared browser package plus React consumer fixture | consumer-bundler ESM | React provider or hook integration without domain-logic duplication | browser ports; React lifecycle; SSR boundary | client and SSR/hydration React applications | target-not-supported | `target-react-web` | client E2E; bundler matrix; SSR boundary E2E; hydration proof; rollback | limitations: SDK-LIM-002; SDK-LIM-003; non-claims: a second Rust artifact; React Native support; current runtime certification |
| `react-native` | `blocked` | iOS New Architecture; Android New Architecture; Hermes or other engines remain qualification inputs | ADR 0163 New Architecture/TurboModule package over a qualified native Rust binding | separate iOS and Android native artifacts behind one capability-oriented TypeScript contract | React Native package with generated native interfaces | TurboModule registration and lifecycle | native storage; native networking; secure custody; time; entropy; lifecycle | minimal iOS and Android React Native applications | blocked-not-supported | `target-react-native` | qualified native binding; iOS E2E; Android E2E; engine matrix; lifecycle and memory proof; rollback | limitations: SDK-LIM-002; ADR-0163; issue-223; non-claims: browser WASM reuse; current native binding support; publication |

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
   installing target toolchains. Only `success` satisfies a selected job;
   failure, cancellation, skip, timeout, or a missing result keeps the final
   `fast` aggregator red while preserving the independent Rust-core result.

Bindings and platform adapters depend inward on portable Rust crates only.
Target failures therefore block the target claim without erasing valid core
evidence, and path-only routing cannot suppress relevant binding work.
<!-- ci-contract:end -->

<!-- success-criteria:start -->
## Success criteria

| ID | Title | Pass condition | Evidence | Applicability | Not-applicable policy | Non-claims |
| --- | --- | --- | --- | --- | --- | --- |
| `SC-01` | Capability-oriented TypeScript contract | New public exports, package paths, and primary documentation use capability or standards names and contain none of the closed legacy-public-name vocabulary. | public API and package-export snapshot; case-insensitive legacy-name guard; migration-only exception inventory | all new Rust-backed TypeScript public surfaces | No exemption for new public names; donor-history and private legacy providers are outside the public scan. | identical Rust and TypeScript API shapes; removal of private legacy providers |
| `SC-02` | Target-specific build and runtime proof | Every claimed target has delivered build, artifact, loader, package, consumer E2E, resource, budget, and rollback evidence for its pinned runtime matrix. | target record; exact artifact identity; consumer fixture; accepted target budget; rollback rehearsal | each applicable target of a delivered capability | Requires owner, rationale, review or expiry trigger, and reconsideration rule in the capability record. | cross-target inference; support from compile-only evidence |
| `SC-03` | Applicable SDK-TS E2E reuse | Every discovered donor E2E test is ledgered and each shared journey runs one TypeScript assertion body against legacy and Rust-backed private providers on every applicable target. | donor revision path suite test ID and content hash; discovered-count equality; disposition and authority; no unledgered skip focus or quarantine | all SDK-TS E2E tests discovered from the pinned donor revision | Only adapted, platform-owned, superseded, or excluded with owner, rationale, replacement evidence, and reconsideration trigger. | all donor behavior is normative; unchanged execution for superseded profiles |
| `SC-04` | Rust evidence for additions and deviations | Additional or intentionally different portable behavior has Rust-owned normative, negative, resource-bound, and E2E or conformance evidence plus applicable quality-evidence dispositions. | rust-addition or rust-deviation ledger; selected profile; negative and boundary tests; TypeScript boundary smoke | every Rust addition or deviation from the donor journey | Requires a stable quality-evidence declaration or owner, rationale, review trigger, and reconsideration rule per evidence class. | TypeScript regressions define Rust semantics; coverage alone establishes authority |
| `SC-05` | Coverage and public-operation exercise | Every owned public operation maps to contract or E2E evidence; owned changed executable code reaches at least 80 percent line and 70 percent branch coverage where reliable and never regresses below a higher established package baseline. | public-operation inventory; pinned tool version; base head numerator denominator target and profile; coverage artifact; condition mutation and negative evidence where branch coverage is unavailable | owned changed executable Rust and TypeScript adapter code | Unsupported measures require an owner, tool limitation, strongest alternative evidence, and reconsideration trigger; invented percentages are forbidden. | coverage proves conformance; lowering an established higher baseline |
| `SC-06` | Type-safe and validated boundary | Pinned strict TypeScript declarations expose no explicit any, narrow unknown at ingress, validate wire values, and map numbers, bigint, bytes, null, undefined, errors, lifecycle, Promises, cancellation, and AbortSignal deterministically while Rust revalidates domain invariants. | strict compiler receipt; public declaration snapshot; runtime wire-validation tests; DTO error and API drift snapshots; lifecycle and cancellation tests | all owned TypeScript binding and facade surfaces | Platform-owned values still require strict host types and a bounded mapping rationale. | unknown is forbidden at ingress; TypeScript validation replaces Rust validation |
| `SC-07` | Shared portable semantics and thin adapters | Portable core crates contain no inward binding or platform dependency; bindings depend inward only and TypeScript duplicates no cryptography, canonical parsing, format, or protocol-state logic. | dependency-direction guard; semantic-duplication review; generated mapping drift test; host-port contract tests | all Rust-backed capabilities | Generated outer allocation bounds and explicit host policy are permitted when Rust remains authoritative. | one universal artifact; platform-owned behavior moves to Rust |
| `SC-08` | Operable target with accepted budgets | Before support, each target has accepted size, startup, latency, throughput, and memory budgets as applicable plus bounded diagnostics, observability, fallback, and rollback. | pinned runner and protocol; compressed and uncompressed artifact size; cold and warm p50 p95; representative latency throughput and memory; variance regression and rollback rules | each target support claim | A measure may be omitted only with owner, rationale, review or expiry trigger, and reconsideration rule; until a budget is accepted results remain measurement-only. | universal performance threshold; measurement activates support |
| `SC-09` | Rust-core-first isolated CI | The independent Rust-core job passes first, a fail-closed exact-head impact plan selects all dependent target jobs, selected jobs do not cancel siblings, and the existing protected fast aggregator is satisfied only when core and every selected target succeeds. | core package-set guard; API and schema fingerprints; impact-plan receipt; isolated target receipts; single fast aggregation receipt | every pull request after CI implementation | Unselected targets emit an inapplicable receipt without installing target tools; unknown impact selects all targets. | changing SDK-LIM-009; a target failure erases core evidence; path-only impact routing |

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

| Capability | Rust today | SDK-TS today | Rust owner | TypeScript owner | Target | B/N/RW/RN | Deviation / missing proof |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `crypto.key-operations` | Mature portable key generation, import/export, signing, verification, and shared-secret operations. | Generate, import, export, sign, verify, and derive shared secrets with supported key types. | identus-crypto | key-custody adapters and ergonomic facade | Rust-owned cryptographic operations behind a thin typed TypeScript key facade; secret custody remains host-owned. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Secret custody and key-object ergonomics remain TypeScript adapter responsibilities.; Missing proof: Cross-SDK vectors, error mapping, secret-handling review, and browser, Node.js, React web, and React Native target E2E evidence. |
| `crypto.hd-derivation` | Mature deterministic derivation and path semantics for the selected profiles. | Derive deterministic keys and paths from mnemonic or seed material. | identus-crypto | secret-source and custody adapters | Rust-owned derivation behind a thin TypeScript facade; mnemonic sources and custody remain host-owned. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | CIP-0003 applies only where an explicit profile selects it.; Missing proof: Frozen cross-language vectors, profile-specific negative tests, adapter E2E coverage, and target-runtime evidence. |
| `did.syntax` | Mature DID and DID URL parsing, validation, component access, and serialization with an existing WASM value surface. | Parse and serialize DID and DID URL values with stable component semantics. | identus-did | versioned compatibility facade and package ergonomics | Capability-named DID and DID URL TypeScript values backed by Rust semantics on supported targets. | browser=partial; nodejs=blocked; react-web=blocked; react-native=blocked | TypeScript constructors and error classes remain compatibility adapters.; Missing proof: Equivalent TypeScript E2E tests, constructor and error mapping, package loading, bundler evidence, and rollback proof. |
| `did.document` | Substantial portable DID document, verification method, service, and relationship model. | Represent DID documents, verification methods, services, and relationships. | identus-did | versioned compatibility facade and platform serialization ergonomics | Rust-owned DID document semantics exposed through generated or versioned typed TypeScript DTO mappings. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Method-specific extensions remain versioned adapters.; Missing proof: Round-trip vectors, stable field and error mapping, extension policy, and resolver-consumer E2E evidence. |
| `did.method.peer` | Bounded experimental SDK-Rust support for peer DID numalgo 4 and opt-in numalgo 2 creation, parsing, local resolution, and generic DID-document projection. | Create and resolve peer DIDs required by pairwise messaging relationships. | identus-did-peer | thin compatibility and host adapter during migration | Rust-owned deterministic creation and resolution for explicitly selected peer DID profiles with a thin TypeScript facade. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Only explicitly selected numalgo profiles are in scope.; Missing proof: Cross-SDK and DID-document equivalence, typed TypeScript facade and error mapping, pairwise messaging consumer E2E tests, per-target receipts, and rollback proof. |
| `did.method.prism` | Portable PRISM method semantics are consumer-proven downstream but are not yet a complete SDK-Rust core and binding surface. | Construct long-form Prism DIDs and resolve Prism identifiers through configured infrastructure. | identus-prism-did with separately selectable identus-prism-vdr and identus-prism-indexer capabilities | thin compatibility facade and host network adapter during transition | Rust-owned portable PRISM identifier and document semantics; network, ledger, persistence, and service composition stay injected. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Cardano observation, submission, persistence, and service composition are not SDK-Rust parity targets.; Missing proof: SDK-Rust integration boundary, cross-SDK vectors, typed binding surface, resolver adapter E2E tests, and rollback evidence. |
| `did.resolution` | Substantial resolver, cancellation, cache, and HTTP contract support with injected network policy. | Resolve identifiers through composable method and HTTP resolver contracts. | identus-did and identus-did-resolver-http | fetch policy and runtime adapters | Rust-owned resolution semantics behind injected browser, Node.js, React web, and React Native transport adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Browser and Node fetch policy remains platform-owned.; Missing proof: Named resolver canary, cache, cancellation, and error equivalence, bounded-network tests, and per-target E2E evidence. |
| `credentials.jwt-vc` | Partial credential support; the exact normative JWT VC profile is not yet fully pinned and delivered. | Parse, represent, issue, and verify JWT-encoded credentials and presentations. | identus-credentials | ergonomic facade and workflow integration | Rust-owned selected JWT VC and JOSE semantics with thin TypeScript workflow and ergonomic adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | No generic historical JWT VC behavior is normative without a selected profile.; Missing proof: Normative profile decision, issuance and verification vectors, negative cases, workflow E2E tests, and target-runtime proof. |
| `credentials.sd-jwt` | Partial RFC 9901 and draft-22 implementation behind an Identus-owned direction. | Issue, present, and verify selectively disclosed JWT claims and the selected VC profile. | identus-credentials behind an Identus-owned facade | facade and compatibility adapter | Rust-owned RFC 9901 and selected VC-profile semantics with a capability-named TypeScript facade. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | dc+sd-jwt is canonical; vc+sd-jwt is only a named-consumer-gated compatibility adapter.; Missing proof: Complete base and profile train, media-type compatibility decision, cross-SDK vectors, security negatives, and workflow E2E evidence. |
| `credentials.anoncreds-v1` | No approved SDK-Rust engine or complete facade. | Issue, store, present, and verify AnonCreds 1.0 credentials with revocation support. | identus-credentials facade over a qualified engine | facade, ledger/VDR, and runtime packaging adapters | Identus-owned Rust facade over a qualified engine; ledger, secret, packaging, and platform adapters remain injected. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | The SDK-TS embedded engine revision is evidence, not an approved SDK-Rust dependency.; Missing proof: Engine qualification, dependency and security review, target matrix, VDR and secret-port contracts, interoperability vectors, and E2E flows. |
| `credentials.status` | Partial portable status and revocation validation support. | Evaluate credential status and revocation evidence with explicit network policy. | identus-credentials | network and cache policy adapters | Rust-owned selected status-profile validation with explicit bounded TypeScript retrieval and cache adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Network retrieval is injected and bounded rather than embedded in the core.; Missing proof: Exact status-profile selection, normative and negative vectors, privacy and cache policy tests, and presentation E2E evidence. |
| `presentations.exchange` | Partial presentation functionality without an accepted Presentation Exchange versus DCQL boundary. | Construct and evaluate presentation definitions, submissions, and format constraints. | identus-presentations | facade and UI query construction | Versioned Rust presentation-query semantics after the protocol boundary is selected, with TypeScript UI and query composition retained. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | No parity target is selected until Presentation Exchange versus DCQL ownership is decided.; Missing proof: Protocol and version decision, ownership boundary, normative vectors, format coverage, and end-to-end presentation evidence. |
| `openid4vc.issuance-wallet` | Substantial OpenID4VCI 1.0 Final wallet-side protocol core. | Execute wallet-side credential offer, authorization, token, proof, and credential flows. | identus-oid4vci | browser redirects, callback, fetch, and consent integration | Rust-owned Final-spec protocol state and validation with thin TypeScript redirect, callback, fetch, consent, and lifecycle adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Rust Final-spec behavior supersedes older donor-flow semantics.; Missing proof: Applicable donor E2E inventory, differential results, browser, Node.js, React web, and React Native target tests, error mapping, and rollback evidence. |
| `messaging.didcomm-v2` | Placeholder support; no qualified production DIDComm v2.1 engine is selected. | Pack, unpack, route, and resolve secrets for DIDComm v2 messages. | identus-messaging DIDComm v2.1 facade over a qualified engine | secret resolution, transport, and runtime adapters | Identus-owned DIDComm v2.1 Rust facade over a qualified engine with injected resolver, secret, and transport ports. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | The SDK-TS embedded fork is not an approved SDK-Rust dependency.; Missing proof: Engine qualification, JOSE and attachment coverage, interoperability vectors, secret review, runtime matrix, and mediator E2E evidence. |
| `messaging.didcomm-application-protocols` | Placeholder protocol support without a complete versioned protocol catalog or production state machines. | Model and execute mediation, pickup, issue-credential, presentation, out-of-band, and basic-message interactions. | versioned protocol-specific Rust state machines | thin host executor, product workflow, consent, and lifecycle adapters | Versioned Rust messages and deterministic state machines per selected PIURI, role, and protocol version; product workflow remains host-owned. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Product workflow, consent, and lifecycle remain host effects.; Missing proof: Protocol catalog, message and state vectors, role and negative coverage, engine prerequisites, and Cloud Agent and Mediator E2E evidence. |
| `wallet.storage-contracts` | Substantial portable repository and domain-contract support, without ownership of a physical browser database. | Persist and query credentials, DIDs, keys, messages, settings, links, and issuance metadata through portable repository contracts. | identus-wallet | database adapter and reactive query integration | Rust-owned repository semantics and typed ports with TypeScript database, reactive-query, and runtime adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Rust contracts do not mandate RIDB or a browser database.; Missing proof: DTO and query mapping, migration contracts, transaction and error semantics, credential repository canary, and multi-runtime E2E evidence. |
| `wallet.storage-browser` | Outside the portable Rust core except for storage ports and shared contracts. | Provide browser and Node persistence, query reactivity, encryption integration, and runtime lifecycle. | language-sdk | browser and Node storage adapter | Platform-owned storage implementations conforming to Rust-defined ports and shared data contracts. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=not-applicable | No duplicate Rust browser database implementation is planned.; Missing proof: Port-conformance suite, schema and migration tests, encryption and lifecycle evidence, and browser, Node.js, and React web runtime coverage. |
| `wallet.backup-format` | Partial backup-related support without an accepted versioned format and recovery contract. | Export and restore versioned wallet data while preserving relationships and cryptographic protection. | identus-wallet | filesystem/download, platform storage, and UX integration | Versioned Rust-owned backup wire and cryptographic semantics with host-owned filesystem, download, storage, and recovery UX. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | No parity target exists until format and recovery ownership are accepted.; Missing proof: Threat model, format decision, compatibility policy, cross-SDK fixtures, corruption negatives, and full restore E2E evidence. |
| `agent.protocol-state` | Placeholder support; reusable production protocol state machines are not yet established. | Provide reusable connection and credential workflow state transitions independent of runtime scheduling. | capability-specific versioned Rust protocol state machines | product workflow and compatibility composition | Capability-specific deterministic Rust state machines reusable across native and WASM-shaped executors. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Host effects and scheduling are outside deterministic transition semantics.; Missing proof: Two unrelated protocol implementations, shared transition vectors, replay and error tests, host-effect separation, and E2E canaries. |
| `agent.runtime-orchestration` | Placeholder effect-driven runtime concepts without a completed bounded reference executor. | Coordinate jobs, events, fetch, lifecycle, user interaction, and plugin workflows in browser and Node runtimes. | portable effect-driven Rust agent runtime | thin host executor, consent, product policy, and lifecycle adapter | Rust-owned deterministic events, transitions, correlation, checkpoints, and typed effects with thin TypeScript host executors. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Product lifecycle, consent, telemetry, and scheduling remain platform effects.; Missing proof: Reference executor, native and WASM-shaped fixtures, cancellation and checkpoint tests, effect-port mapping, and two consumer E2E proofs. |
| `platform.typescript-plugin-host` | Outside the Rust core by design. | Register, compose, and expose optional capability plugins through TypeScript package subpaths. | language-sdk | plugin host, registration, and subpath exports | TypeScript-owned thin plugin host that composes Rust-backed capabilities without duplicating portable semantics. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Portable protocol semantics may move below the host, but the plugin host itself does not.; Missing proof: Architecture checks against duplicated domain logic, plugin compatibility tests, package-subpath tests, and consumer E2E evidence. |
| `platform.browser-node-networking` | Partial portable request, response, correlation, bounds, cancellation, and transport-port contracts. | Provide fetch, WebSocket, callback, redirect, and environment-specific network integration. | identus-ports transport contracts plus selected target adapters | thin fetch/WebSocket host adapter, platform I/O policy, and migration facade | Rust-owned protocol and network contracts with isolated browser, Node.js, React web, and React Native transport adapters. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Fetch, redirects, proxies, cancellation wiring, and credential policy differ by host.; Missing proof: Target-specific binding decisions, transport contract tests, cancellation, error, redirect, and proxy coverage, security review, and isolated target CI. |
| `platform.npm-packaging` | Outside Cargo distribution ownership; Rust and WASM artifacts are package inputs. | Publish ESM, CommonJS, declarations, plugin subpaths, and bundled private workspace packages for supported runtimes. | language-sdk | npm publication and runtime compatibility | A capability-named TypeScript package that loads Rust artifacts per target without exposing historical component names. | browser=blocked; nodejs=blocked; react-web=blocked; react-native=blocked | Rust artifacts may be inputs, but npm distribution is not reproduced in Cargo.; Missing proof: Install and import tests, ESM, CommonJS, and declaration checks, bundler matrix, artifact provenance, target entry points, and legacy-name export guard. |
| `bindings.wasm-did` | Substantial experimental WASM DID value binding backed by mature Rust DID semantics. | Expose the portable DID domain and service capabilities to browser and Node TypeScript consumers through phased WASM facades. | identus-wasm-did | thin opt-in facade, package loading, versioned compatibility mapping, and rollback | Thin DID TypeScript facade using WASM in browser, Node.js, and React web; React Native uses a separate native binding under the same capability contract. | browser=partial; nodejs=blocked; react-web=blocked; react-native=not-applicable | WASM DTOs and JavaScript errors are adapters, not the canonical domain model.; Missing proof: Integrated package canary, shared TypeScript E2E suite, browser, Node.js, and React web matrix, React Native native-binding proof at the capability level, bundle metrics, and rollback proof. |

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

# Tasks: Identus Rust SDK Platform Core

**Input**: Design documents from `specs/001-sdk-rust-platform-core/`

**Prerequisites**: `spec.md`, `research.md`, `plan.md`

**Tests**: Phase 1 is compile-only. Protocol tests start after fixture schema
approval.

## Phase 1: Workspace And Architecture Skeleton

**Goal**: Establish a reviewed Cargo workspace and component map without
production protocol implementation.

- [x] T001 Add root Cargo workspace metadata and shared lint policy.
- [x] T002 Add compile-only component crates for core, crypto, DID, trust,
  credentials, presentations, messaging, OpenID4VC, wallet, adapters, bindings.
- [x] T003 Add component metadata exports so each crate has an initial public
  surface and `cargo check --workspace` can validate dependency direction.
- [x] T004 Add component and relation architecture diagrams.
- [x] T005 Add docs-derived specification baseline from `repos/docs`.
- [x] T006 Add deterministic PRISM DID fixture using the docs test vector.
- [x] T007 Add a legacy SDK migration matrix mapping each public module to the
  Rust crate family.
- [x] T008 Add fixture schema ADR covering DID/key/credential/protocol vectors.

  Acceptance criteria:
  - `docs/architecture/adr-fixture-schema-policy.md` defines common fixture
    fields for schema version, specification id, source evidence, owner crate,
    expected outcome, and redaction policy.
  - The ADR covers DID/key vectors, credential and presentation vectors,
    DIDComm transcripts, and OpenID4VC transcripts.
  - `docs/testing/fixtures.md` and `fixtures/conformance/README.md` link the
    ADR as the governing schema policy.
  - `identus-conformance` checks that the ADR covers required domains,
    redaction, source evidence, ownership, and transcript expectations.

## Phase 2A: BDD Acceptance Seed

**Goal**: Convert the existing BDD scenario corpus into Docker-free Rust
acceptance-test scaffolding before heavy infrastructure is introduced.

- [x] T009 Scan `cloud-agent`, `sdk-ts`, `sdk-kmp`, and `docs` feature files
  and group scenarios by feature, capability, and use case.
- [x] T010 Add `identus-agent` crate with issuer, holder, verifier, peer, and
  embedded mediator queue boundaries.
- [x] T011 Add Rust acceptance tests for connection, issuance, proof,
  revocation, backup/restore, and restored-agent mediator pickup.
- [ ] T012 Add every newly discovered useful deliverable to this task backlog
  during the same iteration that discovers it.
- [x] T013 Add connectionless OOB credential and proof fixtures on top of the
  embedded agent model.

  Acceptance criteria:
  - `identus-agent` has acceptance tests for connectionless credential issuance
    and connectionless proof presentation that do not create persistent
    connections between issuer/holder or verifier/holder.
  - `fixtures/conformance/transcript/didcomm/connectionless-oob-credential-proof.json`
    records OOB invitation, issue-credential, and present-proof message
    sequences with parent-thread links and redacted attachments.
  - `identus-conformance` loads the connectionless fixture and validates message
    type parsing plus the no-persistent-connection expectation.
  - `docs/testing/bdd-scenarios.md` names connectionless OOB as covered by the
    first Rust acceptance harness.
- [ ] T014 Add Cloud Agent DID lifecycle scenario mappings to concrete
  `identus-did` crate tasks: create, publish, list, update services, update
  keys, reject disallowed key purposes, and deactivate.
- [ ] T015 Add VDR scenario mappings to concrete crate tasks: memory driver
  create/resolve/share/update/delete, database driver parity, and optional
  ledger-backed driver gated behind minimal infrastructure.
- [ ] T016 Add OID4VCI scenario mappings to concrete `identus-openid4vc` crate
  tasks: manage issuer, manage credential configuration, metadata endpoint
  behavior, authorization-code JWT issuance, long-form DID issuance, and
  validation failures.
- [ ] T017 Add credential verification API and verification policy scenario
  mappings to concrete `identus-credentials`, `identus-presentations`, and
  `identus-trust` crate tasks.
- [ ] T018 Add Cloud Agent schema and multitenancy scenario mappings to service
  reuse backlog: credential schema CRUD, wallet creation constraints, health,
  metrics, and future Rust Cloud-Service ownership.
- [ ] T019 Add an acceptance-test runner strategy that can execute the
  Docker-free `identus-agent` suite by default and opt into minimal Docker only
  for ledger/database/OIDC infrastructure scenarios.
- [x] T020 Scan the `integration` repository and classify relevant deliverables
  for `sdk-rust` rather than treating integration-runner unit tests as SSI BDD.
- [x] T021 Add a `sdk-rust` integration-runner contract covering clone/build,
  selected version checkout, `cargo test`, and Allure-compatible result export.

  Acceptance criteria:
  - `docs/testing/integration-runner-contract.md` defines the `sdk-rust` runner
    key, repository, default embedded mode, optional external mode, version
    checkout lifecycle, Cargo command, Allure source directory, and `tmp/sdk-rust`
    export path.
  - The contract maps the existing integration matrix rows to initial
    `sdk-rust` flow labels and states when rows should be skipped.
  - `identus-conformance` checks the runner key, repository, command, result
    directories, and flow labels.

- [x] T022 Add environment compatibility for integration flows:
  `AGENT_URL`, `MEDIATOR_OOB_URL`, optional `TEST_RUNNER_PRISM_AGENT_URL`,
  embedded mediator mode, and external service mode.

  Acceptance criteria:
  - `docs/testing/integration-runner-contract.md` defines `SDK_RUST_MODE`,
    `AGENT_URL`, `MEDIATOR_OOB_URL`, `TEST_RUNNER_PRISM_AGENT_URL`,
    `TEST_RUNNER_MEDIATOR_OOB_URL`, `SDK_RUST_ALLURE_DIR`, and
    `SDK_RUST_SELECTED_FLOWS`.
  - Embedded mode requires no Cloud Agent, Mediator, Docker, PostgreSQL,
    Keycloak, PRISM Node, or NeoPRISM.
  - External mode requires service URLs and must fail fast with redaction-safe
    errors when they are missing.
  - `identus-conformance` checks the environment contract names.
- [ ] T023 Add release compatibility gates matching `integration`: component
  under test, released services, weekly all-main testing, manual version matrix,
  and draft/final release metadata.
- [ ] T024 Add dashboard/report metadata requirements so Rust acceptance
  results can be aggregated with sdk-ts, sdk-swift, sdk-kmp, cloud-agent, and
  mediator reports.
- [ ] T025 Add a wrapper parity gate that blocks claiming replacement of
  sdk-ts, sdk-swift, or sdk-kmp behavior until the Rust runner covers the
  integration matrix rows those SDKs currently pass or skip.

## Phase 2B: Specification Conformance Seed

**Goal**: Ensure every supported specification has an executable conformance
entry before protocol implementation starts.

- [x] T026 Add `identus-conformance` crate with a specification catalog and
  executable metadata tests.
- [x] T027 Register planned supported specifications from the docs, research,
  BDD, and integration baselines.
- [x] T028 Require every specification to have a stable id, owner crate, source
  URI, support stage, conformance mode, and backlog task.
- [x] T029 Add checked-in fixture directories for vector, transcript, interop,
  static-model, and optional infrastructure conformance tests.
- [ ] T030 Replace metadata-only entries with behavior tests as each protocol
  crate lands.

## Phase 2: Fixture And Migration Design

**Goal**: Make behavior portable before implementing protocol logic.

- [x] T031 Define shared fixture directory layout and JSON schema policy.
- [x] T032 Add deterministic PRISM DID fixture requirements across all SDKs.
- [x] T033 Add credential verification negative-case fixture requirements.

  Acceptance criteria:
  - `fixtures/conformance/vector/credential-verification/negative-cases.json`
    defines redacted negative cases for expired credentials, not-before
    violations, wrong audience, wrong domain, wrong challenge, unsupported key
    purpose, revoked status, tampered signature, malformed schema, missing
    holder binding, unsupported format, and missing selective disclosure.
  - Each case has a stable `case_id`, capability, redacted input reference,
    credential or presentation format, expected typed error, and policy context.
  - The fixture names affected specification ids across VC JSON Schema, JWT VC,
    SD-JWT, SD-JWT VC, Presentation Exchange, DCQL, and status-list specs.
  - `identus-conformance` validates the fixture against the vector schema and
    checks the required negative-case ids and typed error families.
- [x] T034 Add DIDComm/OID4VC protocol transcript fixture requirements.

  Acceptance criteria:
  - `docs/architecture/adr-fixture-schema-policy.md` defines required fields
    for DIDComm transcripts, including participants, ordered messages,
    threading, expected states, and redaction policy.
  - The ADR names the required DIDComm transcript families: OOB,
    BasicMessage, Trust Ping, Discover Features, Routing/Forward, Coordinate
    Mediation, Message Pickup, Issue Credential, Present Proof, Report Problem,
    Revocation Notification, and legacy Identus compatibility aliases.
  - The ADR defines OpenID4VC transcript requirements for OID4VCI, OID4VP,
    SIOPv2, HAIP, federation-backed trust, supported credential formats, and
    negative cases.
  - `identus-conformance` checks these transcript requirements before protocol
    replay implementation starts.
- [x] T035 Document sdk-ts, sdk-swift, sdk-kmp, and neoprism migration map.

## Phase 3: Foundation Crates

**Goal**: Implement low-level primitives with strict tests.

- [ ] T036 Implement core DTO/error conventions.
- [ ] T037 Implement crypto signer and key-handle traits.
- [ ] T038 Implement DID parsing and resolver traits.
- [ ] T039 Implement trust/status policy traits.

## Phase 4: Credential And Presentation Core

**Goal**: Implement format-neutral credential and presentation behavior.

- [x] T040 Implement credential model traits and format registry.

  Acceptance criteria:
  - `identus-credentials` exposes typed `CredentialFormatId`,
    `CredentialFormatFamily`, `CredentialFormatStage`,
    `CredentialFormatProfile`, `CredentialDescriptor`,
    `CredentialFormatCapability`, `CredentialFormatRegistry`, and
    `StaticCredentialFormatRegistry` boundaries.
  - The static registry covers `jwt_vc_json`, `sd_jwt_vc`, `w3c_vc_json`,
    `anoncreds`, `openbadges_3`, and `iso_mdoc` with specification ids,
    support stages, DIDComm issue-credential support, OpenID4VCI issuance
    support, and wallet-storage support.
  - `identus-conformance` validates the registry and descriptor without
    requiring credential parsers, pack/unpack crypto, OIDC providers,
    databases, ledgers, or production credentials.
- [ ] T041 Implement status/revocation abstraction.
- [ ] T042 Implement Presentation Exchange and DCQL model boundaries.
- [ ] T043 Add OpenBadges and mdoc profile design documents.

## Phase 5: Protocol State Machines

**Goal**: Add DIDComm and OpenID4VC behavior behind stable ports.

- [x] T044 Implement DIDComm message and protocol state boundaries.

  Acceptance criteria:
  - `identus-messaging` exposes typed state, event, transition, and state
    machine boundaries for Issue Credential 3.0, Present Proof 3.0, Report
    Problem 2.0, Trust Ping 2.0, and Identus Revocation Notification 1.0.
  - `fixtures/conformance/transcript/didcomm/credential-presentation-revocation.json`
    drives credential offer, request, issuance, presentation request,
    presentation submission, problem report, and revocation notification
    transitions without Docker, Cloud Agent, Mediator, databases, external
    queues, pack/unpack crypto, or production DIDs.
  - The mediation transcript drives Trust Ping acknowledgement and invalid
    DIDComm protocol sequencing returns a typed, redaction-safe
    `invalid_didcomm_transition` error.
- [x] T045 Implement mediator coordination and pickup state boundaries.

  Acceptance criteria:
  - `identus-messaging` exposes typed `MediationCoordinationState`,
    `MediationCoordinationEvent`, `MediationCoordinationTransition`,
    `MediationCoordinationStateMachine`, `MessagePickupState`,
    `MessagePickupEvent`, `MessagePickupTransition`,
    `MessagePickupStateMachine`, `MediatorRoutingState`,
    `MediatorRoutingEvent`, `MediatorRoutingTransition`, and
    `MediatorRoutingStateMachine` boundaries.
  - `fixtures/conformance/transcript/didcomm/edge-mediation-and-pickup.json`
    drives mediation grant, recipient key registration, routing forward, and
    message pickup completion without Docker, HTTP mediators, WebSockets,
    external queues, databases, or production DIDs.
  - Compatibility aliases for `mediator-coordination/2.0` and `pickup/3.0`
    classify through the same mediation and pickup event ports, and invalid
    sequencing returns a typed, redaction-safe `invalid_didcomm_transition`
    error.
- [x] T046 Implement OID4VCI state machine.

  Acceptance criteria:
  - `identus-openid4vc` exposes typed `CredentialIssuanceFlowKind`,
    `CredentialIssuanceState`, `CredentialIssuanceEvent`,
    `CredentialIssuanceTransition`, and `CredentialIssuanceStateMachine`
    boundaries for authorization-code, pre-authorized-code, and deferred
    credential issuance.
  - `fixtures/conformance/transcript/openid4vc/core-flows.json` drives the
    OID4VCI state machine without Docker, real HTTP, OIDC providers, browsers,
    device APIs, trust-anchor services, or production credentials.
  - Invalid OID4VCI transitions return a typed, redaction-safe
    `invalid_openid4vc_transition` error.
- [x] T047 Implement OID4VP/SIOPv2 state machine.

  Acceptance criteria:
  - `identus-openid4vc` exposes typed `PresentationFlowKind`,
    `PresentationState`, `PresentationEvent`, `PresentationTransition`,
    `PresentationStateMachine`, `SelfIssuedOpenIdProviderState`,
    `SelfIssuedOpenIdProviderEvent`, `SelfIssuedOpenIdProviderTransition`,
    and `SelfIssuedOpenIdProviderStateMachine` boundaries.
  - `fixtures/conformance/transcript/openid4vc/core-flows.json` drives
    same-device OID4VP `direct_post`, cross-device OID4VP `direct_post.jwt`,
    and `SIOPv2` self-issued ID token transitions without Docker, real HTTP,
    browsers, device APIs, OIDC providers, or trust-anchor services.
  - Wrong-nonce OID4VP responses return the typed, redaction-safe
    `nonce_mismatch` verification error while invalid sequencing returns
    `invalid_openid4vc_transition`.

## Phase 6: Adapters And Bindings

**Goal**: Expose the Rust core to products without duplicating semantics.

- [ ] T048 Add in-memory and SQLite storage adapters.
- [ ] T049 Add HTTP and neoprism VDR adapters.
- [ ] T050 Add WASM and Node binding proof-of-concept.
- [ ] T051 Add UniFFI DTO proof-of-concept for Swift and Kotlin wrappers.
- [ ] T052 Compute and pin deterministic PRISM DID method-specific ID and
  long-form DID after the Rust `AtalaOperation` protobuf serializer lands.
- [ ] T053 Add a machine-readable migration manifest that mirrors
  `docs/migration/legacy-sdk-map.md` and can be consumed by conformance tests.
- [x] T054 Add wrapper API parity inventories for TS, Swift, and Kotlin
  packages so replacement gates can compare public exports per wrapper.

  Acceptance criteria:
  - `fixtures/conformance/interop/wrapper-api-parity.json` records public
    exports from local sdk-ts, sdk-swift, and sdk-kmp source files.
  - Each inventory entry names the source export, source path, owner Rust
    crate, binding surface, migration disposition, and fixture or backlog
    coverage.
  - `identus-conformance` validates the inventory as an interop fixture and
    fails if an entry lacks owner, migration, binding, or coverage metadata.
- [x] T055 Add neoprism convergence ADR for crate reuse, crate moves, VDR
  ownership, and future service thinning.

  Acceptance criteria:
  - `docs/architecture/adr-neoprism-convergence.md` maps NeoPRISM crates
    `identus-apollo`, `identus-did-core`, `identus-did-prism`,
    `identus-did-prism-ledger`, `identus-did-prism-indexer`,
    `identus-did-prism-submitter`, `identus-did-resolver-http`,
    `node-storage`, and `neoprism-node` to sdk-rust crate ownership.
  - The ADR defines VDR ownership across `identus-did`, `identus-trust`,
    `identus-adapters`, and future Rust services.
  - The ADR preserves NeoPRISM quality attributes including conformance tests,
    coverage, SonarCloud, OpenSSF Scorecard, OpenSSF Best Practices, release
    automation, Docker/Nix packaging, file hygiene, and linting.
  - The ADR defines migration phases for fixture/port alignment, core module
    porting, adapter porting, and service thinning.
  - `identus-conformance` checks the ADR for crate mapping, ownership, quality
    gates, VDR ownership, and migration phases.
- [x] T056 Add platform secure-storage ADR for browser, Node, iOS, Android,
  JVM, and server stores before binding crates stabilize.

  Acceptance criteria:
  - `docs/architecture/adr-secure-storage.md` defines secure storage as an
    adapter boundary owned by `identus-wallet` and `identus-adapters`, with
    protocol crates depending only on typed ports.
  - The ADR covers secret classes, `SecureStore`, `KeyStore`,
    `SecretResolver`, `BackupStore`, and `EntropySource` ports.
  - Browser/WASM, Node, iOS, Android, JVM, and server adapter requirements are
    documented before wrapper APIs stabilize.
  - The ADR requires redaction-safe diagnostics, non-exporting key handles by
    default, Docker-free deterministic in-memory tests, opt-in infrastructure
    tests, and backup/restore conformance.
  - `identus-conformance` checks the ADR for platform coverage, secret classes,
    ports, safety rules, and conformance expectations.
- [x] T057 Add plugin and extension compatibility ADR for the TS plugin surface.

  Acceptance criteria:
  - `docs/architecture/adr-plugin-extension-compatibility.md` maps the
    TypeScript `Plugin`, `PluginManager`, and `Plugins.Task` surface to a typed
    Rust extension registry.
  - The ADR defines module descriptors, protocol task descriptors, task
    context, typed result envelopes, capability permissions, and redaction-safe
    errors.
  - The ADR maps DIDComm, AnonCreds, DIF, OIDC, and compatibility plugin
    families to Rust owner crates without allowing wrapper-owned protocol
    semantics.
  - `identus-conformance` checks the ADR for source evidence, registry
    descriptors, safety rules, fixture requirements, and wrapper compatibility
    expectations.
- [ ] T086 Generate a machine-readable plugin task parity inventory from
  sdk-ts plugin registrations.

  Acceptance criteria:
  - A checked-in generator scans sdk-ts internal plugin registrations for
    module names, protocol ids, composite task keys, and task classes.
  - The generated fixture maps each plugin task to owner crate, capability id,
    typed error family, fixture coverage, migration disposition, and binding
    target.
  - CI fails when a TypeScript plugin registration changes without an updated
    Rust parity fixture.
- [x] T087 Add capability-driven API analysis for `sdk-rust` conformance and
  public API design.

  Acceptance criteria:
  - `docs/architecture/adr-capability-driven-api.md` explains why the Rust API
    must not be derived as the union or intersection of existing SDK APIs.
  - The ADR defines capability contracts as the conformance source of truth:
    roles, state transitions, expected outputs, typed errors, security
    properties, fixtures, and binding parity expectations.
  - The ADR recommends layered API families for domain primitives, ports,
    protocol state machines, capability services, and binding facades.
  - `identus-conformance` checks that wrapper API inventories are treated as
    compatibility gates rather than API design inputs.
- [x] T088 Define machine-readable capability contracts for the first stable
  Rust public API surfaces.

  Acceptance criteria:
  - Contracts cover issuer, holder, verifier, peer, wallet, mediator, DID,
    credential, presentation, DIDComm, OpenID4VC, trust/status, storage, and
    binding facade surfaces.
  - Each contract names owner crate, required ports, role inputs, outputs,
    typed errors, fixture families, wrapper targets, and acceptance tests.
  - `identus-conformance` fails when a public API surface lacks a capability
    contract.
- [x] T089 Promote capability contracts into an `identus-bindings` typed
  registry and generated wrapper manifest.

  Acceptance criteria:
  - `identus-bindings` exposes typed capability contract metadata derived from
    `fixtures/conformance/static-model/capability-contracts.json`.
  - The registry can emit JSON manifests for TypeScript, Swift, Kotlin, React,
    React Native, Node, and WASM binding packages.
  - Conformance tests fail if fixture contract ids diverge from the typed
    registry or generated wrapper manifests.
- [x] T090 Document the current workspace architecture and module dependency
  graph.

  Acceptance criteria:
  - `docs/architecture/current-workspace.md` lists every current workspace crate,
    its responsibility, implementation depth, production dependencies, and
    test-only workspace dependencies.
  - The document includes Mermaid diagrams for repository shape, production
    dependency graph, and hexagonal boundary layers.
  - `identus-conformance` fails if the current architecture document omits a
    workspace crate or manifest-derived dependency edge.
- [x] T091 Add machine-readable workspace dependency graph evidence for the
  current hexagonal architecture.

  Acceptance criteria:
  - `tools/generate-workspace-dependency-graph.mjs` generates
    `fixtures/conformance/static-model/workspace-dependency-graph.json` from
    `cargo metadata --format-version 1 --no-deps`.
  - The fixture records crate layers, production dependencies, dev-only
    workspace dependencies, production edges, and hexagonal boundary
    constraints.
  - CI and `identus-conformance` fail when the checked-in graph drifts from the
    live Cargo workspace or violates the core/domain/outer-boundary dependency
    rules.
- [x] T092 Add explicit zero-violation policy evidence to the workspace
  dependency graph fixture.

  Acceptance criteria:
  - `tools/generate-workspace-dependency-graph.mjs` records allowed production
    target layers for each architecture layer.
  - The generated fixture includes `production_policy_violations` and
    `production_cycle_violations` arrays that must be empty for the current
    workspace.
  - `identus-conformance` fails if the graph fixture omits policy evidence,
    contains production layer violations, or contains production dependency
    cycles.
- [x] T093 Add initial `identus-core` error and result DTO conventions.

  Acceptance criteria:
  - `identus-core` exposes stable `CapabilityId`, `ErrorCode`, `ErrorKind`,
    `RedactionPolicy`, `IdentusError`, `IdentusResult`, `ResultEnvelope`, and
    `ErrorEnvelope` types without adding non-core workspace dependencies.
  - `IdentusError` display output is redaction-safe and contains only stable
    error code plus public message.
  - `ResultEnvelope` maps typed errors into binding-safe DTO fields for future
    WASM, Node, UniFFI, Swift, Kotlin, TypeScript, React, and React Native
    wrappers.
  - `docs/architecture/core-error-conventions.md` documents ownership, binding
    rules, redaction rules, and initial error families.
  - `identus-conformance` fails if the core convention doc or core public type
    surface is removed.
- [x] T094 Map DID parser errors into the shared core typed error surface.

  Acceptance criteria:
  - `identus-did` keeps `DidParseError` for Rust parser and `FromStr`
    compatibility while exposing conversion into `IdentusError` and
    `ErrorEnvelope`.
  - DID and DID URL parser failures map to stable typed codes:
    `missing_did_scheme`, `invalid_did_method`,
    `invalid_did_method_specific_id`, `did_url_components_not_allowed`, and
    `invalid_did_url_component`.
  - `Did::parse_with_core_error` and `DidUrl::parse_with_core_error` return
    `IdentusResult` for binding and capability-service use.
  - Unit and conformance tests fail if DID parser errors stop exposing
    redaction-safe core error codes.
- [x] T095 Map DIDComm parser and addressing errors into the shared core typed
  error surface.

  Acceptance criteria:
  - `identus-messaging` keeps `DidCommParseError` for Rust parser and `FromStr`
    compatibility while exposing conversion into `IdentusError` and
    `ErrorEnvelope`.
  - DIDComm parser failures map to stable typed codes:
    `missing_didcomm_prefix`, `missing_didcomm_path_segment`,
    `invalid_didcomm_path_segment`, `invalid_didcomm_version`,
    `unexpected_didcomm_path_segment`, `invalid_didcomm_identifier`, and
    `missing_didcomm_recipient`.
  - `DidCommMessageType::parse_with_core_error`,
    `DidCommMessageId::parse_with_core_error`,
    `DidCommThreadId::parse_with_core_error`, and
    `DidCommPlaintextMessage::addressed_with_core_error` return
    `IdentusResult` for binding and transcript-replay use.
  - Unit and conformance tests fail if DIDComm parser errors stop exposing
    redaction-safe core error codes.
- [x] T096 Add a machine-readable typed error catalog fixture for DID and
  DIDComm parser surfaces.

  Acceptance criteria:
  - `fixtures/conformance/static-model/typed-error-catalog.json` records DID
    and DIDComm parser codes, source variants, redaction-safe public messages,
    binding targets, local Rust error types, and core DTO mappings.
  - The catalog satisfies the static-model fixture schema and contains no user
    input, production DIDs, protocol payloads, keys, tokens, credentials, or
    private diagnostics.
  - `identus-conformance` fails if cataloged codes drift from Rust source,
    documentation, or the completed backlog task.
  - Wrapper consumers can use the fixture as a compatibility gate for
    TypeScript, Swift, Kotlin, Node, WASM, React, and React Native error
    mappings.
- [x] T097 Add an Agentic SDLC harness for multi-agent GitHub Issue,
  Discussion, Pull Request, and repository-maintenance workflows.

  Acceptance criteria:
  - `AGENTS.md` and `AGENT.md` define the agent roles, issue status flow,
    GitHub Discussion usage, PR handoff requirements, signed DCO commit rule,
    and local validation harness for multi-agent work.
  - `docs/maintenance/agentic-sdlc.md` defines issue status labels, GitHub
    Project fields, work item types, review lanes, sync rules, and quality
    gates for agent-driven development.
  - `.github/labels.yml`, `.github/ISSUE_TEMPLATE/*.yml`,
    `.github/DISCUSSION_TEMPLATE/*.md`, and
    `.github/pull_request_template.md` provide the GitHub task, discussion,
    label-sync, and PR workflow surface.
  - `tools/check-agent-sdlc.mjs --check` validates the required SDLC files,
    issue status labels, agent labels, templates, workflow gate, and this Spec
    Kit task.
  - `.github/workflows/conformance.yml` runs
    `node tools/check-agent-sdlc.mjs --check` before generated parity,
    dependency graph, lint, and test gates.
- [x] T058 Add a naming audit that blocks legacy SDK codenames from new public
  Rust crates, modules, APIs, docs headings, and backlog items except when they
  are cited as historical source evidence.

  Acceptance criteria:
  - `.specify/memory/constitution.md` requires the SSI-domain naming rule to be
    covered by an executable conformance naming audit.
  - `docs/architecture/naming-policy.md` defines preferred SSI vocabulary,
    the historical source evidence boundary, strict audit scope, and acceptance
    questions for new public names.
  - `identus-conformance` scans workspace manifests, crate source files,
    constitution, Spec Kit plan/tasks, component architecture docs, and binding
    boundary docs for blocked legacy codenames.
  - Migration maps and source evidence documents remain allowed to cite legacy
    names accurately.
- [x] T080 Add binding core boundary ADR and typed target registry.

  Acceptance criteria:
  - `docs/architecture/adr-bindings-core-boundary.md` defines Rust-owned
    semantics for WASM, Node/N-API, UniFFI, Swift, Kotlin, TypeScript, React,
    and React Native wrappers.
  - The ADR requires SSI domain naming, shared conformance fixtures,
    JSON-compatible DTOs, typed error codes, opaque handles, no-secret-leak
    checks, and target-build gates.
  - `identus-bindings` exposes typed binding targets and facade surfaces for
    agent, wallet, DID, credential, presentation, messaging, OpenID4VC, trust,
    and adapter boundaries.
  - Unit and conformance tests check target coverage, owner crates, safety
    rules, and wrapper parity follow-up.
- [x] T081 Generate machine-readable wrapper API parity inventories from
  sdk-ts, sdk-swift, and sdk-kmp public exports.

  Acceptance criteria:
  - Inventories are generated from repository source exports, not hand-written
    from memory.
  - Inventories identify public package/module/class/function names,
    deprecation status, owner Rust crate, target binding surface, and fixture
    coverage.
  - `identus-conformance` fails if a parity inventory entry lacks an owner
    crate, migration disposition, or acceptance-test fixture reference.
- [x] T082 Expand wrapper API parity inventory generation into a repeatable
  checked script.

  Acceptance criteria:
  - A checked-in script regenerates wrapper API parity inventories from sdk-ts,
    sdk-swift, and sdk-kmp source exports without relying on manual editing.
  - The script excludes build outputs, dependency caches, generated protobufs,
    samples, and test-only helpers unless explicitly requested.
  - CI or conformance tests fail when the generated inventory differs from the
    checked-in fixture.
- [x] T083 Add CI gate for wrapper API parity generator drift.

  Acceptance criteria:
  - CI runs `node tools/generate-wrapper-api-parity.mjs --check` before
    conformance tests.
  - The gate documents the expected sibling repository checkout layout for
    sdk-ts, sdk-swift, and sdk-kmp.
  - The gate fails with an actionable message when a wrapper export changes
    without a matching parity fixture update.
- [x] T085 Pin every third-party GitHub Action used by the conformance workflow
  to immutable SHAs or replace it with an Identus shared reusable workflow.

  Acceptance criteria:
  - `.github/workflows/conformance.yml` no longer references floating action
    tags.
  - The workflow keeps the same sibling repository checkout layout and wrapper
    parity drift check.
  - File hygiene or conformance tests document the supply-chain pinning
    expectation.
- [x] T084 Add crate layout ADR and conformance guard.

  Acceptance criteria:
  - `docs/architecture/adr-crate-layout.md` defines dependency rings for
    foundation, cryptography, identity, credential/presentation, protocol,
    wallet/agent, adapters, bindings, and conformance crates.
  - The ADR names every current workspace crate and states which crates may own
    domain semantics, adapters, bindings, fixtures, and generator tools.
  - The ADR blocks domain-to-adapter dependencies, wrapper-owned semantics,
    service binary leakage, and crate splits that only mirror historical SDK
    module names.
  - `identus-conformance` checks the ADR for every crate, ring, package layout,
    and boundary rule.
- [x] T059 Add a DID method support manifest for PRISM, peer, web, key, jwk,
  pkh, and example/test methods, with source evidence and resolver ownership.
- [ ] T060 Evaluate existing Rust DID crates (`ssi`, `ssi-dids`, `did-peer`,
  `did-key`, `did-jwk`, and Affinidi DID crates) before adding method-specific
  resolver dependencies.
- [x] T061 Implement type-safe `Did`, `DidUrl`, DID method, and
  method-specific id primitives with typed parser errors in `identus-did`.
- [ ] T062 Add method-specific resolver adapter ADRs for `did:peer`,
  `did:web`, `did:key`, `did:jwk`, and `did:pkh`.
- [x] T063 Add a constitution rule that every implementation increment must
  have a Spec Kit task with explicit acceptance criteria.

  Acceptance criteria:
  - `.specify/memory/constitution.md` contains a governing principle requiring
    increment tasks, acceptance criteria, executable validation, and same-
    increment backlog updates for discovered work.
  - `specs/001-sdk-rust-platform-core/plan.md` records the constitution check.
  - This DIDComm increment is represented by checked-in tasks with acceptance
    criteria before commit.

- [x] T064 Add type-safe DIDComm v2 message and protocol inventory primitives.

  Acceptance criteria:
  - `identus-messaging` exposes typed `DidCommMessageType`,
    `DidCommMessageId`, `DidCommThreadId`, protocol-kind classification,
    protocol profiles, and `DidCommPlaintextMessage` values that use typed
    `Did` addresses.
  - Unit tests accept Identus protocol URIs for OOB, BasicMessage, Trust Ping,
    Discover Features, Routing, Coordinate Mediation, Message Pickup, Issue
    Credential, Present Proof, Report Problem, and Revocation Notification.
  - Unit tests reject malformed message types and unchecked identifier values.
  - `cargo test --workspace` executes these checks.

- [x] T065 Document DIDComm protocol support, compatibility aliases, component
  relations, and base Rust crate candidates.

  Acceptance criteria:
  - `docs/architecture/didcomm-protocols.md` lists current, planned, and
    compatibility protocol families with source evidence and owner crates.
  - The document records `didcomm`, `didcomm-rs`, and
    `affinidi-messaging-didcomm` as candidate base crates without adding them as
    core dependencies.
  - `identus-conformance` checks that the architecture document includes the
    required DIDComm protocol families and candidate crates.

- [x] T066 Add DIDComm transcript fixtures for OOB, BasicMessage, Trust Ping,
  Discover Features, Routing/Forward, Coordinate Mediation, Message Pickup,
  Issue Credential, Present Proof, Report Problem, Revocation Notification, and
  legacy connection compatibility.

  Acceptance criteria:
  - Fixtures are stored under `fixtures/conformance/transcript/didcomm/` with a
    schema version, source reference, participants, plaintext message sequence,
    expected protocol states, and redaction policy.
  - The fixture set includes both canonical and compatibility aliases observed
    in Cloud Agent, Mediator, docs, and integration material.
  - `identus-conformance` loads every DIDComm transcript fixture and validates
    message type parsing with `identus-messaging`.

- [x] T067 Evaluate DIDComm pack/unpack base crates and define the adapter
  decision record.

  Acceptance criteria:
  - An ADR compares `didcomm`, `didcomm-rs`, and
    `affinidi-messaging-didcomm` for DIDComm v2.1 coverage, crypto provider
    control, resolver hooks, transitive dependencies, WASM/mobile support,
    UniFFI/binding fit, maintenance, and conformance vectors.
  - The ADR identifies whether `sdk-rust` will wrap a base crate, port selected
    code, or implement pack/unpack natively behind Identus crypto and DID
    resolver ports.
  - No pack/unpack dependency is added to `identus-messaging` until the ADR is
    reviewed.

- [ ] T068 Add a DIDComm pack/unpack adapter spike behind Identus-owned ports.

  Acceptance criteria:
  - The spike defines `DidCommPacker`, `DidCommUnpacker`, `DidResolverPort`,
    `KeyAgreementPort`, `SignerPort`, and `SecretResolverPort` without exposing
    upstream message structs or raw private keys across public boundaries.
  - At least one candidate crate from
    `docs/architecture/adr-didcomm-pack-unpack-dependency.md` is wired behind
    the adapter in a feature-gated experiment.
  - The adapter round-trips checked-in DIDComm transcript fixtures into
    `DidCommPlaintextMessage` and records negative-test gaps for malformed
    envelopes, unsupported algorithms, missing secrets, wrong recipient,
    unknown DID method, tampered ciphertext, and redaction-safe errors.

- [ ] T069 Add DIDComm target-build compatibility checks for server, WASM, Node,
  iOS, Android, and UniFFI wrapper viability before promoting any pack/unpack
  dependency.

  Acceptance criteria:
  - CI or local scripts can exercise the chosen candidate dependency across the
    target matrix or explicitly document feature-gated exclusions.
  - The compatibility result is linked from the DIDComm pack/unpack ADR before
    any dependency becomes production code.

- [x] T070 Add machine-readable JSON Schema validation for conformance fixture
  families.

  Acceptance criteria:
  - JSON Schema files are checked in under `fixtures/schema/` for static-model,
    vector, transcript, interop, and infrastructure fixture families.
  - `identus-conformance` reads the schema required-field declarations and
    validates checked-in JSON fixtures against them in the default
    `cargo test --workspace` path.
  - Schema validation enforces source evidence, owner crate, expected outcome,
    and redaction policy for current vector and transcript fixtures without
    requiring Docker, ledgers, databases, OIDC providers, or device adapters.

- [ ] T071 Add full JSON Schema dialect validation for conformance fixtures.

  Acceptance criteria:
  - A reviewed Rust JSON Schema validator is selected or implemented behind
    `identus-conformance` without introducing runtime dependencies into core
    protocol crates.
  - Validation covers `type`, `pattern`, `anyOf`, `oneOf`, arrays, and nested
    object requirements for every schema in `fixtures/schema/`.
  - Invalid fixture examples prove redaction, source evidence, owner crate,
    expected outcome, and protocol transcript failures produce typed,
    redaction-safe errors.

- [x] T072 Add executable credential and presentation verification replay for
  negative-case fixtures.

  Acceptance criteria:
  - `identus-credentials`, `identus-presentations`, and `identus-trust` expose
    verification policy ports that can consume
    `fixtures/conformance/vector/credential-verification/negative-cases.json`.
  - Each stable negative-case id produces the expected typed error without
    network services, Docker, ledgers, OIDC providers, or production secrets.
  - Format-specific implementations cover JWT VC, SD-JWT VC, W3C VC JSON,
    AnonCreds, OpenBadges 3.0, and ISO mdoc as their parsers land.

- [x] T073 Implement secure-storage typed ports and in-memory adapter.

  Acceptance criteria:
  - `identus-wallet` defines `SecureStore`, `KeyStore`, `SecretResolver`,
    `BackupStore`, `EntropySource`, secret-class metadata, typed key handles,
    and redaction-safe storage errors.
  - `identus-adapters` provides a deterministic in-memory adapter that supports
    backup/restore acceptance tests without Docker, OS keychains, device
    keystores, HSM/KMS, cloud secret managers, or browser APIs.
  - `identus-conformance` checks storage-record metadata, non-exporting key
    handle behavior, and redaction-safe error rendering.

- [x] T074 Add OpenID4VC conformance ADR for OID4VCI, OID4VP, SIOPv2, HAIP,
  and OpenID Federation.

  Acceptance criteria:
  - `docs/architecture/adr-openid4vc-conformance.md` records the official
    OpenID source baseline and current status for OID4VCI, OID4VP, SIOPv2,
    HAIP, and OpenID Federation.
  - The ADR defines ownership for `identus-openid4vc` and required ports from
    credentials, presentations, trust, wallet, crypto, DID, and adapters.
  - The ADR covers OID4VCI issuance flows, OID4VP/SIOPv2 presentation flows,
    HAIP, Federation trust, fixtures, state machines, negative cases, and
    conformance gates.
  - `identus-conformance` checks the ADR for protocol coverage, source
    baseline, roles, ports, fixture strategy, state machines, and default
    Docker-free test requirements.

- [x] T075 Add Docker-free OpenID4VC transcript fixtures.

  Acceptance criteria:
  - Fixtures under `fixtures/conformance/transcript/openid4vc/` cover OID4VCI
    authorization code, OID4VCI pre-authorized code, OID4VCI deferred
    credential, OID4VP same-device direct post, OID4VP cross-device direct
    post, SIOPv2, HAIP, and federation-backed trust.
  - Every transcript has source evidence, roles, ordered messages, expected
    states, typed errors for negative cases, and redaction policy.
  - `identus-conformance` validates the transcript schema and the default
    `cargo test --workspace` path does not require Docker, real HTTP, browsers,
    devices, OIDC providers, or trust-anchor infrastructure.

- [x] T076 Replay OpenID4VC transcript fixtures through typed state machines.

  Acceptance criteria:
  - `identus-openid4vc` exposes typed state machines for
    `CredentialIssuanceFlow`, `PresentationFlow`,
    `SelfIssuedOpenIdProviderFlow`, `HighAssuranceProfileFlow`, and
    `FederationTrustFlow`.
  - The state machines consume
    `fixtures/conformance/transcript/openid4vc/core-flows.json` and assert the
    expected states without Docker, real HTTP, browsers, devices, OIDC
    providers, or trust-anchor infrastructure.
  - Negative transcript entries produce typed, redaction-safe errors for wrong
    nonce and trust-chain rejection.

- [x] T077 Add trust/status policy ADR for credential status, trust anchors,
  trust chains, and ecosystem policy.

  Acceptance criteria:
  - `docs/architecture/adr-trust-status-policy.md` records the current source
    baseline for VC Data Model 2.0, Bitstring Status List, IETF Token Status
    List, OpenID Federation, and X.509.
  - The ADR defines `identus-trust` ownership and typed ports for status
    resolving, status verification, trust anchor resolution, trust-chain
    verification, trust policy, and evidence caching.
  - The ADR covers Bitstring Status List, Token Status List, AnonCreds
    revocation, OpenID Federation, trust marks, X.509/IACA, privacy rules,
    fixtures, and conformance gates.
  - `identus-conformance` checks the ADR for source baseline, policy domains,
    ports, status mechanisms, privacy rules, fixture strategy, and follow-up
    tasks.

- [x] T078 Implement trust/status typed ports and Docker-free policy fixtures.

  Acceptance criteria:
  - `identus-trust` defines `StatusResolver`, `StatusVerifier`,
    `TrustAnchorResolver`, `TrustChainVerifier`, `TrustPolicyEngine`,
    `TrustEvidenceStore`, typed status decisions, and redaction-safe errors.
  - Fixtures cover Bitstring Status List, Token Status List draft handling,
    AnonCreds revocation, OpenID Federation trust chains, trust marks, X.509,
    and IACA without Docker or network services in the default test path.
  - Credential, presentation, and OpenID4VC verification paths consume
    `identus-trust` decisions instead of duplicating policy.

- [x] T079 Add NeoPRISM PRISM operation and VDR fixtures.

  Acceptance criteria:
  - Fixtures cover PRISM create, update, deactivate, resolve, long-form DID,
    `AtalaOperation` protobuf serialization, secure-depth handling, and VDR
    create/update/deactivate/resolve metadata.
  - Fixtures separate Docker-free operation/model vectors from opt-in Cardano,
    Oura, DBSync, Blockfrost, PostgreSQL, SQLite, cardano-wallet, and
    embedded-wallet infrastructure tests.
  - `identus-conformance` checks the fixture schema, source evidence, owner
    crates, expected states, and redaction policy.

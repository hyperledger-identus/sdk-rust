# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation is described by a program registry with eight broad
capability families and uses
legacy module labels to correlate the three language SDKs. That is sufficient
for program bootstrap but not for implementation. The labels conceal divergent
scope and encourage accidental one-for-one ports.

The primary donor snapshot is the Apache-2.0 SDK-TS repository at immutable
revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`, which is also the published
`@hyperledger/identus-sdk` 8.1.4 release. The repository contains:

- one public npm SDK package with ESM, CommonJS, and declarations;
- five plugin subpath exports: AnonCreds, DIDComm, OIDC/OID4VCI, DIF
  Presentation Exchange, and OEA workflow behavior;
- private workspace packages for shared domain types, protobuf models,
  AnonCreds WASM, DIDComm WASM, JWE WASM, and build configuration;
- ES2022 plus DOM assumptions, Node 20/LTS guidance, browser targets, and
  browser/Node consumers;
- 216 non-test-like SDK TypeScript files, 67 executable `*.test.ts` entry
  files, 84 TypeScript files under the SDK test tree, and 15 Cloud
  Agent/Mediator-dependent Gherkin feature files.

The source still exports the old building-block names, but its actual behavior
splits into cryptography and key derivation; DID values, documents, methods,
resolution, and operations; JWT/SD-JWT/AnonCreds credential formats;
presentation and status behavior; OID4VCI; DIDComm packing, routing, mediation,
pickup, issue-credential, presentation, out-of-band, and basic-message flows;
wallet storage and backup; agent orchestration; and TypeScript plugin/runtime
integration.

SDK-Rust already exceeds parts of the donor. In particular, its bounded Final
OID4VCI wallet core and its crypto/DID foundations must not regress to the
narrower SDK-TS implementations. The inventory therefore records the strongest
evidence per normalized capability instead of treating SDK-TS as a code donor
for every row.

## Normative sources

The authority order is repository governance: accepted standards and errata,
official conformance suites and vectors, accepted ADR/profile decisions,
current Identus compatibility requirements, independent interoperable
implementations, and only then consumer implementation precedent.

Controlling repository decisions include ADR 0061 for owned facades, ADR 0073
for conditional AnonCreds v1 adoption, ADR 0076 for rejecting the draft-07 OWF
SD-JWT backend, ADRs 0097 through 0101 for experimental bindings, ADR 0110 for
reusable modules, and ADRs 0162 through 0164 for platform-core migration,
capability dispositions, test authority, and deprecation.

For SD-JWT, RFC 9901 is the stable base target. The SD-JWT VC profile must be
pinned separately because a VC media type or profile claim is not implied by
RFC 9901 alone. Current Rust candidates include maintained published packages
that post-date ADR 0076; they require a new exact dependency-cone, target,
profile, resource-bound, JOSE-boundary, and vector assessment before adoption.

For AnonCreds, the donor embeds `input-output-hk/anoncreds-rs` revision
`08fbc3f7afb2d2bbd5c72d4b6aadd70b77f4bef6` through a WASM wrapper. That
revision and wrapper are compatibility evidence, not the SDK-Rust dependency
choice. The target assessment starts from the current AnonCreds 1.0
specification/reference implementation and ADR 0073, with current release,
mobile/WASM, OpenSSL/native, unsafe, secret, VDR, and revocation evidence.

## Candidate decisions

| Decision area | Candidate | Assessment | Direction |
|---|---|---|---|
| Discovery baseline | start from SDK-KMP | Oldest implementation; risks importing missing or stale behavior | `not-adopt` |
| Discovery baseline | inventory all SDKs in parallel | Hides which implementation establishes initial capability granularity and repeats normalization work | `not-adopt` |
| Discovery baseline | SDK-TS first, Swift deviation/parity second, KMP compatibility third | Uses the newest implementation, validates it against the near-current Swift SDK, then accounts for the oldest consumer surface | `adopt` |
| Target taxonomy | preserve Apollo/Castor/Pollux/Mercury/Pluto boundaries | Names differ in scope across SDKs and would create artificial Rust components | `not-adopt` |
| Target taxonomy | responsibility-based capability IDs with donor aliases | Stable, cohesive, and independent of historical language architecture | `adopt` |
| Feature parity | copy donor dependencies and drafts | Confuses released compatibility with current correctness and maintenance | `not-adopt` |
| Feature parity | normalize against standards, current profiles, vectors, and maintained Rust engines | Produces current reusable behavior while retaining explicit compatibility evidence | `adopt` |
| First canary | secret-bearing crypto or wallet backup | High ownership, memory, and migration risk | `not-adopt` |
| First canary | bounded DID and DID URL value behavior through the existing experimental WASM boundary | Non-secret, reversible, already has Rust and browser evidence | `conditional-adopt` pending completed inventory |

## Compatibility and dependency evidence

The SDK package directly depends on `@hyperledger/identus-apollo` 1.4.5,
`@sd-jwt/sd-jwt-vc` 0.7.1 (resolved family 0.7.2), RIDB, DID-JWT, JSON-LD,
multiformats, TypeBox, and other browser-oriented libraries. The WASM packages
are generated from Git submodules for AnonCreds and DIDComm; the DIDComm donor
is `elribonazo/didcomm-rust` revision
`9c4fcdfc2b82f02549acc7ae6b2b86d2f1611d13`. These dependencies are evidence
about SDK-TS behavior and coupling, not approved SDK-Rust dependencies.

The public package exports ESM and CommonJS entry points plus five plugin
subpaths. It targets ES2022/DOM, documents Node 20/LTS, publishes through npm
trusted publishing, and bundles private WASM/domain packages into the SDK.
GitHub code search identifies current public consumers including the Identus
workshop repository, `hyperledger-identus/identity-portal`,
`input-output-hk/lace-kyc`, and third-party React/storage wrappers. Search
results prove source references, not supported-version telemetry; exact
consumer compatibility remains capability-specific.

The donor's BIP-32/BIP-39 test data is a candidate normative fixture only after
its upstream provenance is recorded. Cross-SDK backup fixtures and Cloud
Agent/Mediator E2E scenarios are candidate `identus-contract` or
`consumer-regression` evidence. Most unit tests are
`implementation-regression` until provenance and public behavior are shown.

The exact version and feature evidence is donor-specific. No Rust dependency is
added here, so no new direct and resolved dependency cone is activated. Future
engine issues must record the exact minimal and default feature graphs, MSRV,
host/WASM/iOS/Android target evidence, public and wire compatibility, owned
facade boundary, migration, and rollback. The SDK-TS Apache-2.0 license and
provenance are pinned at the source revision; third-party license and
provenance remain per-candidate gates.

## Security, privacy and maintenance evidence

Secret-bearing keys, mnemonics, link secrets, AnonCreds material, database
encryption, and backup JWE must not cross new bindings as raw general-purpose
DTOs. SDK-TS currently exposes broad object graphs and uses JavaScript/WASM
memory; the Rust migration needs purpose-built handles or operations,
redacted stable errors, bounded parsing, explicit ownership, and runtime proof.

OID4VCI URL retrieval, JSON parsing, metadata, callbacks, and tokens in SDK-TS
are useful compatibility evidence but do not provide the resource-bound and
parsed/verified/trusted guarantees already present in SDK-Rust. The plugin host,
fetch/networking, browser storage, npm packaging, bundler compatibility,
runtime feature detection, and UI/lifecycle integration remain TypeScript
responsibilities unless a narrower portable contract is proven.

Future adoption reports must record reachable unsafe and native code,
supply-chain advisories and source integrity, maintenance, release and security
posture, and exact protocol/draft currency. A donor package's popularity or
release recency is not security or conformance approval.

## Rejected or deferred candidates

No entire donor module is adopted. SDK-TS's AnonCreds and DIDComm forks are
deferred as production dependencies. RIDB remains a TypeScript/browser storage
adapter, not a generic Rust core dependency. The SDK-TS SD-JWT package is not
the Rust backend decision. PRISM/Cardano ledger operations remain outside a
chain-neutral generic crate until a separately owned adapter boundary is
accepted.

SDK-Swift discovery follows this issue and may refine capability granularity or
identify Apple-only responsibilities. SDK-KMP discovery follows both and may
add compatibility requirements, but does not reopen normalized behavior solely
because its historical implementation differs.

The reconsideration trigger for any rejected donor dependency is a maintained
release that implements the selected current profile and passes the SDK's
facade, resource, dependency-cone, MSRV, target, unsafe/native, security,
conformance, and rollback gates.

## Open questions and blockers

There are no blockers to the inventory. The following are required outputs,
not reasons to delay it: exact symbol-to-capability mapping, fixture provenance,
published consumer compatibility, current SD-JWT Rust engine selection,
current AnonCreds 1.0 adapter viability, DIDComm engine/version direction,
persistence migration semantics, and a final canary contract. Each unresolved
implementation decision remains `defer` and receives a bounded child issue.

## Sources

- https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e
- https://github.com/hyperledger-identus/sdk-ts/releases/tag/%40hyperledger%2Fidentus-sdk%408.1.4
- https://www.npmjs.com/package/@hyperledger/identus-sdk/v/8.1.4
- https://www.rfc-editor.org/rfc/rfc9901.html
- https://github.com/anoncreds/anoncreds-rs
- SDK-Rust ADRs 0061, 0073, 0076, 0097 through 0101, 0110, and 0162 through 0164
- SDK-Rust issues #163, #415, #417, #420, #421, and #416

## Evidence commands

Evidence was gathered with immutable `git` checkout and status inspection,
`find`, `rg`, `jq`, manifest/lockfile/source/test review, `npm view`, `cargo
search`, `cargo info`, `gh api`, `gh release list`, and `gh search code`. No
donor repository was mutated. Production dependency probes, fixture promotion,
browser/Node runtime execution, package migration rehearsal, and downstream
canary execution remain explicitly unrun and belong to child implementation
issues.

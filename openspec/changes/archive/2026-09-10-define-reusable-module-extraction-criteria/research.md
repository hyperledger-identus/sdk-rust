# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-10
Source retrieval date: 2026-09-10
Research blockers: none

## Problem and existing implementation

The current implementation already establishes the directional boundary:
sdk-rust owns chain-neutral SSI types, ports, protocol engines, cryptographic
utilities, and conformance evidence; consumers retain chain, product, custody,
trust, UI, deployment, and concrete storage behavior. ADR 0061 adds the rule
that an adopted dependency stays behind an Identus-owned facade.

The source matrix classifies NeoPRISM `lib/apollo` and `lib/did-core` as
`adapt`, `lib/did-resolver-http` as `extract`, and PRISM, Cardano, node, and
concrete storage packages as `remain-downstream`. Those labels do not yet
specify mandatory module-level evidence, how to treat an SDK capability that
now already exists, or when downstream deletion is safe.

sdk-rust `develop@78e0656f860c0e569df2edb25d087c42561c51a5`
already contains `identus-crypto`, `identus-did`, and
`identus-did-resolver-http`. The first milestone is therefore primarily SDK
adoption and compatibility assessment, not bulk source movement. Consumer
evidence is initially NeoPRISM's historical tests and the isolated beta; it is
not yet a completed downstream adoption receipt.

## Normative sources

- [W3C DID Core 1.0](https://www.w3.org/TR/did-core/) defines generic DID data
  model behavior; donor code does not outrank the Recommendation.
- [W3C DID Resolution](https://w3c-ccg.github.io/did-resolution/) is a living
  draft, so each resolver slice pins its exact protocol/draft currency.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) inform
  cohesive library APIs but do not override SDK security or ownership rules.
- The accepted SDK blueprint, ADR 0061, SSI source matrix, and canonical
  OpenSpec requirements are the repository-local normative architecture
  sources.

| Source | Immutable revision | Evidence | Limitation |
| --- | --- | --- | --- |
| sdk-rust | `78e0656f860c0e569df2edb25d087c42561c51a5` | Current generic crates, factory contracts, ADR 0061, upstream source matrix | An integration branch is not a published release |
| NeoPRISM main | `d4608fe3d662ebaf425c4a6380f6d241cffe74c3` | Apache-2.0 root license; current package graph and historical tests | Source inspection is not compatibility proof |
| NeoPRISM beta | `faead035915b1f51bec20f76fa17a9c404b71acd` | `lib/apollo` delegates crypto to sdk-rust revision `9ff2fa87337d3f25d67f38486c3a4d0a656bd9d6`; 20 files, 145 insertions, 386 deletions | No hosted CI run or merged adoption; the SDK pin predates current `develop` |
| NeoPRISM adoption issue | `hyperledger-identus/neoprism#321` | Separates crypto facade from later DID feasibility and forbids a big-bang replacement | Open issue and unchecked acceptance tasks |

The NeoPRISM root license content at the assessed main revision has SHA-256
`33bc52fec3f5206f59f87f6c13ac849bab08a17a2d9648d5298232078b16c73c`.
File-level provenance and history still require verification in every source
or fixture extraction slice.

## Candidate decisions

The table distinguishes the factory research decision, donor source
disposition, and next delivery action. No row authorizes implementation by this
documentation change.

| NeoPRISM surface | Research decision | Source disposition | Next action and reconsideration trigger |
| --- | --- | --- | --- |
| `lib/apollo` | `adopt` existing SDK capability downstream | `adapt` | Refresh the thin NeoPRISM compatibility facade against current `identus-crypto`; port only an evidenced missing generic behavior or fixture. Reconsider extraction when a mapped Apollo capability is absent upstream. |
| `lib/did-core` | `conditional-adopt` existing SDK capability downstream | `adapt` | Produce a compile- and fixture-driven mapping to `identus-did`; extract only an evidenced semantic gap. Reconsider direct mapping after its public and wire compatibility matrix is complete. |
| `lib/did-resolver-http` | `conditional-adopt` existing SDK capability downstream | `extract` in the source matrix | Compare historical HTTP fixtures against `identus-did-resolver-http`; adapt or extract only missing behavior. Reconsider source extraction only when normative behavior is proven absent upstream. |
| `lib/did-prism` | `retain-local` | `adapt` for a future DID-method slice | Keep PRISM method/operation/protobuf behavior downstream. Reconsider a standard-neutral primitive only through a separate candidate assessment. |
| PRISM indexer, ledger, submitter, `lib/node-storage`, and node binaries | `retain-local` | `remain-downstream` | Keep Cardano observation/submission, node orchestration, concrete databases, migrations, executors, and deployment in NeoPRISM. Reconsider only after an independent chain-neutral capability and second consumer exist. |
| Cross-package historical tests and vectors | `oracle` | `conformance-only` when generic | Pin provenance and use as differential evidence. Reconsider source adoption only if a cohesive implementation unit passes every hard gate. |
| Generic-looking pagination, uniqueness, and source-location helpers | `not-adopt` | `reject` unless a named SDK capability proves demand | Leave local or use standard-library/local code. Reconsider only when a named capability, independent change axis, and second credible consumer are evidenced. |

## Compatibility and dependency evidence

The beta removes direct Ring, Ed25519, secp256k1, and X25519 implementations
from NeoPRISM `lib/apollo`, maps its features to `identus-crypto`, and retains
local base64/hex/JWK compatibility types. This is a sound incremental facade,
but acceptance still requires historical vectors, full relevant tests, and the
effective target/toolchain evidence on the refreshed branch.

The exact version and features are sdk-rust Git revision
`9ff2fa87337d3f25d67f38486c3a4d0a656bd9d6`, `default-features = false`,
with `ed25519`, `hash`, `jwk`, `secp256k1`, and `x25519` enabled through
NeoPRISM compatibility features. sdk-rust's effective MSRV/temporary etalon is
Rust 1.98.1; the beta moved NeoPRISM to a later nightly and therefore does not
yet prove a shared compiler policy.

The direct and resolved dependency cone has not been captured for the refreshed
candidate and is an implementation gate. The observed direct beta change
removes Ring, `k256`, and `x25519-dalek` from `identus-apollo` production
dependencies in favor of `identus-crypto`, while retaining base64/hex and a
dev-only Ed25519 dependency. Every future slice attaches `cargo tree` evidence
for minimal and enabled features, audits reachable unsafe and native code, and
records current supply-chain advisory and license evidence.

Each extraction also records host/target support, public and wire
compatibility, the facade boundary, and rollback. No platform claim is
inherited from either repository merely because a host build succeeds.

## Security, privacy and maintenance evidence

The architecture decision adds no executable code. Future reusable modules
enforce resource bounds at untrusted input boundaries, stable redacted errors,
explicit parsed/validated/verified/trusted states, and safe secret ownership.
Concrete network, database, filesystem, clock, entropy, executor, custody,
consent, and trust behavior remains behind narrow ports or downstream
adapters. Unsafe Rust still requires a bounded safety ADR.

Maintenance, release, and security posture remain separate claims. Repository
activity and a beta commit do not establish a maintained public crate release,
security audit, or supported target. Each adopted surface proves compatibility
behind the facade with an exact rollback to the last proven NeoPRISM
implementation or SDK revision.

## Rejected or deferred candidates

Bulk copying NeoPRISM, an `identus-neoprism-common` umbrella, donor-type
re-exports, SDK dependencies on a consumer, cross-repository path dependencies,
and score-based acceptance are `not-adopt`. PRISM/Cardano/node and concrete
storage packages are `retain-local`. Historical behavior may be an `oracle`
without entering production artifacts. Every row above includes an objective
reconsideration trigger; a future issue proves that trigger before broadening
the disposition.

## Open questions and blockers

There are no blockers to accepting the architecture contract. The exact
remaining `lib/apollo` compatibility shell, DID semantic gaps, HTTP behavior,
target matrix, and dependency cones are evidence tasks in separate
implementation/adoption issues. They are not silently treated as passing.

## Evidence commands

- `scripts/factory doctor` passed on the dedicated issue #253 worktree before
  planning edits.
- Exact commands run include `git rev-parse origin/main
  origin/codex/sdk-rust-beta`, `git merge-base origin/main
  origin/codex/sdk-rust-beta`, `git diff --shortstat
  origin/main...origin/codex/sdk-rust-beta`, source and manifest inspection,
  `gh issue view 321`, and `scripts/factory doctor`.
- The NeoPRISM repository was read only; its source, branch, index, and working
  tree were not changed.
- Unrun commands for implementation include `cargo tree`, refreshed Cargo
  builds/tests, target builds, supply-chain/advisory scans, and NeoPRISM hosted
  CI. They remain future evidence and are not represented as passing.

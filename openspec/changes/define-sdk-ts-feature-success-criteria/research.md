# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-07
Source retrieval date: 2026-11-07
Research blockers: none

## Problem and existing implementation

### Current implementation

The canonical SDK-TS inventory pins 24 capabilities to
`@hyperledger/identus-sdk@8.1.4` at
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`. The TS1 matrix already carries
the target relation, current evidence state, rationale, known deviations,
evidence links, next action, consumer, rollback, and non-claims for every
capability. Existing WASM DID work proves a bounded browser ESM value facade;
it does not prove a whole SDK, Node.js packaging, React integration, or React
Native engine support.

The inventory also identifies the SDK-TS test paths and their authority. Those
paths mix implementation regression, normative candidates, Identus contract
candidates, and consumer regression candidates, so copying a test count would
not establish conformance. A maintained applicability disposition is required
before a donor test becomes a cross-language acceptance test.

## Normative sources

The target and evidence policy is governed by the accepted SDK-Rust ADRs,
OpenSpec `sdk-ts-parity-conformance` capability, the pinned SDK-TS inventory,
and each capability's selected W3C, IETF, OpenID, DIF, or Identus profile. The
primary source repositories are https://github.com/hyperledger-identus/sdk-rust
and https://github.com/hyperledger-identus/sdk-ts. The donor inventory pins the
exact SDK-TS source revision and package version; implementation tests remain
evidence according to their recorded authority rather than becoming standards.

## Target-runtime decision

Browser and client-only React web share the browser/WASM execution class.
React server rendering crosses the Node.js target and must pass both relevant
contracts. Node.js can load WASM but needs separate package, module, filesystem,
networking, worker, and crypto evidence. Under accepted ADR 0163, React Native
is a distinct native runtime and uses a dedicated New Architecture/TurboModule
package over a qualified native binding adapter; it is not a browser-WASM
fallback. The public capability contract can be common even when loaders and
available platform-owned capabilities differ.

This avoids two failure modes: pretending React Native is merely a browser, or
duplicating portable protocol semantics in TypeScript to compensate for a
missing binding. The portable Rust core remains shared; bindings and host
adapters are target-owned.

## CI decision

The Rust core and language/runtime adapters answer different questions while
effective constraint `SDK-LIM-009` preserves one required Linux `fast` signal.
CI therefore forms a layered DAG inside that policy boundary: the existing
Rust-core job runs first and retains its own visible result; a deterministic
exact-head binding-impact plan selects browser, Node.js, React web, React
Native, package/type-surface, and other downstream jobs. Those target jobs use
separate caches, artifacts, logs, timeouts, and job identities, then the one
protected `fast` result aggregates only the selected gates. Unselected targets
emit a machine-readable inapplicable receipt without installing their tools.

A target failure does not cancel sibling targets, overwrite the core-job
result, or make its core evidence disappear. It makes the single protected
`fast` aggregator red when the exact impact plan selected that target, blocks
the target's delivery/support claim, and blocks integration until repaired.
Unrelated core-only changes do not acquire a false dependency on TypeScript
toolchain availability. Impact selection is fail-closed and includes exported
binding/API fingerprints, transitive bound crates, relevant lockfiles,
generated mappings, package metadata, and target contracts; file paths alone
are insufficient. Target outputs are rebuilt from the exact source and lock
identity rather than reusing an incompatible native core artifact.

## E2E decision

SDK-TS E2E tests are reused at the journey level when their observable behavior
is applicable to the selected Identus profile. Every discovered donor test
receives one disposition: `shared`, `adapted`, `platform-owned`, `superseded`,
or `excluded`, with a reason, applicable targets, hermetic/external dependency
classification, and immutable donor revision, path, suite/test ID, and content
hash. Discovered count must equal ledger count; unledgered skip, focus,
quarantine, or omission fails. `shared` tests use one TypeScript assertion body
against legacy and Rust-backed private provider factories. `adapted` tests
preserve the journey but make an explicit profile or host boundary change.
Private legacy providers may retain old names; the new public facade may not.

Rust-only evidence is kept in a separate ledger with `rust-addition` and
`rust-deviation` dispositions. Deviated or additional functionality is proven in Rust with
normative, negative, resource-bound, property/differential, and protocol E2E
evidence as applicable; selected TypeScript smoke tests prove the binding and
mapping boundary.

## Quality decision

Success requires more than passing journeys:

- every owned exported TypeScript operation maps to contract/E2E evidence and
  new or changed executable code has reproducible line/branch evidence without
  lowering an established higher package baseline;
- the TypeScript adapter compiles with pinned strict flags, exports no explicit
  `any`, narrows `unknown` at ingress, and maps numbers/bigints, bytes,
  null/undefined, errors, lifecycle/free, Promises, cancellation, and
  `AbortSignal` deterministically while Rust owns validated domain invariants;
- generated/versioned DTO and error mappings fail on drift rather than relying
  on handwritten duplicate semantics;
- the same portable Rust crate semantics are reused across WASM and native
  bindings, with target-specific artifacts and host effects supplied through
  ports;
- TypeScript may enforce generated outer allocation bounds and host policy
  before WASM/native ingress, while Rust revalidates domain invariants
  authoritatively; TypeScript does not duplicate cryptography, canonical
  parsing, formats, or protocol state;
- historical component names are absent from new exports, package paths, and
  primary documentation;
- size, startup, throughput/latency, diagnostics, observability, fallback, and
  rollback evidence are recorded per selected target.

Coverage thresholds are quality gates for owned changed executable code, not a
claim that line coverage proves protocol correctness. Each receipt pins the
tool/version, base and head, include/exclude denominator, numerator,
target/profile, and artifact. The default is at least 80 percent line and 70
percent branch coverage where reliable, while a higher established package
baseline remains controlling. Where Rust branch coverage is unavailable, the
train supplies condition, mutation, negative-selector, and public-operation
mapping evidence rather than waiving the gate with prose.

Performance is measurement-only until a target has an accepted budget record.
Before support is claimed, that record pins runner and protocol plus compressed
and uncompressed artifact size, cold/warm startup p50/p95, and representative
latency, throughput, and memory measures with variance and regression rules.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| One identical WASM artifact path for all four platforms | `not-adopt` | React Native and Node.js have distinct runtime and packaging constraints. |
| One standards-oriented TypeScript facade over target-specific loaders | `adopt` | Keeps the consumer contract stable while preserving target truth. |
| Historical SDK-TS component names in the new facade | `not-adopt` | They encode donor architecture rather than capability ownership. |
| Run every SDK-TS E2E test unchanged | `not-adopt` | Platform-only and superseded-profile journeys need explicit dispositions. |
| Reuse applicable TypeScript journeys against both routes | `adopt` | Provides direct consumer-level differential evidence. |
| Prove intentional differences only in TypeScript | `not-adopt` | Normative and resource/security behavior belongs with the Rust core. |
| Shared Rust core plus thin bindings and host ports | `adopt` | Maximizes reuse and prevents protocol duplication in adapters. |
| One monolithic Rust-plus-TypeScript job | `not-adopt` | A target/toolchain failure would obscure valid core evidence and cancel unrelated targets. |
| Rust-core-first DAG with isolated jobs and one protected aggregator | `adopt` | Preserves the effective single-fast-signal policy while making every impacted target accountable. |

## Compatibility and dependency evidence

This change adds no dependency, unsafe code, runtime target, public API, wire
shape, secret boundary, or consumer mutation. The new criteria are target-state
contracts, not effective support promises. Binding implementation remains
subject to target-specific input/resource bounds, redaction, supply-chain,
unsafe/native, package, and specialist review.

The exact donor is `@hyperledger/identus-sdk@8.1.4` at
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`; its package features, source
paths, public surfaces, test paths, and dependency bases are captured in
`docs/architecture/identus-platform-ts-capabilities.toml`. The donor and this
repository are Apache-2.0, and this change copies no source or fixture. The
direct and resolved dependency cone, Cargo lock, npm lock, Rust MSRV 1.89.0,
primary Rust 1.98.1 compiler, feature matrix, and target matrix are unchanged.

### License and provenance

SDK-Rust and SDK-TS are Apache-2.0 repositories. The source inventory records
the immutable donor revision and paths; this planning change adapts no donor
source or fixture and therefore introduces no additional attribution packet.

## Security, privacy and maintenance evidence

The roadmap contains only public capability metadata. It stores no secret,
credential, wallet, provider response, or consumer payload. Maintenance,
release, security, advisory, unsafe/native, and package-publication posture
remains owned by each implementation train and the accepted repository policy.
No release or support state changes here.

Protocol and draft currency remains capability-specific. The matrix pins or
requires the exact selected profile, including OpenID4VCI 1.0 Final, RFC 9901,
SD-JWT VC draft-22, selected DIDComm/Peer DID versions, and unresolved versions
where the capability is still blocked. SDK-TS regression behavior does not
silently replace those authorities.

## Rejected or deferred candidates

Runtime implementation, React Native binding qualification, workflow changes,
package publication, test migration, coverage tooling, performance budgets,
and target activation are deferred to capability trains. Identical APIs,
historical module parity, one monolithic CI verdict, browser evidence reused as
React Native evidence, path-only CI routing, and unclassified donor-test copying are rejected.

Reconsideration trigger: reconsider the target model if a selected runtime cannot implement the common
TypeScript facade without leaking unsafe secret material or duplicating
portable semantics. Reconsider a threshold only with measured tooling evidence
and a replacement that is at least as reviewable; ordinary missed coverage is
not a trigger to lower it.

## Open questions and blockers

There are no blockers to this documentation/checker change. React Native's
qualified native adapter, exact target budgets, test-level applicability rows, and
capability-specific engine/profile decisions remain explicit downstream work
and cannot be claimed as delivered by this change.

## Sources

- Issue #594 and program #502
- `docs/architecture/identus-platform-ts-capabilities.toml`
- `docs/roadmap/sdk-ts-parity-conformance.toml`
- ADRs 0162, 0163, 0164, 0169, 0170, 0174, and 0193
- W3C DID Core, selected IETF/OpenID/DIF profiles, and capability-owned Identus
  contracts referenced by the source inventory

## Evidence commands

Planning evidence is repository-local inspection plus the existing factory,
inventory, parity, support-policy, and OpenSpec validators. Runtime builds,
binding tests, SDK-TS E2E execution, React Native devices, coverage reports,
performance measurements, publication, release, and hosted CI are unrun and
remain future capability-train evidence.

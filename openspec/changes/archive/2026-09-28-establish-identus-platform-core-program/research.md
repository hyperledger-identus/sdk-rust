# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation is split across the TypeScript, Swift, Kotlin, and
Rust repositories. The three language repositories each expose the historical
Apollo, Castor, Pollux, Mercury, Pluto, and edge-agent decomposition, but their
implementations, dependency choices, persistence layers, and test suites have
diverged. The Rust workspace already owns reusable crypto and DID primitives,
generic credential and presentation contracts, selected OpenID4VC behavior,
and experimental DID bindings. Messaging, concrete credential formats,
storage adapters, edge-agent workflows, binding breadth, and consumer
migration evidence remain incomplete.

Fresh shallow snapshots were inspected at:

| Repository | Revision | Approximate authored source | Tests / test files |
|---|---|---:|---:|
| `sdk-ts` | `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | 20,463 TypeScript lines in the primary source tree | 84 SDK test files |
| `sdk-swift` | `ebbfdb666a3b0c9905dd443589e03ccf2ae8087b` | 30,533 Swift lines | 87 test files |
| `sdk-kmp` | `5a8fda770bbbca84192979b2c8e6090ee94790e3` | 22,740 Kotlin lines | 63 test files |

Counts are scoping evidence, not quality or parity scores. Exact API and test
normalization remains a per-repository discovery task.

## Normative sources

The authoritative sources are applicable RFCs, W3C Recommendations, OpenID
Final specifications, DIDComm specifications, AnonCreds specifications and
vectors, published Identus compatibility contracts, and immutable consumer
fixtures. Repository READMEs and implementation tests describe behavior but do
not automatically make accidental behavior normative.

ADR 0061, ADR 0097 through ADR 0101, ADR 0110, the SDK support policy, and the
repository architecture rules constrain dependency adoption, binding
isolation, portability, and reusable-module boundaries.

## Candidate decisions

| Decision area | Candidate | Assessment | Direction |
|---|---|---|---|
| Repository disposition | archive all language SDKs after parity | Removes duplication but destroys idiomatic/platform integration and creates a flag-day migration | reject |
| Repository disposition | keep all implementations independent | Preserves current consumers but retains security and semantic drift | reject |
| Repository disposition | thin language facades and platform shells over Rust | Removes reusable semantic duplication while preserving ecosystem APIs and adapters | adopt as default target |
| Migration unit | repository-wide rewrite | Difficult to verify and roll back | reject |
| Migration unit | normalized capability with canary and conformance evidence | Bounded, stackable, and independently reversible | adopt |
| Tests | trust every legacy test equally | Fossilizes accidents and contradictory behavior | reject |
| Tests | tiered evidence authority | Separates normative vectors, cross-SDK contracts, regressions, and implementation-local tests | adopt |
| Legacy bugs | silently correct all differences | Creates invisible breaking changes | reject |
| Legacy bugs | classify, reproduce only when required, and sunset deliberately | Makes compatibility cost and removal explicit | adopt |

The overall candidate decision is `conditional-adopt`: adopt Rust-core
convergence capability by capability, subject to the registry and consumer
gates, while retaining language shells until their unique responsibilities
have a reviewed target.

The preliminary repository targets are: TypeScript remains a browser/Node and
package facade; Swift remains an Apple integration shell; KMP remains an
Android/JVM integration shell. Reusable domain/protocol semantics move behind
Identus-owned Rust and binding types. A repository can be retired only when no
unique supported platform responsibility remains and every consumer gate is
satisfied.

## Compatibility and dependency evidence

`identus-uniffi-did` currently proves a narrow UniFFI 0.32 DID surface for
Swift and Kotlin, while `identus-wasm-did` proves a narrow wasm-bindgen browser
surface. These are experimental and do not establish supported bindings.
React Native remains a separate adapter program: the current upstream
`uniffi-bindgen-react-native` release `0.31.0-6` still pins UniFFI `=0.31`, while
the SDK native binding line uses 0.32. Mixing generator/runtime versions is not
acceptable; issue #223 remains the qualification gate.

Migration work must not leak third-party or raw Rust types across bindings.
Every slice requires stable Identus-owned DTO/error/state contracts,
cross-language vectors, target/runtime evidence, and a rollback path. The
migration ledger records compatibility and dependency substitutions before a
consumer changes.

The inspected SDK snapshots are pinned source revisions. Their exact version
and feature sets come from the repository package manifests and will be copied
into the normalized inventory rather than inferred from the default branch.
The bootstrap adds no direct dependency or resolved dependency cone. A later
binding or engine change must record both the direct and resolved dependency
cone before adoption. Public and wire compatibility includes source APIs,
generated ABI, serialized values, error codes, persistence, and package
requirements; none may be assumed from similarly named modules.

## Security, privacy and maintenance evidence

Moving sensitive semantics to one core can reduce duplicate cryptographic and
protocol implementations, but FFI and WASM create memory, ownership, redaction,
threading, async, cancellation, and supply-chain boundaries. Secrets and
untrusted inputs require purpose-built binding surfaces. Native platform
keychains, secure enclaves, databases, networking, lifecycle, and user-consent
policy remain outside generic Rust unless a separate capability proves a
portable contract.

Each candidate capability must record maintainership, specification currency,
unsafe/native reach, dependency cone, licensing, MSRV/target evidence, test
authority, consumer impact, and operational ownership. Service repositories
need separate threat and runtime analysis before any port.

License and provenance are known at repository level as Apache-2.0 for the
three inspected SDKs; exact transitive license and provenance evidence remains
mandatory per migrated dependency. Supply-chain evidence must include the
lockfile, source, release, advisories, features, unsafe/native-code reach, and
supported targets. Maintenance, release, and security posture are reassessed
at implementation time rather than frozen by this survey. Protocol or draft
currency is recorded per capability so a legacy draft does not silently
override a Final standard or reviewed Identus profile.

## Rejected or deferred candidates

Immediate archival of any SDK is rejected. A single universal binding
technology is rejected: browser WASM, native UniFFI, and React Native have
different runtimes and support evidence. Porting `cloud-agent` or `mediator` in
this program bootstrap is deferred because their service, persistence,
deployment, and operational surfaces are much larger than an SDK facade.

Legacy-bug simulation is deferred to capability decisions. The default is to
fix a bug and document a break; simulation is allowed only when a released
consumer contract depends on it, cannot migrate atomically, and a bounded,
observable, time-limited compatibility mode has an owner and removal release.

The reconsideration trigger for the default thin-facade disposition is evidence
that a language repository has no unique supported facade, packaging, or
platform responsibility and all consumers have completed migration. The
reconsideration trigger for the React Native hold is an exact released
generator/runtime combination compatible with the selected UniFFI line plus
iOS and Android New Architecture runtime/package evidence.

## Open questions and blockers

There are no blockers to program bootstrap. Per-capability blockers include
which legacy tests represent public contracts, current downstream usage,
package-version compatibility windows, missing Rust behavior, exact binding
support, and whether a reported divergence is a bug or supported behavior.
Those questions are outputs of the inventory issues rather than reasons to
delay governance.

## Sources

- https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e
- https://github.com/hyperledger-identus/sdk-swift/tree/ebbfdb666a3b0c9905dd443589e03ccf2ae8087b
- https://github.com/hyperledger-identus/sdk-kmp/tree/5a8fda770bbbca84192979b2c8e6090ee94790e3
- https://github.com/jhugman/uniffi-bindgen-react-native/releases/tag/0.31.0-6
- sdk-rust issues #163, #223, and #415
- ADR 0061, ADR 0097 through ADR 0101, and ADR 0110

## Evidence commands

Exact commands included `git clone --depth 1`, `git rev-parse HEAD`, `find`,
`rg`, manifest inspection with `jq`, `cargo metadata --no-deps
--format-version 1`, and `gh api` queries for repository and release metadata.
No legacy repository was mutated. Unrun checks are per-SDK symbol/API
inventories, test-vector provenance, downstream telemetry, mobile runtime
proofs, dependency/license scans, package migration rehearsals, and consumer
canaries; child issues must run and attach those before implementation.

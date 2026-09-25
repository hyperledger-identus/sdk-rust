# OID4VC Rust reuse evidence completion

**Issue:** [#391](https://github.com/hyperledger-identus/sdk-rust/issues/391)
**Decision:** [ADR 0156](../../adr/0156-retain-oid4vci-and-spike-a-private-dcql-engine.md)
**Retrieval and measurement date:** 2026-09-26
**Baseline:** `develop@cd4d4eceef92cf18bb5c991f2957f2fac261d53e`

## Executive answer

Do not replace any delivered `identus-oid4vci` component. Start a future
generic OID4VP capability as an Identus-owned shell and consider exact
`siros-dcql 0.3.0` only as its private DCQL execution engine. Keep protocol
wire/state, request security, JOSE, trust, nonce/replay, consent, transport,
credential formats, bounds, and errors owned by the SDK. Keep Affinidi,
Impierce, and Spruce as independent conformance oracles.

The SIROS engine has positive reuse economics: one prospective incremental
package name can avoid independently authoring and maintaining a substantial
query/path/selection engine. It is not ready to adopt because its public parser
and models are unbounded and intentionally more tolerant than SDK policy.

## Capability and ownership matrix

Legend: **complete** means the assessed surface directly covers the seam;
**partial** means useful behavior exists but not the full SDK need; **oracle**
means framework behavior can inform differential tests but is not accepted as
an SDK boundary; **absent** means the assessed component does not own it.

| Seam | Current SDK | SIROS DCQL 0.3.0 | Affinidi OID4VC | Impierce OpenID4VC | Spruce OpenID4VP | SDK decision |
| --- | --- | --- | --- | --- | --- | --- |
| OID4VCI Final wire and state | **complete/partial by the 24-row conformance ledger** in `identus-oid4vci` | **absent** | **oracle** framework | **oracle** framework | **absent** (VP repository) | Retain local bounded implementation. |
| OID4VP public wire and state shell | **partial** generic request/candidate/disclosure/artifact/lifecycle semantics; no accepted wire engine | **partial** DCQL model only | **oracle** VP framework, PEX-oriented at assessed release | **oracle** Final-labelled framework | **oracle** Final-labelled framework | Build an Identus-owned generic shell. |
| Bounded parsing and structural limits | **complete** pattern in current protocol crates; OID4VP wire parser not built | **absent**; `from_json` and public collections are unbounded | **not accepted** as SDK boundary | **not accepted** as SDK boundary | **not accepted** as SDK boundary | SDK facade must validate before delegation. |
| Stable redacted public errors | **complete** pattern and golden contracts | **absent**; diagnostics may contain verifier ids | **not accepted** | **not accepted** | **not accepted** | Map to Identus-owned categories. |
| OAuth authorization/PKCE | **complete for delivered OID4VCI seams**; `oauth2` remains oracle | **absent** | **oracle** integrated client behavior | **oracle** integrated behavior | **oracle/partial** VP request transport | Keep local/injected mechanics. |
| DCQL wire model | **partial** generic presentation queries, no Final wire parser | **complete but tolerant/unbounded** | **absent at assessed VP surface** | **oracle**, coupled to framework models | **oracle**, coupled to SSI/JOSE graph | Candidate may stay private behind strict owned model. |
| DCQL claim-path evaluation | **not implemented as a Final DCQL engine** | **complete** for JSON and mdoc path rules | **not established** | **oracle** | **oracle** | Conditional private reuse. |
| DCQL credential/set selection | **partial** generic candidates/plans | **complete** with caller policy and bounded combination enumeration | **not established** | **oracle** | **oracle** | Conditional private reuse; bound inventory before execution. |
| Signed request objects | **absent for OID4VP**; reusable JOSE primitives exist | **absent** | **oracle** framework | **oracle** framework | **oracle** framework | Compose from `identus-jose`; do not delegate policy. |
| Client identifier schemes | **absent for OID4VP** | **absent** | **oracle** framework | **oracle** framework | **oracle** framework | SDK-owned typed policy. |
| Response modes and response binding | **absent for OID4VP**; generic presentation artifacts exist | **absent** | **oracle** framework | **oracle** framework | **oracle** framework | SDK-owned state transition and lineage. |
| Digital Credentials API adapter | **absent** | **absent** | **not established** | **oracle/adapter coupled** | **oracle/adapter coupled** | Outer optional adapter, never generic core. |
| Request/response encryption | **absent for OID4VP**; JOSE primitives reusable | **absent** | **oracle** framework | **oracle** framework | **oracle** framework | Compose through `identus-jose` with explicit algorithms. |
| Nonce and replay correlation | **complete pattern in OID4VCI/JOSE; absent for OID4VP** | **absent** | **oracle** framework | **oracle** framework | **oracle** framework | SDK-owned injected state/replay ports. |
| Trust injection and verification states | **complete architecture pattern** | **policy hook only for format/meta; no trust engine** | **oracle** framework policy | **oracle** framework policy | **oracle** framework policy | SDK-owned explicit parsed/verified/trusted states. |
| Credential-format abstraction | **complete generic envelope/presentation boundary; formats staged separately** | **partial** string format plus caller policy | **oracle** framework model | **oracle** framework model | **oracle** SSI model | Identus types; adapter maps privately. |
| JOSE/crypto | **complete reusable primitives for current needs** | **absent** | **oracle** coupled implementation | **oracle** coupled implementation | **oracle** git-pinned/coupled implementation | Reuse `identus-jose`/`identus-crypto`. |
| HTTP/browser/mobile transport | **injected ports / outer adapters** | **absent** | **oracle** runtime coupling | **oracle** Reqwest/Tokio coupling | **oracle** Reqwest/Tokio coupling | Keep outside generic protocol crates. |

“Oracle” does not assert feature parity. It records that the framework can
produce independent interoperability evidence while its public model and
runtime remain excluded. Exact candidate provenance and reasons are in the
[portfolio](../rust-library-reuse/repository-portfolio.md) and archived #391
research.

## Clean-room differential vectors

The fixture now has eight independent tests. They are derived from OpenID4VP
1.0 Final rules rather than copied from candidate tests:

| Vector | Expected Final-profile behavior | Candidate result |
| --- | --- | --- |
| One bound credential with requested claim | Satisfiable; select exact claim | Pass |
| Missing claim or missing holder binding | Credential is not offered | Pass |
| Duplicate credential query id | Reject ambiguity | Pass through redacted adapter error |
| Empty credential-query list | Not satisfiable | Pass at evaluation; parser tolerance remains a facade mismatch |
| Claim `values` type mismatch (`true` vs `"true"`) | No match | Pass |
| Required credential-set alternatives | One complete option satisfies the set | Pass |
| Combination enumeration limit | Emit at most caller limit and disclose dropped count | Pass |
| Missing `meta` and dotted identifier | SDK strict facade must reject/normalize policy explicitly | Candidate deliberately accepts; preserved mismatch |

The existing tests also prove the 16 KiB pre-parse boundary, category-only
diagnostics, and candidate-free summary output. The SDK has no OID4VP engine to
compare byte-for-byte yet; this is candidate-versus-normative differential
evidence, not two-implementation parity.

## Dependency and deletion payoff

| Measure | Exact observation | Interpretation |
| --- | ---: | --- |
| Direct candidate dependencies | 2 (`serde`, `serde_json`) | Already-established workspace families. |
| Resolved candidate cone | 12 registry packages | Small relative to assessed full frameworks. |
| Prospective incremental root package names | 1 (`siros-dcql`) | The fixture itself is unpublished and excluded; existing names may resolve to workspace-selected versions. |
| Candidate production source | 1,421 physical Rust lines | Proxy for generic engine maintenance avoided; not guaranteed SDK deletion. |
| Candidate tests | 1,254 physical Rust lines | Substantial upstream behavioral evidence, still non-normative. |
| Current adapter before tests | 43 physical Rust lines | Demonstrates type/error isolation is small at spike scope. |
| Current complete fixture | 160 lines before this follow-up | Research harness cost; not production code. |

There is no existing local DCQL engine to delete, so “code deletion” is
prospective avoided implementation rather than a negative diff. The decision
becomes unfavorable if the future strict mapper, limits, and result translation
approach the candidate's 1,421-line engine surface or duplicate most selection
logic. That is an explicit production reconsideration gate.

## Compile and resource evidence

On the current `aarch64-darwin` host with Rust 1.98.1, a clean exact locked
`cargo check` in a fresh target directory measured:

- 7.83 seconds wall, 5.27 seconds user, 1.15 seconds system;
- 290,275,328 bytes maximum resident set size reported by `/usr/bin/time -l`;
- successful compilation of the fixture and all 12 registry packages.

This single cold diagnostic includes compiler/dependency work and is not a
runtime benchmark or CI SLO. Incremental builds, other hosts, linking, package
size, and device runtime were not measured.

Runtime allocation is structurally assessed, not instrumented: parsing owns
strings, vectors, maps, and JSON values; execution builds a result for every
query/credential match and clones selected ids/paths; the combination limit
bounds enumerated combinations but does not bound the earlier match matrix.
Production use therefore requires SDK limits for input bytes/depth/nodes,
query/claim/set counts, path length, credential inventory, values, matched
candidates, and combinations. Adding an unsafe global allocator solely to
count allocations was rejected under the unsafe-code policy.

## Target, security, and rollback evidence

Exact locked host tests and strict Clippy pass on Rust 1.98.1. Host compilation
passes with Rust 1.89.0, and Rust 1.98.1 compile checks pass for
`wasm32-unknown-unknown`, `aarch64-apple-ios`, and
`aarch64-linux-android`. These remain compile-only claims.

The candidate source denies unsafe and has no native/FFI/I/O surface. The exact
lock passes repository license/advisory policy. The adapter rejects oversized
bytes before candidate parsing and maps candidate errors without reflecting
verifier data. Rollback deletes the private dependency and mapper before
release, or removes this research fixture now; no wire, data, or consumer
migration is required.

## Acceptance reconciliation

| Issue #391 requirement | Evidence |
| --- | --- |
| Immutable source/license/publication/maintenance/MSRV/cone/unsafe/targets | ADR 0156, archived research, portfolio, exact fixture lock, and this report |
| Full capability matrix | Matrix above |
| Positive and negative spec-derived vectors | Eight fixture tests and vector table above |
| SIROS private bounded mapping | Fixture adapter and root-graph checker |
| Deletion/payoff, packages, compile, targets, allocation/resources, redaction, rollback | Measurement sections above |
| Per-seam decision | Capability matrix and ADR 0156 |
| Replace delivered OID4VCI? | No |
| OID4VP architecture? | Identus-owned shell, conditional private DCQL engine |

No acceptance item activates production adoption. The next implementation
issue may be created only when IDR-024 has a named consumer and complete public
facade/limit design.

## Reproduction commands

```text
wc -l <siros-dcql-0.3.0>/src/*.rs <siros-dcql-0.3.0>/tests/*.rs
comm -23 <fixture-lock-package-names> <root-lock-package-names>
/usr/bin/time -l env CARGO_TARGET_DIR=<fresh-temp> cargo check --locked --manifest-path docs/research/siros-dcql-spike/Cargo.toml
scripts/check-siros-dcql-spike.sh
cargo +1.89.0 check --locked --ignore-rust-version --manifest-path docs/research/siros-dcql-spike/Cargo.toml
cargo check --locked --manifest-path docs/research/siros-dcql-spike/Cargo.toml --target <wasm|ios|android>
```

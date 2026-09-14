# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

The assessed source is sdk-rust
`develop@707a5a22c3fad18724d5c5cac953e7387f7e49d8`. The earlier issue baseline
at `dbef9923e65d1a8332c4ba38f42532c8a12811f7` counted 35,482 physical lines in
117 `crates/*/src` files and 24,513 lines in 67 wider test files. A current
`wc` count is close but cannot establish a production population: 21 source
files contain `#[cfg(test)]` items, and path-only classification counts their
inline tests as shipped code.

The current implementation has no repository audit wrapper.
`rust-code-analysis-cli` exposes source locations and function SLOC,
cyclomatic, and cognitive metrics. The exact locked nixpkgs revision resolves
version 0.0.25 on the assessed host. It does not evaluate conditional
compilation itself, so the repository wrapper must recognize Rust comments,
literals, contiguous attributes, item/field/variant boundaries, ordinary
out-of-line module resolution, and three-valued `cfg` expressions.
An item is excluded from production only when its `cfg` expression is
definitively false with `test = false`; unknown features and targets remain in
production. Only Cargo `tests/` and `benches/` trees are intrinsic test targets;
`src/tests.rs` and similarly named source trees require proven test-only module
reachability. Nested inline module names remain part of ordinary out-of-line
resolution. This prevents a regex, directory name, or same-named sibling module
from silently discarding shipping code. V1 rejects `#[path]` overrides in
test-only reachability rather than guessing custom path semantics, and treats
outer doc comments as part of the attributed item. Production-active or unknown
module reachability takes precedence over a test-only edge to the same file and
propagates through its descendants.

Rust cfg metadata also admits comments, raw strings, raw identifiers and
conditional attribute application. The repository evaluator removes comments
without changing literal tokens, distinguishes raw identifiers from boolean
literals, accepts Unicode identifiers, and evaluates nested `cfg_attr`
recursively. False predicates short-circuit applied metadata; unknown
applicability or syntax outside the pinned evaluator remains production.

Review also established that hand-parsing every Rust member, generic and
expression form would itself become an architecture liability. V1 therefore
subtracts only locally proven semicolon/comma and recognized block-item/macro
spans, with production winning on a mixed line. Ambiguous angles, closing
containers and nested block forms remain production. Issue #275 tracks a
future non-published `syn` classifier rather than extending this heuristic.
Inner `#![cfg(...)]` scopes likewise remain production until that classifier
can model their exact AST scope.

Review of the first implementation found that canonical structure alone could
not bind a report to source and that generic generated-marker text was
ambiguous. The final contract pins revision, source fingerprint and full report
digest; fast validation recomputes source evidence from the Git tree, and a
weekly exact-engine gate regenerates the entire report. Generated exclusion is
an exact path-and-header-marker policy entry only.

## Normative sources

- [Rust reference: conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
  supplies the primary `cfg` semantics; the wrapper evaluates only the
  production fact `test = false` and treats other predicates as unknown.
- [rust-code-analysis 0.0.25](https://github.com/mozilla/rust-code-analysis/tree/v0.0.25)
  supplies versioned function metrics, not an architecture verdict.
- ADR 0110 and the SDK blueprint define cohesion, orthogonality, dependency
  direction, and reusable boundaries.

Protocol/draft currency is not applicable to this architecture-tooling slice;
the DID refactor preserves the existing pinned W3C behavior without making a
new protocol interpretation.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `rust-code-analysis` | 0.0.25 from locked nixpkgs | `adopt` | Mature syntax analysis and function metrics behind an SDK-owned report facade | Reconsider when it cannot parse the effective Rust edition or a maintained compatible release fixes a demonstrated defect. |
| repository Python wrapper | schema/algorithm v1 at the exact Git revision | `retain-local` | Population, disposition, privacy, and anti-gaming rules are repository policy | Reconsideration trigger: a neutral maintained tool implements the complete contract without broader coupling. |
| raw `wc`/path/regex baseline | not versioned | `not-adopt` | Cannot separate inline tests or provide function evidence | Reconsideration trigger: none; it may remain a diagnostic only. |

## Compatibility and dependency evidence

No Cargo dependency or supported feature changes. The direct and resolved
dependency cone for runtime crates is unchanged; the analysis executable is a
development-shell package only. The effective MSRV and primary compiler remain
Rust 1.98.1. The command is host tooling and makes no new iOS, Android, WASM,
or other target compatibility claim. Public and wire compatibility remains
exact because the Rust refactor is crate-private and characterization-bound.
The facade boundary is the repository Python command and canonical JSON schema;
`rust-code-analysis` types or output are not exposed to crates. Rollback is one
PR revert.

## Security, privacy and maintenance evidence

The analyzer reads tracked source and reports only paths, spans, counts, and
classifications; it neither executes source nor emits source bodies. Authored
unsafe and external native code are unchanged because no Cargo dependency is
added. The Nix-locked package preserves supply-chain provenance through the
existing flake input and lock review. rust-code-analysis maintenance and
release posture is isolated behind the local facade; the SDK does not claim
its security audit or runtime support.

## Rejected or deferred candidates

Hard global thresholds, a new public quality crate, regex-only `cfg(test)`
exclusion, and cross-protocol HTTP helper extraction are rejected. Wallet
`run_exact`, derive token-generation, presentation/DID module splits, and
OID4VCI work are deferred with owners in the table below; they require their
own characterization and exact issues.

## Evidence interpretation and hotspot disposition

Metrics are attention signals, not architecture verdicts. Cognitive or
cyclomatic complexity above 15, a function above 100 SLOC, or a module above
1,000 authored nonblank production lines enters review only. A large module is
a decomposition candidate only when it also owns at least two independent
invariants or change axes. Exact normalized duplicate sequences are triage
evidence; shared extraction is allowed only when invariants, bounds, error
projection, ownership, and likely change cadence match.

Moves, renames, forwarding wrappers, formatting, generated output, macro
hiding, or splitting one semantic responsibility across files do not establish
improvement. The touched scope is the semantic responsibility and its call
cluster, not merely edited lines. A review records any new or worsened signal
and its disposition; numeric signal changes are reported and do not fail CI by
themselves.

| Surface | Disposition | Evidence and owner |
| --- | --- | --- |
| `did/src/cache.rs::resolve_inner` | `defer-with-owner` | Cache orchestration is cohesive and already tracked by #50; characterize concurrency/failure behavior before decomposition. DID owner. |
| `did/src/did.rs::parse_did_url` | `document-exception` | One grammar/state-machine invariant; splitting would obscure cursor state. DID owner. |
| `derive/src/attr.rs::from_attributes` | `document-exception` | One compile-time attribute grammar and diagnostic surface. Derive owner. |
| `presentations/src/model.rs` constructors/validation | `decompose` | Request, submission, candidate, and verification invariants have separate change axes. Presentation owner; future issue. |
| `did/src/document.rs` validation | `decompose` | Document model and validation traversal can gain private cohesive modules after characterization. DID owner; future issue. |
| `did/src/resolution.rs` | `decompose` | Value types, result models, wire preflight, and validation are independent private-module candidates. DID owner; future issue. |
| `did/src/registration.rs` | `decompose` | Lifecycle model and recursive extension validation require characterization first. DID owner; future issue. |
| `oid4vci/src/json.rs`, `limits.rs` | `defer-with-owner` | #168 owns resource-bound semantics; this issue does not change them. OID4VCI owner. |
| `oid4vci/src/error.rs` | `defer-with-owner` | Error architecture is issue #271 / ADR 0116, not this slice. OID4VCI owner. |
| `wallet-conformance/src/lib.rs::run_exact` | `decompose` | Scenario orchestration and evidence projection have separate axes; explicitly excluded from this PR. Wallet-conformance owner; future issue. |
| DID/OID HTTP token grammar | `document-exception` | Similar syntax serves separate protocol ownership and error/bounds contracts; no upward dependency is justified. |
| DID cache/registry standard resolution failure | `deduplicate` | Identical W3C failure invariant in one crate; centralize now in the owning resolution module. |
| derive numeric/string serde generation | `deduplicate` | Shared token-generation structure merits a later focused refactor with trybuild snapshots; excluded now. Derive owner. |

## Open questions and blockers

There are no research blockers. Exact current counts and signal locations will
be generated after the planning-only commit because the final report is an
implementation artifact. The declared attention values do not need sponsor
selection because they are non-failing prompts, not budgets or promises.

## Evidence commands

Commands completed include `gh issue view 270`, `git rev-parse HEAD`, `find`
and `wc` source inventory, `rg` hotspot inspection,
`rust-code-analysis-cli --version`, JSON analysis of
`crates/core/src/lib.rs`, ADR/constraint inspection, and `scripts/factory
doctor`.

Unrun commands are the implemented audit and its tests, Cargo
format/build/Clippy/test, and Nix/factory gates. Exact version, license,
provenance, consumer evidence, feature/target behavior, direct and resolved
dependency cone, unsafe/native and supply-chain limitations, public/wire
compatibility, maintenance/release/security posture, protocol/draft currency,
facade, rollback, and reconsideration triggers are recorded above; no unrun
gate is represented as passing.

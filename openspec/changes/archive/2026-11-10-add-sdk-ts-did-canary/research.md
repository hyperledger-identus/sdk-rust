# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-10
Source retrieval date: 2026-11-10
Research blockers: none

## Problem and existing implementation

The current implementation uses `identus-did` as the canonical bounded parser
for DID and DID URL values.
`identus-wasm-did` already exposes binding API version 1, exact component views,
stable redacted error codes, a generated TypeScript declaration snapshot, two
reproducible `wasm-pack --target web` builds, and Chromium/Firefox execution.
The generated surface exposes WASM-owned classes and `free()`, has no Node.js
package, and has no TypeScript facade, provider rollback seam, React lifecycle,
shared SDK-TS assertions, or target-isolated CI receipts.

The canonical `did.syntax.v1` packet contains 22 positive, negative,
consumer-regression, and exact-boundary cases at SHA-256
`a7ed2b8f1351c8aedcb2b38f8849a1e3d81e058f6b5636413340cf4157733508`.
The four active language mappings bind SDK-TS 8.1.4 at
`4bf86ebf69d5e96616a148e4c973f831f95fa38e` to canonical source record
`did.source.identus-did.v1`. Issues #420, #422, #501, #504, #505, and #512 are
closed, so the A1 prerequisites are complete.

## Normative sources

- W3C DID Core 1.0, sections 3.1 and 3.2:
  <https://www.w3.org/TR/2022/REC-did-core-20220719/>.
- SDK-Rust `identus-did`, `identus-wasm-did`, browser binding specification,
  adapter mapping registry, vector catalog, and quality declarations.
- ADRs 0101, 0163, 0169, 0170, and 0174.
- SDK-TS 8.1.4 at immutable revision
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`; its tests are consumer evidence,
  not canonical grammar or public API authority.
- Issues #415, #420, #422, #492, #501, #504, #505, #512, #594.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Expose generated WASM classes directly | `not-adopt` | Leaks allocation/lifecycle and generated API details into the SDK contract. |
| One universal web WASM artifact for browser and Node | `not-adopt` | Loader, module, filesystem, worker, and asset semantics differ; browser success is not Node evidence. |
| Private plain-value facade plus provider boundary | `adopt` | Hides WASM ownership, keeps Rust authoritative, and supports opt-in rollback before initialization. |
| Separate `web` and `nodejs` wasm-pack artifacts | `adopt` | Gives each runtime an explicit loader and independently testable artifact. |
| Runtime environment guessing | `not-adopt` | Conditional package exports are deterministic and avoid accidental dual initialization. |
| React package with duplicated DID logic | `not-adopt` | React owns lifecycle only and must reuse the browser/Node providers. |
| React Native over browser WASM | `not-adopt` | ADR 0163 requires a later native New Architecture/TurboModule package. |
| Modify SDK-TS in this repository | `not-adopt` | The consumer repository is separately owned; this slice prepares its exact reusable package and evidence. |
| Local-first package and target gates | `adopt` | Hosted CI is final confirmation, not the primary edit/test loop. |

## Public TypeScript shape

The additive facade exports readonly plain `DidValue` and `DidUrlValue`, a
closed `DidErrorCode`, a normal redacted `DidError`, a `DidProvider`, and
target-specific provider creation. Generated classes, initializer functions,
raw errors, memory, and `free()` remain private. Projection copies fields into
frozen objects and frees every raw success or error in `finally` paths.

Provider selection occurs before initialization. A Rust rejection never falls
through to a second parser. Rollback selects the legacy provider explicitly;
the repository-local fixture proves the seam but does not claim that SDK-TS has
adopted it.

## Target and package evidence

- Browser: private web ESM artifact, direct and bundled imports, Chromium and
  Firefox, exact vector assertions, asset pairing, lifecycle, and rollback.
- Node.js: private node artifact, Node 24, ESM import and CommonJS require,
  singleton initialization, exact vector assertions, and rollback.
- React web: client lifecycle over the browser provider; SSR and hydration cross
  the Node contract and reuse the same public facade.
- React Native: explicit not-applicable receipt for this WASM canary; no native
  toolchain or support claim.

## Compatibility and dependency evidence

The repository default shell already pins Node 24 and the WASM shell pins
`wasm-pack` 0.15.0, `wasm-bindgen-cli` 0.2.121, Chromium, and Firefox. Registry
inspection on 2026-11-10 selected exact development candidates: TypeScript
7.0.2 (Apache-2.0), `@types/node` 24.19.2 (MIT), React/React DOM 19.3.1
(MIT), `@types/react`/`@types/react-dom` 19.3.0 (MIT), Vite 8.3.7 (MIT), and
`playwright-core` 1.65.0 (Apache-2.0). The package lock is mandatory; no runtime
dependency enters Rust or the published Cargo graph.

Alternative full test frameworks were rejected for the canary. Node's built-in
test runner, strict TypeScript, Vite, and `playwright-core` are sufficient for
module, browser, React, and shared-contract evidence with a smaller dependency
cone.

The exact version and minimal feature selection is recorded above. The Rust
workspace retains MSRV 1.89.0 and the existing target matrix. The TypeScript
tools are development-only: their direct and resolved dependency cone is
locked independently and adds no Cargo feature, published runtime dependency,
or transitive Rust dependency. License and provenance were inspected through
the official npm registry metadata; lockfile integrity, registry provenance,
and the existing repository advisory checks provide the supply-chain boundary.
The generated bindings remain private, so this additive facade changes neither
the existing Rust public API nor wire compatibility. Rollback removes the
private package and target artifacts without changing canonical Rust values.

## Security, privacy and maintenance evidence

Only public identifier strings cross the boundary, but they remain untrusted
and bounded at 131,072 bytes for DID and 139,264 bytes for DID URL. Diagnostics
may record operation, provider route, result class, mapping ID, stable code, and
bounded timings; they never record input, component values, generated errors,
stacks, or WASM memory.

The canary must detect double initialization, use-after-free, missed disposal,
module-format drift, mismatched JS/WASM artifacts, stale exact-head receipts,
and accidental legacy public names. Size, initialization, parse latency, and
memory are measurement-only until a later target-support decision accepts
budgets. Coverage remains supporting evidence and follows SC-05.

The existing Rust/WASM boundary contains no new unsafe or native code in this
slice. The selected tools have active maintenance, release, and security
posture, but they remain locked development dependencies rather than SDK
runtime authorities. W3C DID Core 1.0 is the stable protocol authority; no
unpublished draft grammar or legacy SDK-TS parser becomes normative.

## Quality evidence decision

`did.syntax.quality.v1` proves the Rust foundation but its A1 benchmark was not
applicable before a named target consumer existed. This change adds
`did.syntax.typescript-canary.quality.v1` with four required pending obligations:
provider/property parity, fresh DID fuzz evidence, target resource benchmarks,
and TypeScript/Rust differential execution of all 22 vectors. No stale fuzz or
historical browser receipt is promoted as current canary evidence.

## Rejected or deferred candidates

The `not-adopt` candidates in the decision table are rejected for this slice.
Actual SDK-TS repository wiring, a default-route switch, npm publication,
accepted support budgets, and React Native/TurboModule delivery are deferred.
The reconsideration trigger is a separately approved consumer change with a
named SDK-TS rehearsal, exact target evidence, accepted budgets, and an owned
rollback window.

## Open questions and blockers

There are no blockers to the repository-local facade/package/harness. The
separate SDK-TS repository rehearsal, default routing decision, npm publication,
support activation, accepted budgets, and React Native package remain future
gates and keep issue #492 partial after this local train.

## Evidence commands

Read-only inspection used `gh issue view`, repository-local checkers, exact
source/ADR/spec review, `npm view` against the official registry, and current
Node/npm version inspection. No workflow was dispatched or rerun, no package
was published, and no consumer repository was mutated. The exact commands are
captured by the shell history and this change's task list; the slow target,
fuzz, benchmark, complete factory, and flake checks are intentionally unrun
until implementation exists.

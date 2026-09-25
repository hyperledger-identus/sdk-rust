# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-26
Source retrieval date: 2026-09-26
Research blockers: none

## Problem and existing implementation

The reproducible mdBook site, Graphviz sources and offline link checker already
exist. Its navigation and prose were intentionally scoped to the first crypto
train. The protected base now also contains the closed DID train registry,
candidate descriptor, package READMEs, API baselines, ADRs 0153/0154 and
archived verification receipts from #382/#384. No new documentation framework,
renderer or dependency is required.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Add a separate DID candidate page and diagram | `adopt` | Keeps the two trains and their different lifecycle states explicit without overloading crypto crate pages. | The handbook adopts a machine-generated train catalog. |
| Rewrite the first train as one five-crate release | `not-adopt` | The crypto train is published/release-gated while DID remains candidate-only with an independent primary-package tag. | A later coordinated release ADR changes train ownership. |
| Add DID crates to the existing crate index | `adopt` | The index is the discoverable package responsibility surface; status and links can remain explicit. | Navigation structure changes. |
| Present docs.rs or crates.io commands | `not-adopt` | Canonical DID manifests remain `0.0.0`, `publish = false`; no registry artifact exists. | Protected publication succeeds and has an immutable receipt. |
| Use exact Git revision examples | `adopt` | This is the current supported evaluation identity and already governed by source-distribution policy. | Registry publication replaces source-only evaluation. |
| Reuse current mdBook/Graphviz/Nix pipeline | `adopt` | It already proves deterministic static rendering and offline links. | The site toolchain receives a separate accepted replacement. |

## Normative sources

The repository sources of truth are M5 coordinator #381, issue #386, ADRs
0132, 0153 and 0154, `docs/release/release-trains.toml`,
`docs/release/did-candidate.toml`, the canonical `release-candidate-trains` and
`sdk-documentation-site` specifications, and the archived #382/#384 review and
verification receipts. W3C behavior and consumer repositories are unchanged;
no external specification interpretation is introduced.

## Compatibility and dependency evidence

This slice changes static documentation inputs only. It adds no Cargo, Nix or
browser runtime dependency, public Rust API, wire shape, feature, error,
compiler or target behavior. The DID package dependency direction is copied
from the closed descriptor and normalized manifests: `identus-did` uses
published `identus-core`/`identus-derive`; the optional resolver HTTP package
uses `identus-did`/`identus-core` plus host HTTP dependencies.

## Evidence matrix

- **Current implementation:** protected base `cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16`.
- **Source/provenance:** repository-owned Apache-2.0 text and tracked DOT only;
  no external fixture or donor code.
- **Tools:** existing locked mdBook, Graphviz and lychee derivation; no version
  change.
- **Features/targets:** documentation describes current package features but
  activates no support claim; #387 owns compiler/target qualification.
- **Dependency/unsafe/native:** no code or dependency change.
- **Compatibility:** additive/reconciliatory documentation only; canonical
  Rust and wire behavior unchanged.
- **Security/privacy:** static site remains script-, analytics-, cookie-, auth-
  and credential-free; no secret/PII input exists.
- **Release:** candidate-only status is preserved; #388 owns final exact-SHA
  approval and #344 owns administrative publishing hardening.
- **Rollback:** revert the new page/diagram and reconciled prose/navigation;
  code, candidates and external state remain unchanged.
- **Commands still unrun:** site build, diagram render, offline links,
  Markdown/factory/OpenSpec checks and hosted CI run after planning preflight.

## Rejected or deferred candidates

A combined five-crate train, registry commands, platform qualification,
publication, downstream adoption and a new site framework are rejected or
deferred as recorded above.

## Security, privacy and maintenance evidence

The generated site remains static and uses no mutable client script,
analytics, cookies, authentication or credentials. Content contains no secrets
or PII and introduces no executable or dependency surface. Maintenance reuses
the existing site owner, tracked DOT sources and locked build; links to issue
and ADR evidence make later lifecycle changes reviewable rather than silently
stale.

## Open questions and blockers

None. Existing accepted records resolve package identity, layering and claim
boundaries.

## Evidence commands

Before implementation, run factory doctor, research/constraints readiness,
strict OpenSpec validation and immutable preflight. After preflight, build the
site through the locked Nix target or exact resolved tools, inspect generated
diagrams/pages, run offline links, Markdown/factory tests and exact-head hosted
CI.

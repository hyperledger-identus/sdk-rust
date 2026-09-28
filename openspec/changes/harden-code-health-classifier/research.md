# Research

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

ADR 0126 replaced a hand-written Python syntax subset with an unpublished
`identus-conformance` binary using workspace-locked `syn` and `proc-macro2`.
At the assessed `develop` revision, all protocol types, cfg semantics, AST
visitors, module graph traversal, line projection, orchestration, and tests are
co-located in a 1,972-physical-line binary source. The latest architecture
audit counts 1,179 authored nonblank production lines, exceeding the 1,000-line
attention threshold.

## Normative sources

- ADR 0115 defines population separation, the attention threshold, and the
  touched-semantic-scope ratchet.
- ADR 0126 defines `syn` ownership, protocol v2, production-wins reachability,
  fail-closed path behavior, Cargo-derived roots, and linear projection.
- `SDK-ARCH-004` requires explicit hotspot disposition and characterization.
- Issue #301 records the differential, stress, evidence-binding, path, and
  target-normalization acceptance criteria.

## Compatibility and dependency evidence

The implementation remains an unpublished binary inside the existing
`identus-conformance` package. It reuses the exact workspace-locked `syn`,
`proc-macro2`, `serde`, and `serde_json` dependencies, adds no feature, and
does not change MSRV or supported SDK targets. Baseline schema growth is an
internal repository-evidence migration with no downstream compatibility cost.

## Candidate decisions

| Candidate | Decision | Evidence |
| --- | --- | --- |
| Keep one binary source | `not-adopt` | It couples six independently testable responsibilities and remains above the attention threshold. |
| Mechanical file split | `not-adopt` | ADR 0115 forbids counting moves/wrappers as improvement. |
| Private cohesive modules | `adopt` | Keeps the binary unpublished while isolating protocol, cfg, spans/projection, graph reachability, and orchestration invariants. |
| Add another parser/grammar | `not-adopt` | `syn` already owns the supported authored Rust grammar and is workspace-locked. |
| Full fuzzing dependency in fast CI | `defer` | Deterministic bounded generated cases provide fast evidence; heavy boundary campaigns belong in weekly/manual execution. |

## Correctness findings

1. Canonical evidence lacks digests for classifier source and resolved parser
   versions.
2. Nested `cfg_attr` path predicates need evaluation in the active production
   and test configurations before ambiguity is decided.
3. The migration document's canonical report digest can drift without a
   mechanical check.
4. Conditional paths must be compared only in configurations where the module
   edge is reachable.
5. Cargo target paths containing lexical `.` or `..` components need contained
   normalization before matching source keys.
6. Seeded property cases showed that boolean literals nested inside `all`,
   `any`, or `not` were conservatively downgraded to unknown because `syn::Meta`
   does not represent literal booleans; ADR 0126 requires exact evaluation.

## Security, privacy and maintenance evidence

The classifier receives repository source through a bounded local JSON
protocol and emits line numbers and paths only. It performs no network, secret,
credential, or runtime SDK work. Fail-closed behavior protects evidence from
silently undercounting production. Cohesive modules reduce the blast radius of
future Rust syntax, Cargo target, and report-schema changes.

## Rejected or deferred candidates

An additional parser, a public classifier crate, macro expansion, unbounded
random fuzzing in fast CI, and hard wall-clock assertions are rejected or
deferred. A nightly fuzz target may be considered later only if deterministic
generated and near-bound fixtures expose a capability gap.

## Open questions and blockers

None for this bounded private-tooling change.

## Evidence commands

```text
scripts/factory research-ready harden-code-health-classifier
scripts/factory constraints-ready harden-code-health-classifier
cargo test -p identus-conformance --bin code-health-classifier
python3 scripts/tests/code-health-audit.py
python3 scripts/code-health-audit.py --verify-baseline
scripts/factory check
nix flake check
```

## Evidence strategy

Fast evidence uses table-driven rustc-shaped module fixtures, deterministic
generated cfg/path cases, protocol-bound rejection tests, and operation-count
assertions that prove projection and graph work are linear after sorting. A
separate ignored stress test records representative and near-bound elapsed
time; weekly/manual CI executes it so the PR fast lane remains focused.

## Blockers

None. The parser and serialization dependencies are already locked, issue #301
provides the exact decision, and the work changes private repository tooling
only.

## Context

The repository needs comparable evidence without turning a metric into an
architecture oracle. The tool must distinguish shipping source from external
and inline tests and must not reward moving or hiding a responsibility.

## Goals / Non-Goals

**Goals:** deterministic exact-head reports; locked analyzer version; explicit
population semantics; finite hotspot dispositions; touched-scope anti-gaming;
one proven low-risk deduplication.

**Non-Goals:** numeric CI thresholds, public crate/API changes, broad module
splits, parser/orchestration rewrites, OID4VCI bounds, or #7/#168 behavior.

## Decisions

### Keep a repository-owned wrapper around a pinned engine

The default Nix shell supplies `rust-code-analysis-cli` 0.0.25. A Python
wrapper checks that exact version, obtains function metrics as JSON, applies
repository population semantics, emits canonical JSON, and validates the
checked baseline. JSON uses sorted keys and deterministic path ordering.

### Classify cfg(test) by syntax and conservative evaluation

The wrapper lexes comments and Rust literals before finding outer attributes
and balanced item boundaries. It parses `cfg` combinators using three-valued
logic with `test = false`; only a definitively false expression marks an item
test-only. Exact tests cover comments/strings, `all`, `any`, `not`, adjacent
attributes, inline modules, and unknown predicates.

### Separate observation from policy

The snapshot reports production, external-test, and inline-test populations,
function signals, and explicit exclusions. ADR 0115 and a classification
manifest own dispositions. Threshold crossings and touched-scope deltas are
printed as review prompts. Exit failures are limited to missing/wrong tool,
invalid schema/configuration, analysis failure, stale exact-tree snapshots, or
unclassified declared hotspots—not numeric quality.

### Ratchet the semantic responsibility

A refactor review compares the responsibility and call cluster at base/head.
New or worsened signals require one disposition and owner. Moves, renames,
wrappers, formatting, generated/macro relocation, file splitting, or deletion
of tests cannot count as improvement. Shared code is accepted only if semantic
invariants and change cadence match.

### Centralize only the DID failure constructor

`resolution.rs`, which owns `DidResolutionResult` invariants, gains one
crate-private standard failure constructor. Cache and registry call it.
Characterization tests assert exact error kind, empty content, metadata, and
serialized JSON through public behavior. The helper does not enter the public
API and adds no dependency edge.

## Alternatives rejected

- `wc`, path globs, or regex-only `cfg(test)` removal cannot identify shipping
  source correctly.
- Hard file/function limits encourage wrappers and arbitrary splits.
- A new public utility crate would increase coupling for repository tooling.
- Deduplicating DID/OID HTTP syntax would merge distinct protocol error and
  bounds ownership based only on textual similarity.

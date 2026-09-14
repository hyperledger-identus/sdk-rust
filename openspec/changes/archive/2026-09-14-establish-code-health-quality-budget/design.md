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

The wrapper lexes comments and Rust literals before finding contiguous outer
attribute groups and context-aware item, field, and variant boundaries. It
parses `cfg` combinators using three-valued logic with `test = false`; only a
definitively false expression marks an item test-only. A test-only out-of-line
module declaration recursively transfers that classification through ordinary
Rust module resolution. Only Cargo `tests/` and `benches/` trees are intrinsically
external tests; all `src` files begin as production candidates. Nested inline
module names remain resolution context, so a test-only nested `helper` cannot
hide a shipping `helper.rs` in its parent directory. Exact tests cover
comments/strings, `all`, `any`, `not`, preceding outer doc comments and
adjacent attributes,
supported comma-terminated nodes, recognized brace-delimited item macros,
inline and out-of-line module trees,
same-named module collisions, macro token trees, rejected `#[path]` overrides,
source test files with and without proven test-only reachability, and unknown
predicates. Test reachability first produces a candidate tree. A fixed-point
pass over active or unknown edges from parents that remain production removes
mixed-reachable files and their descendants from inline-test classification.
Cfg predicate comments are blanked while literal tokens retain their value.
Raw string values and raw identifiers are tokenized, and nested `cfg_attr`
metadata recursively applies only when its predicate is true. Unknown
applicability, Unicode identifiers and unsupported predicate syntax cannot
remove an item from production. Stable cfg booleans are exact and inactive
`cfg_attr` branches short-circuit. The v1 terminator whitelist deliberately
retains ambiguous angles, comma-less container members, block expressions,
unrecognized macros and mixed test/shipping lines as production. Broader syntax
coverage belongs to the non-published `syn` helper tracked by issue #275.
Inner cfg scopes and Rust-valid whitespace-separated attribute openers such as
`# [cfg(test)]` are also retained as production until that helper exists. Such
an opener cannot seed test-only out-of-line module inheritance.

The policy pins baseline revision, source fingerprint and canonical report
digest. Fast validation reloads the Git tree and recomputes authored source
evidence; source-only Nix fixtures enforce the complete schema and digest. A
fetch-depth-zero slow command invokes the pinned analyzer and compares the
entire regenerated report. It is ready in the slow workflow but runs locally
or through explicit external orchestration while GitHub's reserved empty
default `main` cannot schedule the `develop` workflow. Issue #276 owns hosted
schedule activation. Generated exclusion requires an exact allowlisted path
and exact header marker.

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

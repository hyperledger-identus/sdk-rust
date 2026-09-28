# Design

## Module ownership

The binary entry point only performs bounded I/O and diagnostics. Private
modules own these cohesive responsibilities:

- `protocol`: request/response types, byte/file/path bounds, validation, and
  deterministic serialization;
- `cfg`: three-valued cfg/cfg_attr inclusion and path selection per active
  configuration;
- `spans`: AST attribute ownership, test-span collection, merging, and linear
  authored-line projection;
- `modules`: Rust module roles, contained path normalization/resolution, and
  bounded production-wins graph traversal;
- `engine`: per-source parsing and whole-request orchestration;
- `tests`: behavior, differential fixtures, generated cases, and stress checks.

All visibility stays private or `pub(super)` within the binary crate. No type
enters the `identus-conformance` library API.

## Conditional path semantics

For each reachable configuration, recursively evaluate `cfg_attr` predicates
under that configuration and collect only direct path assignments that apply.
A disabled configuration contributes no candidate. Multiple applicable values
within one reachable configuration or different values across two reachable
configurations are ambiguous and fail closed. Unknown predicates that can
apply a path retain both candidates and therefore fail closed unless they are
identical.

## Target normalization

Cargo manifest target paths are joined to the manifest directory, normalized
lexically, rejected if they escape the repository, and then matched against
the exact source-key population. The checker does not require filesystem
canonicalization and therefore remains valid for Git-tree fixtures.

## Evidence identity

A deterministic digest covers path/name plus bytes for every classifier module.
A second digest covers the resolved lockfile package tuples for `syn` and
`proc-macro2`. Both are declared in policy, embedded in the canonical report,
and checked before population output is trusted. The migration digest prose is
derived/verified against the same canonical fields.

## Complexity evidence

Projection exposes a test-only operation counter showing one monotonic pass over
merged spans and authored bytes. Graph traversal records bounded edge visits;
generated chain/diamond fixtures assert visits scale with declared edges.
Ignored stress tests exercise representative and near-limit inputs and print
elapsed timing for weekly/manual evidence without making wall-clock timing a
flaky fast-CI assertion.

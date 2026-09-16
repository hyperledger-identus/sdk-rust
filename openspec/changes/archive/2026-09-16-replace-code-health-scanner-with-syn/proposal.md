# Replace the code-health scanner with a Rust syntax classifier

## Why

The code-health v1 population classifier deliberately implements a small,
production-conservative subset of Rust syntax in Python. That was safe for the
first baseline, but extending it for fields, variants, parameters, statements,
match arms, macros, inner attributes, and future syntax would create a second
Rust parser. Issue #275 owns the planned migration to the workspace-pinned
`syn` family.

## What changes

- Add a non-published `identus-conformance` binary that parses authored Rust
  with locked `syn` 2.0.118 and reports deterministic test-only line and module
  reachability evidence through a bounded JSON protocol.
- Move cfg/cfg_attr evaluation, AST node spans, line projection, raw/ordinary
  module identity, path resolution, and production-wins reachability into that
  Rust classifier.
- Reduce `scripts/code-health-audit.py` to Git/source loading, classifier
  orchestration, metric-engine orchestration, and canonical report validation.
- Upgrade the code-health contract and canonical baseline with an explicit
  classifier identity and a reviewed v1-to-v2 population-delta report.
- Run fast source-evidence validation in the pinned Nix shell without invoking
  `rust-code-analysis-cli`; keep complete metric regeneration weekly/manual.

## Capabilities

### Modified capabilities

- `code-health-governance`: replaces partial source parsing with deterministic
  AST classification while preserving separate populations and conservative
  production ownership.
- `nix-tooling`: makes the classifier execution path available in the pinned
  shell used by fast and slow evidence.

## Non-goals

- No SDK public, wire, persisted-data, runtime dependency, or release change.
- No numeric architecture score or hard complexity threshold.
- No classification of macro-generated syntax that does not exist in the
  authored source AST.
- No relaxation of production-wins for mixed lines or shared module reachability.

## Delivery

Issue #275 owns implementation and migration evidence. The change targets
protected `develop` and is complete only after exact-head local/hosted gates,
review resolution, and canonical archive.

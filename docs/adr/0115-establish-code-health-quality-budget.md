# ADR 0115: establish a code-health quality budget

- **Status:** Accepted for implementation
- **Date:** 2026-09-14
- **Decision authority:** sponsor-directed issue
  [#270](https://github.com/hyperledger-identus/sdk-rust/issues/270)
- **Related:** ADR 0110, issues #50, #168 and #271
- **Assessed revision:** sdk-rust
  `707a5a22c3fad18724d5c5cac953e7387f7e49d8`

## Context

The growing monorepo has real concentration and duplication signals, but line
count and complexity alone cannot establish cohesion. The original baseline
also counted inline `cfg(test)` code as production. A hard threshold would
reward arbitrary file splits, wrappers, macro hiding, and loss of tests.

## Decision

The repository adopts the deterministic v1 evidence contract in
[`code-health.md`](../architecture/code-health.md). The locked default Nix
shell supplies `rust-code-analysis-cli` 0.0.25; a repository wrapper owns
population semantics and canonical reporting. Production, external-test,
inline-test, and generated populations stay separate. Conditional compilation
uses conservative three-valued evaluation with `test = false` rather than a
path or regex heuristic, and a test-only out-of-line module declaration passes
that classification through its ordinary Rust module tree.

The policy pins the baseline Git revision, authored-source fingerprint and
whole canonical report digest. Fast validation reloads that Git tree and
recomputes its source fingerprint, generated allowlist and line/file
populations. The weekly slow gate regenerates analyzer-derived function and
signal evidence with the exact engine and compares the entire report. Generated
Rust is excluded only by an exact policy path plus an exact header marker.

Attention prompts are cognitive or cyclomatic complexity above 15, a function
above 100 SLOC, or a module above 1,000 authored nonblank production lines.
They never fail merely because of the number. A named hotspot receives exactly
one `decompose`, `deduplicate`, `document-exception`, or `defer-with-owner`
disposition with evidence and an owner.

Every code-health refactor ratchets the touched semantic responsibility and
call cluster. A new or worsened signal must be classified. Moves, renames,
wrappers, formatting, generated or macro relocation, arbitrary file splits,
or test deletion/reclassification are not improvement. Extraction is justified
only by shared invariants, bounds, errors, ownership, and change cadence.

The first production slice centralizes identical standard DID resolution-
failure construction in the owning `resolution.rs` module behind a
crate-private helper. Cache and registry retain their exact public result,
metadata, standard error kind, wire JSON, features, and dependencies.

## Consequences

- Developers and agents receive exact, comparable triage evidence without a
  repository quality score.
- Large cohesive parsers may remain documented exceptions; large modules with
  independent invariants have a reasoned decomposition backlog.
- Production and test investment cannot be confused or traded against each
  other.
- The wrapper and classification manifest require maintenance as Rust syntax,
  the analyzer, or repository responsibilities change.

## Alternatives rejected

- Hard size/complexity CI limits are gameable and ignore domain ownership.
- `wc`, directory-only classification, and regex `cfg(test)` removal cannot
  establish the production population.
- A public quality crate would add runtime/API coupling for repository policy.
- Cross-protocol helper extraction based on token similarity would merge
  different bounds, error types, and normative change cadence.

## Verification and rollback

Focused scanner tests cover comments, literals, balanced attributes, fields,
variants, out-of-line module inheritance and cfg logic. Report mutation tests
and fast/slow regeneration cover the complete schema, engine pin, policy/tree
binding, population separation, exclusions and dispositions. DID tests cover registry and
failed-closed cache results. Full factory, formatting, strict Clippy, tests,
builds, and relevant Nix gates prove the refactor preserves repository health.

Revert this issue-linked PR to remove the contract and restore the two private
DID constructors. No public, wire, persisted-data, dependency, #7, or #168
migration is required.

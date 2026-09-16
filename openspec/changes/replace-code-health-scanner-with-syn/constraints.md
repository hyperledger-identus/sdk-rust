# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/275
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-004`: code-health evidence remains a review input, not a score.
- `SDK-RUST-001` and `SDK-COMPAT-005`: classifier compilation uses exactly Rust
  1.98.1; it creates no publication/MSRV promise for the helper itself.
- `SDK-SUPPLY-001`: `syn`, `proc-macro2`, serde, and JSON execution remain
  Cargo.lock/Nix-owned with no runtime download or floating tool.
- `SDK-LIM-004`: full metrics regeneration stays weekly/manual; fast validation
  runs only source population evidence through the small AST helper.

## Introduced or changed constraints

Python SHALL NOT parse Rust item/member/statement/match-arm/module boundaries.
The internal Rust classifier SHALL own syntax, cfg evaluation, spans, line
projection, and reachability under a bounded versioned protocol. Unknown
inclusion remains production. Mixed lines and production-reachable shared
modules remain production. Parser/tool upgrades require an explicit contract
and baseline migration with exhaustive delta evidence.

## Introduced or changed limitations

The helper classifies authored AST only; it does not expand declarative or
procedural macros. Span behavior is tied to locked `syn` 2.0.118 and
`proc-macro2` 1.0.106 with span locations. Fast validation incurs compilation
or cache lookup for one internal binary but does not invoke the heavyweight
metrics analyzer.

## Consumer and product impact

There is no SDK public, wire, persistence, runtime, platform, cryptographic,
identity, or downstream consumer behavior change. The added dependencies are
reachable only from the unpublished conformance tool.

## Activation and rollback

Activation requires the v2 baseline, migration report, archived OpenSpec,
protected PR review/CI, and merge into `develop`. Rollback restores v1 Python,
policy, report, and workflow atomically; no external artifact is affected.

## Evidence

Required evidence includes bounded-protocol tests, syntax/adversarial fixtures,
v1/v2 delta review, exact dependency and unsafe/native inventory, fast timing,
weekly regeneration, strict Clippy/tests, factory/OpenSpec, and compatible Nix.

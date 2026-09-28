# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The current implementation in `crates/derive/src/num.rs` and
`crates/derive/src/str.rs` independently emits the same scalar `Serialize`
implementation and the same two `Deserialize` shapes. The only branch is
whether the configured validation function is invoked after deserializing the
inner value. String-only parsing and numeric value semantics remain outside
that shared shape.

## Normative sources

The canonical `domain-newtype-macro` OpenSpec, ADR 0088's generated-output
safety decision, ADR 0093's borrowed-string allocation decision, ADR 0115's
code-health ratchet, current expansion/runtime tests, and the exact
`derive-serde-token-generation` baseline entry are authoritative.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One private scalar serde token generator | `adopt` | Owns the identical invariant once without adding a public or runtime boundary. | String and numeric wire or validation semantics intentionally diverge. |
| Generic runtime trait/helper | `not-adopt` | Adds consumer-visible coupling and runtime code for a compile-time template concern. | Several crates need an independently useful public abstraction. |
| Merge complete string/numeric expansion | `not-adopt` | Category-specific parse, conversion, and ownership behavior is cohesive and intentionally different. | The capability specification itself unifies those contracts. |
| Keep both copies | `not-adopt` | Preserves a known drift point already assigned a deduplication disposition. | The generated templates become semantically different. |

## Compatibility and dependency evidence

No dependency or feature changes are required. Generated public methods,
traits, serialized forms, validation calls, error mapping, spans, MSRV, and
target support remain unchanged. The facade remains the `Newtype` derive; the
new helper is crate-private and compile-time only.

## Security, privacy and maintenance evidence

The helper emits no new unsafe syntax and remains covered by the existing
syntax-aware output-safety validator. It handles no secrets or runtime data.
Single ownership reduces maintenance drift between string and numeric serde
paths without broadening the proc-macro dependency cone.

## Rejected or deferred candidates

A public abstraction, a runtime trait, bytes-category unification, and a full
derive rewrite are rejected for this slice. Attribute parser complexity is a
separate baseline hotspot and remains deferred to its owner.

## Open questions and blockers

There are no blockers. The review must confirm that the helper remains private
and that generated validated and unvalidated behavior is equivalent for both
scalar categories.

## Evidence commands

Planning inspected `num.rs`, `str.rs`, the derive tests, canonical OpenSpec,
ADRs 0088/0093/0115, and the code-health baseline. Unrun at planning time are
the focused derive tests, trybuild suite, syntax-aware code-health refresh,
strict Clippy, workspace tests, factory/Nix checks, and protected hosted CI.

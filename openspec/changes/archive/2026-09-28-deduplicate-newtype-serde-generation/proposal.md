# Deduplicate newtype serde generation

## Why

`identus-derive` emits identical `Serialize` implementations and structurally
parallel `Deserialize` implementations from both `num.rs` and `str.rs`. The
code-health baseline classifies this exact pair as
`derive-serde-token-generation` with disposition `deduplicate`. Keeping two
copies makes generated safety and serde behavior easier to change unevenly.

## What changes

- Move the scalar string/numeric serde token templates into one private,
  cohesive generator owned by `identus-derive`.
- Keep validated and unvalidated deserialization explicit inside that helper.
- Preserve category-specific accessors, constructors, parsing, validation,
  documentation, spans, public API, wire form, and diagnostics.
- Add positive compile/runtime evidence for validated and unvalidated string
  and numeric newtypes, then refresh syntax-aware code-health evidence.

## Capability

### Modified capability

- `domain-newtype-macro`: single-own common scalar serde generation while
  preserving the existing emitted contract.

## Non-goals

This change does not merge string and numeric validation policy, change bytes
encoding, expose a helper API, introduce a generic runtime abstraction, alter
dependencies, or redesign unrelated macro expansion.

## Delivery

Issue #402 owns this focused slice. Planning and ADR evidence precede the
implementation receipt. The implementation is reviewed through generated
behavior, trybuild/runtime evidence, code-health evidence, strict Rust gates,
and protected CI before integration into `develop`.

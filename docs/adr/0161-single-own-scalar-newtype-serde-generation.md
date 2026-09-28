# ADR 0161: single-own scalar newtype serde generation

- **Status:** Accepted for implementation
- **Date:** 2026-09-28
- **Decision authority:** issue
  [#402](https://github.com/hyperledger-identus/sdk-rust/issues/402)
- **Related:** ADR 0088, ADR 0093, ADR 0115, Discussion #399
- **Constraint impact:** routine; no effective constraint or limitation changes

## Context

`identus-derive` currently contains identical `Serialize` templates and
parallel validated/unvalidated `Deserialize` templates in both `num.rs` and
`str.rs`. The code-health baseline names the pair
`derive-serde-token-generation` and directs a focused deduplication slice.
String and numeric categories otherwise have intentionally different parsing,
conversion, ownership, and validation-entry semantics.

## Decision

Introduce one crate-private compile-time module that emits only the shared
scalar serde implementations. String and numeric category expanders append its
tokens when `serde` is selected. The helper reads the existing parsed context
and selects the validated or unvalidated deserialization template. It does not
own attribute parsing, category selection, constructors, conversions, string
parsing, errors, or bytes encoding.

Bytes serde stays in `bytes.rs`: it has an independent hex/base64url wire
contract and decode-before-validate invariant. The helper remains private and
does not add a runtime trait, public macro, dependency, feature, or facade.

Generated behavior is the compatibility boundary. Positive compile/runtime
tests cover validated and unvalidated string/numeric combinations, and the
existing syntax-aware output validator continues to reject prohibited emitted
constructs.

## Consequences

- Scalar serde templates have one owner and cannot drift by category.
- Category modules retain high cohesion around their distinct APIs.
- The proc-macro public surface, wire format, diagnostics, dependency cone,
  target support, and MSRV remain unchanged.
- A future intentional divergence must supersede this ADR rather than growing
  conditionals in both category modules.

## Alternatives rejected

- Keep both copies: retains the known drift point.
- Merge the complete string and numeric expanders: combines unrelated category
  policies and lowers cohesion.
- Add a public or runtime abstraction: exposes implementation mechanics and
  adds coupling without consumer value.
- Include bytes: obscures its distinct encoded-string wire invariant.

## Verification and rollback

Focused expansion/runtime and trybuild evidence, output-safety checks,
syntax-aware code-health refresh, strict Clippy, workspace tests, factory/Nix
checks, distinct review, and green protected CI are required. Reverting the
private helper and restoring the two prior token blocks is behavior-neutral
and requires no consumer migration.

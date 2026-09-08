## Context

`opaque_identifier!` generates six DID Registration identifier types. Each
borrowed `parse(&str)` currently calls `to_owned()` and delegates to
`try_new(String)`, so the shared validator sees the input only after allocation.
The validator already applies the correct byte ceiling before whitespace and
control-character traversal.

## Goals / Non-Goals

**Goals:**

- Enforce the existing byte ceiling before allocation on borrowed input.
- Keep all accepted values, owned construction, APIs, and diagnostics stable.
- Establish boundary and precedence evidence for every generated type.

**Non-Goals:**

- Prevent allocations made by transports before calling the SDK.
- Change identifier grammar, limits, storage, serialization, or lifecycle policy.
- Add a dependency or move method-, chain-, or product-specific behavior.

## Decisions

1. Generated `parse(&str)` calls `validate_identifier(value, limit)` directly,
   then allocates once on success. Calling `try_new(value.to_owned())` is rejected
   because it preserves the defect. A generalized constructor abstraction is
   rejected because one private macro and one validator already own the policy.

2. Keep `try_new(String)` unchanged. Its caller has already allocated the value,
   and moving accepted input avoids an additional copy.

3. Test exact and one-over boundaries for all six generated types, plus malformed
   and over-limit precedence and redacted diagnostics. Allocation ordering is
   established by the direct source path and distinct review; a custom global
   allocator would require unsafe test machinery disproportionate to this change.

4. Add no crate. This is call ordering around existing SDK limits, not reusable
   parsing or protocol logic; a dependency would not replace the decision.

## Risks / Trade-offs

- [A later macro edit restores clone-before-validation] → Keep the normative
  requirement and an explicit source-focused review checkpoint.
- [Outer parsing allocates first] → Retain `SDK-LIM-007` and its consumer limit.
- [Test repetition across six types] → Prefer explicit calls so coverage remains
  visible and a newly generated type requires a deliberate test update.

## Migration Plan

Land specification and ADR before Rust implementation, run focused and full
gates, then merge the issue-linked PR into `develop`. Reverting that PR restores
the former ordering; no source, wire, or stored-data migration exists.

## Open Questions

None. Remaining inherited boundaries stay tracked by #168.

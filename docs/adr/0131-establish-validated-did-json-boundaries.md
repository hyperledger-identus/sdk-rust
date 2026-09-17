# ADR 0131: establish validated DID JSON boundaries

- **Status:** Accepted for implementation
- **Date:** 2026-09-17
- **Decision authority:** issue #315 decision comment and the
  `establish-validated-did-json-boundaries` OpenSpec change
- **Related:** issue #297; ADR 0125; `SDK-SEC-003`; `SDK-LIM-007`

## Context

`OneOrMany<T>` preserves scalar-versus-array wire shape but also applied the
DID-specific 128-item ceiling. `ContextEntry::Object` directly accepted an
arbitrary `BTreeMap<String, serde_json::Value>` despite the domain's depth and
node limits. An oversized generic context array could therefore reject before
domain validation and recursively destroy a 32,768-level value. Constructor-
specific guards alone cannot intercept that earlier generic failure.

The affected crates are unpublished at version `0.0.0`. Current named-consumer
searches found no direct construction of the raw sdk-rust context-object
variant, so an explicit pre-release source migration is preferable to custom
destruction semantics on every context value.

## Decision

1. Treat `OneOrMany<T>` only as a non-empty scalar-or-array representation.
   Enforce `MAX_DOCUMENT_ITEMS` at validated DID domain owners.
2. Replace the raw context-object payload with opaque `ContextObject`. Expose
   fallible construction plus borrowed/consuming accessors, transparent serde,
   and redacted diagnostics; expose no unchecked or mutable raw-map path.
3. Require every successful public context object to satisfy the existing DID
   JSON depth, node, property, collection, name, and string budgets.
4. Arm one crate-private ownership guard before validation of raw recursive
   JSON. On error, one iterative worklist dismantles all arrays/objects; on
   success, the exact allocation enters the validated domain value.
5. Reuse that mechanism across the complete native DID document, resolution/
   dereferencing, query, and registration rejection family from #297.
6. Do not implement public custom `Drop`, unsafe code, intentional leaks, stack
   enlargement, or a new dependency.
7. Preserve accepted wire representation, high-level semantics, limits, and
   static error taxonomy. Record the source migration honestly.

## Consumer migration

Replace direct raw construction:

```rust
let object = ContextObject::new(map)?;
let entry = ContextEntry::Object(object);
```

Use `as_map` for borrowed inspection and `into_map` when ownership is required.
Do not rely on `OneOrMany::try_many` for DID cardinality validation; construct
the enclosing `Service` or `DidDocument` to apply domain policy.

## Consequences

- No public validated DID context entry can contain JSON beyond the documented
  resource limits.
- Generic cardinality representation no longer needs type-specific destruction
  behavior.
- Rejection after typed ownership is stack-safe across the audited DID family;
  allocation before typed entry remains caller/transport owned.
- Direct raw context-object construction and generic limit-check assumptions
  are intentional pre-release source changes. Accepted JSON is unchanged.

## Rejected alternatives

- Public custom `Drop` changes ordinary field-move semantics and treats the
  symptom in every valid object.
- Returning rejected ownership from the generic constructor is sound but does
  not make invalid context states unrepresentable and is unnecessary after
  policy separation.
- Constructor-only guards leave the generic early-rejection path open.
- `serde_stacker`, `stacker`, larger stacks, unsafe traversal, or leaks do not
  establish the invariant and conflict with dependency/safety policy.

## Rollback

Restore the raw context variant, generic collection ceiling, prior constructors,
and native cleanup limitation together. No accepted wire or stored data needs
migration.

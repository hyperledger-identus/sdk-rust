# ADR 0201: compose PRISM only at the WASM target boundary

- **Status:** Accepted for issue #610 implementation
- **Date:** 2026-11-20
- **Decision authority:** issue
  [#610](https://github.com/hyperledger-identus/sdk-rust/issues/610) and its
  material OpenSpec constraint review
- **Related:** ADRs 0163, 0174, 0176, and 0199; issues #492 and #594
- **Constraint impact:** material; adds one closed outer-boundary dependency
  exception without permitting a generic domain, native binding, or product
  dependency on PRISM

## Context

ADR 0174 prohibits generic SDK crates from depending on the portable PRISM
family. That rule keeps method policy out of generic DID, crypto, credential,
protocol, and orchestration crates. ADR 0199 separately selected
`identus-wasm-did` as the private generated assembly boundary for an
unpublished TypeScript DID facade, but its first implementation composed only
generic DID and peer-DID operations.

Issue #610 adds a PRISM-specific TypeScript capability backed by the canonical
`identus-prism-did` engine. Keeping the semantics in Rust requires the final
WASM artifact to link that method engine. Moving PRISM semantics into
TypeScript would duplicate validation and state reduction; moving generated
bindings into `identus-prism-did` would couple the portable method crate to one
language toolchain. A second WASM module would also split generated ownership,
initialization, disposal, and artifact verification without creating a new
security boundary.

`identus-wasm-did` is an outer target adapter and artifact assembler, not a
generic domain crate. Its generated symbols are private and are wrapped by the
strict capability-named TypeScript facade.

## Decision

Permit exactly this additional workspace edge:

```text
identus-wasm-did -> identus-prism-did
```

The exception applies only to the exact in-tree packages. It does not permit:

- `identus-did`, `identus-crypto`, credentials, protocols, orchestration, or
  another generic production crate to depend on a PRISM crate;
- `identus-uniffi-did` or another native/language adapter to infer equivalent
  support;
- `identus-wasm-did` to depend on `identus-prism-vdr`,
  `identus-prism-indexer`, Cardano, storage, transport, or product code;
- generated WASM owners, chain effects, private key material, or historical
  SDK component names to cross the public TypeScript facade.

`identus-prism-did` remains the sole owner of parsing, operation encoding,
signature verification, authenticated atomic history reduction, tombstones,
and DID Document projection. The target adapter owns only bounded generated
conversion and ownership. TypeScript owns structural validation and an
injected exact-history effect.

The dependency guard records this as a closed source-target pair and keeps all
other PRISM-family edges denied.

## Consequences

Browser, Node.js, and React-web can share one deterministic private artifact
and one initialization/disposal boundary. The core method remains portable and
binding-framework-free. The WASM artifact grows and becomes method-aware, so
artifact hashes, symbols, size deltas, target isolation, and rollback evidence
remain mandatory.

This decision does not publish or stabilize the package, activate SDK-TS,
certify a runtime, or authorize React Native. A future native or language
binding needs its own reviewed edge and target evidence.

## Alternatives rejected

- Implement PRISM rules in TypeScript: duplicates canonical security logic.
- Put `wasm-bindgen` in `identus-prism-did`: contaminates the portable method
  crate with one target framework.
- Ship a second generated module: duplicates lifecycle and artifact ownership
  for no independent trust boundary.
- Broaden the PRISM-family allowlist: permits unrelated method, VDR, indexer,
  or chain coupling.

## Verification and rollback

Conformance tests accept the one exact WASM-to-method edge, reject the same
edge from the native adapter and generic crates, and continue validating every
real manifest. Target tests prove generated ownership, stable errors,
cross-runtime behavior, deterministic assembly, and fail-closed routing.

Rollback removes the private PRISM facade, generated PRISM symbols, the
`identus-wasm-did` dependency, and this exact guard pair as one unreleased
unit. Canonical `identus-prism-did` semantics and all generic DID APIs remain.

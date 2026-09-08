# ADR 0088: reject unsafe first-party procedural-macro output

- **Status:** Accepted
- **Date:** 2026-09-08
- **Decision authority:** sdk-rust issue #189
- **Constraint impact:** strengthens `SDK-SEC-001` for direct first-party
  procedural-macro output and narrows `SDK-LIM-008`

## Context

ADR 0087 makes `unsafe_code = "forbid"` effective for authored first-party
targets. Rust 1.98.1 intentionally skips the lint for expansion spans where
`span.allows_unsafe()` is true, so a safe procedural macro can otherwise emit
unsafe syntax into an opted-in consumer without a diagnostic.

`identus-derive` is the only first-party procedural-macro crate. Its
`#[identus::port]` attribute returns parsed caller input unchanged. Its
`#[derive(Newtype)]` implementation emits finite item templates for string,
byte and numeric newtypes. Those templates currently contain no unsafe syntax.

A stable experiment initially suggested applying the caller identifier span to
every generated token. The actual derive suite rejected that approach: Rust
also attributed unrelated generated-code warnings to the caller, causing 21
errors across existing valid derives. Blanket caller-spanning is therefore not
compatible, and broad generated `allow` attributes would hide rather than
solve the regression.

## Decision

1. Parse each completed direct `Newtype` expansion as Rust items before it
   leaves `identus-derive` and visit the syntax tree for constructs covered by
   the pinned `unsafe_code` lint: unsafe blocks, functions, traits, trait
   implementations and extern blocks; `global_asm!`; and
   `allow_internal_unsafe`.
2. Also reject the Rust Reference unsafe attributes `export_name`,
   `link_section`, `naked` and `no_mangle`, including legacy unwrapped forms.
3. Return a compile error at the rejected construct instead of emitting the
   expansion. The check runs after helper token streams are composed and
   recursively visits nested item/expression syntax.
4. Existing runtime and trybuild suites remain the semantic and diagnostic
   regression evidence for every supported `Newtype` shape. Unit tests cover
   each forbidden construct, nesting and safe output.
5. Reuse `syn` with its stable `visit` feature. Add no package, expanded-source
   snapshot, nightly lane, scanner executable or `RUSTC_BOOTSTRAP` escape.

## Assurance boundary

This decision covers direct Rust syntax emitted as items by first-party derive
expansion. It does not execute or inspect syntax created later by nested macro
expansion and does not certify external procedural macros, dependency
internals, logical safety or side-channel behavior. Those residual cases stay
disclosed by `SDK-LIM-008` and dependency review.

## Consequences

- `identus-derive` fails closed before returning prohibited direct syntax, even
  when rustc would suppress the downstream expansion lint.
- Existing generated-code spans and downstream warning behavior remain
  compatible.
- The `syn` version and resolved package cone are unchanged; enabling `visit`
  adds a small build-time feature surface.
- A future Rust etalon or new generated construct triggers review against the
  compiler and Reference inventories.

## Rollback

Revert the issue #189 change, remove the syntax visitor and broaden
`SDK-LIM-008` to all procedural-macro output. No public API, wire format,
runtime data or downstream migration is involved.

# Exact-diff local review

Review status: completed
Review date: 2026-09-08
Implementation head: 2af77774aa34532bf1d5f0f8e5517953e28a6dba
Specification parent: c0591ea
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `origin/develop...HEAD` diff, the exact Rust
1.98.1 unsafe-lint cases, the Rust Reference unsafe-attribute inventory, every
generated `Newtype` output family, the selected `syn` feature graph, focused
and complete gate output, canonical constraints, and the residual assurance
boundary.

## Findings

1. **Boundary placement — accepted.** Validation runs once after helper output
   is composed and before the direct item stream leaves `identus-derive`. No
   public probe macro or duplicated consumer implementation is introduced.
2. **Unsafe construct inventory — accepted.** The visitor rejects unsafe
   blocks, signatures, traits, implementations and extern blocks;
   `global_asm!`; and `allow_internal_unsafe`, matching the pinned compiler
   cases used by this decision.
3. **Unsafe attributes — accepted after correction.** Direct wrapped and
   legacy forms of `export_name`, `link_section`, `naked` and `no_mangle` are
   rejected. Exact-diff review found that default `syn` attribute traversal
   leaves `cfg_attr` arguments opaque; the implementation now parses attribute
   branches recursively, rejects latent unsafe attributes independent of the
   active configuration, and skips the condition to avoid name-based false
   positives.
4. **Compatibility — accepted.** The rejected caller-span design caused 21
   warnings-as-errors across valid derives and was fully reverted. The selected
   syntax guard leaves token spans unchanged; all 14 runtime and 10 trybuild
   cases pass.
5. **Parsing behavior — accepted.** `Newtype` is an item-producing derive, so
   parsing completed output as `syn::File` matches its contract. Parse failure
   returns a compile error. `cfg_attr` grammar is parsed only to inspect its
   attribute branches.
6. **Dependency cohesion — accepted.** No package or lockfile entry changes.
   The existing parser gains one stable, compile-time-only feature and remains
   private to the procedural-macro implementation.
7. **Security and privacy — accepted.** No runtime code, input data, secret,
   diagnostic value, native boundary or unsafe exception is added. Rejection
   diagnostics name only the unsafe syntax category.
8. **Governance accuracy — accepted.** `SDK-SEC-001` states the direct
   first-party guard. `SDK-LIM-008` still discloses external procedural macros
   and syntax created later by nested macro expansion instead of overstating
   compiler coverage.

## Residual limitations

- External procedural macros and output created later by nested macro
  expansion remain outside the direct syntax visitor.
- The guard proves absence of the pinned syntax inventory, not logical safety,
  side-channel resistance or dependency internals.
- A new Rust etalon, unsafe attribute/lint case or first-party generator is an
  explicit review trigger.
- Local Nix omitted x86_64-linux as host-incompatible; hosted `fast` remains
  merge authority for Linux.

## Review decision

The implementation is focused, reversible and compatible. The conditional
attribute gap found during the distinct review is corrected and covered by
machine tests. No unresolved correctness, architecture, security, privacy,
compatibility or supply-chain blocker remains for delivery.

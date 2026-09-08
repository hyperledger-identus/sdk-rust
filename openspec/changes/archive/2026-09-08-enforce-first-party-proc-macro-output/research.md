# First-party procedural-macro output research

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation from ADR 0087 and issue #169 proves inherited
`unsafe_code = "forbid"` reaches authored first-party targets. A safe
procedural macro can still emit unsafe tokens which compile when rustc treats
their spans as allowing internal unsafe. `identus-derive` is the sole
first-party proc-macro crate: `port` returns caller input unchanged and
`Newtype` emits finite direct item templates from three modules.

## Normative sources

- Exact Rust 1.98.1 `UnsafeCode` implementation, revision
  `48a229ceaefd4985c50990b14116b6d856af0985`:
  https://github.com/rust-lang/rust/blob/48a229ceaefd4985c50990b14116b6d856af0985/compiler/rustc_lint/src/builtin.rs
- Rust Reference unsafe attribute inventory:
  https://doc.rust-lang.org/reference/attributes.html#unsafe-attributes
- Stable `syn::visit` API:
  https://docs.rs/syn/2/syn/visit/index.html
- Stable procedural-macro Span API:
  https://doc.rust-lang.org/stable/proc_macro/struct.Span.html
- Historical external-expansion decision:
  https://github.com/rust-lang/rust/issues/53975

The exact compiler source defines the lint surface and expansion exception;
the Reference defines unsafe attributes; `syn` is the selected stable parser.

## Candidate decisions

1. `adopt`: parse completed generated items and use `syn::visit` to reject the
   pinned unsafe constructs and attributes before output.
2. `not-adopt`: apply caller spans to all direct tokens. A disposable compiler
   probe rejected unsafe output, but the production derive suite then failed
   with 21 unrelated generated-warning errors.
3. `not-adopt`: add blanket generated `allow` attributes around caller-spanned
   code. This changes downstream lint behavior and masks compatibility issues.
4. `not-adopt`: recursively scan only token identifiers. It is either
   incomplete (`global_asm!` and unsafe attributes) or falsely rejects caller
   identifiers without syntax context.
5. `not-adopt`: render and lexically scan expanded source, use cargo-expand or
   `-Zunpretty`. Rendering is not semantic proof and nightly adds an unpromised
   tool/CI cone.
6. `retain-local`: keep dependency and review evidence for opaque external and
   nested macro expansion.

## Compatibility and dependency evidence

Exact version/MSRV stays Rust/Cargo 1.98.1. Existing `syn 2.0.118` remains
pinned by Cargo.lock and gains only its stable `visit` feature; `quote` and
`proc-macro2` features are unchanged. The direct and resolved dependency cone,
package count, license and source provenance stay unchanged. No executable or
artifact provenance is added; supply-chain exposure is limited to existing
locked code already built in the proc-macro cone.

The public API, wire representation, runtime behavior and facade boundary do
not change. Default generated spans remain intact. Existing runtime/trybuild
tests cover finite macro shapes. Consumer evidence is therefore unchanged API
compilation plus the existing generated-behavior suite. This is not a protocol or draft decision;
protocol and draft currency are unaffected. Supported targets still compile
the same runtime output.

## Security, privacy and maintenance evidence

Exact Rust source shows `UnsafeCode` visits unsafe blocks, functions, traits,
trait implementations and extern blocks; `global_asm!`; and
`allow_internal_unsafe`, returning early when a span allows unsafe. The Rust
Reference lists four unsafe attributes: `export_name`, `link_section`, `naked`
and `no_mangle`. The selected structured visitor rejects both inventories,
including nested direct syntax, legacy unwrapped unsafe attributes and latent
unsafe attributes in nested `cfg_attr` branches.

The failed caller-span production experiment is retained as negative decision
evidence: 21 errors arose from generated inherent-impl attributes and unused
methods, and disappeared after complete code reversion. The syntax visitor
does not alter spans. Unit tests will cover every category plus safe controls;
the existing 14 runtime and 10 trybuild cases are the compatibility baseline.

No runtime unsafe/native code, FFI, secret, persisted data or privacy boundary
is introduced. Maintenance, release and security posture improve through a
small private check using the existing parser instead of a separate scanner.

## Rejected or deferred candidates

Blanket caller-spanning, generated lint allowances, raw token-name scans and
expanded-source/nightly tools are rejected as above. External macros,
dependency internals, syntax created by later nested expansion and unsupported
build combinations remain deferred from the assurance claim.

## Open questions and blockers

No blocker remains. Stop and retain the broader limitation if syntax parsing
changes current expansion behavior, a finite derive shape fails, or complete
gates disagree. The reconsideration trigger is a Rust etalon change to the
unsafe lint/attribute inventory, a new first-party proc macro, or a generator
that emits non-item output. Rollback removes the visitor and restores the
broader limitation without runtime or data migration.

## Evidence commands

Research and implementation commands on exact Rust/Cargo 1.98.1 include:

```text
cargo test -p identus-derive
cargo test -p identus-conformance unsafe_policy
cargo clippy --workspace --all-targets --all-features -- -D warnings
nix flake check --print-build-logs
```

The derive command passed after full reversion of the caller-span experiment;
the other repository commands are pending immutable implementation. Miri,
sanitizers, dependency audit, Windows and nightly are intentionally unrun
checks because there is no runtime unsafe, package/target change or nightly
promise. Hosted Linux remains merge authority for its platform.

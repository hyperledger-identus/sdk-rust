# First-party procedural-macro span research

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation from ADR 0087 and issue #169 proves the inherited
`unsafe_code = "forbid"` reaches
authored first-party target classes. A separate fixture showed that a safe
procedural macro could return parsed unsafe tokens which compiled in the
consumer. `identus-derive` is the sole first-party procedural-macro crate. Its
`port` attribute returns parsed caller input unchanged, while `Newtype` emits
finite direct templates from `bytes.rs`, `str.rs` and `num.rs`.

## Normative sources

- Rust 1.98.1 `UnsafeCode::report_unsafe` implementation at exact release
  revision `48a229ceaefd4985c50990b14116b6d856af0985`:
  https://github.com/rust-lang/rust/blob/48a229ceaefd4985c50990b14116b6d856af0985/compiler/rustc_lint/src/builtin.rs
- Stable `proc_macro::Span` API:
  https://doc.rust-lang.org/stable/proc_macro/struct.Span.html
- `quote_spanned!` behavior:
  https://docs.rs/quote/latest/quote/macro.quote_spanned.html
- Historical rustc external-expansion decision:
  https://github.com/rust-lang/rust/issues/53975
- rustc command-line stability, including nightly-only `-Z` options:
  https://doc.rust-lang.org/rustc/command-line-arguments.html

Rust source is the behavior authority, `proc_macro` and `quote` document the
stable implementation surface, and repository policy defines the assurance
scope.

## Candidate decisions

1. `adopt`: use `quote_spanned!` with the derived identifier span. It reuses an
   existing dependency, is stable on Rust 1.98.1, preserves interpolated token
   spans and makes rustc the enforcement mechanism.
2. `not-adopt`: recursively rewrite every final token span. The probe works,
   but broad re-spanning degrades hygiene and diagnostics and can overwrite
   meaningful interpolated spans.
3. `not-adopt`: lexically scan rendered expansion output. Rendering is not a
   semantic Rust parser and cannot prove arbitrary macro behavior.
4. `not-adopt`: use `cargo-expand` or direct `-Zunpretty=expanded`. The
   mechanism uses nightly compiler options, expanded output is
   debugging-oriented and lossy, and it would add a tool/version/CI cone.
5. `not-adopt`: use `cargo-geiger`. Its expansion coverage is incomplete and
   it would add a scanner cone without matching compiler enforcement.
6. `retain-local`: keep review-only evidence for external and nested expansion
   output that first-party templates cannot govern.

## Compatibility and dependency evidence

The exact version is Rust/Cargo 1.98.1 and the `quote` feature set is unchanged.
The change reuses workspace `quote` and `proc-macro2`; the direct and resolved
dependency cone and Cargo lock remain unchanged. Existing crate license and
provenance evidence therefore remains authoritative, with no new artifact or
source provenance. The MSRV and supported target matrix do not change.

The public API, wire representation, runtime behavior and facade boundary do
not change. Existing unit/runtime and trybuild suites exercise the finite macro
shapes and detect semantic or diagnostic regressions. This is not a protocol or
draft-version decision; protocol and draft currency are not affected.

## Security, privacy and maintenance evidence

A disposable two-crate workspace used exact Rust/Cargo 1.98.1, inherited
workspace `forbid` and a safe procedural macro that emitted the same unsafe
block with different spans.

| Generated-token strategy | Consumer result |
| --- | --- |
| parsed/default spans | compiled |
| recursively applied `Span::call_site()` | compiled |
| recursively applied `Span::mixed_site()` | compiled |
| caller-input span on only the `unsafe` identifier | compiled |
| caller-input span on generated syntax/groups | rejected by `unsafe_code` |
| `quote_spanned!` with caller-input span | rejected by `unsafe_code` |

The distinction matches rustc's early return when `span.allows_unsafe()`.
Applying only a keyword span is not a sufficient control. Applying the caller
span to directly generated syntax is effective on the pinned stable etalon.

The new conformance fixture is dependency-free, offline, process-isolated and
requires both a failed compilation and the unsafe-code lint identifier. It
does not compile unsafe code into an SDK artifact. It proves the pinned
compiler premise, not arbitrary external macro safety. No secrets, FFI,
native code, persisted data or privacy boundary is introduced.

Supply-chain exposure is unchanged because no package or executable is added.
Maintenance, release and security posture improve through a small stable
compiler-native control instead of a separately versioned scanner.

## Rejected or deferred candidates

Recursive re-spanning, lexical scanning, cargo-expand/nightly and cargo-geiger
are rejected for the reasons in the candidate matrix. External macros, nested
expansions, dependencies, unsupported feature/target combinations and
arbitrary token transformations remain deferred from the assurance claim.

## Open questions and blockers

No research blocker remains. Stop and retain the broader limitation if
caller-spanning changes macro semantics or hygiene, causes incompatible
diagnostics, fails an existing shape, or cannot be reproduced by the committed
fixture and complete gates. The reconsideration trigger is a Rust etalon span
behavior change, a new first-party procedural macro, or a template mechanism
that cannot preserve caller-origin spans. Rollback restores default quote
spans and the broader limitation without runtime or data migration.

## Evidence commands

Research commands used exact Rust/Cargo 1.98.1 in a disposable workspace:

```text
cargo check --offline -p consumer
cargo test -p identus-derive
cargo test -p identus-conformance unsafe_policy
```

The first command was repeated for parsed/default, call-site, mixed-site,
keyword-only and caller-spanned generated tokens. The repository test commands
are implementation evidence and have not yet run for this branch. Full Nix and
hosted Linux execution are reserved for the immutable implementation. Miri,
sanitizers, dependency audit, Windows and a nightly lane are intentionally
unrun checks because there is no runtime unsafe, dependency/target change or
nightly promise.

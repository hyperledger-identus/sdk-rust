# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-16
Source retrieval date: 2026-09-16
Research blockers: none

## Problem and existing implementation

The current implementation in `scripts/code-health-audit.py` sanitizes Rust
text and maintains a conservative Python grammar for cfg attributes, item ends,
macros, line projection, and module reachability. ADR 0115 intentionally
deferred full syntax to issue #275. The consumer is repository code-health
evidence used by fast source checks and weekly full regeneration; no SDK crate
or downstream product consumes these parser functions.

## Normative sources

- Repository revision `5dff6f38c861b858dd62dc8310246f7d485d3e92`, ADR
  0115, canonical `code-health-governance`, and issue #275 define population
  and delivery authority.
- Locked Cargo evidence pins `syn` version 2.0.118, `proc-macro2` version
  1.0.106, and `quote` version 1.0.46. Primary API references are
  https://docs.rs/syn/2.0.118/syn/ and
  https://docs.rs/proc-macro2/1.0.106/proc_macro2/struct.Span.html.
- The Rust Reference conditional-compilation and module source rules remain the
  protocol/draft currency source:
  https://doc.rust-lang.org/reference/conditional-compilation.html and
  https://doc.rust-lang.org/reference/items/modules.html.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Workspace `syn` classifier | 2.0.118 at Cargo.lock revision | `adopt` | Existing pinned full AST dependency, portable Rust, and cohesive verification boundary. | Locked parser cannot represent a stable Rust 1.98.1 construct needed by policy. |
| `proc-macro2` span locations | 1.0.106 | `conditional-adopt` | Supplies byte/line span evidence without rustc-private APIs; tooling-only feature. | Span instability produces unexplained baseline deltas. |
| Python v1 classifier | issue #270 baseline | `retain-local` until migration evidence, then remove | Provides a differential oracle for the one-time population comparison. | Retain only if v2 cannot explain every production decrease. |
| tree-sitter-rust | current external project, not locked | `not-adopt` | Adds a second grammar/dependency and native build considerations without need. | `syn` cannot meet incremental or error-recovery requirements. |
| rustc private APIs | Rust 1.98.1 internals | `not-adopt` | Nightly/compiler coupling and maintenance cone are disproportionate. | Repository later standardizes a compiler-plugin toolchain. |

## Compatibility and dependency evidence

The public and wire compatibility impact is none because the binary stays in
unpublished `identus-conformance`; parser types do not cross an SDK facade.
Locked metadata reports `syn` and `quote` MSRV 1.71 and `proc-macro2` MSRV 1.68,
below repository Rust 1.98.1. All are MIT OR Apache-2.0 with crates.io/Cargo.lock
provenance. The direct dependency cone is `syn`, `proc-macro2`, serde, and
`serde_json`; the resolved dependency cone already exists in Cargo.lock and is
verified by workspace build/audit gates. Targets are host Linux x86_64 and
macOS ARM64 tooling; no WASM/iOS/Android runtime package depends on the helper.
Features are `syn/full`, parser/visit support, and
`proc-macro2/span-locations`. Rollback restores the v1 facade atomically.

## Security, privacy and maintenance evidence

The bounded stdin/file/span/module protocol limits memory retained from audit
inputs and emits paths/locations rather than source or secrets. The helper adds
no authored unsafe, native code, networking, credential material, telemetry, or
external write. The resolved crates contain their established proc-macro
implementation paths but introduce no new native dependency. Existing Cargo
deny/audit and Nix supply-chain gates remain authoritative. Maintenance,
release, and security posture are tied to Cargo.lock and explicit baseline
migrations; the helper is never published. Reconsideration trigger: a parser
advisory, license/MSRV change, unsupported stable syntax, or unexplainable span
delta.

## Rejected or deferred candidates

Extending Python is rejected because it becomes a second Rust parser. Macro
expansion is deferred because authored population evidence must not execute or
trust generated syntax. Incremental parsing and IDE-oriented recovery are
deferred because audits run on committed, rustc-valid source and should fail
actionably on parse errors.

## Open questions and blockers

No blockers remain before implementation. The exact v1/v2 numeric delta cannot
be known until the helper exists; it is a required migration artifact, not an
assumption. Any unexplained production decrease returns research to draft.

## Evidence commands

Commands run: `cargo metadata --locked --format-version 1`, `cargo tree
--locked -p identus-conformance -e normal,dev`, repository searches over the v1
scanner/tests/workflows/Nix, and `git rev-parse HEAD`. `cargo deny` was unrun
outside the Nix shell because the ambient command is absent; the pinned Nix
deny/audit gates are required before delivery. Network documentation retrieval
returned no captured body, so locked local crate metadata and repository
evidence are authoritative for the pre-implementation decision.

# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

Manual slow run `34972066673` targets merged protected `develop` revision
`59a8db6b7c34439f4f84d762bc1fd56c6ecba9d0`. Job `104390541635` failed in
`Prepare isolated candidate evidence` after the parser reported:
`toolchain 'nightly-x86_64-unknown-linux-gnu' is not installed`. The preceding
FlakeHub authentication annotation was non-causal; other Nix-backed jobs passed
with the same unauthenticated cache condition.

The runner already constructs `RUSTC_BOOTSTRAP=1` for its API subprocess, but
invokes `cargo public-api` without a JSON input. Thus the environment permits
stable rustdoc JSON while the parser still independently chooses rustup nightly.
This current implementation passed on a developer host only because ambient
rustup state concealed the undeclared dependency.

## Normative sources

- Repository ADR 0113 and the current `unpublished-crypto-candidate`
  specification require Rust 1.98.1, locked release tools and a narrowly scoped
  bootstrap escape hatch.
- The exact nixpkgs source package for `cargo-public-api 0.52.0` records that
  rustdoc JSON requires recent nightly semantics and supports JSON as an input.
- Its `resolve_toolchain` implementation detects a stable Cargo and assigns
  toolchain `nightly`; its builder passes a selected toolchain to the
  `rustdoc-json` crate. `RUSTC_BOOTSTRAP` is not part of this selection.
- Cargo/rustdoc 1.98.1 are already the only primary compiler tools in the
  candidate Nix app. Cargo's `rustdoc` command accepts arguments after `--`, so
  the existing scoped bootstrap can produce the JSON explicitly.

Upstream source: https://github.com/cargo-public-api/cargo-public-api/tree/v0.52.0

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Stable `cargo rustdoc` then parse local JSON | `adopt` | Matches ADR 0113, avoids rustup discovery, adds no compiler or dependency, and creates a testable boundary. | Rust 1.98.1 cannot produce parser-compatible JSON or stable rustdoc JSON becomes available. |
| Add rustup nightly to CI | `not-adopt` | Adds mutable toolchain state/network and violates Nix ownership. | Repository replaces Nix toolchain authority through a separate ADR. |
| Reuse pinned sanitizer nightly | `not-adopt` | Changes candidate compiler evidence and broadens the sanitizer-only exception. | Candidate policy explicitly adopts a reviewed nightly compiler. |
| Make all candidate commands nightly | `not-adopt` | Weakens the Rust 1.98.1 preparation claim and increases coupling. | Release compiler matrix selects nightly, which is currently prohibited. |
| Remove API rendering | `not-adopt` | Loses accepted package/API drift evidence. | Another exact semantic API checker replaces it. |

## Compatibility and dependency evidence

No Cargo manifest, lockfile, crate feature, runtime dependency or public API
changes. Exact version and feature evidence remains `cargo-public-api 0.52.0`,
the all-feature crypto surface, and primary Rust 1.98.1. The direct and resolved
dependency cone is unchanged because no package or lock input changes. WASM,
Android, iOS, MSRV and NeoPRISM target evidence is unaffected because this is
host-only candidate evidence.

Consumer evidence is unchanged: the output is an unpublished review artifact,
not a dependency consumed by downstream applications. Public and wire
compatibility are unchanged because only the build path to the same committed
API rendering changes. The `identus-crypto` facade boundary remains intact;
third-party parser and rustdoc types never enter SDK APIs.

## Security, privacy and maintenance evidence

No registry/network credential, ambient rustup state, native code or unsafe
Rust is introduced. A dedicated target directory remains under temporary
candidate scratch and is removed on completion/failure. The parser receives one
repository-generated path rather than caller content. Existing API output and
receipt redaction semantics remain unchanged.

The maintenance burden is one explicit Cargo command plus structural tests,
which is smaller than provisioning and auditing a second compiler manager.
Supply-chain evidence remains Nix input lock -> exact Cargo/rustdoc/parser.
License and provenance remain the repository Apache-2.0 source plus the already
reviewed upstream parser package; no source is copied into production code.
Maintenance, release and security posture remains pre-release and unchanged.

## Rejected or deferred candidates

Rustup provisioning, a second Nix compiler, sanitizer-nightly reuse, parser
removal and nightly-wide candidate compilation are rejected by the table above.
Protocol or draft currency is not applicable to this host tooling correction;
no SSI message, cryptographic suite or wire format changes. Rollback is a
repository revert and a return to known-red candidate evidence while a new
tooling decision is researched.

## Open questions and blockers

No planning blocker remains. End-to-end execution must prove Rust 1.98.1 emits
JSON accepted by parser 0.52.0 when the runner cannot discover a rustup nightly.
If that fails, implementation stops and this decision is reconsidered rather
than adding an implicit compiler.

## Evidence commands

- Strict OpenSpec/research/constraint readiness and planning receipt.
- Mutation tests for explicit JSON generation, expected artifact, parser input
  and bootstrap scope.
- End-to-end `nix run .#crypto-candidate` with ambient rustup state hidden.
- Factory and proportional Nix checks, exact-head protected PR CI, then a full
  exact-merged-head manual slow run.

Exact commands and outcomes will be recorded in `verification.md`. Unrun checks,
including the not-yet-due natural weekly schedule and unrelated runtime target
matrices, will be named explicitly rather than implied.

## Reconsideration triggers

- Stable rustdoc JSON becomes available without the bootstrap escape hatch.
- Rust 1.98.1 or parser 0.52.0 no longer reproduces the committed rendering.
- Release compiler policy changes through a separate ADR.

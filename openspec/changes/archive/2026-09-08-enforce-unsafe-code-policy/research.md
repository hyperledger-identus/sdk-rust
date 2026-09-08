# Unsafe-code enforcement research

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

`SDK-SEC-001` is effective policy and `SDK-LIM-008` discloses incomplete
machine enforcement. The current implementation has all 17 workspace manifests
declare `[lints] workspace = true`, but the root only has
`warnings = "deny"`. Six library roots contain `#![forbid(unsafe_code)]`; the
other 11 do not. A repository source inventory found no first-party unsafe
block. The consumer outcome is uniform evidence for the existing policy, not a
new safety or compatibility promise.

## Normative sources

- Cargo manifest lints and workspace inheritance:
  https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section
- Cargo workspace lint table:
  https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table
- rustc lint levels and non-overridable `forbid`:
  https://doc.rust-lang.org/rustc/lints/levels.html#forbid
- rustc `unsafe_code` lint surface:
  https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unsafe-code
- Cargo check target selection:
  https://doc.rust-lang.org/cargo/commands/cargo-check.html#target-selection

These Rust 1.98.1 Cargo/rustc documents are the normative mechanism authority.
The repository policy and constraints define scope. This is not a protocol or
draft-version decision; NeoPRISM and other donors are not authorities for this
workspace-specific compiler enforcement.

## Candidate decisions

1. Duplicating crate-root attributes is `not-adopt`: separate package targets
   remain outside those crate roots and future copies can drift.
2. Workspace `deny` is `not-adopt`: nested source can locally lower it to
   `allow` without changing the manifest policy edge.
3. CI-only `RUSTFLAGS=-Funsafe-code` is `not-adopt`: ordinary local Cargo would
   differ from CI and workflow configuration would hide the policy.
4. Inherited workspace `forbid` plus member/configuration guards and behavioral
   probes is `conditional-adopt`: it is the narrowest stable,
   local/CI-consistent, fail-closed mechanism for authored first-party source,
   but it does not cover every procedural-macro expansion.

No third-party crate is a useful candidate. Cargo/rustc already owns compiler
lint propagation and macro expansion; another scanner would add a dependency
without providing equivalent compilation evidence.

## Compatibility and dependency evidence

Exact version/MSRV is the repository's Rust and Cargo 1.98.1 etalon. No new
feature, target, package, normal or development dependency is introduced, so
the direct and resolved dependency cone is unchanged. There is no new crate
license or artifact provenance; pinned Nix inputs and the repository revision
remain the tool provenance. The public API, wire behavior, error surface,
facade boundary, persisted data and downstream repositories are unchanged.

`cargo check --all-targets` covers libraries, binaries, tests, examples and
benchmarks; build scripts compile as prerequisites. Existing Nix build,
Clippy, nextest, feature and portable-target lanes compile supported
first-party source with package lints. Dependencies remain under Cargo
cap-lints and are intentionally outside this first-party assurance claim.

## Security, privacy and maintenance evidence

A disposable dependency-free workspace with root
`[workspace.lints.rust] unsafe_code = "forbid"` and member
`[lints] workspace = true` was compiled offline with Cargo 1.98.1. Targeted
commands rejected unsafe blocks in library, binary, integration-test, example,
benchmark, build-script and proc-macro implementation targets with
`requested on the command line with -F unsafe-code`.

A second probe used a safe proc-macro implementation whose returned tokens
contained an unsafe block. The opted-in consumer compiled successfully. Exact
Rust 1.98.1 tag revision `48a229ceaefd4985c50990b14116b6d856af0985`
implements `UnsafeCode::report_unsafe` by returning when
`span.allows_unsafe()`; rust-lang/rust#53975 records this as intentional
external-expansion lint behavior. This evidence stops complete limitation
retirement and is tracked by #189.

The SDK change adds no compiled unsafe or native code, FFI, secret material,
public diagnostic or serialization. The committed fixture will bound synthetic
compiler stderr before test diagnostics. The supply-chain and maintenance
posture is smaller than a scanner dependency because Cargo/rustc are existing
pinned release inputs. Security evidence remains bounded: safe Rust does not
prove logical correctness or side-channel resistance.

## Rejected or deferred candidates

Per-crate duplication, workspace `deny`, CI-only flags and third-party scanners
are rejected as described above. Automated doctest execution is deferred
because current release gates do not promise a doctest lane and issue #169 does
not authorize CI-cost expansion. Dependency-internal unsafe rejection is also
deferred permanently from this first-party lint; dependency ADRs own that
evidence. Expansion scanning, `cargo-expand`, nightly expanded output and
current-macro-specific expansion assertions are deferred to #189 because the
naive compiler-negative premise failed and the replacement needs its own
stability/cost/false-negative research.

## Open questions and blockers

No blocker remains for authored-source enforcement. Full generated-output
enforcement is blocked by the reproduced compiler behavior and remains a named
limitation under #189. The current exception set is empty. A reconsideration trigger
is an evidenced need for first-party unsafe, a Cargo lint-inheritance or
macro-span change, a newly supported source target, or failure of any
compile-negative fixture.

Any future exception requires a new issue, dedicated safety ADR and indexed
record with exact scope, rejected safe alternatives, invariants/evidence,
owner/specialist reviewer, security and maintenance cost, activation and
rollback. It may change only the named package and must not weaken unrelated
workspace lints.

## Evidence commands

Research commands on exact Cargo 1.98.1 included:

```text
cargo check --offline -p unsafe-policy-probe --lib
cargo check --offline -p unsafe-policy-probe --bin unsafe-policy-probe
cargo check --offline -p unsafe-policy-probe --test unsafe
cargo check --offline -p unsafe-policy-probe --example unsafe
cargo check --offline -p unsafe-policy-probe --bench unsafe
```

Separate variants selected an unsafe build script and proc-macro library; all
authored targets failed on `-F unsafe-code`. The generated proc-macro variant
unexpectedly passed, and exact Rust source plus rust-lang/rust#53975 reconciled
that result. Repository inventory used `rg` over crate roots, manifests and
unsafe constructs. Implementation evidence will add focused conformance tests,
strict Clippy, format, factory and complete Nix checks.

Intentionally unrun checks at research stage are full Nix/hosted CI (reserved
for immutable implementation), dependency audit (no cone delta), Miri and
sanitizers (no unsafe runtime implementation), Windows (unsupported) and
doctests (no current lane promise). Rollback restores `SDK-LIM-008` with no
runtime or data migration.

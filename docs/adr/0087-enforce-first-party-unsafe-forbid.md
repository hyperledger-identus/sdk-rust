# ADR 0087: enforce a workspace-wide first-party unsafe forbid

- **Status:** Accepted
- **Date:** 2026-09-08
- **Decision authority:** sdk-rust issue #169 under governance issue #166
- **Constraint impact:** material enforcement of existing `SDK-SEC-001`;
  narrows `SDK-LIM-008` to the procedural-macro expansion gap under #189

## Context

Unsafe first-party Rust is prohibited by policy, but only six of 17 library
roots currently add `#![forbid(unsafe_code)]`. Every package already inherits
workspace lints, so Cargo/rustc can enforce one root rule across separate target
crate roots that a library attribute cannot cover.

Rust 1.98.1 research confirmed exact workspace `forbid` is forwarded as
`-F unsafe-code` to library, binary, integration-test, example, benchmark,
build-script and proc-macro targets. Root configuration alone can still drift
if a future package omits `[lints] workspace = true`, so static and behavioral
evidence are both required.

## Decision

1. Add `unsafe_code = "forbid"` to `[workspace.lints.rust]` and keep every
   package on exact workspace lint inheritance.
2. Retain existing crate-root forbids as harmless defense in depth.
3. Add one conformance guard for root/member configuration plus dependency-free
   offline compile-fail probes across supported authored target classes,
   including proc-macro implementations.
4. Limit the assurance claim to authored first-party SDK source. External dependencies
   retain cap-lints behavior and their unsafe/native posture is governed by
   dependency ADRs and research.
5. Narrow `SDK-LIM-008` when full local and hosted evidence passes. Exact Rust
   1.98.1 skips procedural-macro expansion spans that allow internal unsafe;
   issue #189 owns the stronger expansion-evidence decision.

## Exception contract

There is no approved exception. A future exception is a separate material
decision requiring a new issue, dedicated safety ADR and indexed record with
exact crate/target/source scope, rejected safe alternatives, invariants and
evidence, owner/specialist reviewer, security and maintenance cost, exit
trigger, activation and rollback.

Because workspace lint inheritance cannot be selectively overridden, the
approved package would mirror every unrelated workspace lint locally, retain
package-boundary `deny(unsafe_code)` and allow only the recorded inner scope.
The conformance guard would reject the manifest/source deviation unless the
exception record changed atomically. Other packages remain on `forbid`.

## Consequences

- Ordinary local Cargo and Nix/CI builds reject unauthorized authored
  first-party unsafe code without a separate scanner or dependency.
- New packages cannot silently omit enforcement.
- Exceptions are possible but intentionally expensive and visible.
- The lint does not certify dependency internals, procedural-macro generated
  output, logical correctness, side-channel resistance or unsupported builds.
- Verification code runs nested dependency-free Cargo probes, adding a small
  test cost without network or release-artifact impact.

## Rollback

Revert the issue #169 PR and restore the broader `SDK-LIM-008`. No runtime,
public API, wire, dependency, persisted-data or downstream migration is
involved.

# ADR 0064: separate primary Rust validation from evidence-driven MSRV

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-07
- **Issue:** [#170](https://github.com/hyperledger-identus/sdk-rust/issues/170)
- **Supersedes:** ADR 0062 and its stable-minus-three policy
- **Retains:** ADR 0002 as the NeoPRISM etalon and Nix-baseline decision

## Context

ADR 0062 correctly separated MSRV from the maintainer compiler but selected
Rust 1.95 using a fixed three-release distance from current stable. That
formula does not describe a consumer requirement. The current locked workspace
passes on Rust 1.85, 1.89, 1.90 and 1.95; of 94 external packages, 73 declare a
`rust-version`, 21 omit it, and the highest declared value is 1.85.

Rust 1.98.1 is the corrected current stable release. Rust 1.98.0 can generate
an invalid trait-object vtable in some circumstances. Meanwhile, the immutable
NeoPRISM baseline pins nightly `2026-03-18` and uses the nightly-only
`error_reporter` feature. Its pinned rust-overlay can resolve Rust 1.89 but not
1.95 or 1.98.1.

Edition, primary validation compiler, public MSRV and integration etalon solve
different problems and must not move as one number.

## Decision

1. Keep Edition 2024.
2. Rust **1.98.1** is the pinned primary development and validation compiler.
   Full build, test, Clippy, formatting, documentation and supported
   compile-target gates use it.
3. Rust **1.85.0** remains the effective public MSRV. Every declared feature
   surface keeps an independent MSRV build gate.
4. Rust **1.89.0** replaces 1.95 as the next MSRV candidate. It is a target,
   not a support promise. Activation requires a focused issue, supported-target
   checks, named downstream evidence, migration impact and atomic Cargo/Nix/
   policy changes.
5. NeoPRISM's pinned nightly remains an independent etalon workspace gate. It
   cannot substitute for primary stable or MSRV evidence.
6. The NeoPRISM-derived nixpkgs, rust-overlay and Nix version remain pinned. A
   second exact rust-overlay revision supplies primary stable until a later
   coordinated etalon refresh can remove that need.
7. Sanitizer fuzz campaigns use an explicitly named shell backed by the pinned
   etalon nightly because libFuzzer requires nightly compiler instrumentation.
   This operational exception does not make nightly the default compiler or
   substitute for primary/MSRV gates.
8. Beta or moving nightly checks may be advisory but are never release or MSRV
   evidence.
9. Dioxus is an Oxid consumer concern and cannot set the generic SDK MSRV. A
   future FFI or platform boundary crate may request a higher crate-local MSRV
   only through its own material ADR; core crates do not rise automatically.

## MSRV activation evidence

A future uplift must identify concrete value: a required maintained dependency,
a security fix, a supported compiler/target limitation or a named consumer
baseline. It must prove the locked graph and all declared feature surfaces on
the candidate; compile supported WASM/mobile targets where Rust supplies the
artifacts; inspect downstream build constraints read-only; and publish migration
and rollback guidance. Time, average crate metadata and current-stable distance
are insufficient on their own.

## Consequences

- Contributors get current stable diagnostics and corrected code generation.
- Consumers keep the existing Rust 1.85 compatibility promise.
- NeoPRISM interoperability remains visible without making nightly the SDK's
  ordinary compiler.
- Fuzz campaigns keep deterministic sanitizer instrumentation through a
  dedicated pinned-nightly shell rather than inheriting the stable devshell.
- Hosted Linux CI reclaims preinstalled Android, .NET and Haskell toolchains
  that this Rust/Nix job does not use. This preserves all three compiler
  evidence classes within the runner's finite disk budget.
- Two rust-overlay pins add explicit maintenance cost and must be updated only
  through focused, lock-reviewed changes.
- Rust 1.89 ecosystem candidates can be evaluated without pre-committing every
  consumer to that floor.

## Alternatives rejected

### Continue stable minus three

Release arithmetic is predictable but not product evidence and would raise the
floor to 1.95 without a current dependency or consumer need.

### Make 1.98.1 the MSRV

This maximizes dependency access but forces every consumer onto current stable
and eliminates a meaningful qualification window.

### Keep the NeoPRISM nightly as the only primary compiler

This preserves exact compiler symmetry but does not prove supported stable Rust
behavior and keeps ordinary diagnostics tied to a consumer's nightly feature.

### Activate Rust 1.89 immediately

Host compilation passes, but the candidate has not yet completed the full
supported-target and downstream compatibility matrix. It remains a target.

## Rollout and rollback

The issue-linked implementation adds the stable overlay/provider, migrates full
gates to 1.98.1, retains independent etalon/MSRV gates and updates the machine
contract atomically. Rollback restores full gates to the etalon provider and
removes the second overlay. The public Rust 1.85 floor is unchanged throughout.

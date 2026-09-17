# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

The current implementation under ADR 0081 uses Rust 1.98.1 for the workspace floor, primary compiler, and
etalon so active PRs do not pay for redundant compiler lanes. The policy is
explicitly release-ineligible. The existing slow manifest already preserves
independent MSRV-labelled host and feature gates, but its MSRV provider is
currently aliased to the primary compiler and therefore proves no lower
compatibility floor.

The unpublished candidate contains `identus-derive`, `identus-core`, and
`identus-crypto`. Its default, all-features, no-default-features, and
`kmp-compat` closures are deterministic. A real narrow consumer also needs the
generic hash-only cone, which is present in Cargo but not yet an independently
declared candidate profile.

## Normative sources

- Issue #325 and M3 coordinator #326 define the release outcome.
- The sponsor-provided Rust toolchain recommendation selects Rust 1.98.1 for
  development/primary CI and recommends Rust 1.89 as the initial SDK MSRV when
  practical, with FFI/WASM/UI kept outside generic core.
- Cargo `rust-version` defines the package compiler floor; Cargo packaging and
  registry documentation require the published manifest to carry it
  truthfully.
- Primary source URL: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
- ADRs 0064 and 0081 separate primary validation, MSRV, and tooling-nightly
  purposes and require consumer/target evidence before publication.

## Compatibility and dependency evidence

Locked `cargo metadata` reports 150 external packages: 108 declare
`rust-version`, 42 omit it, and the highest declared external requirement is
Rust 1.85.0. No selected package declares a requirement above Rust 1.89.0.
Earlier ADR 0064 evidence proved the workspace on Rust 1.85, 1.89, 1.90, and
1.95 before the temporary compiler alias was introduced.

The direct and resolved dependency cone is the locked workspace graph observed
by `cargo metadata --locked --format-version 1`. Dependency licenses and
provenance remain governed by the existing Cargo-deny, audit, source, and
candidate archive gates; this change adopts no new source or license. The
public and wire compatibility surface is unchanged: only the declared compiler
floor and evidence matrix change. The Identus-owned crate APIs remain the
facade, with no third-party type newly re-exported.

The NeoPRISM SDK-adoption branch
`e8c504c1d2f7ac39c33eb32c50ff6c99204bb91a` uses nightly 2026-09-02, which is
newer than the candidate floor. Other consumer repositories remain downstream
evidence only: their compiler and product decisions do not enter SDK code or
dependencies, and this change does not mutate them.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep Rust 1.98.1 as both public MSRV and primary | `not-adopt` | Preserves PR simplicity but unnecessarily forces every library consumer onto the newest compiler and contradicts the reusable-SDK recommendation. | A required dependency or language correctness fix makes the lower floor unsafe. |
| Restore Rust 1.85.0 | `not-adopt` | Technically feasible but expands the compatibility window beyond the sponsor-selected initial target without a release consumer requiring it. | A supported consumer demonstrates material value and the full matrix remains affordable. |
| Rust 1.89.0 MSRV plus Rust 1.98.1 primary/etalon | `adopt` | Matches the approved recommendation, clears the complete locked dependency graph, creates a meaningful qualification window, and adds only weekly/release evidence rather than another PR build. | Required dependency, security/compiler defect, or supported-consumer constraint changes. |
| Rust 1.95.0 MSRV | `not-adopt` | Lower qualification cost than 1.89, but no current source/dependency requirement justifies excluding 1.89 consumers. | The 1.89 slow lane becomes materially costly or blocks required maintained dependencies. |
| Per-PR MSRV plus primary matrix | `not-adopt` | Reintroduces the feedback delay the fast/slow decision removed. | MSRV regressions recur between weekly runs or release cadence becomes continuous. |
| Weekly/manual MSRV plus release receipt | `adopt` | Keeps one fast integration signal while making compatibility a mandatory production-promotion gate. | Evidence freshness or escaped-regression data shows the cadence is insufficient. |

## Target, feature, and boundary evidence

The release train remains ordinary Rust. Linux x86_64 and macOS ARM64 are
host-tested. Browser WASM, Android ARM64, and iOS ARM64 are compile-only for
the release crates; no runtime, binding, device, package-store, custody, secure
storage, performance, or certification claim follows. Windows, WASI, and FFI
remain unsupported/deferred. Primary and MSRV gates cover default,
all-features, no-default-features, `kmp-compat`, and hash-only candidate cones.

## Security, privacy and maintenance evidence

The change introduces no dependency, unsafe code, primitive, secret surface,
native code, protocol, secret surface, or wire behavior. The existing
supply-chain audit, license, advisory, and exact-lock controls remain intact. A
lower manifest floor increases only the compatibility
claim and is backed by independent builds. Rust 1.98.1 remains the compiler
used for tests, Clippy, docs, candidate packaging, and full security evidence.
Rollback restores the release-ineligible single-compiler policy and blocks
publication; it does not require a consumer migration before publication.

After publication, an MSRV increase within the `0.1.x` line is prohibited.
A future increase requires a focused compatibility ADR, named payoff,
migration notice, complete target/profile evidence, and at least the next
minor pre-1.0 release line.

Maintenance and release posture is reviewed no later than 2027-03-17.
Protocol/draft currency is not applicable because the slice changes compiler
compatibility rather than an SSI wire/profile version.

## Rejected or deferred candidates

Beta/nightly compatibility, Windows, WASI, FFI, mobile/browser runtime,
application frameworks, and chain-specific consumer builds remain deferred.
The sanitizer nightly stays a tooling-only exception.

## Open questions and blockers

None for repository implementation. Hosted Linux/macOS and cross-target Nix
evidence must pass on the exact merged candidate before #326 can approve a
release.

## Evidence commands

Planning evidence uses locked Cargo metadata, policy/ADR inspection, immutable
consumer revisions, and current Cargo package semantics. Implementation
evidence will use Rust 1.89.0 Cargo checks for the workspace and release
profiles, Rust 1.98.1 normal gates, target builds, factory checks, candidate
packaging, hosted `fast`, and the complete exact-SHA `slow` receipt.

Commands run during planning: `cargo metadata --locked --format-version 1`,
`git show`/`git grep` over immutable consumer revisions, `scripts/factory
doctor`, `scripts/factory research-ready`, `scripts/factory constraints-ready`,
and the OpenSpec archive-preservation checker. Unrun before implementation:
Rust 1.89 builds, portable-target builds, Nix checks, candidate assembly, and
hosted workflows; these are implementation and exact-candidate evidence.

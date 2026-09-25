# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-26
Source retrieval date: 2026-09-26
Research blockers: none

## Problem and existing implementation

The current implementation at protected revision
`e5e4c0c914dc9b7a8129f475ba39243c8b8e0bae` already has a machine support
policy naming Rust 1.89.0 as the
`0.1.x` MSRV, Rust 1.98.1 as primary/etalon, Linux x86_64 and macOS ARM64 as
host-tested systems, and browser WASM/Android ARM64/iOS ARM64 as compile-only
targets. Its portable package set contains `identus-did` but deliberately does
not contain `identus-did-resolver-http`.

The existing Nix gates operate on the canonical workspace. The DID builder
separately renders `0.1.0-rc.1` manifests in a VCS-independent scratch tree,
but only verifies their feature profiles under the preparation compiler on the
current host. M5 therefore lacks one exact receipt proving the staged identity
across both compilers and native hosts while retaining honest portable target
scope.

## Normative sources

- M5 coordinator #381 and focused issue #387.
- ADR 0133 and `docs/architecture/sdk-support-policy.toml` for compilers,
  hosts, targets, tiers, and CI cadence.
- ADRs 0153/0154, `docs/release/did-candidate.toml`, and
  `docs/release/release-trains.toml` for candidate identity and API origin.
- `scripts/prepare-did-candidate.py` for the existing bounded staged workspace.
- `nix/rust-toolchain.nix`, `nix/checks/gates.toml`, and
  `.github/workflows/nix-checks.yml` for pinned execution and weekly/manual
  evidence.
- [Cargo rust-version](https://doc.rust-lang.org/cargo/reference/rust-version.html)
  for the declared compiler floor and Cargo diagnostics.
- [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) for host
  test execution, feature selection, and target-mode semantics.
- [Cargo target glossary](https://doc.rust-lang.org/cargo/appendix/glossary.html)
  for the distinction between compiler host and output target triples.

All repository source is Apache-2.0 under its tracked `LICENSE`; this change
reuses repository-owned provenance and imports no donor code or fixture.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Qualify canonical workspace sources | Canonical `0.0.0` | `not-adopt` | It cannot prove the staged version, exact internal requirements, or release-shaped manifest. | Never as candidate evidence. |
| Reuse the staged workspace renderer | `e5e4c0c914dc9b7a8129f475ba39243c8b8e0bae` | `adopt` | It is bounded, VCS-independent, mutation-tested, and produces the candidate manifests under review. | Candidate builder is superseded. |
| Test both packages on both native hosts | Rust `1.98.1` | `adopt` | Host runtime tests exercise domain behavior and the Axum adapter where meaningful. | Host support policy changes. |
| Compile/check both packages on both native hosts | Rust `1.89.0` | `adopt` | Independently proves the MSRV without adding a PR compiler lane. | MSRV changes through a new ADR. |
| Compile-check only `identus-did` on portable targets | Rust `1.89.0` and `1.98.1` | `adopt` | Matches the effective portable package set and generic domain/port boundary. | A consumer-backed adapter portability issue supplies runtime and deployment semantics. |
| Treat resolver HTTP as portable when Cargo compiles it | Axum `0.8.9` workspace lock | `not-adopt` | Incidental compilation is not a support decision; the adapter has no browser/mobile server contract. | Focused HTTP target/support ADR. |
| Add matrix jobs to required PR CI | Current `fast` contract | `not-adopt` | Duplicates compilers and platforms on the inner loop contrary to the accepted model. | Measured CI-policy decision. |
| Add two native slow lanes and aggregate receipts | Slow schema v1 | `adopt` | Uses actual Linux/macOS hosts, pinned compilers, bounded artifacts, and weekly/manual control. | Native runner or artifact model changes. |
| Dispatch slow CI from this PR | M5 #388 | `not-adopt` | A PR merge proves wiring; final M5 owns natural/manual exact-candidate execution. | Explicit release-manager request after the candidate freezes. |

## Package support decision

| Package | Linux x86_64 | macOS ARM64 | WASM | Android ARM64 | iOS ARM64 |
| --- | --- | --- | --- | --- | --- |
| `identus-did` | Host test on 1.98.1; MSRV check on 1.89.0 | Host test on 1.98.1; MSRV check on 1.89.0 | Compile-check on both | Compile-check on both | Compile-check on both |
| `identus-did-resolver-http` | Host test on 1.98.1; MSRV check on 1.89.0 | Host test on 1.98.1; MSRV check on 1.89.0 | Not supported | Not supported | Not supported |

`openapi` is included in the HTTP host all-features surface. `identus-did` has
no features, so its default and no-default candidate profiles are equivalent
but remain independently named in the staged contract.

## Compatibility and dependency evidence

No Cargo dependency, feature, public API, wire shape, unsafe policy, MSRV, or
global target tier changes. The direct and resolved dependency cone is the
existing staged candidate closure; no dependency is added. The staged lockfile
is generated once per lane and hashed; aggregate validation requires the same
lock identity across compiler and host lanes. Exact internal candidate
dependencies remain `=0.1.0-rc.1`. Existing dependency/license checks remain
the supply-chain evidence; reachable unsafe and native code posture is
unchanged because this slice adds no product dependency or compiled source.

Public and wire compatibility are unchanged. The crate facade boundary remains
`identus-did` for generic domain/ports and `identus-did-resolver-http` for the
optional host adapter. Rollback removes only repository-local matrix evidence.

## Security, privacy and maintenance evidence

The matrix runner accepts only an exact current Git SHA, a closed compiler
class, the detected allowed host, and an absolute new output path. It reuses
the builder's sanitized environment, command allowlist, symlink rejection, and
external scratch boundary. Lane and aggregate JSON are regular, atomic,
closed-schema, duplicate-safe, byte-bounded files. They contain tool and build
metadata only—no logs, environment dump, credentials, prompts, PII, or source
payload. Publication and network mutation commands remain impossible. The
maintenance, release, and security posture remains the existing M5 protected
flow. Protocol/draft currency is not changed: DID Core/Resolution semantics and
versions remain those already governed by their capability specifications.

## Evidence model

Each native runner emits one lane receipt per compiler. Entries bind candidate
version, package, profile or target, operation, exact toolchain, host, outcome,
and limitation. Slow CI uploads the two lane files for each host. The final
job downloads exactly those four receipts and emits one bounded aggregate
receipt only if every expected identity is unique, clean, successful, and
complete and every portable HTTP entry is explicitly unsupported.

## Rejected or deferred candidates

Canonical `0.0.0` qualification, portable HTTP support, per-PR matrix builds,
workflow dispatch, runtime/device/browser claims, new toolchains, new Cargo
dependencies, FFI/package publication, and downstream mutation are rejected or
deferred as recorded in the decision table.

## Evidence commands

Commands planned are the focused checker/mutation suite, local matrix Nix apps,
`nix flake check`, formatting, factory/OpenSpec validation, and hosted `fast`.
Unrun checks are Linux-host and complete cross-host slow evidence. Descriptor/checker mutations, synthetic receipt rejection tests, Nix app
evaluation, local host lanes, workflow policy, factory/OpenSpec, and exact-head
fast CI run after immutable preflight. Linux-host and complete cross-host
evidence remains deferred to the next natural or authorized manual slow run.

## Open questions and blockers

None. The effective support policy and #387 resolve the compiler, package,
host, target, cadence, and non-claim decisions.

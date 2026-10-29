# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

The descriptor binds `docs/release/did-candidate.lock` at SHA-256
`1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447`.
Candidate preparation renders unchanged `0.1.0-rc.1` manifests, copies that
lock, and invokes Cargo with `--locked`. Cargo now rejects the snapshot because
current manifest constraints require a different resolution.

The current implementation and consumer evidence are pinned to source revision
`7189d256c22b049849882320506962f7ea7fe888`; NeoPRISM is not changed by this
candidate-evidence refresh.

The credential-free `--refresh-staged-lock` mode generated a proposal outside
the worktree. Its report is clean and closed: six registry packages are
replaced by patch releases and the proposed lock SHA-256 is
`f7535a6819c9b9e4474c95b427b54646d46ccac51703c680086ef75c790fba4f`.

## Normative sources

Issue #581, coordinator #388, ADR 0155, the `release-candidate-trains`
specification, `docs/release/did-candidate.toml`, and
`scripts/prepare-did-candidate.py` govern the refresh. Cargo `--locked`
semantics remain the enforcement mechanism. The registry package sources and
checksums in the generated lock are provenance metadata, not imported donor
code.
Primary source URL: https://doc.rust-lang.org/cargo/commands/cargo-generate-lockfile.html

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Keep the stale lock | `not-adopt` | Clean preparation fails closed. |
| Remove `--locked` | `not-adopt` | Reintroduces time-dependent cross-host evidence. |
| Hand-edit versions | `not-adopt` | Bypasses the reviewed resolver and closed drift report. |
| Adopt the review-only proposal | `adopt` | Preserves exact staged manifests and makes the changed closure explicit. |
| Widen dependency ranges | `not-adopt` | No direct requirement change is needed or authorized. |

## Compatibility and dependency evidence

The closed drift is `equivalent 1.0.2 -> 1.0.3`, `lazy_static 1.5.0 -> 1.5.1`,
`serde_json 1.0.151 -> 1.0.154`, `syn 3.0.6 -> 3.0.9`,
`tokio 1.53.1 -> 1.54.1`, and `unicode-ident 1.0.26 -> 1.0.27`. All sources
remain crates.io registry entries; no Git/path dependency, package addition,
feature, direct requirement, API, wire shape, or candidate identity changes.
Tokio 1.54.1 also aligns the staged closure with the root workspace fix for
RUSTSEC-2026-0386.

The direct dependency requirements and resolved dependency cone remain
separate concepts: direct version/features are unchanged, while the staged
lock updates the six transitive packages above. Rust 1.89.0 remains the MSRV;
Linux/macOS and the package-specific portable target evidence rerun. License
and provenance stay registry-backed. Supply-chain evidence includes advisory,
license, source, SBOM, checksum, unsafe and native-code review. The public and
wire compatibility contract and the facade boundary remain unchanged.

## Security, privacy and maintenance evidence

The proposal contains package names, versions, sources, checksums, and edges
only. License, source, advisory, unsafe/native, SBOM, MSRV, host, portable-
target, and deterministic archive checks all rerun. No credential, secret, PII,
prompt, or runtime value is present. Future drift again requires a new issue
and review-only report.
Protocol or draft currency is not applicable because this is build provenance,
not SSI protocol behavior. Maintenance, release and security posture remain
governed by M5. Rollback restores the stale lock and therefore reopens the
known preparation failure.

## Rejected or deferred candidates

Unlocked resolution, manual editing, dependency-range changes, publication,
PRISM train activation, downstream mutation, and final approval are rejected
or remain deferred to their owners.
Reconsideration trigger: Cargo gains an immutable signed registry-snapshot
primitive or the candidate dependency requirements change intentionally.

## Open questions and blockers

The exact proposal and complete drift report are available locally. After the
lock was adopted, review-mode qualification completed archives, closure, API
extraction, and SBOMs. Normal qualification then isolated an additive
`identus-did` API-origin drift; issue #582 owns its classification and
rebaseline. That independent gate blocks final #388 qualification but does not
change or invalidate the closed lock decision.

## Evidence commands

Exact commands and unrun checks: run research/constraints/OpenSpec readiness
and issue preflight before copying
the proposal. Then run deterministic candidate preparation, API/SBOM/source
checks, both compiler lanes where locally available, factory/site/Nix gates,
advisory/license/source checks, and independent exact-head review. #388 retains
the complete hosted Linux/macOS slow and human-approval gates; #582 owns the
normal-mode API comparison before those final gates.

# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

`crates/crypto/tests/secp256k1_compat.rs` reads
`docs/conformance/fixtures/crypto/v1/vectors.json`, three parents above the
test. Workspace tests succeed because the repository file exists. The candidate
tool packages, extracts, and tests `identus-crypto` in an independent workspace;
compilation fails because the file is outside the archive. Protected run
40872151111 is the immutable reproduction at protected revision
`6ebd4633c987539689ae180fbccbb629572ca45a`.

The public Apache-2.0 packet has SHA-256
`d357c46c0369e5597c89203d92332b94dbf9684fb28b120bbe3241cdb55e09cb`.
The catalog, quality declaration, Nix filter, mutation tests, and Rust test all
point to its repository-level location.

## Normative sources

Cargo's official package-selection contract defines files admitted to a crate
archive, and `cargo package --list` exposes that selection. The canonical
`unpublished-crypto-candidate` and `source-distribution` specifications add the
isolated verification and SDK governance requirements. The existing vector
catalog remains authoritative for provenance, license, targets, and digest.

## Candidate decisions

| Candidate | Evidence | Decision |
| --- | --- | --- |
| Keep the packet above the crate and customize staging | An extra staged file tests a tree consumers never receive | `not-adopt` |
| Copy the packet into the crate while retaining the docs copy | Two mutable files can drift | `not-adopt` |
| Generate one copy from the other during packaging | Generation obscures ordinary package-list evidence and adds a preparation path | `not-adopt` |
| Move unchanged bytes into `crates/crypto/tests/fixtures/` | Test and archive share one bounded package-owned source; the repository catalog can reference it | `adopt` |

No new Rust crate or dependency is needed.

## Compatibility and dependency evidence

The move changes no public or wire behavior and adds no direct or transitive
dependency. Features, MSRV, host and portable targets, manifests, lockfiles,
unsafe-code policy, and version remain unchanged. Workspace and extracted
candidate paths execute the same test over byte-identical input.

## Security, privacy and maintenance evidence

The packet stays test-only, bounded, public, non-secret, and outside runtime
APIs. Packaging it adds one small fixed public-data file but no executable
content or production dependency. The unchanged digest prevents silent vector
substitution. The catalog preserves human discoverability after the move.

## Rejected or deferred candidates

Repository-relative staging, duplicate fixtures, generated copies, symlinks,
and excluding the test from candidate verification are rejected because they
test the wrong artifact, introduce drift, weaken the gate, or violate the
regular-file boundary. No candidate is deferred.

## Open questions and blockers

No blocker remains. Implementation must prove unchanged packet bytes, exact
single ownership, package membership, and complete candidate success without a
repository fallback.

## Evidence commands

Planning uses `scripts/factory research-ready`, `constraints-ready`, `validate`,
`check`, a signed/DCO planning commit, and `scripts/factory preflight
package-crypto-conformance-vectors --issue 586 --write`. Delivery uses focused
catalog and crypto tests, `cargo package --list -p identus-crypto`, candidate
preparation tests, `nix run .#crypto-candidate`, `nix flake check`, readiness,
receipt, independent Codex or Claude CLI review, protected CI, guarded merge,
and metrics publication.

Release, publication, tagging, downstream changes, and a retry of the old
protected run are outside this change.

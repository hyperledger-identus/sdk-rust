# ADR 0154: use first-candidate API snapshots as the SemVer origin

- **Status:** Accepted under standing routine authority
- **Date:** 2026-09-25
- **Issue:** [#384](https://github.com/hyperledger-identus/sdk-rust/issues/384)
- **Milestone:** M5 — DID SDK consumable release candidate
- **Review no later than:** before the next DID candidate or publication

## Context

The DID crates are canonical unpublished `0.0.0` workspace packages. M5 stages
their first candidate as `0.1.0-rc.1`. There is no earlier released DID crate
whose API could serve as an honest cargo-semver-checks baseline. Comparing the
candidate to its current workspace source or to itself would always pass while
making a false compatibility claim.

The repository already governs exact cargo-public-api and cargo-cyclonedx tool
versions for its crypto train. The DID train also needs reproducible API and
dependency evidence before publication review.

## Decision

1. Generate committed public-API snapshots for both staged DID candidate
   packages from explicit Rust 1.98.1 rustdoc JSON and cargo-public-api 0.52.0.
2. Treat these snapshots as the compatibility origin for subsequent DID
   candidates/releases. They are not a stable API promise by themselves.
3. Record SemVer comparison for `0.1.0-rc.1` as `not-applicable`; do not execute
   cargo-semver-checks against workspace `0.0.0` or the candidate itself.
4. Retain cargo-semver-checks 0.50.0 as the repository-governed future tool.
   The first later candidate with a real predecessor must use it (or accept a
   superseding ADR) before promotion.
5. Generate one validated CycloneDX 1.5 JSON SBOM per candidate package using
   cargo-cyclonedx 0.5.9. This is local review evidence, not signed provenance,
   an advisory result, registry availability or certification.
6. API/SBOM generation must not change the deterministic package inputs; the
   archive hashes accepted by #382 remain invariant.

### Prepublication rebaseline

On 2026-10-29, before the DID train had a published predecessor, issue #582
rebased the `identus-did` origin from SHA-256
`3ce68d571d087e78ece200154ba0e185565fbf5da9c2733eb7f70d3b1f7bbf9d`
to `5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639`.
Pinned cargo-public-api 0.52.0 found zero removals and exactly 13 additions:
the bounded JSON wire validator, its limits, and its non-exhaustive error type.
The resolver API remained byte-identical. The closed
`did-api-origin-rebaseline.toml` receipt binds the source revision, hashes,
classification, and exact added symbols.

This is a one-time correction of an unpublished candidate origin, not a stable
API promise or a substitute for comparing a later candidate with its real
published predecessor. Further origin drift requires another issue, decision
record, exact receipt, mutation coverage, and candidate reproduction.

## Consequences

Reviewers get exact, diffable API origins and machine-readable dependency /
license evidence without pretending an initial candidate has a predecessor.
Future candidates have an explicit compatibility gate. The candidate toolchain
grows, but only in release evidence and with versions already governed by the
repository.

Repository Cargo-deny and slow security evidence remain responsible for
advisory/license policy. The SBOM does not duplicate or replace those gates.

## Alternatives rejected

- **Workspace `0.0.0` baseline:** same development source, not a released
  contract.
- **Self-comparison:** produces meaningless green evidence.
- **No committed API origin:** defers compatibility review until after a public
  artifact exists.
- **Run a second mutable advisory database in candidate assembly:** undermines
  deterministic evidence and duplicates repository policy.

## Verification and rollback

The descriptor/checker bind exact tools and baseline paths. A clean candidate
run must match both API snapshots, validate both bounded SBOMs and reproduce
the prior archive SHA-256 values. Rollback removes additive evidence fields,
snapshots and generation; canonical manifests and remote state do not change.

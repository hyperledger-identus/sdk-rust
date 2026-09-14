# Prepare an unpublished cryptography package candidate

## Why

Apollo behavior is covered, but `identus-crypto` is still a workspace-only
`0.0.0` package. Discussion #252 identifies package reproducibility, metadata,
API-change evidence, SBOM/provenance, and clean-consumer verification as the
next deprecation-readiness gap. Issue #266 deliberately separates that
technical packaging work from crates.io ownership and human release authority.

## What changes

- Select `identus-crypto` as the durable Rust package brand.
- Define an isolated `0.1.0-rc.1` candidate closure containing
  `identus-derive`, `identus-core`, and `identus-crypto`.
- Add deterministic candidate staging, packaging, archive verification,
  checksum, SBOM, provenance, and API-baseline evidence.
- Add package READMEs and complete candidate metadata without making canonical
  development manifests publishable.
- Add a candidate-only slow validation entry point and fail-closed contract
  tests.

## Capabilities

### Added capabilities

- `unpublished-crypto-candidate`: reproducible preparation and validation of a
  three-package cryptography release candidate without publication authority.

## Non-goals

- No crates.io upload, namespace reservation, trusted-publishing activation,
  Git tag, GitHub release, or `main` change.
- No claim that Rust 1.98.1 is the published-release compatibility matrix.
- No foreign-language package, ABI, downstream migration, or Apollo lifecycle
  transition.
- No new cryptographic primitive or change to runtime behavior.

## Delivery

Issue #266 owns the change from protected `develop@8110277`. The PR targets
`develop`, preserves this planning commit through merge, and must pass the
existing required gates plus the candidate-specific evidence command.

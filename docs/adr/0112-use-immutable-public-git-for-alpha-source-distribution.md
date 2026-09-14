# ADR 0112: use immutable public Git for alpha source distribution

- **Status:** Accepted under standing routine authority
- **Date:** 2026-09-14
- **Issue:** [#255](https://github.com/hyperledger-identus/sdk-rust/issues/255)
- **Supersedes:** undocumented source-consumption conventions
- **Review no later than:** before the first registry release candidate

## Context

The SDK is public but unreleased. NeoPRISM already proves that five packages can
be consumed anonymously from the public repository at an exact commit. The SDK
needs to make that source identity reproducible without implying SemVer,
registry publication, support lifetime, or a second Nix package surface.

## Decision

1. The alpha source channel is the canonical public HTTPS Git repository at a
   full lowercase 40-hex commit.
2. The proven package set is `identus-core`, `identus-derive`,
   `identus-crypto`, `identus-did`, and `identus-did-resolver-http`.
3. A consumer commits `Cargo.lock`, selects features deliberately, and records
   the old and new SDK commits plus its validation evidence.
4. A Nix consumer also retains its ordinary locked or fixed-output source
   evidence. The SDK does not publish a supported Nix package in this phase.
5. Branches, tags, pull-request refs, short revisions, and authenticated source
   URLs are not supported artifact identities.
6. Workspace version `0.0.0` and `publish = false` remain deliberate. This ADR
   authorizes source evaluation, not release or publication.
7. `identus-apollo` remains a downstream NeoPRISM compatibility facade; it is
   not an SDK package alias.

## Consequences

Public Rust consumers can build an exact SDK snapshot without repository
credentials. Each consumer owns its feature and target validation, and source
updates remain explicit dependency changes. A crate that is itself published
to crates.io must replace Git dependencies with registry dependencies first.

The channel has no registry checksum, SemVer promise, binary artifact,
foreign-language package, or production-readiness claim. Rust 1.98.1 remains
the current consumer floor until a separate release-phase decision replaces
it.

## Alternatives rejected

- **Floating `develop`, tag, or pull-request ref:** mutable or disposable.
- **Authenticated preview source:** makes credentials part of public use.
- **Vendoring per consumer:** duplicates provenance and security maintenance.
- **SDK-owned Nix package:** expands the distribution surface without a named
  consumer need.
- **Immediate registry publication:** namespace, release, compatibility, and
  protected publishing decisions are not complete.

## Verification and rollback

The offline source-distribution checker binds package names, manifest state,
the canonical repository, exact-revision examples, lock requirements, compiler
floor, and README discoverability. Focused mutation tests prove that mutable
refs, short revisions, publication drift, and missing evidence fail closed.

Rollback removes this documented channel and its checker. Existing consumers
remain pinned to immutable source, but the SDK would no longer advertise that
path.

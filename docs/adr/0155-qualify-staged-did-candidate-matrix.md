# ADR 0155: qualify the staged DID candidate with a closed compiler and target matrix

- **Status:** Accepted under standing routine authority
- **Date:** 2026-09-26
- **Issue:** [#387](https://github.com/hyperledger-identus/sdk-rust/issues/387)
- **Milestone:** M5 — DID SDK consumable release candidate
- **Review no later than:** before expanding DID candidate target or runtime claims

## Context

The DID release train stages `identus-did` and
`identus-did-resolver-http` as exact `0.1.0-rc.1` candidate sources while the
canonical workspace remains unpublished at `0.0.0`. Repository-wide checks do
not by themselves prove that those staged sources build with both the primary
compiler and MSRV, nor do they distinguish the portable DID model from its
Axum transport adapter.

Running every compiler and target on every pull request would duplicate the
fast lane and slow normal development. Conversely, treating incidental Cargo
success as a supported target would overclaim portability. Release evidence
therefore needs a bounded, package-specific matrix tied to one exact source
revision and one staged lockfile.

## Decision

1. Qualify staged sources, never canonical `0.0.0` manifests.
2. On Linux (`x86_64-linux`) and macOS (`aarch64-darwin`), run every declared
   profile for both packages as tests with Rust 1.98.1 and compile checks with
   MSRV 1.89.0.
3. Compile-check `identus-did` with both compilers for
   `wasm32-unknown-unknown`, `aarch64-linux-android`, and
   `aarch64-apple-ios`. These are compile-only claims, not runtime, packaging,
   browser, device, simulator, FFI, or certification claims.
4. Record `identus-did-resolver-http` as explicitly unsupported on those
   portable targets. The Axum adapter remains host-only even if a future
   dependency combination happens to compile elsewhere.
5. Run the matrix only in the existing weekly/manual slow workflow. It does
   not add pull-request triggers or required statuses. Final candidate approval
   requires a natural or explicitly approved manual slow run; this ADR does
   not authorize dispatch or rerun.
6. Produce one bounded lane receipt per host/compiler pair and aggregate the
   exact four receipts only when source revision, candidate identity, staged
   lockfile, compiler, host, target assignments, and outcomes match the closed
   descriptor. Reject missing, duplicate, extra, failed, dirty, or overclaiming
   evidence.
7. Keep runtime and binding evidence independent. Existing browser and Android
   jobs may complement the compile matrix but cannot change its support tier.

## Consequences

Reviewers can distinguish host-tested support, portable compile support, and
explicit non-support without reading CI implementation details. Both compiler
versions share the same candidate source contract, while normal pull requests
retain the single fast lane. Weekly evidence costs four native lane executions
across two hosted operating systems.

The HTTP adapter cannot inherit portable support from `identus-did`; a future
portable resolver transport requires a new package boundary or a superseding
decision.

## Alternatives rejected

- **Run the complete matrix on every pull request:** disproportionate during
  active development and contrary to the fast/slow CI decision.
- **Compile every package for every target:** converts incidental dependency
  behavior into an unsupported product promise.
- **Test canonical workspace manifests:** does not qualify the staged version,
  exact internal dependencies, or release lockfile.
- **One unstructured workflow log:** cannot be validated or cited as an exact,
  immutable release receipt.
- **Claim portable runtime support from `cargo check`:** conflates compilation
  with execution, packaging, and platform integration.

## Verification and rollback

The release-candidate checker validates the descriptor, Nix applications, and
workflow wiring. Mutation tests reject target drift and portability overclaims.
The slow workflow produces attempt-scoped lane artifacts and a bounded aggregate
receipt tied to the checked-out SHA. Rollback removes the additive matrix apps,
jobs, descriptor fields, and receipts; the candidate archives and canonical
manifests remain unchanged.

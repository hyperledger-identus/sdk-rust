# ADR 0018: establish reproducible DID lexical fuzzing

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-005 roadmap mandate and sdk-rust issue #35
- **Related work:** issues #5, #34, #38, #41 and OpenSpec
  `fuzz-did-lexical-boundaries`

## Context

The SDK owns a bounded dependency-free `Did`/`DidUrl` parser with cached byte
ranges and deterministic conformance tests. That baseline does not continuously
search hostile byte combinations for a panic, inconsistent public construction
path, or invalid range. All four intended consumer families benefit from the
generic assurance; none benefits from moving PRISM/Midnight state semantics or
wallet policy into the SDK.

## Decision

1. A standalone root `fuzz/` workspace owns two libFuzzer targets, one for
   `Did` and one for `DidUrl`. No fuzz-only package enters a published crate.
2. Targets accept arbitrary bytes, return for non-UTF-8, permit rejection, and
   assert exact public access/Display/owned/`FromStr`/serde invariants for every
   accepted value. DID URL ranges and reconstruction receive additional checks.
3. The repository Nix shell pins nightly `2026-03-18`, cargo-fuzz `0.13.2` from
   nixpkgs `7241bcbb`, and exact `libfuzzer-sys 0.4.13`.
   The OSI-approved NCSA portion of the runtime license receives a version-
   exact cargo-deny exception; the general license allow-list is unchanged.
4. A single wrapper owns three modes: committed-corpus replay, one-worker/fixed-
   seed 4,096-run smoke, and a 300-second-per-target soak. Generated inputs are
   capped at 8 KiB, each input at five seconds, and RSS at 1 GiB.
5. Original corpora and grammar dictionaries cover W3C, PRISM, Midnight, web,
   key, delimiter, escape, Unicode and size shapes without method semantics.
6. Path-scoped Linux CI runs smoke on pull request and `develop`; schedule and
   manual dispatch run soak. It checks the independent lock with cargo-deny and
   RustSec first. Failure artifacts are uploaded for minimization.
7. Every accepted finding is minimized, retained as corpus, and expressed as a
   deterministic named regression when distinct. Throughput is diagnostic only.

## Consequences

- The generic parser gains continuous sanitizer and coverage-guided evidence
  with no public API, wire, feature, MSRV, runtime, or normal dependency change.
- PR latency stays bounded while longer campaigns continue independently.
- The Nix shell grows one development tool and Linux CI grows a focused job.
- Bounded fuzzing reduces risk but cannot prove absence of parser defects.
- Method correctness, DID documents/results, resolution, chain/runtime state,
  trust, custody, FFI, release, and product policy remain outside this change.

## Provenance

The normative source is W3C DID Core 1.0 (19 July 2022). Editor/test-suite pins
are `a2bb463` and `939b31d`. Rust Fuzz Book and LLVM libFuzzer documentation
define the runner controls. Apollo `ccee22b`, NeoPRISM `d6ad1ec`,
midnight-identity `427f857`, Lace `804de0a`, and Oxid `bfe3b48` are read-only
compatibility evidence. No donor source or fixture is copied.

## Rollback

Revert issue #35's focused PR. No released crate, persisted SDK data, consumer
repository, chain state, or reserved `main` branch is changed.

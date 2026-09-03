# ADR 0019: reproducible public-key representation fuzzing

- **Status:** Accepted for implementation on `develop`
- **Date:** 2026-09-03
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issue #58, parent #9 / `IDR-004`, ADRs 0005–0007 and 0018

## Context

The validated JWK, COSE Key, and thumbprint implementations have deterministic
standards and negative vectors. Their untrusted parsers, deterministic
encoders, and cross-format conversion paths still lack sanitizer-backed
coverage-guided search. The repository now has a pinned independent cargo-fuzz
workspace for DID lexical boundaries that can safely host another component
without changing a production dependency cone.

## Decision

1. Generalize the independent fuzz package name and documentation, preserving
   the existing DID targets and commands.
2. Add one arbitrary-byte JWK target covering validating serde, canonical
   coordinates, RFC 7638 thumbprints, and structural full-coordinate COSE
   conversion.
3. Add one arbitrary-byte COSE target covering the 4,096-byte/16-level/32-
   parameter profile, deterministic encoding, and structural JWK conversion.
   Text-only committed positive seeds may use a harness-local `hex:` transport;
   unprefixed inputs remain raw CBOR.
4. Reuse the exact nightly, cargo-fuzz/libFuzzer pins, fixed replay/smoke
   budgets, bounded soak, temporary campaign corpus, and artifact discipline
   established by ADR 0018. Keep crypto and DID wrappers/workflows separate.
5. Treat rejection as valid and assert only public representation invariants.
   Do not test curve membership, proof of possession, key authorization, secret
   operations, chain behavior, or product policy in these targets.
6. Keep fuzz-only dependencies in the independent lock. Do not change the
   production API, feature graph, root lock, target support, or consumer trees.

## Consequences

- JWK, COSE, thumbprint, and conversion regressions receive reproducible
  sanitizer coverage before higher protocol crates depend on them.
- A shared `fuzz/**` infrastructure change may trigger both bounded workflows;
  component-only changes remain path scoped.
- Fuzzing remains evidence rather than proof, and findings require minimization
  plus deterministic regression before a production behavior change.
- Continuous hosted fuzzing, structure-aware generation, raw JWK public limits,
  and secret/derivation/signature fuzzing remain separate triggered work.
- No publication, release, repository setting, downstream adoption, Apollo
  deprecation, NeoPRISM reduction, or `main` promotion is authorized.

# ADR 0039: fuzz the bounded JWS Compact boundary

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-06
- **Decision authority:** issue #100 and the IDR-004 roadmap
- **Related work:** issues #8, #95 and #100; ADRs 0018, 0019, 0034 and 0035;
  OpenSpec change `fuzz-jws-compact-boundary`

## Context

JWS Compact is an attacker-controlled representation used by credential and
proof protocols. Its parser already enforces resource bounds, canonical
base64url, a closed header and exact signing input through deterministic tests.
Coverage-guided mutation is the remaining planned assurance layer.

## Decision

1. Extend the standalone fuzz workspace with one `jws_compact` target and no
   production dependency or feature.
2. Interpret every UTF-8 input under default limits and, when possible, derive
   five capped positive limits from ten prefix bytes for a second suffix parse.
   A harness-only `text:` transport removes one repository line ending from
   committed exact-text seeds; a ten-byte `limits:` transport does the same for
   its derived-limit suffix. Arbitrary unprefixed bytes remain raw.
3. Assert exact input/signing-input preservation, independent segment
   canonicality, same-limit reparsing, staged semantic encoding and fixed error
   bridges. Canonical rebuild may minimally widen representation-size limits
   while same-limit reparsing remains exact. Treat rejection as valid.
4. Keep reviewable independently authored RFC/Oxid/Lace, key-reference,
   accepted derived-limit and negative seeds. Reuse
   the pinned Nix/cargo-fuzz runtime and fixed replay/smoke/bounded-soak model;
   soak is local or externally scheduled while reserved `main` stays empty.
5. Upload only failing artifacts; minimize and promote real defects to corpus
   and deterministic tests before merge.
6. Stop at the bounded campaign and existing full gates. Cryptographic
   verification, proof semantics and broader continuous fuzzing stay separate.

## Consequences

- JWS representation invariants receive repeatable sanitizer-backed regression
  search without widening SDK APIs or compile graphs.
- Shared `fuzz/**` changes conservatively trigger the existing DID and crypto
  campaigns as well as JWS; source-only JOSE work triggers only its own lane,
  while crypto source changes trigger both crypto and dependent JWS campaigns.
- The campaign reduces risk but cannot prove absence of defects.

## Rejected alternatives

- **Fuzz cryptographic verification together:** expands authority, dependencies
  and triage surface beyond the codec.
- **Use only defaults:** misses caller-selected limit interactions.
- **Use a structured-generator dependency:** adds complexity before byte-level
  mutation evidence demonstrates a gap.
- **Run unbounded PR fuzzing:** makes delivery nondeterministic and slow.

## Rollback

Remove the target, seeds, dictionary, wrapper/workflow and contract evidence.
No public API, persisted data, downstream repository or protected branch needs
migration.

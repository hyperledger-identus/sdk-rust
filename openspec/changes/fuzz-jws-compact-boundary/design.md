## Context

`UnverifiedCompactJws::parse` accepts a bounded `&str`, validates exactly three
canonical base64url segments, parses a closed protected header, preserves the
exact received signing input and returns static `JoseError` variants. Callers
can also construct a canonical `JwsSigningInput` and attach bounded signature
bytes. The boundary is safe Rust but processes attacker-controlled tokens and
caller-controlled positive `JwsLimits`.

Immutable authorities and tooling are already pinned by the repository:

| Source | Pin | Use |
| --- | --- | --- |
| RFC 7515 | `https://www.rfc-editor.org/rfc/rfc7515` | JWS Compact and signing input |
| RFC 8725 | `https://www.rfc-editor.org/rfc/rfc8725` | JWT/JWS defensive guidance |
| Rust toolchain | nightly `2026-03-18` | sanitizer-capable compiler |
| Nixpkgs | `7241bcbb4f099a66aafca120d37c65e8dda32717` | cargo-fuzz `0.13.2` |
| libfuzzer-sys | exact `0.4.13` | runtime binding |

The RFC example and the existing independently reconstructed Oxid/Lace tests
provide semantic shapes. No donor source or fixture is copied. Apollo has no
JWS Compact parser to retain; NeoPRISM, Oxid and Lace remain consumers rather
than owners of the generic codec.

## Goals / Non-Goals

**Goals:**

- Search arbitrary bytes under AddressSanitizer while treating rejection as a
  valid outcome.
- Exercise both SDK defaults and many bounded positive limit combinations.
- Detect alternate base64url acceptance, exact-signing-input drift, accepted
  value instability, dynamic error detail and resource-boundary defects.
- Keep PR evidence deterministic and the longer campaign time-boxed.
- Preserve a small independently reviewable corpus and promote real findings
  to deterministic regressions.

**Non-Goals:**

- Cryptographic verification, algorithms, keys, proof claims, JSON JWS, JWE,
  network/state behavior, trust, custody, product policy, new public limits,
  OSS-Fuzz, release or exhaustive-security claims.

## Decisions

### Decision 1: extend the independent fuzz workspace with one cohesive target

The existing standalone `fuzz/` workspace adds `identus-jose` by path and exact
`base64 0.22.1` for independent segment canonicality checks. Neither dependency
enters a published crate or the root lock. One `jws_compact` target is enough:
structure, header, limit and round-trip invariants all meet at one parser call,
and splitting them would duplicate decoding and campaign cost.

### Decision 2: run every UTF-8 input under defaults and a second bounded tuple

The complete byte slice is interpreted as JWS text under SDK default limits
when it is UTF-8. Committed exact-text seeds may use a harness-only `text:`
prefix; that route removes one repository line ending before parsing so Git
text files remain exact examples. Unprefixed bytes are never normalized. When
at least ten bytes exist, five little-endian `u16` values
select positive limits and the remaining bytes are independently interpreted as
JWS text. Modulo mapping caps the derived compact/header/payload/signature/string
limits at 8,192/4,096/4,096/1,024/2,048 bytes.

This dual interpretation keeps committed seeds plain and reviewable while raw
mutations explore configuration space. It never decodes invalid UTF-8 lossily.
Automation permits input up to 131,072 bytes so mutations can cross the default
65,536-byte complete-token bound; parser/harness work remains inside five-second
per-input and one-GiB process ceilings.

### Decision 3: assert only public, policy-neutral invariants

Rejection is valid. Every accepted value must:

- preserve the exact compact text and its prefix through the second period as
  `signing_input()`;
- contain three segments that independently decode and re-encode to the exact
  same unpadded base64url text;
- reparse to an equal public value under the same limits; and
- rebuild through `JwsSigningInput` plus the original payload/signature, then
  parse to the same protected-header, payload and signature semantics.

Every parser error must be one of the static codec boundary variants and map to
its fixed `jose.*` code/capability. Assertions and custom panic messages never
contain input, payload, signature, header or identifier bytes.

### Decision 4: reuse the established deterministic campaign envelope

`scripts/fuzz-jws.sh` owns selection and all libFuzzer flags. Replay uses the
committed corpus once. PR/push smoke uses seed `424242`, one worker and 4,096
runs. Scheduled/manual soak uses at most 300 seconds. All modes use exact
cargo-fuzz `0.13.2`, no corpus reload, a five-second execution timeout, a
one-GiB RSS limit and a temporary writable corpus copy.

The path-scoped Ubuntu workflow runs formatting, strict Clippy, cargo-deny and
RustSec against the independent lock, then smoke or soak. Failure artifacts are
retained for 14 days as untrusted input. A real defect is reproduced, minimized,
committed as a seed and named deterministic regression, then fixed or filed.

## Threat Contract

**Assets:** parser availability, allocation bounds, canonical representation,
exact signing input, static diagnostics, explicit unverified state and reusable
consumer compatibility.

**Threats addressed:** malformed structure; padded, whitespace, standard-base64
or trailing-bit aliases; duplicate/unknown/ambiguous/private headers; malformed
UTF-8/JSON; empty required segments; limit arithmetic and encode/parse drift;
panic, sanitizer failure and excessive per-input work.

**Residual risk:** bounded fuzzing is probabilistic. It does not verify a
signature, authorize a key, validate a proof profile or prove absence of bugs.
Compiler/platform-specific findings require minimization and deterministic
reproduction before compatible production behavior changes.

## Performance and effort

The implementation target is one small harness, one script and one workflow,
reusing all runner/toolchain infrastructure. Smoke elapsed time and execution
count are recorded but no host threshold is introduced. The stop point is a
green 4,096-run campaign plus existing full gates; broader structure-aware or
continuous fuzzing becomes a follow-up instead of polishing this slice beyond
the requested 70–80 percent maturity.

## Migration and Rollback

No consumer or data migration exists. Remove the target, corpus, dictionary,
wrapper/workflow and contract evidence to roll back. Production packages,
public APIs and `main` remain unchanged.

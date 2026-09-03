## Context

`Did` accepts at most 2,048 bytes and `DidUrl` at most 4,096 bytes. Both are
owned validated strings whose accessors borrow cached byte ranges. The parser
is safe Rust and rejects non-ASCII grammar, but hostile inputs can still expose
logic panics, inconsistent validation paths, or invalid cached ranges. Coverage-
guided mutation is the smallest reusable assurance increment after the fixed
conformance suite.

Normative and tooling sources are pinned as follows:

| Source | Immutable pin | Use |
| --- | --- | --- |
| W3C DID Core 1.0 Recommendation | <https://www.w3.org/TR/2022/REC-did-core-20220719/> | normative DID/DID URL grammar |
| W3C editor source | `w3c/did-core@a2bb463373369ad96a50e241c11e871eef84deb3` | inspected drift/source evidence |
| W3C DID test suite | `w3c/did-test-suite@939b31d07d5b1699340ac0702ec0fa46ffcdef0a` | example/conformance evidence; no copied code |
| Rust toolchain | nightly `2026-03-18` in `nix/rust-toolchain.nix` | sanitizer-capable compiler |
| cargo-fuzz | `0.13.2` from `NixOS/nixpkgs@7241bcbb4f099a66aafca120d37c65e8dda32717` | reproducible runner |
| libfuzzer-sys | exact `0.4.13` | libFuzzer runtime binding |
| Rust Fuzz Book / LLVM libFuzzer docs | live guidance inspected 2026-09-03 | CI, run, dictionary and artifact semantics |

The Rust Fuzz Book recommends small CI smoke runs and failure-artifact upload.
LLVM defines fixed `-runs`, random `-seed`, bounded `-max_total_time`, input
length, timeout, RSS, dictionary, and artifact controls. The committed wrapper
selects those controls so local and hosted invocations do not drift.

Read-only evidence was inspected at Apollo `ccee22b`, NeoPRISM `d6ad1ec`,
midnight-identity `427f857`, Lace `804de0a`, and Oxid `bfe3b48`. Only
midnight-identity has directly relevant generative tests: bounded fast-check
properties for Midnight-specific off-chain state. That model stays downstream;
the SDK adapts only the principle of bounded generated invariants. No donor
source or fixture is copied. Exact status/license receipts are in issue #35.

## Goals / Non-Goals

**Goals:**

- Search arbitrary hostile byte combinations under sanitizer instrumentation.
- Exercise both rejection and every existing public accepted-value path.
- Detect invalid borrowed component ranges through public observations only.
- Make the fast campaign fixed and repeatable; keep longer time-boxed search
  separate from ordinary PR latency.
- Keep fuzz-only dependencies out of all published crates and target builds.

**Non-Goals:**

- DID documents/results, URI equivalence, method/state semantics, resolution,
  networks, storage, cryptography, trust, custody, product behavior, FFI,
  downstream migration, OSS-Fuzz, publication, or release.
- A proof that no bug exists or an unbounded continuous campaign.

## Decisions

### Decision 1: use an independent cargo-fuzz workspace

The root `fuzz/` directory is a standalone Cargo workspace. It depends on
`identus-did` by path, exact `libfuzzer-sys 0.4.13`, and only the serialization
support needed to exercise the existing public serde surface. No fuzz feature,
runner, sanitizer, or dependency enters a published package.

Two targets keep failures attributable: `did` exercises bare identifiers and
`did_url` exercises URL components. Each receives arbitrary bytes. Non-UTF-8
is returned immediately because the public boundary is `&str`; valid UTF-8,
including non-ASCII rejection and values above production limits, reaches the
parser.

### Decision 2: assert semantic invariants, not parse success

Rejection is always a valid outcome. For acceptance, a target asserts exact
`as_str`/Display spelling, `TryFrom<String>` and `FromStr` equality, serde JSON
round-trip equality, and valid non-empty DID components. The DID URL target also
asserts `as_did_str`/`to_did`, path/query/fragment delimiter-aware exact
reconstruction, and that every borrowed component occupies an in-bounds range
of the one owned string. The harness does not decide method validity.

### Decision 3: cross both production limits within a small resource envelope

Automation caps generated inputs at 8 KiB, crossing both 2 KiB/4 KiB parser
limits while bounding harness allocation. Each input has a five-second timeout
and the process a 1 GiB RSS ceiling. PR/push smoke uses one worker, a committed
seed and 4,096 runs per target. Scheduled/manual soak uses at most 300 seconds
per target under the same input/resource ceilings.

Throughput and wall time are evidence only. Machine-specific executions per
second never become an acceptance threshold.

### Decision 4: commit curated grammar seeds, not generated campaign growth

Small original corpora cover W3C example syntax, PRISM, Midnight, web and key
shapes, URL delimiter combinations, percent escapes, Unicode rejection, empty
or truncated structures, and exact/over-limit sizes. Dictionaries contain
grammar tokens rather than method semantics. Campaign-created corpus growth is
not committed automatically.

On a failure, automation uploads `fuzz/artifacts`. A maintainer or agent runs
`cargo fuzz tmin`, promotes the minimized input into the committed corpus, and
adds a named deterministic unit regression when the defect is distinct and can
be expressed clearly. Artifacts may contain hostile or sensitive bytes and are
not treated as safe for logs or publication.

### Decision 5: Nix owns installation; one script owns invocation

The locked Nix development shell exposes `cargo-fuzz 0.13.2`. The wrapper script
checks the target/mode and supplies every libFuzzer limit. `replay` runs only
the committed corpus inputs, `smoke` performs fixed-run mutation, and `soak`
performs the time-boxed campaign. CI invokes the wrapper through `nix develop`.
This avoids runtime downloads and prevents YAML/local option drift.

The focused workflow runs only on Linux because cargo-fuzz requires sanitizer
support on Unix x86-64/AArch64, while the existing Nix matrix continues to
prove ordinary macOS/Linux portability. Paths constrain PR/push activation;
schedule and manual dispatch exercise both targets independently.

## Threat Contract

**Assets:** parser availability, component-range integrity, exact identifier
spelling, constructor/serde agreement, dependency isolation, and downstream
method ownership.

**Threats addressed:** panic/abort on hostile text, delimiter state confusion,
truncated percent escapes, invalid offset slicing, Unicode boundary errors,
oversized input work, and disagreement among accepted construction paths.

**Residual boundaries:** fuzzing is probabilistic and bounded. It does not
authenticate an identifier, validate a DID method, establish equivalence,
resolve a document, authorize a key, or make later display/network contexts
safe. Sanitizer findings may still be platform/compiler-specific and require a
deterministic regression before a parser behavior change is accepted.

## Migration and Rollback

No consumer or persisted-data migration exists. Local contributors enter the
Nix shell and use the wrapper; existing production commands are unchanged.
Rollback is a focused PR revert. No released crate, consumer repository, chain
state, or reserved `main` branch is affected.

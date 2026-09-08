# ADR 0086: retain the self-contained DID parser after dependency parity

- **Status:** Accepted
- **Date:** 2026-09-08
- **Decision authority:** sdk-rust issue #159 under #151 and DID epic #5
- **Supersedes:** no ADR; confirms ADR 0008 with new dependency evidence
- **Research:** OpenSpec `decide-did-url-parser-parity` and the
  [reproducible harness](../research/rust-library-reuse/did-url-parser-parity/README.md)

## Context

The SDK should reuse cohesive standards crates where doing so removes risky
mechanics without coupling public domain types to upstream models. ADR 0008
nevertheless retained a local DID/DID-URL parser because it had explicit
resource, exact-representation and facade requirements. Issue #159 tested
whether exact `did_url_parser 0.3.0` now offered a smaller, safer engine.

The candidate is focused, pure Rust, `no_std + alloc`, dual licensed and only
adds itself, `form_urlencoded` and `percent-encoding` to a minimal normal graph.
It passes its 19 tests and all three SDK portable compile targets on Rust
1.98.1. Common W3C, NeoPRISM, Midnight, Lace and Oxid-shaped inputs agree.

Material incompatibilities remain:

1. Validation trims surrounding ASCII controls/whitespace while storage keeps
   the untrimmed input. Such values parse successfully but have shifted or
   trailing component slices and retain a serialization outside DID syntax.
2. The parser accepts terminal-colon method-specific identifiers and 22
   `%+<hex>` spellings because integer parsing accepts a leading plus; DID Core
   and RFC 3986 require a final `idchar` and two literal HEXDIG characters.
3. It has no 2 KiB/4 KiB ceiling and allocates before validation.
4. `TryFrom<String>` copies rather than retaining the supplied allocation.
5. Its public unchecked setters can create a `DID` rejected by its own parser.
6. Its relative-join path contains `unsafe { from_utf8_unchecked(...) }`, and
   its form-query API is outside the SDK's generic lexical boundary.

The pinned comparison found 7 mismatches in 30 curated cases and 95 mismatches
across 17,284 generated comparisons. A private wrapper cannot repair these
without retaining local prevalidation, copying and facade mechanics, which
would add a dependency while failing to remove the code it was meant to replace.

## Decision

1. Retain the dependency-free parser and public values governed by ADR 0008.
2. Do not add `did_url_parser 0.3.0` to any production or development SDK
   manifest. Keep it only in the isolated, unpublished research harness.
3. Treat the candidate as `retain-local`, not as a conformance oracle: its
   known accepted-language errors make it unsuitable as an independent truth
   source for those boundaries.
4. Preserve the harness and attributable corpus so a future release can be
   evaluated without rediscovering the decision criteria.
5. Do not copy or fork candidate source. The current parser is smaller for the
   consumed capability (455 lines in one file versus 1,042 candidate source
   lines including unused mutation/query/join behavior) and already has SDK
   fuzz/resource/error evidence.

## Consequences

- The SDK continues to own a small security-sensitive parser, but its behavior
  stays explicit, bounded, fuzzed and independent of form/query semantics.
- No production lockfile, public API, serialized value, error or target changes.
- The research harness has a separate 26-package lock because it links both
  implementations; it cannot enter SDK release artifacts.
- Candidate strict Clippy fails on three Rust 1.98 lifetime-syntax warnings.
  This is maintainability evidence, not the primary correctness blocker.
- Candidate normal-cone audit is clean. Its published development lock fails
  current audit because old `proptest` reaches vulnerable `rand 0.7.3`; this
  does not imply a runtime vulnerability but weakens source-test reproducibility.

## Reconsideration trigger

Evaluate a new exact release only if it:

- rejects surrounding controls/whitespace, terminal-colon identifiers and all
  non-HEXDIG percent spellings;
- offers caller-enforced pre-allocation limits or a borrowed parser-only result;
- permits immutable Identus facades and owned-string reuse without a second copy;
- removes or isolates unsafe relative joining and makes form-query behavior
  optional or absent;
- passes the committed corpus, strict Rust 1.98 lint, portable targets and
  current advisory checks.

Even then, production adoption requires a new issue and specification. Upstream
patching is not authorized by this decision.

## Rollback

Revert the issue #159 decision PR. No runtime or data migration is involved;
ADR 0008 and the production implementation remain unchanged either way.

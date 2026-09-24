# ADR 0137: permit append-only OID4VCI error contract evolution

- **Status:** Accepted for implementation
- **Date:** 2026-09-25
- **Decision authority:** issue
  [#346](https://github.com/hyperledger-identus/sdk-rust/issues/346)
- **Related:** ADR 0119 and issue
  [#345](https://github.com/hyperledger-identus/sdk-rust/issues/345)
- **Applies to:** `identus-oid4vci` only

## Context

ADR 0119 froze an independently captured 171-row pre-refactor golden and added
a wildcard-free exhaustive router. The immutable fixture correctly protects
all original contracts, but a crate-local test equates the complete live
inventory with that historical snapshot. This accidentally forbids every
future additive error even when all prior contracts remain exact.

Rewriting the fixture would let new implementation state bless its own
history. Creating a complete versioned snapshot for every feature would add
large ceremony without improving protection of existing consumers.

## Decision

The 171 v1 variants remain the exact ordered prefix of the live exhaustive
inventory. Their fixture, hash, provenance, discriminants, codes, kinds,
messages and conversions remain immutable. Every future variant and router
entry appends after the live inventory; the complete inventory remains unique.

The router remains wildcard-free, so an unmapped variant fails compilation.
Each feature independently tests every appended constant, code, kind, message,
conversion and redaction property. The v1 fixture remains historical
compatibility evidence and is never presented as coverage for the new suffix.

## Consequences

- Existing consumer-visible contracts and discriminants remain exact.
- OID4VCI can grow explicit errors without rewriting independent evidence.
- Feature PRs own tests for their suffix; the v1 golden protects only its
  stated 171-row baseline.
- Insertions or reordering within the baseline fail exact-prefix validation.
- Duplicate live entries fail complete uniqueness validation.

## Alternatives rejected

Rewriting the golden destroys its independence. Removing inventory validation
loses prefix and uniqueness evidence. A wildcard router hides omissions.
Reusing semantically inaccurate old errors avoids governance work at the cost
of a misleading public API.

## Verification and rollback

Verification requires the unchanged fixture SHA-256, exact 171-name prefix,
complete live uniqueness, wildcard-free compilation, focused crate tests,
factory/OpenSpec, strict formatting/Clippy/docs and hosted CI. Rollback restores
the permanent live-inventory ceiling and removes this decision; it changes no
runtime behavior in either state.

# Specify the A1 shared compatibility foundation

## Why

The platform-core roadmap correctly places shared compatibility evidence before
peer DID, SD-JWT, DIDComm, the portable agent runtime, and language-SDK
adoption. Its A1 row is still too coarse for independent agents: one issue
mixes vector authority, adapter compatibility, consumer changes, and quality
evidence, while the first consumer canary is listed as if it were foundation
infrastructure.

Without versioned machine contracts, later agents could copy donor tests as
normative truth, let TypeScript DTOs constrain Rust, omit consumer-visible
changes, or claim a quality class from an unrelated test. A1 therefore needs a
small dependency graph and explicit schemas before implementation.

## What changes

- Make issue #504 and milestone 5 the single A1 delivery boundary.
- Use four sub-issues for four cohesive contracts: source/vector provenance
  (#420), Rust-to-language DTO and error mappings (#505), the consumer change
  ledger (#422), and risk-routed quality evidence (#501).
- Define the required records, identifiers, references, validators, seed DID
  packet, and completion receipts for those contracts.
- Classify pinned SDK-TS, SDK-Swift, SDK-KMP, Apollo, NeoPRISM, SDK-Rust, and
  official-standard evidence without promoting copied consumer tests to
  normative authority.
- Keep the SDK-TS DID canary (#492) downstream and blocked by all four A1
  contracts; it proves adoption but is not part of the foundation itself.
- Update the platform-core roadmap to use the parent issue and the explicit
  dependency graph.

## Capabilities

### Added capabilities

- `cross-language-compatibility-foundation`: govern language-neutral vectors,
  canonical Rust-to-language mappings, consumer-visible changes, and
  risk-routed quality evidence as linked versioned records.

### Modified capabilities

None. This change refines the delivery plan around the already accepted
platform-core migration rules without weakening them.

## Non-goals

This change does not add a protocol engine, DID method, runtime, binding API,
SDK-TS code, default switch, release, dependency, or support claim. It does not
copy donor fixtures into production tests. It specifies A1 and its first DID
seed packet; implementation remains in the four child issues.

## Delivery

Issue #504 owns the planning packet and parent closure. Issues #420 and #505
may start independently. Issue #422 depends on both so ledger records can
reference stable vector and adapter identifiers. Issue #501 may design in
parallel but its first concrete declaration uses #420's DID packet. Issue #492
is blocked by all four and remains the first downstream adoption proof.

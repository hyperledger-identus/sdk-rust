# ADR 0173: sequence A1 through versioned evidence contracts

- **Status:** Accepted as implementation plan
- **Date:** 2026-09-30
- **Decision authority:** milestone
  [#5](https://github.com/hyperledger-identus/sdk-rust/milestone/5) and issue
  [#504](https://github.com/hyperledger-identus/sdk-rust/issues/504)
- **Related:** ADRs 0164, 0169, and 0170; issues #420, #422, #492, #501,
  and #505
- **Constraint impact:** material; fixes A1 ownership and dependency order
  without changing any runtime or consumer

## Context

The accepted platform-core roadmap puts shared compatibility evidence before
feature migration. A1 still bundled four concerns and listed its first SDK-TS
canary beside foundation work. That shape is difficult for autonomous agents:
parallel changes contend on one issue, identifiers appear too late, and a
passing donor test can be mistaken for normative authority.

Pinned discovery also shows that SDK-TS, SDK-Swift, and SDK-KMP repeat DID
parser examples. Agreement is useful consumer evidence, but copied tests do
not become a standard through repetition. Likewise, language DTOs and errors
must remain migration inputs rather than canonical Rust architecture.

## Decision

A1 has one milestone parent, #504, and exactly four cohesive implementation
contracts:

- #420 owns vector sources, authority, provenance, immutable payloads, and the
  first DID packet;
- #505 owns mappings from canonical Rust DTO/error contracts to versioned
  language compatibility shapes;
- #422 owns the consumer-visible change ledger and depends on #420 and #505;
- #501 owns risk-routed property, fuzz, benchmark, and differential evidence.

Issues #420 and #505 may proceed in parallel. #501 may design concurrently but binds
its first concrete declaration to #420's DID packet. #422 consumes stable IDs
from #420 and #505. The downstream SDK-TS DID canary #492 is blocked by all
four and is not an A1 sub-issue.

Records are versioned separately and linked by stable IDs. One owner stores a
payload; other records reference it. Offline validators reject duplicate IDs,
dangling references, unpinned sources, missing authority or redistribution
decisions, invalid dependency order, and undeclared quality outcomes. GitHub
sub-issues and blocked-by relationships are the hosted work graph; concise
checklists are human summaries, not a duplicate backlog.

The first packet is bounded DID/DID URL syntax and redacted errors. Peer DID,
Prism operations, credential formats, DIDComm, runtimes, and bindings stay in
later milestones. A1 planning and infrastructure cannot mutate SDK-TS.

## Consequences

- Agents can deliver two independent roots and two dependent records without
  editing the same capability boundary.
- Standards, Identus profiles, consumer regressions, implementation tests, and
  exploratory cases remain distinguishable.
- Rust contracts stay canonical while language migration remains possible.
- Four small schemas and validators add governance work, but prevent every
  later capability from inventing incompatible evidence conventions.
- A1 completion is an infrastructure claim only; adoption begins in #492.

## Alternatives rejected

- One umbrella issue and schema: high coupling and noisy ownership.
- One issue per donor repository: organizes by source rather than capability
  and duplicates decisions across languages.
- Let SDK-TS fixtures and DTOs define the contract: contradicts ADRs 0169 and
  0170 and preserves historical component boundaries.
- Implement the canary first and backfill evidence: makes the first consumer a
  de facto schema and bypasses the accepted dependency order.

## Verification and rollback

The planning PR must contain a validated OpenSpec change and machine-readable
delivery graph whose parent, children, blockers, artifacts, and stop boundary
match GitHub. Each child closes with schema, validator, mutation tests, seed
records, documentation, and an evidence comment. #504 closes only after the
combined graph validates.

Before implementation, rollback removes this planning slice. After records
exist, a superseding ADR and schema migration must preserve or explicitly map
stable IDs; provenance cannot be silently rewritten.

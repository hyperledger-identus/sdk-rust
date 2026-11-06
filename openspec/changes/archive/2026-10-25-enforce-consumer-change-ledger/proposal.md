# Enforce the consumer-visible change ledger

## Why

The A1 vector catalog, Rust-to-language adapter mappings, and quality-evidence
registry now have stable, validated identifiers. The existing platform change
ledger is still a schema-v1 comment block with no executable contract. A later
consumer migration could therefore omit a compatibility decision, cite stale
or nonexistent evidence, or produce release and migration prose that diverges
from the reviewed change.

Issue #422 is the final A1 child needed to make those records usable together.
It must preserve an honest empty canonical ledger because this governance slice
does not change runtime behavior, while proving the full cross-record contract
with a clearly synthetic DID migration fixture.

## What changes

- Upgrade `docs/architecture/identus-platform-change-ledger.toml` to a closed,
  independently versioned schema-v2 registry with an explicit empty state.
- Add strict offline validation for change classification, compatibility
  dimensions, affected packages and consumers, version windows, migration,
  deprecation, legacy-bug treatment, observability, fallback, rollback,
  removal gates, exact evidence, and immutable lifecycle links.
- Resolve vector, adapter-mapping, and quality-declaration identifiers from the
  three delivered A1 registries without copying their payloads or treating a
  historical quality receipt as current promotion evidence.
- Generate deterministic Markdown sections for release notes, migration
  actions, compatibility windows, rollback, and an explicit empty-ledger
  state.
- Add a machine-readable compatibility-impact declaration to qualifying
  OpenSpec changes so the factory can require ledger IDs for a
  consumer-visible change and a bounded rationale for behavior-neutral work.
- Prove the complete contract with mutation tests and a noncanonical synthetic
  DID record referencing the #420 packet, #505 mappings, and
  `did.syntax.quality.v1` from #501.

## Capabilities

### Added capabilities

- `consumer-change-ledger`: classify consumer-visible changes and derive
  deterministic migration and release evidence from validated A1 records.

### Modified capabilities

- `cross-language-compatibility-foundation`: make the planned ledger contract
  and behavior-neutral factory disposition executable.

## Non-goals

This change does not migrate SDK-TS, SDK-Swift, or SDK-KMP; change a Rust public
API, wire format, error, persistence model, target, or runtime requirement;
activate a deprecation; authorize a release; evaluate promotion freshness; or
make a synthetic record canonical. It does not infer compatibility from diffs
or language DTO shapes.

## Delivery

Issue #422 owns this change under parent #504 and milestone A1. Its delivered
dependencies are #420, #505, and #501. Completion unblocks A1 closeout #504;
downstream canary #492 remains separately owned and must not be changed here.

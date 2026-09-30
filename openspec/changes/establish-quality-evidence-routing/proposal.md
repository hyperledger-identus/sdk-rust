# Establish quality evidence routing

## Why

SDK-Rust already runs property-shaped tests, sanitizer fuzz campaigns,
measurement-only benchmarks, and differential/conformance packets, but those
assets are selected independently by each slice. A generic CI link or an
unreviewed omission can therefore be mistaken for sufficient evidence, while
moving every expensive campaign into pull-request CI would make the active
development line unnecessarily slow.

Issue #501 establishes the third A1 compatibility-foundation contract: a
versioned declaration for the property, fuzz, benchmark, and differential
obligations of each delivered capability. The contract records exact commands,
selectors, targets, lane/cadence, freshness, budgets, ownership, receipts, and
visible evidence debt without making any one technique normative authority.

## What changes

- Add a closed TOML registry for capability-level quality declarations and all
  four quality classes.
- Add an offline validator and mutation suite for class completeness, exact
  evidence locators, lane/cadence routing, freshness, debt ownership, and
  cross-language vector references.
- Add a deterministic quality-plan rendering grouped by focused, fast, slow,
  and release routes and expose it through the factory facade.
- Seed the registry with generic DID/DID URL syntax evidence from the existing
  Rust tests, DID fuzz campaign, and #420 vector packet; explicitly justify why
  a DID parser benchmark is not applicable to this A1 infrastructure slice.
- Add stable declaration-ID references to the issue and evidence templates so
  later changes point to the registry instead of copying free-form plans.

## Capabilities

### Added capabilities

- `quality-evidence-declarations`: declare and validate risk-based engineering
  evidence independently from standards authority and release approval.

### Modified capabilities

None. Existing test, fuzz, benchmark, conformance, support-policy, and CI
contracts retain their current owners.

## Non-goals

This change does not invent a new property/fuzz/benchmark framework, execute
every slow campaign on each pull request, define product performance SLOs,
change Rust/target support, certify a component, authorize release, or mutate a
language SDK. It does not make donor implementations normative.

## Delivery

Issue #501 owns this change under parent #504 and milestone A1. It consumes the
stable #420 DID packet identifiers. Issue #422 will consume declaration IDs;
SDK-TS canary #492 remains blocked until all A1 contracts close.

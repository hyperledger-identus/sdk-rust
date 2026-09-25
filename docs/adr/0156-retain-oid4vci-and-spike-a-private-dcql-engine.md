# ADR 0156: retain OID4VCI and spike a private DCQL engine

- **Status:** Accepted under standing material authority; production adoption deferred
- **Date:** 2026-09-26
- **Issue:** [#391](https://github.com/hyperledger-identus/sdk-rust/issues/391)
- **Review no later than:** before activating IDR-024 or adding an OID4VP production dependency

## Context

The SDK has mature bounded OID4VCI 1.0 Final behavior but no accepted OID4VP
engine. Current Rust choices range from broad OID4VC frameworks to one narrow
DCQL query engine. Replacing local OID4VCI code because a framework advertises
the same protocol would discard explicit resource, error, state, and transport
boundaries without proving parity. Implementing every OID4VP selection
mechanic locally would also ignore a potentially cohesive reusable component.

The exact research matrix, provenance, dependency counts, and unrun checks are
recorded in the issue #391 OpenSpec research. The decisive candidate is
`siros-dcql 0.3.0`: a current BSD-2-Clause, pure-Rust engine with a Rust 1.82
floor and a 12-package resolved cone. It also accepts unbounded JSON, exposes
unbounded models and diagnostics, and deliberately tolerates some invalid
identifier and missing-metadata shapes.

## Decision

1. Retain `identus-oid4vci` as the production OID4VCI implementation. No
   assessed framework demonstrates lower-risk bounded parity.
2. Keep Impierce, Spruce, and Affinidi OID4VC implementations as read-only
   conformance oracles. Do not adopt Equs or Credibil now.
3. Keep future OID4VP orchestration, trust, consent, nonce, verification,
   transport, and lifecycle policy in Identus-owned crates and public types.
4. Accept `siros-dcql 0.3.0` only as a research spike. Its separately locked
   fixture proves valid selection, pre-parse byte bounds, error redaction,
   type isolation, dependency size, and known validation mismatches.
5. Production DCQL reuse is `conditional-adopt`, not activated. A separate
   issue must define the consumer and facade, enforce complete structural and
   identifier limits before candidate parsing, map owned credentials/results,
   pass primary/MSRV/WASM/iOS/Android evidence, and update this ADR.
6. No candidate dependency or type enters the root workspace, release graph,
   public API, or fast CI through this decision.

## Consequences

OID4VCI maintenance remains an SDK responsibility, but the decision protects
already-proven Final-profile behavior. The future OID4VP implementation can
reuse a focused DCQL engine if its final adapter remains smaller and safer
than local selection code. Full frameworks continue to inform differential
testing without controlling architecture.

The fixture is evidence, not a transitive dependency or support promise.
Portable targets remain unclaimed until their exact commands run.

## Alternatives rejected

- **Adopt a full OID4VC framework:** current candidates fail one or more of
  release provenance, MSRV, dependency-cone, target, or coupling gates.
- **Replace existing OID4VCI now:** no candidate has bounded behavioral parity
  and measurable deletion benefit.
- **Implement DCQL immediately:** the consumer facade and strict validation
  contract are not yet specified.
- **Treat SIROS models as SDK types:** this would expose unbounded strings,
  vectors, JSON values, tolerant validation, and verifier-controlled errors.

## Verification and rollback

`scripts/check-siros-dcql-spike.sh` runs exact locked tests, strict Clippy,
license/advisory checks, the 12-package cone assertion, and root-graph
isolation. The tests preserve both useful behavior and semantic mismatches.
Rollback removes the fixture, checker, ADR, and research spec without code,
wire, data, release, or downstream migration.

# Design

## Handbook structure

Add one overview chapter under Architecture and three crate reference pages.
The overview starts from adopter intent, shows the dependency/sequence diagram,
walks the deterministic flow, identifies source/store/submitter seams, and
links to the executable E2E test, ADRs, archived contracts, quality baseline,
and downstream parity evidence. Each crate page states responsibility,
dependencies, non-goals, target status, limitations, and relevant API source.

## Diagram

Track one Graphviz DOT source using the established docs build. It shows the
inward dependency chain and, with dashed edges, caller-owned ledger source,
atomic store, cancellation/retry/runtime, and publication submitter. Generated
SVG remains a build artifact.

## Executable E2E scenario

Add a focused integration test in `identus-prism-indexer` because that crate is
the outer portable composition layer. The test:

1. creates and signs a PRISM DID with master and VDR keys;
2. creates a signed VDR entry owned by that DID;
3. maps both records into one complete ordered transaction;
4. plans and atomically commits through an in-memory implementation of the
   existing lookup/receipt/cursor/commit ports;
5. proves DID-document projection and VDR entry resolution;
6. replays the exact transaction and proves no second commit occurs; and
7. applies a second bounded VDR update transaction and proves the new head.

The adapter lives only in test code. It does not become a public convenience
store or conceal the atomic-commit contract.

## Roadmap reconciliation

Keep IDR-040 at `in_progress` under new closeout issue #554. Its outcome and
evidence text state that PRISM and peer profiles are delivered by #521–#523,
while `did:key`, `did:jwk`, and `did:web` require focused future issues. This is
more accurate than either stale `specified` or portfolio-wide `delivered`.

## Verification

Run the focused E2E test, all prism crate tests/all targets, mdBook/Nix docs
check, factory and OpenSpec checks, full workspace/Nix evidence routed by the
quality plan, and an exact-head CLI review. Link validation must cover all new
internal targets; external GitHub availability is not inferred from offline
validation.

# Publish the portable PRISM adoption packet

## Why

Issues #521 and #522 delivered the portable PRISM DID, VDR, and indexer
capabilities, but the public handbook still explains only the original crypto
and generic DID trains. The repository has strong focused tests without one
readable end-to-end scenario that an adopter can use to understand the complete
portable flow and its effect boundaries. The canonical backlog also still
reports IDR-040 as merely specified.

## What changes

- Add handbook reference pages for `identus-prism-did`,
  `identus-prism-vdr`, and `identus-prism-indexer`.
- Add a static dependency/sequence diagram and adopter-oriented walkthrough.
- Add a deterministic E2E test covering DID creation, VDR mutation, ordered
  atomic indexing, exact replay, DID projection, and VDR lookup through
  in-memory ports.
- Reconcile IDR-040 so delivered PRISM/peer profiles are distinguished from
  separately unplanned DID methods.
- Bind the knowledge packet to the governing ADRs, archived specifications,
  quality baseline, and immutable NeoPRISM parity evidence.

## Modified capabilities

- `sdk-documentation-site`: document the portable PRISM stack truthfully.
- `prism-vdr-indexing`: retain one readable executable full-stack scenario.
- `ssi-upstream-program`: report mixed per-profile delivery without claiming
  the complete DID-method portfolio is finished.

## Non-goals

No new wire behavior, Cardano client, persistence implementation, HTTP, CLI,
runtime, downstream mutation, release, publication, or additional DID method.

## Delivery

Issue #554 owns this behavior-neutral A2 closeout slice. The implementation is
SDK-Rust-only and does not merge or mutate the open NeoPRISM adoption PRs.

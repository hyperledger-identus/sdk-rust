## Why

Issue #65 is the recommended bounded follow-up under #5 / #20 (`IDR-005` and
`IDR-006`). The SDK has a mature generic DID model and ports, but the parent
acceptance criterion that NeoPRISM, midnight-identity, Lace ID Portal and Oxid
are each expressible through the current API is recorded mostly as prose and
mixed examples. A named executable compatibility suite makes that boundary
reviewable without changing or building downstream repositories.

## What Changes

- Add four independently authored compatibility cases that exercise the exact
  generic DID shapes required by the pinned consumer revisions.
- Record immutable source revisions and the accepted evidence-only treatment
  of Lace ID Portal in the test module and architecture receipt.
- Prove strict bounded parsing, semantic JSON round trips, typed projections,
  legacy resolution-error migration and multi-method query composition.
- Document the method, chain, transport and product policy that deliberately
  remains downstream.

## Capabilities

### Modified Capabilities

- `did-core`: require a provenance-recorded, four-consumer compatibility suite
  for the generic DID model and query seams.

## Impact

- **Issue:** #65; child of #5 and #20.
- **API:** no public API or wire-format change is planned.
- **Dependencies:** none; test and documentation evidence only.
- **Consumers:** no downstream repository is edited or built.
- **Rollback:** revert the focused test/documentation pull request; no stored
  data, release, chain state or public compatibility commitment is involved.

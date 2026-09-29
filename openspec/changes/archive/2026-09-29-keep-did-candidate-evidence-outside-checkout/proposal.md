# Keep DID candidate evidence outside the checkout

## Why

The first natural weekly run containing the DID candidate matrix failed on both
native hosts. The primary lane wrote its receipt below `artifacts/` in the Git
checkout; the following MSRV lane correctly rejected that now-dirty checkout.
The failure is workflow orchestration, not candidate or compiler behavior.

## What changes

- Write both DID matrix lane receipts below the GitHub runner temporary root.
- Assert explicitly that the primary lane leaves the checkout clean before the
  MSRV lane starts.
- Upload the same attempt-scoped artifact from its new runner-temporary path.
- Extend the offline workflow checker and mutation suite so checkout-local
  output or removal of the clean-tree assertion fails fast.

## Capabilities

### Modified capabilities

- `weekly-slow-evidence`: candidate evidence production must not contaminate
  the source checkout used by a later lane.

## Non-goals

No candidate identity, compiler, package, target, operation, artifact name,
retention, trigger, required PR gate, clean-source policy, workflow dispatch,
rerun, publication, or support claim changes.

## Delivery

Issue #480 owns the repair. Its PR targets protected `develop` and uses normal
required fast CI. Final M5 issue #388 still requires a later natural or
explicitly authorized exact-SHA slow run; this change does not create that
external evidence.

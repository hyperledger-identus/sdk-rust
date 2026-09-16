# Publish bounded delivery metrics

## Why

The factory already defines privacy-bounded v1/v2 records, owner-only storage,
Pi usage harvesting, and an idempotent issue-comment renderer. Publication is
optional, issue-only, and restricted to the caller's current checkout head.
That prevents ordinary post-merge publication and has left four of five
retained historical work-item records invisible to collaborators. Issue #306
turns the product sponsor's direction into an executable dual-receipt contract.

## What changes

- Require the authoritative metric record to be retained locally before one
  allowlisted aggregate is published.
- Publish to the recorded PR by default and fall back to the issue when no PR
  exists; allow an explicit issue target for historical backfill.
- Verify historical records against their exact hosted PR head rather than the
  caller's unrelated current checkout.
- Keep publication idempotent per publisher, schema marker, and target; make a
  failure visible telemetry debt with one bounded retry.
- Backfill the retained records for issues #246, #247, #250, #257, and #259 to
  their existing issues without reading raw Pi session content.

## Capabilities

### Modified capabilities

- `factory-operations`: changes metrics completion from optional issue-only
  publication to local retention plus a bounded PR-or-issue receipt.

## Non-goals

- No prompt, transcript, command/output, identifier, credential, provider,
  model, cost, billing, or local-path publication.
- No raw Pi session replay or upload.
- No product runtime, SDK API, wire format, dependency, target, release, or
  `main` change.
- No requirement that telemetry publication block otherwise safe product
  integration.

## Delivery

Issue #306 owns the change. It targets protected `develop` as the bottom slice
of the approved rolling stack. ADR 0128 records the public-receipt decision.

## Why

The repository declares weekly slow and sanitizer workflows on `develop`, but
GitHub schedules only workflow files on the default branch. At
`develop@fa14193d398333d6fb272cadc323355e717f800c`, reserved `main` is still
the default, so the hosted cadence is intentionally inactive and the existing
policy cannot yet produce truthful weekly evidence.

Issue #276 owns activation. The least-coupled mechanism is to make the already
protected integration branch `develop` the GitHub default while keeping
`main` protected, minimal, non-delivery and non-release. This uses GitHub's
native scheduler and ephemeral read-only workflow token rather than a broad
external dispatch credential or a scheduler shim committed to `main`.

## What changes

- Accept ADR 0120 selecting protected `develop` as the default and scheduled
  workflow branch during the temporary active-development phase.
- Pin the reserved-branch ruleset to `refs/heads/main` before changing the
  default so `main` does not lose its existing protection.
- Harden the `slow` workflow with bounded run concurrency, job timeouts and an
  always-run metadata job that records event, requested/actual SHA, UTC
  timestamps, job conclusions, retention and the immutable run URL.
- Update machine policy, validation tests, target planning, governance,
  factory, architecture and capability specifications from inactive/pending to
  active native weekly/manual evidence.
- Record the organization-enforced seven-day Actions retention ceiling and add
  a read-only live freshness check plus supervisor heartbeat for missed, stale
  or failed scheduled runs.
- Merge the repository change before mutating the protected settings; then
  change the ruleset/default branch, manually dispatch the merged workflow and
  retain the first native scheduled run as the completion receipt.

## Capabilities

### Added capabilities

- `weekly-slow-evidence`: defines native scheduling, immutable run evidence,
  bounded execution, retention, liveness monitoring and recovery.

### Modified capabilities

- `sdk-support-policy`, `factory-operations`, `code-health-governance`,
  `nix-tooling`, `apollo-crypto-parity`, `browser-did-bindings`,
  `native-did-bindings`, `crypto`, `did-core`, and `jws-compact`: replace the
  issue-#276 pending-hosted qualifiers with the activated default-branch
  contract while preserving the single required `fast` PR lane.

## Non-goals

- No slow, fuzz or sanitizer gate becomes required for ordinary pull requests.
- No publication, release, promotion, direct push or ordinary delivery from
  `main` is enabled.
- No external PAT, GitHub App, publishing credential or workflow write token
  is introduced.
- No runtime SDK, public API, wire model, dependency, compiler or support-tier
  behavior changes.
- The activation canary does not itself satisfy the first naturally scheduled
  weekly-run receipt.

## Delivery

Issue #276 owns a two-receipt infrastructure slice. The first PR targets
`develop` and uses `Refs #276`; after it merges, the sponsor-authorized live
settings transition and manual canary execute. Issue #276 remains open until a
natural scheduled run and the follow-up dated settings/run receipt are merged.

# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/306
Constraint blockers: none

## Existing entries affected

- ADR 0108: exact-head metrics remain private-authoritative; only bounded
  aggregates may be public.
- ADR 0127: telemetry publication follows the one-review/one-remediation flow
  and must not create another product integration gate.
- `SDK-SUPPLY-001`: no dependency, action, credential, or cache authority is
  added.

## Introduced or changed constraints

Every completed production-ready factory work item SHALL retain one canonical
private metrics record and attempt one public allowlisted receipt. The default
target SHALL be its recorded PR, with issue fallback when no PR exists and an
explicit issue override for issue-centric or historical reporting. Historical
publication SHALL verify the exact hosted PR head and repository identity.

The public derivative SHALL contain no content-bearing Pi or agent data. One
bounded retry is permitted. Persistent publication failure SHALL remain visible
telemetry debt but SHALL NOT invalidate independently green product evidence.

## Introduced or changed limitations

GitHub availability and authenticated comment permission are external. A
public receipt can therefore lag the local authoritative record. Private-store
retention is 90 days and pruning remains explicit; this change does not create
an indefinite telemetry archive or a remote backup of raw data.

## Consumer and product impact

No SDK consumer behavior changes. Maintainers gain comparable public delivery
evidence without receiving prompts, transcripts, credentials, identities,
commands, raw output, provider/model data, costs, billing data, or local paths.

## Activation and rollback

Activation requires closed-schema/mutation tests, privacy review, factory and
Nix gates, signed/DCO commits, protected PR checks, and merge to `develop`.
Rollback restores optional issue-only publication while preserving all local
records and already-public versioned comments.

## Evidence

Authority is issue #306 and the recorded user direction. Technical evidence is
ADR 0108, existing v1/v2 schemas and markers, the supervisor privacy contract,
the retained exact-head inventory, and focused tests required by this change.

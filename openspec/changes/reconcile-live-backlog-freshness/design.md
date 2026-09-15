## Context

The factory deliberately separates deterministic repository validation from
external coordination. The SSI checker is therefore the wrong place to embed
GitHub access, but the supervisor needs a reliable gate between roadmap
selection and worker launch.

## Decisions

### Keep structural and live validation separate

The existing Python checker remains network-free. A sibling live checker first
reuses its CSV validation, then resolves only unique referenced issue numbers.
`scripts/factory backlog-live` exposes it as an explicit operation.

### Use a strict reproducible snapshot boundary

Production mode queries `gh issue view` with explicit repository identity and
requests only `number`, `state`, and `url`. Test/replay mode accepts a bounded
JSON snapshot with exact schema, repository identity, unique positive issue
numbers and `OPEN`/`CLOSED` states. Extra fields, duplicates, wrong repository,
missing issues and malformed JSON fail.

### Treat only `in_progress` as active execution ownership

Every referenced issue must exist and be visible. An `in_progress` row must
reference an open issue. `specified` means a reusable contract exists but does
not itself authorize selecting a closed delivery child for implementation;
moving it back to active work requires a new focused issue and status change.
`delivered` evidence may be closed, and queued/conditional rows continue to
use the open program issue.

### Repair ownership without inventing implementation scope

`IDR-004` moves to audit issue #286, which will decide completion or create
bounded missing slices. `IDR-023` moves to open component epic #7 until the
next bounded child is crystallized. This avoids falsely marking either row
delivered.

## Risks and mitigations

- GitHub outage or auth drift: fail the explicit live audit without affecting
  deterministic fast CI.
- Excessive API traffic: deduplicate issue numbers and cap input through the
  fixed 30-row canonical ledger.
- Issue state mistaken for completion: document that the audit proves only
  visibility and active ownership.
- Shell injection: pass repository and issue arguments as argv.
- Snapshot forgery: snapshots are test/replay evidence only and are clearly
  reported as such; live selection uses the default GitHub mode.

## Rollback

Revert the command, tests, documentation, CSV owner updates and platform alias
replacement. No SDK data, consumer state or external configuration changes.

# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

`scripts/factory-tools/metrics.mjs` validates closed v1/v2 records, writes them
under the Git common directory, renders allowlisted summaries, and publishes an
owned marker comment to an issue. `publish` currently requires the record head
to equal the caller's checkout and does not prove that the same record was
retained locally. The supervisor harvest separately reduces one supported Pi
v3 session and event stream to content-free `usage.json` counters.

The current Git-common-dir inventory contains v1 work-item records for issues
#246, #247, #250, and #257, a v2 record for #259, and two harvested Pi usage
aggregates for #259. Only #257 and #259 currently expose a public metrics
comment. Every retained work-item record names an exact merged PR and head.

## Normative sources

The project-sponsor direction in issue #306 controls the outcome. ADR 0108,
the canonical `factory-operations` specification, `.factory-policy.json`, the
v1/v2 metric schemas, `pi-usage-v1`, and the supervisor privacy contract govern
the implementation. The linked Oxid PR comment is presentation evidence, not a
schema or dependency source.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep publication optional | `not-adopt` | Durable local-only records do not give collaborators delivery evidence. | None under issue #306. |
| Publish raw Pi artifacts | `not-adopt` | Violates the established privacy boundary and exposes content-bearing data. | Never without a new protected privacy decision. |
| Publish only to issues | `not-adopt` | The PR is the natural exact-head delivery surface when it exists. | GitHub removes PR issue-comment compatibility. |
| PR-first with issue fallback/override | `adopt` | Binds normal evidence to review while supporting no-PR and historical issue-centric records. | Target ambiguity or duplicate ownership is observed. |
| Require current checkout for history | `not-adopt` | Makes valid post-merge and historical publication impossible. | None; hosted identity is stronger for remote evidence. |
| Verify exact hosted PR head | `adopt` | Preserves exact-head integrity after branch merge/deletion without trusting current local state. | GitHub ceases retaining PR head identity. |
| Local-before-remote idempotent publish | `adopt` | Guarantees the private authority exists before the public derivative. | Private store becomes remotely durable by an accepted decision. |

## Compatibility and dependency evidence

Existing v1/v2 schemas and hidden public markers remain unchanged. Existing
comments can be updated by the same authenticated publisher. No dependency,
MSRV, target, feature, Nix, license, public SDK, or wire contract changes. The
CLI gains target selection while preserving the existing issue argument.

## Security, privacy and maintenance evidence

Remote reads verify repository issue/PR identity and exact hosted head before
the first mutation. Private records stay owner-only below the canonical Git
common directory; conflicting retained content fails closed. Publication uses
the existing allowlisted renderer and byte bound. Raw sessions are never read
for backfill. A single bounded retry covers transient comment failure without
turning telemetry into a product-code merge bypass or infinite loop.

## Open questions and blockers

No blocker remains. Publication is a required closeout duty, but a failed
comment is recorded as telemetry debt and does not invalidate independent
product evidence. Release and public SDK support remain out of scope.

## Rejected or deferred candidates

Optional publication, raw artifact upload, issue-only targeting, remote-only
storage, and current-checkout-only historical verification are rejected for the
reasons recorded in the candidate table. Automatic pruning and a remotely
durable private metrics service remain deferred because they require separate
retention, credential, and operating-authority decisions.

## Evidence sources

Sources inspected at `develop@a6268735c3c50213b00b9998a02da31dd908b1aa`:
factory metrics/supervisor documentation and schemas, policy, metrics and Pi
harvest implementations/tests, the private store inventory, and GitHub issue
comments for #246/#247/#250/#257/#259.

## Evidence commands

Commands run: repository searches over policy/docs/implementation/tests;
bounded `find`/`jq` inspection of retained metric and aggregate files; `gh`
issue/comment inspection for #246/#247/#250/#257/#259. Implementation and
negative tests are unrun until the planning receipt exists.

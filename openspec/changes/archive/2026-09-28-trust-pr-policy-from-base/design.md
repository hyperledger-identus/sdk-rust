# Design

## Trust boundary

GitHub loads `pull_request_target` workflow YAML from the protected base
context. The job checks out `github.event.pull_request.base.sha`, verifies
`HEAD` equals that value, and runs only the base revision's shell and JavaScript
policy code and configuration. Pull-request title, body, refs, commits, and API
responses are untrusted data passed through environment variables or files.

The job may fetch `refs/pull/<number>/head` into the object database so the
base-owned checker can validate ancestry and synchronization merges. It first
checks that the fetched object equals the event's exact head SHA. It never
checks out that tree, invokes a local action from it, installs its dependencies,
or executes any file from it.

## Least authority

Repository, issue, and pull-request permissions remain read-only.
`actions/checkout` receives `persist-credentials: false`. No secret is exposed,
no untrusted expression is interpolated directly into shell program text, and
all third-party actions remain pinned to immutable revisions.

## Verification

Contract tests assert the event, exact base checkout, disabled credential
persistence, identity guard, exact head-object fetch, and absence of a head
checkout. Existing policy tests continue to exercise issue/body, contribution,
signature, and synchronization-merge behavior. Hosted CI provides the
end-to-end proof because the change's own PR must be judged by the pre-change
base workflow. The bootstrap therefore temporarily retains both events; an
immediate canary removes the legacy event once the new trust boundary is owned
by the protected base. No unrelated integration may land between them.

## Risks and mitigations

- `pull_request_target` has access to a base-context token: explicit read-only
  permissions, no secrets, and no head execution keep that authority bounded.
- Git object parsing can encounter attacker-controlled history: only Git's
  non-executing fetch and inspection commands consume it.
- A forged ref can disagree with the event: exact SHA checks fail closed.
- This PR cannot prove the newly merged workflow on itself: source tests and
  local action validation cover implementation, then the first subsequent PR
  supplies natural hosted evidence.

## Rollback

Revert the workflow, tests, specification, and ADR. No SDK API, wire format,
stored data, package, or consumer migration is involved.

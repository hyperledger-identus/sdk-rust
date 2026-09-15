## Context

GitHub couples native schedule authority to the repository default branch. The
project already couples normal delivery, required checks and all maintained
workflow files to protected `develop`; only the default-branch pointer and the
default-relative `main` ruleset prevent a coherent native schedule.

## Decisions

### Make branch identity match operational ownership

`develop` becomes the GitHub default and remains protected by ruleset
`23274352`. The setting affects GitHub navigation and default PR targets, not
release semantics. `main` remains at its minimal bootstrap revision and is
protected explicitly by rebinding ruleset `16710155` before the default moves.

### Use native scheduling with no dispatch credential

The existing schedule and manual triggers remain in the workflows now present
on the default branch. Workflows declare only `contents: read`; no Actions,
packages, contents-write, ID-token, environment or publishing authority is
added. All third-party actions remain full-SHA pinned.

### Bind evidence to one run and revision

The slow workflow gains a concurrency group with `cancel-in-progress: false`,
per-job timeouts and a final `always()` metadata job. It checks out the same
`GITHUB_SHA`, verifies the actual SHA, records UTC start/end, event, run ID,
attempt, URL, retention ceiling and every dependency result, then uploads a
SHA/run-attempt-qualified manifest for seven days. GitHub's run page is the
authoritative server conclusion and artifact index.

### Separate schedule from liveness monitoring

An offline checker makes the machine policy and workflow contract fail closed.
A network-explicit factory command reads repository/default-branch and scheduled
run metadata. A weekly Desktop supervisor heartbeat invokes it after the slow
window. It cannot dispatch, push, approve, publish or mutate; recovery remains
an explicit maintainer `workflow_dispatch` operation.

### Preserve two-phase truth

The first PR describes the accepted mechanism and post-merge transition but
does not claim live activation. After merge, protected settings change and an
exact-head manual canary executes. A second dated receipt records live state.
The issue closes only after the first natural weekly run is observed.

## Risks and mitigations

- Default-relative ruleset drift: pin `main` before changing the default and
  verify both rule conditions after each mutation.
- Scheduled delay/drop: schedule away from the top of the hour and run an
  independent read-only heartbeat after the window.
- Concurrent/relabelled evidence: use one non-cancelling concurrency group and
  include SHA/run-attempt in every artifact/manifest.
- Failure before metadata job: `always()` plus `needs` captures job conclusions;
  GitHub itself retains the run conclusion even if metadata upload fails.
- Short retention: record the organization seven-day maximum and monitor before
  expiry; do not claim durable release evidence.
- Unexpected default-base effect: this aligns GitHub's default PR base with the
  already mandatory integration target and does not alter `main` content.

## Rollback

Change the default back to protected `main`, restore the default-relative or
explicit ruleset only through a reviewed settings receipt, disable the
heartbeat, and revert the policy/docs/workflow metadata. Local/manual slow
commands and required fast CI remain available throughout.

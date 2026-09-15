# Required repository settings

This file is the reviewable desired state for GitHub controls. Maintainers must
compare it to the live settings after this governance packet reaches `develop`.
The activation procedure and rollback boundary are recorded in
[ADR 0120](../adr/0120-run-weekly-evidence-from-protected-develop.md). A dated
live receipt must record the post-merge settings before issue #276 closes.

| Setting | Required value |
| --- | --- |
| GitHub default branch | `develop` |
| `main` ruleset target | explicit `refs/heads/main` |
| `develop` ruleset target | explicit `refs/heads/develop` |
| Actions token | read-only contents |
| Actions retention | seven days (organization maximum) |

## `main` reserved-branch ruleset

- protect the default branch from deletion and non-fast-forward updates;
- require verified commit signatures and DCO;
- prohibit direct pushes and ordinary feature pull requests;
- permit no automatic merge, sync or release from `develop`;
- require a separately accepted ADR and maintainer-governed pull request before
  the branch is populated or activated;
- prohibit bypass except the documented Hyperledger emergency path.

`main` remains at the pre-bootstrap revision while this branch model is active.
Its ruleset names the branch explicitly and does not rely on default-branch
indirection.
Do not attach checks that cannot run on its intentionally minimal tree.

## `develop` integration ruleset

- block deletion and non-fast-forward updates;
- require verified commit signatures and DCO;
- require pull requests and prohibit direct pushes after bootstrap creation;
- require the stable PR-policy and applicable implementation checks to pass;
- set no blanket human-approval minimum for `develop`; an issue-backed contract,
  recorded local review and green required CI delegate merge authority to
  humans and agents;
- keep CODEOWNERS and specialist reviews advisory unless an issue contract or
  risk-specific ruleset makes one blocking;
- require all review threads resolved;
- require branches current with `develop` or use a merge queue;
- allow auto-merge so an eligible PR can enter the queue while checks run;
- prohibit bypass except the documented Hyperledger emergency path.

`develop` is the GitHub default branch so native scheduled workflows execute
the reviewed integration revision. This selection does not authorize direct
pushes, publication, release, or population of reserved `main`.

## Required checks on `develop`

During the temporary active-development phase, configure these stable required
statuses:

- `DCO`;
- `pull-request-policy`;
- `fast`;
- the applicable file-hygiene job names.

`fast` is the single Rust/factory merge signal and includes factory structure,
formatting, workspace build, strict Clippy and normal tests. The `slow` and
nightly sanitizer workflows provide native scheduled, manually dispatched and
local evidence and are deliberately not required for active-development pull
requests. Before any
release candidate, ADR 0081 requires a new compatibility decision and current
green slow evidence.

The active `develop` ruleset requires these exact statuses and an up-to-date
head. Each subsequent issue-linked pull request is continuing canary evidence
that merge and auto-merge remain blocked while a required check is pending.

CodeQL must not be represented as Rust coverage unless GitHub supports Rust for
this repository. A ruleset must never require an impossible or differently
named check. During baseline stabilization, required checks are enabled only
after the workflow exists and has succeeded on `develop`.

The `pull-request-policy` check requires a ready PR targeting `develop`, a
corresponding issue reference and completed local review evidence. Missing,
pending, cancelled or failing required checks block both human and agent merges.

## Workflow security

- declare minimal job permissions;
- pin third-party actions to full commit SHAs;
- use dependency review and hardened runners where practical;
- never expose publishing credentials to pull-request jobs;
- generate attestations and SBOMs from the tagged source;
- periodically test the release workflow with a dry-run package.

## Protected environments

Create a `crates-io` environment with:

- crates.io trusted-publishing identity scoped to repository and workflow;
- at least two human maintainer approvals;
- no long-lived crates.io token when trusted publishing is available;
- during bootstrap, release only from a signed tag reachable from the approved
  protected `develop` revision;
- no release or promotion from `main` until a later ADR activates it;
- auditable deployment history.

Security releases use a separate private path managed by the Identus security
team and retain two-person control.

## Ownership and community files

Before the first public code release, verify:

- Apache-2.0 `LICENSE`;
- `MAINTAINERS.md`, `GOVERNANCE.md`, `CONTRIBUTING.md`, `SECURITY.md`,
  `CODE_OF_CONDUCT.md`, `DCO.md` and `RELEASING.md`;
- issue forms and pull-request template;
- CODEOWNERS entries that point only to existing approved teams;
- public Discussions categories for architecture and standards;
- private vulnerability reporting enabled;
- crates.io owners and recovery contacts tested.

## Bootstrap transition

At ADR 0001 selection time, `main` blocked deletion, non-fast-forward updates
and unsigned commits, and required DCO. `develop` is created once at the
selected baseline plus governance commit. Immediately after creation,
maintainers should apply the integration ruleset and reconcile workflow names,
review requirements, Rust-compatible scanning, dependency policy and release
controls with this target state.

## Current activation

Issue #26 activated the dedicated `develop` integration ruleset on 2026-09-14.
Private vulnerability reporting, dependency alerts, Dependabot security updates
and auto-merge are active. GitHub's enterprise policy rejected repository-level
activation of secret scanning, push protection, non-provider patterns and
validity checks. The dated receipt records the exact rules and this explicit
enterprise-owned deviation; no repository workflow may represent those four
controls as enabled.

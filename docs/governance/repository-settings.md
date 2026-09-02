# Required repository settings

This file is the reviewable desired state for GitHub controls. Maintainers must
compare it to the live settings after this governance packet reaches `develop`.

## `main` reserved-branch ruleset

- protect the default branch from deletion and non-fast-forward updates;
- require verified commit signatures and DCO;
- prohibit direct pushes and ordinary feature pull requests;
- permit no automatic merge, sync or release from `develop`;
- require a separately accepted ADR and maintainer-governed pull request before
  the branch is populated or activated;
- prohibit bypass except the documented Hyperledger emergency path.

`main` remains at the pre-bootstrap revision while this branch model is active.
Do not attach checks that cannot run on its intentionally minimal tree.

## `develop` integration ruleset

- block deletion and non-fast-forward updates;
- require verified commit signatures and DCO;
- require pull requests and prohibit direct pushes after bootstrap creation;
- require two approvals for code, public API, security, crypto, protocol,
  governance and release changes; one for administrative-only changes;
- dismiss stale approvals when code changes;
- require CODEOWNERS review where ownership is configured;
- require all review threads resolved;
- require branches current with `develop` or use a merge queue;
- prohibit bypass except the documented Hyperledger emergency path.

## Required checks on `develop`

Checks should converge on stable names so rules survive workflow refactors:

- `DCO`;
- `file-hygiene`;
- `rust-fmt`;
- `rust-clippy`;
- `rust-test`;
- `rust-doc`;
- `rust-msrv`;
- `rust-wasm` for eligible workspace members;
- `dependency-policy` (`cargo deny`, source, license and ban rules);
- `security-advisories`;
- `architecture` (layer/dependency and generated/fixture drift);
- language-appropriate code scanning;
- `scorecard` or equivalent supply-chain posture.

CodeQL must not be represented as Rust coverage unless GitHub supports Rust for
this repository. A ruleset must never require an impossible or differently
named check. During baseline stabilization, required checks are enabled only
after the workflow exists and has succeeded on `develop`.

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

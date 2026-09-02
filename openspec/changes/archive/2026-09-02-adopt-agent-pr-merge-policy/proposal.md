## Why

The factory currently stops agents before publishing or merging even when a
bounded change has passed local review and every repository gate. That policy
adds a human handoff to routine integration without improving the protected
release, disclosure or `main` controls. The project sponsor has delegated
feature-branch publication and green-CI merge authority for `develop`, provided
that every pull request has a corresponding issue.

## What Changes

- Require a repository issue for every pull request, including administrative
  changes, and require the pull request to reference that issue.
- Permit humans and agents to push locally reviewed feature branches and open
  ready pull requests targeting `develop` without per-push approval.
- Permit humans and agents to merge non-draft pull requests into `develop` only
  after all required CI checks pass and no blocking review remains.
- Keep direct pushes to `develop` prohibited and preserve explicit human
  authority for scope, repository administration, security disclosure,
  publishing, releases and promotion to `main`.
- Add an executable pull-request policy check and fixtures to the factory.
- Update governance, contributor guidance, agent instructions, repository
  settings, issue intake and pull-request evidence fields consistently.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `ai-software-factory`: Delegate issue-linked, CI-gated `develop` integration
  to agents while retaining protected human authority outside integration.
- `spec-driven-delivery`: Require an issue and local review evidence before
  every pull request is eligible for CI-gated integration.

## Impact

The change affects governance and factory documentation, repository-local agent
instructions, GitHub templates, CI workflow definitions, factory scripts and
OpenSpec capability contracts. It changes no Rust API, crate dependency, wire
format, release artifact, live GitHub setting or downstream repository.

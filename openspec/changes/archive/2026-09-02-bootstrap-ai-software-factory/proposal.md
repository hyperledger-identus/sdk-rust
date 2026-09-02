## Why

The repository contains OpenSpec assets and AI-agent guidance, but the workflow
is optional, client-specific and not verified by CI. The `develop` work stream
needs one portable, evidence-producing delivery contract so humans can direct
scope while agents plan, implement and verify changes reproducibly.

## What Changes

- Establish a repository-local AI Software Factory operating model with human
  decision gates, bounded agent roles, worktree isolation and evidence handoff.
- Make OpenSpec the required planning contract for behavioral, public API,
  architecture, protocol, security and multi-step changes.
- Add a portable factory command that works through the pinned Nix environment
  and exposes doctor, status, validate and CI-safe check operations.
- Add deterministic structural checks for active change artifacts, task state,
  branch policy, forbidden generated state and repository-owned templates.
- Add GitHub issue forms and a pull-request evidence contract that feed the same
  specification lifecycle.
- Add CI enforcement for the factory and OpenSpec contracts on pull requests to
  `develop` and pushes to `develop`.
- Replace the seed-specific Pi AFK chain with generic, change-parameterized
  factory chains and document client-neutral invocation.

## Capabilities

### New Capabilities

- `spec-driven-delivery`: Repository-observable planning lifecycle, artifact
  requirements, validation behavior and change-state rules.
- `ai-software-factory`: Human/agent authority, portable automation, evidence
  receipts, repository isolation and CI enforcement for AI-led delivery.

### Modified Capabilities

- None.

## Impact

The change affects repository documentation, `AGENTS.md`, GitHub issue and pull
request templates, CI workflows, the Nix devshell, Pi chains, OpenSpec
configuration and new repository-local factory scripts. It does not change any
Rust public API, wire format, crate dependency or downstream consumer.

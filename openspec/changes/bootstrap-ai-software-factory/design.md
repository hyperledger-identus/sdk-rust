## Context

`develop` inherits OpenSpec 1.5.0 through Nix, generated OpenSpec agent skills,
Pi prompts/chains and a useful archive of specification-driven changes. Today
those pieces are discoverable only through README prose, the AFK chain is tied
to an old change name, CI does not expose a stable factory check, and no single
command tells an agent or maintainer whether the planning contract is healthy.

The factory must remain repository-local, work for different LLM clients, avoid
new hosted services and preserve human authority under the existing governance.
It must not introduce AI runtime dependencies into SDK crates.

## Goals / Non-Goals

**Goals:**

- make the specification lifecycle executable through one portable entrypoint;
- make incomplete or structurally invalid change contracts fail predictably;
- expose a fast, named CI gate independent of the full Rust/Nix matrix;
- give agents and humans the same intake, readiness and evidence vocabulary;
- retain OpenSpec as the canonical artifact model and pin its runtime with Nix;
- prove the workflow by delivering this bootstrap through an OpenSpec change.

**Non-Goals:**

- autonomous scope approval, merging, publishing or repository administration;
- an agent scheduler, model gateway, MCP server or hosted orchestration plane;
- replacing GitHub issues, Discussions, reviews or Hyperledger governance;
- modifying Rust APIs, crate topology or downstream consumer repositories;
- guaranteeing semantic correctness from structural automation alone.

## Decisions

### 1. OpenSpec remains the planning source of truth

The factory uses the pinned OpenSpec `spec-driven` schema for proposal, delta
specification, design and tasks. GitHub issue forms capture intake and point to
an OpenSpec change; pull requests carry the resulting evidence. A second custom
planning schema was rejected because it would create two authorities and make
archive/sync behavior ambiguous.

### 2. A repository-local shell facade is the universal entrypoint

`scripts/factory` exposes `doctor`, `status`, `validate`, `check`, `ready` and
`receipt`. It uses an existing `openspec` binary when running inside the pinned
devshell and otherwise re-enters `nix develop`. A Nix app and `just` aliases
delegate to the same script. This keeps Codex, Pi, Claude-compatible skills and
ordinary humans on one implementation without requiring global packages.

A Rust CLI was rejected for bootstrap because it would couple factory mechanics
to SDK compilation and increase the trusted code surface before requirements
stabilize.

### 3. Readiness is separate from structural validity

`factory check` validates repository structure, forbidden local state, OpenSpec
relationship health and every change/spec in strict non-interactive mode. It
allows incomplete task checkboxes so draft work can receive CI feedback.

`factory ready <change>` additionally requires every planning artifact and task
to be complete. `factory receipt <change>` runs the readiness gate before
printing immutable branch/change identifiers for the PR evidence section. This
separation avoids blocking draft collaboration while preventing an unfinished
change from being represented as review-ready.

### 4. The Nix check is canonical; GitHub exposes a named fast lane

A `factory-contract` Nix derivation runs the structural tests and strict
OpenSpec validation. `nix flake check` therefore remains the complete local and
cross-platform gate. A dedicated GitHub workflow builds only that derivation on
Ubuntu, producing a stable `factory-contract` status for branch protection
without duplicating the full Rust matrix.

### 5. Automation produces evidence, not authority

The factory may generate a readiness receipt, but it cannot approve scope,
waive findings, merge, publish, disclose vulnerabilities or change repository
settings. The issue form and PR template record the responsible human decision
and required independent reviews. Semantic review remains a separate agent and
human activity; `openspec validate` is explicitly structural.

### 6. Client integrations are adapters

The generated `.agents/skills/openspec-*` files remain upstream OpenSpec assets.
Pi chains become generic change-parameterized adapters and documentation uses
client-neutral commands first. Personal MCP, model, token and workspace state
is forbidden from version control. New client adapters may be added without
changing the factory contract if they delegate to the same commands.

## Risks / Trade-offs

- **Shell portability can drift across Linux and macOS** → run ShellCheck,
  exercise the script in the cross-platform Nix matrix and avoid GNU-only flags.
- **A pinned OpenSpec revision can become stale** → update it as a reviewed
  dependency change and regenerate upstream-owned skills separately.
- **Structural validation can be mistaken for semantic assurance** → keep the
  semantic review gate and independent human approval explicit in docs and PRs.
- **A dedicated CI workflow duplicates some Nix setup** → build only the small
  factory derivation and retain the full matrix as the candidate gate.
- **Readiness rules can discourage early drafts** → allow incomplete tasks in
  `check` and enforce completion only through `ready` and review policy.

## Migration Plan

1. Add the specs, scripts, Nix app/check, templates, docs and generic Pi chains
   on a feature branch based on `develop`.
2. Run the bootstrap itself through `factory ready`, strict OpenSpec validation
   and the existing repository gates.
3. Sync the two new capability specs and archive this change after verification.
4. Merge through a reviewed PR targeting `develop`; do not modify `main`.
5. Enable the stable `factory-contract` status in the `develop` ruleset after it
   has completed successfully.

Rollback is a revert of the factory commit. Existing Rust crates and historical
OpenSpec archives remain usable because the change adds no runtime migration.

## Open Questions

None block bootstrap. Model evaluation, task scheduling and MCP-based agent
orchestration remain later capabilities that require their own accepted specs.

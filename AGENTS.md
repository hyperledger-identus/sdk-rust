# SDK Rust - Agent Instructions

This repository is the Rust SDK for the
[Identus](https://github.com/hyperledger-identus) ecosystem. The working model
is an agentic SDLC: human maintainers, Codex-style engineering agents, review
agents, security agents, release agents, and documentation agents coordinate
through GitHub Issues, Pull Requests, Discussions, Spec Kit tasks, and checked
conformance fixtures.

The detailed SDLC contract lives in
`docs/maintenance/agentic-sdlc.md`. This file is the short operating manual
that every agent must read before changing the repository.

## Tech Stack

- **Language**: Rust
- **Build Tool**: Cargo
- **Entrypoint**: `Cargo.toml`

## Development

```bash
# Full local harness before opening or updating a PR
node tools/check-agent-sdlc.mjs --check
.specify/scripts/bash/check-prerequisites.sh --json --include-tasks
node tools/generate-wrapper-api-parity.mjs --check
node tools/generate-workspace-dependency-graph.mjs --check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Nix

A Nix flake may be available for development. To use it:

```bash
nix develop -c <command>
```

<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
<!-- SPECKIT END -->

## Agentic SDLC Contract

### Agent Roles

- `manager`: triages Issues and Discussions, maintains the GitHub Project
  status, assigns agents, and keeps the user-facing summary current.
- `planner`: turns accepted work into Spec Kit specs, plans, tasks, acceptance
  criteria, fixture impact, and migration impact.
- `engineer`: implements one small task slice on a branch, updates fixtures and
  docs, and keeps commits signed and DCO-compliant.
- `conformance`: owns fixtures, protocol transcripts, parity inventories, and
  executable drift checks.
- `reviewer`: performs code review for API shape, hexagonal boundaries,
  Rust correctness, and maintainability.
- `security`: reviews cryptography, storage, dependency, privacy, and
  redaction-sensitive changes.
- `docs`: keeps architecture, testing, migration, and release documentation in
  sync with code-bearing changes.
- `release`: owns release readiness, version notes, compatibility gates, and
  cross-repository coordination.
- `maintenance`: watches drift, stale work, issue hygiene, labels, project
  fields, and automation failures.

### Work Item Rules

- Every implementation increment needs a Spec Kit task with acceptance criteria
  before merge.
- Every Issue that changes behavior must name the owner crate, affected SSI
  capability, conformance impact, documentation impact, and GitHub Discussion
  link when design debate is needed.
- Every PR must link an Issue or explain why the change is administrative, list
  validation commands, and state whether security and docs review are required.
- Every code-bearing PR must keep domain semantics in Rust crates, keep
  bindings thin, and preserve the hexagonal dependency direction.
- Every commit must be GPG-signed and DCO-signed according to workspace policy.

### GitHub Status Flow

Use the status labels and GitHub Project status values defined in
`.github/labels.yml` and `docs/maintenance/agentic-sdlc.md`:

1. `status:triage`
2. `status:needs-spec`
3. `status:ready`
4. `status:in-progress`
5. `status:blocked`
6. `status:review`
7. `status:security-review`
8. `status:docs-review`
9. `status:conformance`
10. `status:ready-to-merge`
11. `status:released`

Do not skip from triage to implementation for product behavior. First record
the specification evidence, acceptance criteria, conformance strategy, and
wrapper or migration impact.

### Synchronization

- Use GitHub Issues for actionable tasks.
- Use GitHub Discussions for architecture choices, standards interpretation,
  API tradeoffs, and cross-repository coordination before the work is ready.
- Use Pull Requests for reviewable deltas; keep PRs small enough that another
  agent can review them without reconstructing the entire program.
- Use Spec Kit tasks as the durable backlog and acceptance criteria source.
- Use conformance fixtures as the executable source of truth for behavior and
  interoperability.

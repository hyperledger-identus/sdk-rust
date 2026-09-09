## Why

The SDK already has a strong OpenSpec facade and green Rust/Nix gates, but it
does not yet provide the operational controls needed for recoverable,
continuous agent delivery: a pinned worker entrypoint, machine-readable
delivery profiles, an OpenSpec-before-implementation receipt, bounded
worktree ownership, local provenance hooks, exact-head metrics, or an
immutable CI plan.

Issue #243 directs the repository to adopt the `factory` Obsidian vault as
guidance. The vault is not a version-upgrade mandate and does not override
accepted SDK governance.

## What changes

- Add a single Nix-backed `bootstrap.sh` entrypoint for audit, Git setup and
  the repository-pinned Pi development shell.
- Add machine-readable delivery, sub-agent, contribution, CI-routing,
  privacy and capacity contracts.
- Make an issue-bound OpenSpec pre-implementation receipt a dev-loop
  prerequisite before implementation paths may change.
- Add safe managed-worktree audit/create/closeout tools, repository-owned Git
  hooks, a private exact-head metrics record and a redacted public renderer.
- Add contract tests and run them through the existing `factory-contract`
  gate without changing the temporary Rust 1.98 fast/weekly slow strategy.
- Document recovery, role ownership and the post-merge Pi canary/tuning loop.

## Capabilities

### New capabilities

- `factory-operations`: governs the bounded runtime and evidence controls
  around the existing specification-driven delivery lifecycle.

## Non-goals

- No forced upgrade to the Oxid snapshot's Pi or package versions.
- No milestone-train branch, database, scheduler or cloud-worker platform.
- No GitHub settings mutation, release, publication or `main` activation.
- No product, protocol, Rust public API or downstream repository change.

## Delivery

Issue #243 owns the production-ready factory slice from
`develop@f68ace2b83e344d26b9b5b4fa1f616cb560393b0`. After it merges to
`develop`, a separate issue-backed SDK slice will run through the pinned Pi
shell. Harness tuning will be based on that canary's observed evidence.

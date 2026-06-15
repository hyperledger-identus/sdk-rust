# SDK Rust Agent Entry Point

`AGENTS.md` is the canonical repository instruction file. This compatibility
entry exists for tools that search for a singular `AGENT.md` file.

Before changing this repository, read:

- `AGENTS.md`
- `docs/maintenance/agentic-sdlc.md`
- `specs/001-sdk-rust-platform-core/tasks.md`
- `.specify/memory/constitution.md`

All implementation increments must have a Spec Kit task with acceptance
criteria, signed DCO commits, updated fixtures or docs when behavior changes,
and the local harness in `AGENTS.md` must pass before PR handoff.

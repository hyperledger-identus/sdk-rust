# SDK Rust — Agent Instructions

This is a **Rust SDK** for the [Identus](https://github.com/hyperledger-identus) ecosystem.

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Code Comments

- **Default to no comments.** Do not add comments unless the user explicitly asks for them, or a block is genuinely non-obvious.
- **When commenting, explain *why*, not *what*.** Only add short, high-value comments ahead of complex logic that would otherwise take effort to parse. Avoid restating what the code already says (e.g. "assigns the value to the variable").
- **Keep comments sparse and succinct.** They should be the exception, not the rule.
- **Never use comments to communicate with the user** or to describe/narrate your changes — use tool output and text for that, not inline code comments.
- **Don't touch unrelated comments** that are separate from the code you are modifying.
- **Shell snippets:** `#` comments are fine when needed to make copyable commands clear, one command per line.

## Nix

A Nix flake may be available for development. To use it:

```bash
nix develop -c <command>
```

<!-- BACKLOG.MD GUIDELINES START -->
<CRITICAL_INSTRUCTION>

## Backlog.md Workflow

This project uses Backlog.md for task and project management.

**For every user request in this project, run `backlog instructions overview` before answering or taking action.**

Use the overview to decide whether to search, read, create, or update Backlog tasks.

Use the detailed guides when needed:
- `backlog instructions task-creation` for creating or splitting tasks
- `backlog instructions task-execution` for planning and implementation workflow
- `backlog instructions task-finalization` for completion and handoff

Use `backlog <command> --help` before running unfamiliar commands. Help shows options, fields, and examples.

Do not edit Backlog task, draft, document, decision, or milestone markdown files directly. Use the `backlog` CLI so metadata, relationships, and history stay consistent.

</CRITICAL_INSTRUCTION>
<!-- BACKLOG.MD GUIDELINES END -->

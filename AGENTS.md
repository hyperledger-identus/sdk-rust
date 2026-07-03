# SDK Rust — Agent Instructions

This is a **Rust SDK** for the [Identus](https://github.com/hyperledger-identus) ecosystem.

## Development

A Nix flake devshell is available for setting up the development environment (requires Nix). Run:

```bash
nix develop -c <command>
```

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

## Nix Flake Checks

`nix flake check` validates the entire project in one command. It runs all linting (Nix, TOML, text), Rust formatting, clippy, nextest, `cargo deny`, and security audits via `crane` + `advisory-db`. All checks must pass before merging.

## Code Comments

- **Default to no comments.** Do not add comments unless the user explicitly asks for them, or a block is genuinely non-obvious.
- **When commenting, explain *why*, not *what*.** Only add short, high-value comments ahead of complex logic that would otherwise take effort to parse. Avoid restating what the code already says (e.g. "assigns the value to the variable").
- **Keep comments sparse and succinct.** They should be the exception, not the rule.
- **Never use comments to communicate with the user** or to describe/narrate your changes — use tool output and text for that, not inline code comments.
- **Don't touch unrelated comments** that are separate from the code you are modifying.
- **Shell snippets:** `#` comments are fine when needed to make copyable commands clear, one command per line.

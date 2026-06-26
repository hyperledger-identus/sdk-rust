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

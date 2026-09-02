# SDK Rust — agent instructions

This repository is the chain-agnostic Rust SDK for the
[Identus](https://github.com/hyperledger-identus) ecosystem. Humans retain
scope, governance, merge, security-disclosure and release authority; agents
implement and verify bounded component slices.

Before changing files, read:

- `GOVERNANCE.md` and `CONTRIBUTING.md`;
- `docs/architecture/sdk-rust-blueprint.md`;
- `docs/adr/0001-bootstrap-branch-selection.md`;
- `docs/governance/agentic-sdlc.md`;
- the issue or OpenSpec change and any crate-local `AGENTS.md` in scope.

## Branch and repository boundaries

- `develop` is the active integration branch. Start focused branches and
  dedicated worktrees from its current protected tip.
- `main` is intentionally reserved and minimal. Do not target, populate or
  release from it until maintainers accept a separate activation decision.
- The `yet-another-seed` tree at `662f8d7` is the selected `develop` baseline.
  Existing implemented crates are foundations to stabilize, not immutable API
  commitments. Empty placeholder crates are not roadmap or release promises.
- Treat Oxid, midnight-identity, neoprism, Portal and other SDK repositories as
  read-only unless a separate adoption issue explicitly authorizes changes.
- Do not add `midnight-*`, `compact-runtime`, chain clients or product
  repositories to a generic SDK crate.
- Port later donor components one accepted slice at a time and record source
  SHA, path, license, transformation and conformance evidence.
- Do not expose raw secret bytes through errors, debug, serialization or FFI.
- Do not push, merge, publish or change repository settings without explicit
  human maintainer authority.

## Development gates

A Nix flake devshell supplies the reproducible maintainer environment. Plain
stable Cargo remains a supported consumer path.

```bash
nix develop -c cargo build --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --no-deps
nix flake check
```

Run the issue-specific MSRV, WASM/mobile, minimal-feature, fuzz, conformance,
dependency and public-API gates in addition. `nix flake check` is mandatory
after code, build or dependency changes. Report every unrun or failing command
exactly; never infer one gate from another.

All repository-facing commits require both DCO and a verified signature:

```bash
git commit -S -s -m "type: concise summary"
```

## Code comments

- Default to no comments unless a block is genuinely non-obvious.
- Explain why, not what, and keep comments short.
- Never use source comments to narrate work to the user.
- Do not alter unrelated comments.
- Shell snippets may use concise `#` comments where needed for copyability.

# Identus SDK for Rust

Rust SDK for building decentralized identity solutions with the [Identus](https://github.com/hyperledger-identus) ecosystem.

## Getting Started

> **⚠️ Work in Progress** — This SDK is under active development.

### Prerequisites

- [Nix](https://nixos.org/) (with flakes enabled)

All other tooling (Rust toolchain, C toolchain, cargo dev tools, Nix hygiene tools) is provided by the project's Nix flake.

### Development

Enter the reproducible development shell, then run the usual cargo commands:

```bash
# Enter the devshell
nix develop

# Build
nix develop -c cargo build

# Test
nix develop -c cargo test

# Format (Rust + Nix)
nix run .#format

# Run all flake checks (fmt, clippy, test, deny, audit, nix hygiene)
nix flake check
```

The flake supports `x86_64-linux` and `aarch64-darwin`. CI runs `nix flake check` on both via the `nix-checks` workflow.

## Development Workflow

We use [OpenSpec](https://github.com/Fission-AI/OpenSpec) to align on what to build *before* writing code. Each change lives in its own folder under `openspec/changes/` with a proposal, specs, design, and task list.

```text
   /opsx:explore          think it through
        │
        ▼
   /opsx:propose <idea>   creates openspec/changes/<idea>/
        │                  (proposal, specs, design, tasks)
        ▼
   /openspec-review       validate the spec before building
        │                  (.pi/prompts/openspec-review.md)
        │                  optional when using pi
        ▼
   /opsx:apply            implement the tasks
        │
        ▼
   /opsx:archive          move to archive/, update specs
```

**Quick flow (≈5 steps):**

1. **Explore** — Not sure yet? Run `/opsx:explore` to think it through with the AI before committing.
2. **Propose** — Know what you want? Run `/opsx:propose <idea>`. This creates `openspec/changes/<idea>/` containing `proposal.md`, `specs/`, `design.md`, and `tasks.md`.
3. **Review** *(optional if you use pi)* — Run the `/openspec-review <change-id>` prompt (`.pi/prompts/openspec-review.md`) to semantically validate the spec before any code is written. Fix any blockers it surfaces.
4. **Apply** — Run `/opsx:apply` to implement the tasks.
5. **Archive** — Run `/opsx:archive` to move the change to `openspec/changes/archive/` and update the specs.

> OpenSpec is provided by the project devshell — no separate install needed.

## Resources

- [Identus Documentation](https://hyperledger-identus.github.io/)
- [Hyperledger Identus GitHub](https://github.com/hyperledger-identus)

# Identus SDK for Rust

Chain-agnostic Rust foundations for decentralized identity and verifiable
credential products in the
[Hyperledger Identus](https://github.com/hyperledger-identus) ecosystem.

> **Work in progress:** `develop` is a pre-release integration line. No crate
> in this repository should yet be treated as a stable production SDK release.

## Direction

The repository owns generic SSI domain models, protocol engines, cryptographic
utilities, ports and conformance evidence. Midnight, PRISM/Cardano and future
chain families consume the SDK and keep ledger-specific behavior outside it.
Wallet products keep custody, storage, consent, trust and UI policy.

- [Technical blueprint and component sequence](docs/architecture/sdk-rust-blueprint.md)
- [Bootstrap governance and crate inventory](docs/architecture/sdk-bootstrap-inventory.md)
- [Alpha source distribution](docs/architecture/source-distribution.md)
- [Bootstrap branch decision](docs/adr/0001-bootstrap-branch-selection.md)
- [NeoPRISM toolchain alignment](docs/adr/0002-neoprism-toolchain-alignment.md)
- [Roadmap](ROADMAP.md)
- [Governance](GOVERNANCE.md)
- [Constraints and limitations](docs/governance/constraints-and-limitations.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Release policy](RELEASING.md)
- [AI Software Factory](docs/factory/README.md)

## Branch model and current baseline

`develop` starts from `yet-another-seed` revision `662f8d7` and is the active
integration branch. It contains working validated-newtype, derivation,
cryptography, DID, entropy-adapter and conformance foundations. The baseline
also contains known bootstrap debt: quarantined placeholder names, version
`0.0.0`, enterprise-controlled secret-scanning gaps, and no approved publishing
ownership. The protected `develop` integration ruleset and private
vulnerability reporting are active. Every package is `publish = false`. The
MSRV and target lanes are executable; the remaining items are not evidence of
a production release.

`main` remains intentionally minimal and is not an integration or release
target until maintainers explicitly activate it through a later decision.
Feature branches and pull requests target `develop`.

## Getting started

### Prerequisite

- [Nix](https://nixos.org/) with flakes enabled for the reproducible toolchain;
  the flake pins primary stable Rust separately while retaining NeoPRISM's Nix
  baseline and exact stable Rust 1.98.1. The pinned nightly is available only
  in the explicit sanitizer-fuzz shell.

```bash
# Enter the devshell
nix develop

# Build and test
nix develop -c cargo build --workspace
nix develop -c cargo test --workspace

# Format Rust and Nix
nix run .#format

# Run sanitizer fuzzing with the pinned nightly shell
nix develop .#fuzz -c ./scripts/fuzz-jws.sh smoke

# Run the complete repository gate
nix flake check

# Reproduce the complete slow Clippy surface locally
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The flake supports `x86_64-linux` and `aarch64-darwin`. Pull requests and
`develop` pushes run one Ubuntu `fast` factory/build/lint/test status. Native
weekly slow evidence runs from protected `develop`, which is the GitHub default
branch while reserved `main` remains minimal. The same complete Linux/macOS
matrix can be dispatched manually or reproduced locally; it includes the
all-target/all-feature Clippy surface. See the
[Clippy policy and exception registry](docs/governance/clippy-policy.md).

## Development workflow

The [AI Software Factory](docs/factory/README.md) uses pinned
[OpenSpec](https://github.com/Fission-AI/OpenSpec) artifacts as the contract
between human intent and agent implementation.

```text
   /opsx:explore
        │
        ▼
   /opsx:propose <idea> ──► openspec/changes/<idea>/
        │
        ▼
 research-ready + constraint-ready
        │
        ▼
   /openspec-review
        │
        ▼
   /opsx:apply ──► /opsx:verify ──► factory ready ──► /opsx:archive
```

Use an OpenSpec change for new behavior, public API, architecture, protocol or
multi-step work. Direct edits are suitable for typos, non-behavioral bug fixes,
formatting, dependency chores and behavior-preserving refactors. Archived seed
decisions are historical evidence: branch, release and architecture rules in
the current governance documents take precedence.

```bash
./scripts/factory doctor
./scripts/factory check
./scripts/factory research-ready <change>
./scripts/factory constraints-ready <change>
./scripts/factory ready <change>
./scripts/factory receipt <change>
```

Feature branches and dedicated worktrees start from `develop`; reviewed pull
requests return to `develop`. The factory never targets `main` automatically.

## Resources

- [Identus documentation](https://hyperledger-identus.github.io/)
- [Hyperledger Identus GitHub](https://github.com/hyperledger-identus)
- [Hyperledger governing documents](https://toc.hyperledger.org/governing-documents/)

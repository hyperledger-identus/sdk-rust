# Identus SDK for Rust

Rust SDK for building decentralized identity solutions with the [Identus](https://github.com/hyperledger-identus) ecosystem.

## Getting Started

> **Warning:** Work in Progress - This SDK is under active development.

### Prerequisites

- Rust (latest stable)
- `cargo`

### Development

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Check compile-only phase-1 workspace
cargo check --workspace

# Run the Docker-free BDD acceptance seed
cargo test -p identus-agent

# Run the specification conformance catalog
cargo test -p identus-conformance
```

## Architecture

The phase-1 workspace skeleton and component relationships are documented in
[`docs/architecture/components.md`](docs/architecture/components.md).
The exact current crate graph and dependency table are documented in
[`docs/architecture/current-workspace.md`](docs/architecture/current-workspace.md).
Core error and result conventions are documented in
[`docs/architecture/core-error-conventions.md`](docs/architecture/core-error-conventions.md).

Testing plans are tracked in [`docs/testing/bdd-scenarios.md`](docs/testing/bdd-scenarios.md),
[`docs/testing/conformance.md`](docs/testing/conformance.md), and
[`docs/testing/fixtures.md`](docs/testing/fixtures.md).

## Resources

- [Identus Documentation](https://hyperledger-identus.github.io/)
- [Hyperledger Identus GitHub](https://github.com/hyperledger-identus)

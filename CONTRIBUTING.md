# Contributing to the Identus SDK for Rust

Thank you for contributing. The repository follows the
[Hyperledger Identus contribution policy](https://github.com/hyperledger-identus/.github/blob/main/CONTRIBUTING.md)
and the repository-specific rules below.

## Before starting

1. Read [GOVERNANCE.md](GOVERNANCE.md), the
   [SDK blueprint](docs/architecture/sdk-rust-blueprint.md), and the closest
   `AGENTS.md` instructions.
2. Search existing issues, Discussions, ADRs and pull requests.
3. For behavior, public API, new crate, standard/profile, crypto, FFI or
   architecture work, obtain an accepted issue before implementation.
4. Use a dedicated worktree and a focused branch from current `develop`.
5. Record source revisions and licenses before porting code or fixtures.

Small typo or administrative fixes may proceed directly to a PR. Security
reports must use [SECURITY.md](SECURITY.md), never a public issue.

## Component issue contract

Implementation issues must state:

- consumer outcome and owner crate;
- normative source/profile version;
- exact source SHAs and read-only reference paths;
- public API, wire and error compatibility;
- scope, non-scope and dependency cone;
- input bounds, threats and misuse cases;
- conformance/fixture provenance;
- target/feature matrix and acceptance commands;
- docs, security and migration impact;
- release candidate and rollback plan;
- separate downstream adoption work.

## Development rules

- Use stable Rust and the declared MSRV. Do not introduce nightly features.
- Keep default features minimal and list every meaningful feature combination.
- Put dependency versions in the workspace manifest and keep member manifests
  on workspace dependencies.
- Deny warnings in CI and forbid unsafe code by default. An unsafe exception
  requires a dedicated ADR, safety invariants and reviewer expertise.
- Reuse reviewed cryptography. Do not implement novel primitives inside a
  protocol task.
- Keep raw secret material out of debug/display/errors, serialization and FFI.
- Keep product policy, chain clients and consumer storage out of SDK crates.
- Add constructor/deserializer equivalence, negative and resource-bound tests
  for every untrusted input type.
- Do not edit a consumer repository to make an upstream component appear done.

## Donor and fixture provenance

The selected baseline is documented in ADR 0001. New ports from other branches
or repositories are reviewed components, not automatic history merges. Every
adapted source or fixture records:

```yaml
origin_repository: <owner/repository>
origin_revision: <full SHA or immutable release>
origin_path: <path>
license: <SPDX identifier>
retrieved_at: <ISO-8601 date>
transformation: <none or description>
expected_result: <machine-checkable outcome>
```

Never copy production credentials, keys, witnesses, wallet databases or other
personal/secret material.

## Validation

Run the documented focused checks first and the repository gate before handoff.
Once the bootstrap lands, the expected baseline is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --no-deps
nix flake check
```

Eligible crates also run MSRV, WASM/mobile target, minimal-feature, fuzz,
conformance and public-API checks defined by their component issue. If a tool is
unavailable, report the exact unrun gate; do not claim it passed.

## Commits

Every commit must:

- be a small, coherent change;
- follow Conventional Commits;
- carry a Developer Certificate of Origin `Signed-off-by` trailer;
- have a cryptographic signature that GitHub can verify.

Typical command:

```bash
git commit -S -s -m "docs: define sdk governance"
```

See [DCO.md](DCO.md) for the controlling policy and setup guidance.

## Pull requests

Open a draft PR targeting `develop` early. Complete the repository template
with source revisions, compatibility, security, documentation and validation
evidence. A PR cannot be its own independent review, even when separate agents
authored and reviewed parts of it under one human account.

Keep each PR independently reversible. Do not combine upstream implementation
with downstream Oxid, Midnight, neoprism or other consumer adoption.

## AI-assisted contributions

AI assistance is welcome and expected. The human submitter remains accountable
for provenance, correctness, license compliance, private-data handling and the
truthfulness of validation claims. Agents must follow
[agentic-sdlc.md](docs/governance/agentic-sdlc.md) and cannot publish, merge or
change governance without human maintainer authority.

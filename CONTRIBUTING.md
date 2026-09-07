# Contributing to the Identus SDK for Rust

Thank you for contributing. The repository follows the
[Hyperledger Identus contribution policy](https://github.com/hyperledger-identus/.github/blob/main/CONTRIBUTING.md)
and the repository-specific rules below.

## Before starting

1. Read [GOVERNANCE.md](GOVERNANCE.md), the
   [SDK blueprint](docs/architecture/sdk-rust-blueprint.md), and the closest
   `AGENTS.md` instructions.
2. Search existing issues, Discussions, ADRs and pull requests.
3. Select the corresponding repository issue or create one. Every pull request
   requires an issue. Agents may originate and refine routine scope within the
   product mandate. For behavior, public API, new crate, standard/profile,
   crypto, FFI, architecture or multi-step work, complete and review the
   OpenSpec contract before implementation; separate human acceptance is not a
   precondition.
4. Use a dedicated worktree and a focused branch from current `develop`.
5. Record source revisions and licenses before porting code or fixtures.
6. Run `scripts/factory doctor` and create or select the OpenSpec change for
   qualifying work before editing implementation files.
7. Read the [constraint index](docs/governance/sdk-constraints.toml), complete
   `constraints.md`, and run `scripts/factory constraints-ready <change>`.
   Keep future targets and known limitations separate from effective promises.

Small typo or administrative fixes may use a lightweight delivery issue and an
OpenSpec exemption. Security reports must use [SECURITY.md](SECURITY.md), never
a public issue.

## Specification-driven lifecycle

Behavior, public API, architecture, protocol, security and multi-step changes
require a repository-local OpenSpec proposal, capability specs, design and task
list. Use the [factory handbook](docs/factory/README.md) and keep every scenario
objectively testable.

```bash
openspec new change <change>
openspec validate <change> --strict --no-interactive
./scripts/factory check
./scripts/factory constraints-ready <change>
./scripts/factory ready <change>
./scripts/factory receipt <change>
./scripts/factory archive <change>
```

Use the issue or Discussion for durable collaboration. Check tasks only after
their implementation and focused evidence pass. Before opening a ready pull
request, complete a distinct local review pass, sync reviewed delta specs and
archive the completed change through the factory facade.

A `MODIFIED` delta is a complete requirement replacement. Copy the entire
canonical requirement block before editing it. Purely additive replacements
must preserve every existing nonblank line in order. If an intentional rewrite
or deletion is required, add `archive-intent.toml` to the active change with
the capability, requirement, exact normalized canonical SHA-256 and a nonempty
rationale. The preservation checker rejects missing, stale, duplicate or unused
acknowledgements; `scripts/factory archive` runs that preflight before OpenSpec
can mutate canonical specifications.

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
- introduced, changed or removed material constraints and limitations;
- decision authority and activation path for any material outcome.

## Development rules

- Use the repository's Nix-pinned, NeoPRISM-aligned Rust toolchain for the full
  gate. Preserve the declared MSRV and do not introduce nightly-only features.
- Do not make a target MSRV, platform, feature, public contract, certification
  or release intent effective without the material constraint activation path.
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

Open a ready pull request targeting `develop` after implementation and a
distinct local review pass are complete. Reference the corresponding issue and
complete the repository template with source revisions, compatibility,
security, documentation, review and validation evidence. The local review may
be performed by a human or by an agent in a fresh review context; it cannot
waive a security, compatibility, provenance or conformance finding.

Keep each PR independently reversible. Do not combine upstream implementation
with downstream Oxid, Midnight, neoprism or other consumer adoption.

Direct pushes to `develop` are prohibited. A human or agent may merge a
non-draft, mergeable pull request into `develop` without a separate per-merge
authorization only when every required CI gate is successful and no blocking
review or unresolved thread remains. Missing, pending, cancelled or failing
required checks prohibit merge. Never bypass branch protection.

## AI-assisted contributions

AI assistance is welcome and expected. Agents are accountable for truthful
provenance, validation and review evidence. The project sponsor and human
maintainers remain accountable for the protected decisions defined in
[agentic-sdlc.md](docs/governance/agentic-sdlc.md). Within the standing product
mandate, agents may select and specify routine work, publish focused branches,
triage CI and integrate eligible pull requests into `develop` without formal or
format approval. Product strategy, repository administration, security
disclosure, publication, releases and promotion to `main` retain human
maintainer authority.

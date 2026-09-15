# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

Manual slow run
[`34951058164`](https://github.com/hyperledger-identus/sdk-rust/actions/runs/34951058164)
at merged `develop@7bc2030a45e59cfea3f994d13d411a32e79f9b7c`
failed with `non-deterministic Cargo archive: identus-derive`. The same command
fails locally with Cargo 1.98.1 when its output is inside the repository.

Archive-member comparison proves that Cargo writes distinct
`.cargo_vcs_info.json` files because the two generated workspaces have
different repository-relative paths below `artifacts/`. The derived
`identus-derive` archive checksum then differs in dependent generated
`Cargo.lock` files. Runtime SDK source, package manifests, and inputs are
otherwise identical.

## Normative sources

- Cargo 1.98.1 `cargo package` is the selected archive assembler under ADR 0113.
- Python 3 `tempfile.TemporaryDirectory` supplies private, automatically cleaned
  build and output staging directories without a new dependency.
- ADR 0113 and the canonical `unpublished-crypto-candidate` capability retain
  byte-identical double assembly, exact source identity, bounded archive checks,
  and atomic completed output.
- Issue #276 and PR #288 provide the exact activation/canary provenance.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| System temporary build scratch outside the SDK VCS tree | `adopt` | Prevents Cargo from deriving stage-specific repository paths while retaining native Cargo archives. | A supported Cargo option provides explicit canonical VCS metadata. |
| Separate temporary output sibling under the requested parent | `adopt` | Keeps final rename on the destination filesystem and preserves atomic success publication. | The output protocol changes from local atomic rename. |
| Normalize or rewrite `.crate` bytes after Cargo packaging | `not-adopt` | Would stop testing Cargo's actual distributable archive and require checksum/lockfile rewriting. | Cargo cannot produce reproducible native archives outside VCS discovery. |
| Initialize synthetic Git repositories in both stages | `not-adopt` | Introduces artificial commit identity and additional Git-state machinery unrelated to the canonical source revision. | A registry requires embedded canonical VCS metadata that Cargo cannot otherwise express. |
| Remove byte-for-byte comparison | `not-adopt` | Conceals reproducibility failure and violates the accepted candidate contract. | Never within this issue. |

## Compatibility and dependency evidence

The change affects candidate tooling only. Canonical package content, versions,
features, dependency cone, public APIs, serialized forms, error contracts,
compiler, and target matrix are unchanged. No library or crate dependency is
added.

## Security, privacy and maintenance evidence

Temporary directories remain library-created with restrictive platform
defaults and automatic cleanup. Build scratch is rejected if it resolves below
the repository root. Completed artifacts are copied to a temporary sibling of
the requested destination and renamed only after every check succeeds. Neither
scratch name nor host path enters the receipt. No credentials, secret material,
environment dump, network mutation, publication, or consumer repository is in
scope.

## Rejected or deferred candidates

Archive post-processing, synthetic staging repositories, and weakening native
byte comparison are rejected as recorded in the candidate table. Registry
publication, signing, attestation, durable storage, and consumer adoption stay
deferred to their existing protected decisions.

## Open questions and blockers

There are no research blockers. A successful local candidate run and hosted
manual slow canary are required evidence; the first natural weekly run remains
separate acceptance under issue #276.

## Evidence commands

- `cargo --version`: `cargo 1.98.1 (797e8a9bc 2026-08-05)`.
- Local exact-base candidate reproduction failed with the same
  `non-deterministic Cargo archive: identus-derive` result.
- Diagnostic archive comparison showed only stage-relative VCS metadata and its
  transitive internal lockfile checksums changing.
- Hosted failing job:
  `https://github.com/hyperledger-identus/sdk-rust/actions/runs/34951058164/job/104321891103`.

## Reconsideration triggers

- Cargo changes VCS metadata or archive reproducibility behavior.
- Candidate output must cross filesystems or become remotely streamed.
- Publication requires source provenance embedded inside the native archive.

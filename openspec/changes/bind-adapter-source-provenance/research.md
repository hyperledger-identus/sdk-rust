# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-10-26
Source retrieval date: 2026-10-26
Research blockers: none

## Problem and existing implementation

Issue #512 follows the independent review of #505/#510. The schema-v1
language-adapter registry repeats `canonical_revision` and
`canonical_bound_path` in four mappings. Its checker validates revision shape
and safely reads the current Rust source file, but it cannot detect a plausible
fabricated revision or prove that current bytes are the reviewed bytes. Nix
intentionally evaluates a source snapshot with no `.git` directory.

The design must bind exact source content offline, share identity across
mappings, reject stale content claims, and avoid unrelated record churn.

## Normative sources

- SDK-Rust revision `5da04223c1c4e029cb368870877e56d699d40388`
  is the exact current implementation and merged A1 base.
- Git's data model defines immutable content-addressed blob, tree, and commit
  objects: https://git-scm.com/docs/gitdatamodel
- Git's loose-object format defines an object ID over
  `<type> <size> NUL <content>`: https://git-scm.com/docs/gitformat-loose
- Git's hash transition specifies SHA-1 and SHA-256 object identities:
  https://git-scm.com/docs/hash-function-transition
- Cargo warns that packaged VCS information does not prove source/revision
  equality: https://doc.rust-lang.org/cargo/commands/cargo-package.html
- Cargo directory sources use SHA-256 per-file checksums:
  https://doc.rust-lang.org/stable/nightly-rustc/cargo/sources/directory/struct.Checksum.html

All assessed implementation is Apache-2.0 repository code. This is internal
provenance evidence, not a new runtime protocol.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Shared exact-byte manifest | schema v2 at `5da04223c1c4e029cb368870877e56d699d40388` | `adopt` | Deduplicates identity and validates in Git-less inputs. | Exact-byte churn becomes operationally dominant. |
| Per-mapping SHA-256 | local schema-v1 extension | `not-adopt` | Repeats revision, path, and digest across mappings. | Mappings cease to share canonical files. |
| Normalized declaration digest | unversioned concept | `not-adopt` | Parser and macro semantics add coupling and incomplete assurance. | A stable compiler-owned digest becomes available. |
| Mandatory `git cat-file` | Git 2.x | `not-adopt` | Fails in source archives without `.git`. | Mandatory inputs acquire verified Git objects. |
| Python SHA-256 plus Git blob hash | Python 3.14 runtime | `adopt` | Exact, offline, and dependency-free. | The factory runtime or Git object format changes. |

The selected manifest stores one source ID, owner, state, Cargo package, full
Git revision, path, regular-file mode, byte length, SHA-256 digest, Git object
format, and Git blob object ID. A normalized declaration digest is rejected
because it needs a parser or incomplete home-grown grammar and can miss macro
or configuration behavior outside a selected item.

## Compatibility and dependency evidence

Mappings replace repeated revision/path fields with one
`canonical_source_id`; canonical API and bound-symbol fields remain mapping
semantics. A source move changes the shared path once. A content or formatting
edit rotates hashes once. No unrelated source file affects the record.

The current version, revision, feature surface, Rust 1.89.0 MSRV, Rust 1.98.1
etalon, tiered targets, public and wire compatibility, ABI, persistence, and
runtime behavior remain unchanged. Consumer evidence is the four delivered
SDK-TS mapping records and the #492 handoff that depends on them. License and
provenance remain Apache-2.0 repository-local evidence. There is no direct and
resolved dependency cone change: standard-library hashing stays in the existing Python checker.
The facade boundary remains outside Rust core. Rollback removes schema-v2
evidence before merge or supersedes it with a new registry version afterward.
Protocol/draft currency is not applicable because this binds repository source.

## Security, privacy and maintenance evidence

The validator reads only bounded regular non-symlink repository files. It
computes SHA-256 over exact bytes and the Git blob identity over
`blob <size>\0<bytes>`. Git SHA-1 remains a compatibility object identity, not
the sole security digest. Unknown object formats fail closed.

No secret, personal, credential, unsafe Rust, native code, or executable input
is introduced. The supply-chain surface is unchanged. Maintenance is one
shared record per file. Release provenance remains out of scope. A malicious
contributor can update source and hashes together; protected review and signed
commits remain authority.

Offline validation cannot independently prove that a remote commit tree
contained the asserted blob. It proves that checked bytes match the reviewed
SHA-256 and Git-blob identities stored with that revision/path assertion.

## Rejected or deferred candidates

Normalized semantic digests, compiler token streams, signed attestations,
remote Git verification, generated adapters, and language-repository source
mirroring are rejected or deferred. The reconsideration trigger for semantic
normalization is a stable compiler-owned format with lower maintenance than
exact bytes.

## Open questions and blockers

None. The dual identities, Git-less boundary, schema migration, and stop
boundary are sufficiently specified. Offline validation intentionally does
not prove maintainer intent or remote commit signatures.

## Evidence commands

Research used `rg`, `sed`, the existing checker and registry, plus primary Git
and Cargo documentation retrieved on 2026-10-26. Planning commands are
`scripts/factory research-ready bind-adapter-source-provenance`,
`scripts/factory constraints-ready bind-adapter-source-provenance`, and
`scripts/factory validate bind-adapter-source-provenance`. Schema-v2 mutation
tests and Nix gates are intentionally unrun until after the planning receipt.

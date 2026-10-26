# Design

## Registry shape

`language-adapter-mappings.toml` advances to schema and registry version 2.
The top level adds `canonical_sources`. Each source record has a stable dotted
ID, owner issue, lifecycle state, `identus-*` crate, full Git revision,
repository-relative path, file mode, byte length, SHA-256, Git object format,
and Git blob object ID.

Mappings replace repeated `canonical_revision` and `canonical_bound_path` with
`canonical_source_id`. Canonical Rust API and bound-symbol fields remain mapping
semantics. Multiple mappings may share a source; every declared source must be
used.

## Offline hash and local Git validation

The existing Python checker reads each path through its bounded component-wise
symlink guard. It hashes the exact bytes with SHA-256. For the declared
`sha1` Git object format it also hashes:

```text
blob <decimal-byte-length> NUL <exact-file-bytes>
```

The result must equal the recorded blob ID and byte length. This content check
does not require a Git process or object database, so source archives remain
verifiable. Unknown object formats fail closed.

When the repository includes local Git metadata, the checker additionally
requires the declared revision to be an ancestor of `HEAD`. An `active` record
requires `<revision>:<path>` to resolve to the declared blob. A source-changing
PR uses `pending-revision`: the prior active revision must differ while
`HEAD:<path>` must resolve to the new declared blob. This closes fabricated
tree assertions without weakening the Git-less archive baseline.

A generic synthetic or exported checkout whose object database does not
contain the pinned revision falls back to the offline content baseline.
Protected PR and `develop` modes receive explicit context and fail when the
required revision object is absent.

PR CI supplies the immutable base SHA. If canonical source bytes differ from
that base, an `active` record is rejected and a pending record must name that
exact base. An active revision must itself be reachable from the PR base, so a
feature-only commit cannot be discarded by the squash. Push CI on `develop`
permits pending only when its revision is the direct parent of `HEAD`; the next
change therefore cannot land until the record activates the squash revision.

## Provenance semantics

The Git revision and path identify the reviewed tree assertion. The Git blob
ID connects that assertion to Git's content model. SHA-256 supplies an
independent exact content identity. Offline validation proves the current
bytes match both stored content identities. Local Git validation proves the
revision-to-path assertion when the object database is available; protected
review remains the authority for choosing and rotating that assertion.

Source moves update the path and registry version. If bytes are unchanged the
two content hashes remain stable, making the move explicit without rewriting
every mapping. A content or formatting edit updates the shared source record
once.

Protected squash merges cannot know their final commit ID inside the source
PR. A source-changing PR therefore records the previous active revision and
sets `pending-revision`; its squash result remains valid because `HEAD` binds
the new bytes. An immediate behavior-neutral follow-up rotates the revision to
that squash commit and returns the record to `active`. `pending-revision` is an
explicit transitional state, not evidence equivalent to `active`.

## Rendering and tests

The Markdown renderer adds a canonical-source section and renders each
mapping's source ID. Mutation tests cover missing/unknown fields, duplicate and
unused sources, missing mapping targets, path escape, symlinks, size drift,
SHA-256 drift, blob drift, unsupported object format, crate/path mismatch,
source moves, and deterministic output.

## Stop boundary

This slice changes registry evidence and its validator only. It does not change
Rust, generated adapters, language DTOs, downstream repositories, releases, or
the consumer change ledger.

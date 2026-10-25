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

## Offline hash validation

The existing Python checker reads each path through its bounded component-wise
symlink guard. It hashes the exact bytes with SHA-256. For the declared
`sha1` Git object format it also hashes:

```text
blob <decimal-byte-length> NUL <exact-file-bytes>
```

The result must equal the recorded blob ID and byte length. No Git process or
object database is used. Unknown object formats fail closed.

## Provenance semantics

The Git revision and path identify the reviewed tree assertion. The Git blob
ID connects that assertion to Git's content model. SHA-256 supplies an
independent exact content identity. Offline validation proves the current
bytes match both stored content identities; protected review remains the
authority for the revision-to-tree assertion.

Source moves update the path and registry version. If bytes are unchanged the
two content hashes remain stable, making the move explicit without rewriting
every mapping. A content or formatting edit updates the shared source record
once.

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

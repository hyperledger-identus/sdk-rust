# Design

## Closed train index

Add `docs/release/release-trains.toml` with a schema version and ordered train
records. Each record contains a stable ID, lifecycle (`published` or
`candidate-only`), primary package, descriptor path and exact tag. The checker
rejects unknown keys, duplicate IDs/tags/descriptors/package ownership, unsafe
paths, missing descriptors and tag/descriptor disagreement.

The published crypto record points at its existing descriptor and preserves
`v0.1.0-rc.1`. The DID record points at a new closed descriptor and uses
`identus-did-v0.1.0-rc.1`.

## Candidate-only DID descriptor

The descriptor owns exact package order, paths, version, staged metadata,
internal candidate dependencies, already-published Identus dependencies,
feature profiles, archive/resource bounds and supported evidence. It states
`publication = "candidate-only"` and contains no workflow, environment or
credential configuration.

## Deterministic assembler

The DID assembler reuses the proven safety shape without changing the crypto
engine:

1. validate the train index/descriptor and exact clean source identity;
2. create two VCS-independent scratch workspaces;
3. copy only allowlisted package source/README files plus LICENSE;
4. render staged release metadata and exact dependencies without editing the
   canonical manifests;
5. package the ordered workspace twice under fixed environment inputs;
6. byte-compare and safely inspect every bounded archive;
7. extract archives and build/test them through local patches, including
   default, no-default and resolver all-feature profiles;
8. atomically emit archives plus a closed JSON receipt with checksums, file
   lists, tool identities and limitations.

The checker statically rejects remote mutation vocabulary in executable code
and verifies the canonical packages remain unpublished.

## Tests

Mutation cases cover duplicate identity/tag/package, invalid lifecycle/tag,
unsafe paths, reordered dependencies, manifest name/path/version/publish drift,
unknown descriptor keys, missing metadata, retained path/Git dependencies,
non-determinism and remote-mutation tokens. Existing crypto/release mutation
tests must remain green.

## Rollback

All changes are additive planning/candidate evidence. Revert them without
changing canonical crate identity, release state or consumers.

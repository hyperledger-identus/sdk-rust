# Bind adapter mappings to offline source provenance

## Why

Issue #512 follows the independent review of #505/#510. The language-adapter
registry records a full Git revision and reads a live Rust source path, but it
does not prove that the bytes being validated are the bytes reviewed at that
revision. Git metadata is deliberately absent from Nix source inputs and crate
archives, so a direct `git cat-file` gate would make local and packaged evidence
disagree.

## What changes

- Add a shared canonical-source manifest to the adapter registry rather than
  repeating provenance in every mapping.
- Bind each source to its repository path, Git revision, file mode, Git blob
  object ID, and SHA-256 content digest.
- Compute both content identities with the Python standard library so Git-less
  Nix and source archives apply the same mandatory validation.
- When local Git metadata exists, validate active revision/path/blob identity
  and a bounded `pending-revision` transition compatible with squash merges.
- Make each mapping reference one canonical-source ID and keep symbol/bound
  validation scoped to that source.
- Render the source identity separately from mapping semantics and add mutation
  coverage for drift, moves, duplicates, symlinks, and fabricated identities.

## Capability

### Modified capability

- `language-adapter-mappings`: bind canonical Rust identities to exact offline
  source content through a reusable provenance record.

## Non-goals

This change does not generate bindings, change a Rust API or language DTO,
prove a Git commit signature, contact GitHub, require `.git`, mutate a consumer,
or claim release provenance. It does not make source formatting semantic.

## Delivery

Issue #512 owns the bounded repository-local slice. It follows closed A1
milestone #504 and should complete before an adapter mapping is used as release
provenance or generated bindings are introduced.

## Context

Pi 0.84.2 binds project package storage to `<cwd>/.pi/npm`. The factory must
reduce repeated worktree-local state without pretending Pi offers a relocation
setting that it does not.

## Decisions

### Preserve Pi's path through a validated symlink

Bootstrap prepares `.pi/npm` as an ignored symlink. Its target is a hidden
repository-specific sibling of the primary checkout, placing generated package
state outside every registered working tree while keeping Pi's `cwd` and
project resource discovery unchanged.

### Address caches by effective inputs

The cache identity is a schema-versioned digest of the exact ordered package
sources plus pinned Pi, Node and npm versions. A package or runtime change
selects a new directory; it never mutates a cache for a different identity.

### Publish complete caches, not mutable staging state

An initializer installs the exact npm sources into its own staging directory
with lifecycle scripts disabled, verifies the requested top-level versions,
writes a closed metadata marker and atomically promotes the directory. Two
initializers may do duplicate first-time download work, but only a verified
complete directory becomes canonical. Later launches are read-mostly and
reuse it directly.

### Fail closed around operator data

The preparer never replaces an existing regular `.pi/npm`, follows an
unexpected symlink or automatically prunes stores. Diagnostics give exact
recovery locations. A root ignore entry is defense in depth for a raw Pi run,
not evidence that the canonical cache boundary was established.

## Risks and mitigations

- Pi can modify its package directory: exact pins and verification detect
  identity drift; a future runtime/package input receives a different cache.
- Concurrent first launch can race: isolated staging plus atomic promotion
  prevents partial canonical state.
- Symlinks can escape intended storage: both link target and store ancestry
  are canonicalized and checked against the derived repository boundary.
- A raw Pi run can leave a large directory: Git ignores it, while bootstrap
  refuses destructive replacement and documents manual migration/removal.
- External state can accumulate: retention is visible and manual in this
  slice; measured growth can justify a later bounded cleanup policy.

## Rollback

Revert the bootstrap/cache code and remove the ignored worktree symlink. The
external content-addressed directory is left intact for recovery and may be
removed manually by exact path when unused.

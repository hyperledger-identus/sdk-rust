# Design

## Context

Issue linkage is evaluated by protected-base code against untrusted PR metadata
and an exact fetched head object. Ordinary delivery closes its branch issue at
merge. A smaller class of acceptance work needs the issue to remain open until
post-merge evidence exists.

## Decisions

### Make linkage mode explicit

The PR template gains `Issue linkage: closes/deferred`. `closes` requires a
standalone closing-keyword line for the branch issue. `deferred` requires a
standalone `Refs` or `References` line for that issue and forbids a closing
keyword for it. Metadata inside fenced code or HTML comments has no authority.

### Bind deferral to archived OpenSpec evidence

Deferred bodies name exactly one path matching
`openspec/changes/archive/YYYY-MM-DD-<change>/post-merge-acceptance.json`.
Protected-base code reads only that bounded blob from the exact fetched PR head
through Git object plumbing; it never checks out or executes head code.

The closed sidecar schema contains `schemaVersion`, `issue`, `disposition`,
`rationale`, and `acceptance`. The issue must match the branch, disposition must
be `deferred`, rationale must be nonempty, and acceptance must contain one to
sixteen unique stable identifiers. Duplicate keys, unknown fields, symlinks,
oversized blobs, invalid paths, malformed JSON, and mismatches fail closed.

### Reuse one validator locally and in hosted policy

`validatePullRequest` owns linkage parsing and accepts an injected exact-head
sidecar loader for deterministic tests. The CLI binds the loader to the hosted
`HEAD_SHA`; file-backed delivery binds it to the current committed head. The
shell layer continues extracting the explicit `Issue` field for repository
existence verification and gains the explicit linkage field check.

### Keep deferred completion honest

The deferred sidecar does not satisfy or auto-close acceptance. The issue owner
must later attach the named evidence and close the issue normally. Metrics,
release authority, workflow dispatch, and branch protection remain unchanged.

## Error and privacy contract

Diagnostics identify only the failed linkage invariant, issue number, or
bounded sidecar path. They do not echo PR bodies, sidecar rationale, acceptance
content, tokens, or GitHub responses.

## Rollback

Revert the additive mode and restore mandatory closing linkage. No SDK data,
package, remote setting, or consumer migration is involved.

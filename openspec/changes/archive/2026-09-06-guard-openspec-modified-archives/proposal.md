# Proposal: guard OpenSpec modified-requirement archives

## Why

Issue #102 records a reproducible integrity gap in the SDK's AI Software
Factory. OpenSpec 1.5.0 correctly treats a `MODIFIED` requirement as a complete
replacement, but strict structural validation does not prove that the delta
contains the complete updated block. An abbreviated delta can therefore delete
unrelated canonical prose or scenarios while every structural check remains
green.

## What Changes

Add an offline repository preflight that rejects silent destructive
`MODIFIED` replacements before archive, expose archiving through the portable
factory facade, and document an auditable escape hatch for an intentional
rewrite. The guard protects future agent work without changing existing
canonical specs or archived evidence.

## Capabilities

- `spec-driven-delivery`: require canonical-content preservation for active
  `MODIFIED` deltas and exact, reasoned acknowledgement for intentional
  replacement.
- `ai-software-factory`: add a guarded `archive` operation to the portable
  factory entrypoint.

## Scope

- Parse active delta and canonical Markdown with a deterministic Python
  standard-library checker.
- Permit additions when every existing canonical nonblank line remains in
  order.
- Require an optional `archive-intent.toml` entry with the exact normalized
  canonical requirement SHA-256 and a nonempty rationale when any canonical
  line is rewritten or removed.
- Resolve a `RENAMED` source before evaluating a modified new-name block.
- Reject missing, stale, duplicate, malformed, or unused intent entries.
- Run the checker from `scripts/factory check`, the Nix factory contract and a
  new `scripts/factory archive <change>` preflight.
- Add mutation fixtures for the issue #102 loss, safe additive/full deltas,
  intentional replacement, stale acknowledgement, rename plus modification,
  and a new capability.

## Non-scope

- Forking or patching OpenSpec.
- Inferring whether a semantic rewrite is correct.
- Rewriting historical archives or canonical requirements.
- Product, protocol, crypto, consumer, publication, release, `main`, or
  repository-setting changes.

## Compatibility and rollback

The change affects repository tooling only. Existing active additive deltas
remain valid when complete; existing archives are not re-evaluated. A future
intentional rewrite gains one archived audit file and no canonical marker.
Rollback removes the checker, its tests, the factory command and the added
contract text; no SDK API or wire migration is required.

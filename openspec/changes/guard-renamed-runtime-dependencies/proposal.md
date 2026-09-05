# Guard renamed workspace runtime dependencies

## Why

PR #92 hardened target-specific runtime-edge inspection, but hosted review
identified one remaining alias bypass: a dependency entry whose key differs
from its `package` identity is not recognized as a workspace edge. Issue #93
tracks the correction before another delivery slice relies on the guard.

## What changes

- Resolve runtime dependency entries to their declared `package` identity,
  falling back to the dependency key when no rename exists.
- Apply the identity rule to top-level and target-specific runtime tables.
- Deduplicate aliases that resolve to the same canonical workspace package.
- Add regression coverage for renamed allowed and unauthorized leaf edges.

## Non-goals

This change does not alter the allowed layer graph, inspect dev/build edges as
runtime edges, change a production crate, or add a dependency.

## Impact

- **Issue:** #93; follow-up to #92 and #91.
- **Owner:** verification-only `identus-conformance` guard.
- **Compatibility:** guard hardening only; no SDK runtime API changes.

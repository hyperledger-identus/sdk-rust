# Harden the code-health classifier

## Why

The private `syn` classifier is the last known production module above the
1,000-authored-line attention threshold. It also owns security-sensitive
fail-closed population semantics, while its current evidence binds only a
logical classifier name and output projection rather than the implementation
and resolved parser dependency set. Issue #301 records five correctness and
evidence findings deferred from the original classifier migration.

## What changes

- Decompose protocol, cfg evaluation, AST span collection, module reachability,
  projection, and orchestration into private cohesive modules.
- Correct conditional path evaluation and lexical target-path normalization.
- Add deterministic differential module fixtures, bounded generated cases, and
  worst-case complexity checks routed between fast and weekly evidence.
- Bind the complete classifier source set and resolved `syn`/`proc-macro2`
  dependency versions into canonical policy and report evidence.
- Mechanically verify migration-document digests against canonical policy and
  report content.

## Capabilities

### Modified capabilities

- `code-health-governance`: strengthen classifier correctness, maintainability,
  implementation identity, and bounded evidence.

## Non-goals

No SDK crate is published, no public SDK API or wire format changes, no new
dependency is added, macro expansion remains out of scope, and conservative
production ownership is not relaxed.

## Delivery

Issue #301 owns the slice. The issue-linked pull request targets protected
`develop` and may merge only after focused classifier tests, factory evidence,
workspace gates, exact baseline regeneration, and hosted CI are green.

# Isolate DID registration result validation

## Why

No production module exceeds the 1,000 authored-nonblank-line attention
threshold, but `validate_result` remains an unowned function signal at 75 SLOC,
cognitive complexity 18, and cyclomatic complexity 28. It combines job/method
correlation, lifecycle-shape dispatch, terminal payload checks, action/wait
continuation checks, document-metadata identity, and public-extension policy.
Issue #459 requires combined-fault characterization before movement so a
maintainability refactor cannot change the first public `RegistrationError`.

## What changes

- Bind every state/job shape and representative combined-fault precedence
  before production edits.
- Give one private borrowed validator explicit job, lifecycle-state,
  document-metadata, and extension-validation phases.
- Keep terminal, action, and wait validation cohesive without creating one
  helper per predicate.
- Preserve every accepted state, returned error, bounded collection, iteration
  order, validation call, and retained public value.
- Remove the touched function signal only if the resulting private ownership
  remains easier to review than the current match.

## Capability

### Modified capability

- `did-registration-module-ownership`: registration result-state validation has
  explicit private phase ownership while the public lifecycle contract remains
  unchanged.

## Non-goals

No registration vocabulary, adapter behavior, new lifecycle state, limit,
error, serialization, public helper, dependency, feature, allocation class,
request validation, public-data policy, registrar execution, or performance
budget.

## Delivery

Issue #459 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.

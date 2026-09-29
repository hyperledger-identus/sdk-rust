# Design

## Private ownership boundary

`validate_result` remains the private entry point used by
`DidRegistrationResult::new`. It constructs one private borrowed validator over
the method, optional job, state, and document metadata. The validator exposes
sequential internal phases for job-method correlation, exhaustive lifecycle
state validation, document-metadata identity, and extension policy.

The lifecycle phase uses the existing exhaustive `(state, job)` match and
delegates only substantive variant invariants: terminal-finished, terminal-
failed, action, and wait. There is no generic rule engine, callback pipeline,
public helper, owned mirror, helper per conditional, or forwarding-only chain.

## Exact validation contract

The design preserves this order and behavior:

1. job method mismatch yields `JobMismatch` before any state failure;
2. state/job shape dispatch selects the same exhaustive arm;
3. finished handle overflow precedes method/document mismatch, which precedes
   public-document validation;
4. failed optional-DID method mismatch yields `MethodOrDidMismatch`;
5. action optional-DID or continuation/action mismatch yields
   `ActionMismatch` without exposing which predicate failed;
6. wait optional-DID, continuation, or advisory-limit mismatch yields
   `InvalidState` without exposing which predicate failed;
7. unmatched terminal/non-terminal job shape yields `InvalidState`;
8. document metadata is checked only after state success, and any failure maps
   to `MethodOrDidMismatch`; and
9. document-metadata extension policy is checked last and retains its exact
   registration error.

Successful construction retains the exact caller-owned values and adds no work
or allocation class.

## Characterization boundary

Before production movement, one table-driven matrix binds every state/job
shape and representative combined faults across job method, handle count,
DID/document identity, public-document private material, action continuation,
wait continuation/bound, document metadata identity, and document-metadata
extension policy. The matrix asserts exact `RegistrationError`, not only
`is_err()`.

## Compatibility and code-health ratchet

The normalized public API, manifests, errors, and wire behavior remain
identical. The touched signal must disappear without an equivalent large
method, repeated state matching, generated code, moved test, waiver, allocation,
or weaker threshold. Canonical evidence is rebound to the protected
implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are changing fail-fast priority, returning a more specific error
for action/wait combined predicates, skipping public-document validation,
changing metadata error mapping, cloning metadata extensions earlier, or
accepting an invalid state/job shape. The combined-fault matrix, existing
registration suite, public/source diff, and exact-diff review make those
visible. Rollback restores the single validator body without consumer or data
migration.

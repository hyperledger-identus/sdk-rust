# Design

## Closed policy model

The base checker validates exact version-1 keys and bounded values. Policy
fields are classified by enforcement semantics:

- `types`, `scopes`, signature envelopes, bot identities, and bot DCO author
  names are allowances; the proposed sets may only be subsets.
- `maxSubjectLength` and `maximumRange` are ceilings; proposed values may only
  decrease.
- `requireScope`, `requireDco`, `requireSignature`, and
  `requireBreakingChangeFooter` are enforcement flags; only false-to-true is a
  strengthening transition.
- `branchExempt` is an exemption flag; only true-to-false is strengthening.
- branch formats and protected names are currently descriptive because the
  executable grammar is fixed in the base checker; they must remain
  set-equivalent until a separate ADR gives them semantics.
- schema version must remain exact.

Every array is bounded, unique, and non-empty where an empty policy could make
safe recovery impossible. Unknown fields and malformed values fail closed.

## Untrusted head ingestion

After the workflow has imported and SHA-bound the numbered PR head object, the
base-owned command reads only `<HEAD_SHA>:.github/contribution-policy.json`
through Git plumbing. Blob size is checked before bounded duplicate-rejecting
parsing. No proposed module, workflow, dependency, or script is loaded.

## Intentional relaxation

This slice provides no single-PR waiver. A future relaxation requires a
separate two-integration ADR that first teaches the protected base an exact,
reviewable approval mechanism. This prevents the proposed relaxation from
authorizing itself.

## Verification and rollback

Table-driven mutations cover every field class, unknown/duplicate/malformed
input, deletion, size bounds, equality, strengthening, and relaxation. Workflow
source tests prove that exact head data is passed to the base command. Rollback
removes the command and workflow step without changing existing policy.

# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/341
Constraint blockers: none

## Existing entries affected

The executable pull-request integration policy in `ai-software-factory`, the
hosted policy workflow, and repository workflow-security guidance gain an
explicit base-owned trust root. Existing signed/DCO, issue linkage, local
review, branch, signature, synchronization, and read-only permission rules
remain effective.

## Introduced or changed constraints

- Workflow and checker code SHALL come from the event's exact protected base
  revision, not the pull-request merge or head tree.
- Checkout credentials SHALL NOT persist and the checkout identity SHALL be
  verified before repository code executes.
- Pull-request-controlled content is data only. The head tree SHALL NOT be
  checked out, installed, sourced, imported, or executed.
- If head Git objects are needed, only the numbered pull-request ref is fetched
  and its object identity SHALL equal the event head SHA.
- Permissions remain explicitly read-only and no secret enters the job.

## Introduced or changed limitations

This slice does not prevent a policy weakening that has already merged to the
protected base. It does not yet require the proposed head policy to be at least
as strict as the base policy; #339 owns that monotonicity decision. The PR that
introduces this event change is still evaluated by the old base workflow and
temporarily retains that legacy trigger, so natural end-to-end evidence and
legacy-trigger removal begin with an immediate canary PR. No unrelated PR may
merge between the bootstrap and canary.

## Consumer and product impact

No Rust, SSI, cryptographic, wire, target, package, or consumer behavior
changes. Contributors retain the same visible policy requirements; only the
authority that evaluates them changes.

## Activation and rollback

Activation requires planning and preimplementation receipts, focused contract
and workflow lint, full factory validation, signed/DCO review, protected CI,
and merge to `develop`. Rollback is a repository revert.

## Evidence

Source mutation tests, existing contribution-policy tests, actionlint,
yamllint, factory gates, exact-diff review, signed/DCO provenance, and hosted
CI are mandatory. A later natural PR records the first hosted execution of the
new base-owned workflow.

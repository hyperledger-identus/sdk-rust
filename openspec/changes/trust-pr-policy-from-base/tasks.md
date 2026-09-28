# Tasks

## 1. Research and specification

- [x] 1.1 Confirm issue #341 and inspect the exact base-owned policy surface.
- [x] 1.2 Record the trust decision, security constraints, official sources,
  non-goals, and rollback in OpenSpec and ADR 0166.
- [x] 1.3 Pass research/constraint readiness and bind the committed planning
  contract with an issue-bound preimplementation receipt.

## 2. Implementation

- [x] 2.1 Move the hosted policy to a base-context event and exact base-SHA
  checkout with disabled credential persistence and identity verification.
- [x] 2.2 Fetch and bind only the exact head Git object needed for base-owned
  history inspection without checking out or executing the head tree.
- [x] 2.3 Add workflow trust-boundary regression tests and update governance
  guidance and the canonical factory specification.

## 3. Review and delivery

- [x] 3.1 Run focused/full gates and an exact-diff security review.
- [x] 3.2 Prepare the completed change for archive and an issue-linked signed/DCO
  PR; integration and metrics remain gated by exact-head protected CI.

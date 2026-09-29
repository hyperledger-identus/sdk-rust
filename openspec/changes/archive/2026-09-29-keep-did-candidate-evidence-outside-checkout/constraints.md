# Constraint impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/480
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-003`: the existing closed compiler/host/target matrix is
  preserved exactly.
- `SDK-DELIVERY-001`: candidate qualification stays weekly/manual and does not
  add a required PR lane.
- `SDK-REL-001` and `SDK-REL-002`: the DID train remains candidate-only and
  this workflow cannot publish it.
- `SDK-LIM-003`: portable results remain compile-only evidence.

## Introduced or changed constraints

Candidate lane output must be outside the Git checkout whenever subsequent
qualification shares that checkout. The workflow must prove the checkout is
still clean between primary and MSRV execution. Neither rule relaxes the
candidate builder's own clean-source precondition.

## Introduced or changed limitations

Required PR CI can validate only workflow policy and offline mutations. It
cannot substitute for a later native Linux/macOS slow execution. The repair
does not authorize dispatch or rerun and creates no new runtime/support claim.

## Consumer and product impact

None. Only evidence placement inside an ephemeral hosted job changes. Artifact
name, content, retention, exact-SHA binding, and downstream aggregation remain
stable.

## Activation and rollback

Workflow, checker, tests, and specification activate atomically on protected
`develop`. Rollback is a normal repository revert, but it reintroduces the
known slow-line failure and cannot be represented as passing evidence.

## Evidence

Planning readiness, structural mutations, workflow lint, factory/OpenSpec,
Nix, review, and required CI are required. Final hosted slow evidence remains
owned by #388.

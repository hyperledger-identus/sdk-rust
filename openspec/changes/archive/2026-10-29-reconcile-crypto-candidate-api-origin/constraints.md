# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/587
Constraint blockers: none

## Existing entries affected

Only crypto candidate comparison and its release evidence are affected. The
published API-origin file remains byte-identical. The canonical candidate-train
and unpublished-crypto-candidate specifications govern the change.

## Introduced or changed constraints

- Generation uses Rust 1.98.1 and cargo-public-api 0.52.0 through the existing
  candidate command.
- The descriptor binds the published-origin status and exact baseline SHA-256.
- Compatibility uses line multisets: every published line and occurrence must
  remain; additive current lines are allowed and reported.
- `--initialize-api` fails while the descriptor selects a published origin.
- Ordinary preparation emits deterministic current/diff evidence and succeeds
  only with an empty removal multiset.

## Introduced or changed limitations

The baseline represents the already-published rc.1 artifact; this change makes
no new stability promise beyond compatibility against that predecessor. It does
not authorize publication, tagging, promotion, downstream mutation, or slow-run
acceptance. Additions remain visible evidence rather than a new frozen origin.

## Consumer and product impact

Consumers see no change: the methods are already present. Maintainers retain
honest release history and gain machine-checkable compatibility evidence.

## Activation and rollback

Activation requires issue-bound preflight, exact multiset diff, report and
mutation checks, normal candidate E2E, full gates, independent CLI review,
green protected CI, and guarded merge. Rollback restores exact-equality tooling
but not the baseline; doing so would reintroduce rejection of additive evolution.

## Evidence

The immutable baseline hash, generated current hash and multiset diff, report,
mutation suite, normal candidate output, factory/full Nix gates, independent
review, protected CI, guarded merge SHA, and subsequent fresh slow run form the
evidence chain.

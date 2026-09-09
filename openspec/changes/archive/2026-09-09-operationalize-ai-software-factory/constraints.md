# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/243
Constraint blockers: none

## Existing entries affected

- `SDK-AGENT-001` and `SDK-AGENT-002`: preserve standing agent authority and
  the protected human boundary exactly.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005`: keep Rust 1.98.1
  as the active-development floor and etalon; factory runtime packages do not
  alter consumer compiler support.
- `SDK-SEC-001`: no authored Rust unsafe code is introduced.
- `SDK-LIM-006`: no release or publication claim is activated.

## Introduced or changed constraints

The factory adds repository-local operational constraints: issue-bound work,
one mutating owner per managed worktree, an OpenSpec pre-implementation
receipt, exact-head evidence, bounded runtime/concurrency/disk policy and
privacy-safe metrics. They govern maintainers and agents, not SDK consumers.

## Introduced or changed limitations

The first delivery does not provision cloud workers, mutate GitHub rulesets,
provide atomic cross-host leases, automate releases, activate milestone
trains, or claim that Pi can deliver every SDK issue. Hosted CI retains the
existing full fast lane rather than becoming a dynamic multi-lane matrix.

## Consumer and product impact

Rust API, wire formats, MSRV, features and target support are unchanged.
Maintainers gain an optional pinned Pi workflow and stronger local evidence.
Oxid, Apollo, NeoPRISM, Midnight and Lace remain unchanged.

## Activation and rollback

The controls activate only after issue #243's PR is reviewed, all required CI
is green and it merges to `develop`. Reserved `main` is not activated.
Rollback is a focused revert. Package/version tuning follows only from the
post-merge Pi canary and a new issue-backed change.

## Evidence

The implementation will include machine-contract tests, known-bad fixtures,
Nix evaluation, factory validation, local review and exact-head hosted CI.

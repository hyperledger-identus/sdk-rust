# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/247
Constraint blockers: none

## Existing entries affected

- `SDK-AGENT-001` and `SDK-AGENT-002`: routine harness tuning remains within
  standing authority and does not cross protected product or release gates.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005`: Rust 1.98.1 and the
  pinned active-development toolchain remain unchanged.
- `SDK-LIM-006`: no release, publication or `main` activation is introduced.

## Introduced or changed constraints

No SDK consumer constraint changes. The local factory gains an operational
invariant: bootstrap-launched project packages use a complete cache whose
identity matches the current exact runtime and package declarations.

## Introduced or changed limitations

Pi 0.84.2 has no dedicated project-package storage override. The factory uses
an ignored `.pi/npm` symlink to preserve its path contract. Raw Pi may create
an ignored local directory; bootstrap will refuse to overwrite it and report
manual recovery. Cache retention remains operator-managed without automatic
pruning.

## Consumer and product impact

Rust APIs, wire behavior, dependency cone, compiler floor, targets, feature
sets and downstream repositories are unchanged. Maintainers avoid repeated
per-worktree Pi package installations for identical pinned configurations.

## Activation and rollback

The behavior activates only through `./bootstrap.sh --pi` after the issue #247
PR merges to `develop`. Rollback is a focused revert; existing external caches
remain recoverable and can be removed manually after confirming no Pi process
uses them. `main` remains reserved.

## Evidence

Contract tests will cover identity stability, changed-input separation,
external containment, unsafe/symlink rejection, concurrent convergence and
clean worktree status. Hosted `fast` remains the required integration gate.

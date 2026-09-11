## Why

The milestone retrospective in discussion #256 found a narrow but important
operator mismatch. `./bootstrap.sh --check` validates repository structure and
the factory test suite but does not audit the effective pinned Pi runtime.
Calling `scripts/factory audit` directly outside the Nix shell then reports
expected host drift instead of using the repository-pinned environment.

Issue #257 owns the focused correction. The effective harness already passes
when invoked through `./bootstrap.sh --audit-pi`; this change makes the normal
health-check surfaces match that reality without upgrading anything.

## What changes

- Make `scripts/factory audit` enter the pinned repository Nix shell when the
  caller has not already entered a Nix shell.
- Keep audit execution direct inside a Nix shell so re-entry cannot recurse.
- Make `./bootstrap.sh --check` include the effective runtime audit between
  structural checks and the operational test suite.
- Add known-good and known-bad tests for routing and failure propagation.
- Document the single-command health contract and retrospective evidence.

## Capabilities

### Modified capabilities

- `ai-software-factory`: makes the portable factory audit command use the
  pinned runtime outside an entered Nix shell.
- `factory-operations`: makes the bootstrap health check cover structural,
  effective-runtime and operational-test evidence.

## Non-goals

- No Pi, Node, npm, Rust, Nix input or project-package upgrade.
- No model, provider, credential, session or personal configuration change.
- No automatic cache or worktree deletion and no new performance threshold.
- No Rust API, product, consumer, release, publication or `main` change.

## Delivery

Issue #257 owns this routine control-plane correction from
`develop@04b45b7fceb094ae601e3cf7a3291de0a0248b57`. The PR will target
`develop` through the normal exact-head factory and hosted `fast` gates.

## Why

The temporary active-development policy requires one rapid Linux merge signal,
but three recent successful pull-request runs spent a median 393 seconds in
`Post Magic Nix Cache` after the substantive gate had completed. That teardown
work is not correctness evidence and currently takes longer than the median
substantive gate itself.

Issue #260 owns a bounded correction: retain cache reads where available, deny
cache publication on the pull-request critical path, and make cache failure a
visible performance degradation rather than a correctness failure.

## What changes

- Give the `fast` job explicit read-only GitHub Actions cache authority.
- Keep the pinned Magic Nix Cache action as a restore accelerator, but select
  GitHub Actions cache explicitly, disable FlakeHub and diagnostics, and allow
  cache setup/failure to degrade to an ordinary Nix cache miss.
- Bound the complete required job to 20 minutes so any action finalizer cannot
  hold the required status open indefinitely.
- Extend offline support-policy validation so the fast-lane cache boundary and
  unchanged substantive gate set cannot drift silently.
- Record the three-run baseline, first exact-head canary and three-run
  post-change follow-up target.

## Capabilities

### Modified capabilities

- `sdk-support-policy`: adds the temporary fast-lane cache authority,
  degradation and latency contract.

## Non-goals

- No Rust, Nix, action, dependency or flake-lock upgrade.
- No removal or weakening of factory, lint, format, build, Clippy or test gates.
- No FlakeHub registration, credential, paid cache, repository-setting or
  billing change.
- No change to weekly/manual slow cache plumbing.
- No release, `main`, downstream or compatibility commitment.

## Delivery

Issue #260 owns this routine factory change from
`develop@71ae2fe82b65e1e2da513ad9091ed9709d10a756`. The PR targets `develop`.
The policy is reviewed no later than 2026-12-08 with the temporary Rust 1.98.1
fast/slow decision.

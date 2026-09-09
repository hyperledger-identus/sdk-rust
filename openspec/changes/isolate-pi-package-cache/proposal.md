## Why

The first issue-backed Pi canary, issue #246, reproduced a concrete harness
friction: Pi 0.84.2 installed the three exact project packages into
`.pi/npm/` in each working tree. One installation used 168,177,664 bytes and
19,386 files, and the generated directory made the worktree appear dirty.

Issue #247 owns a focused tuning response. The factory notes remain guidance;
this change does not use the observation as authority for a Pi, Node, package,
model or provider upgrade.

## What changes

- Prepare one repository-private, content-addressed Pi package store outside
  the repository and linked worktrees before bootstrap launches Pi.
- Key each store by the effective pinned Pi/Node/npm runtime and ordered exact
  project package declarations so different package sets cannot share state.
- Preserve Pi's expected `.pi/npm` path as an ignored symlink to that store.
- Fail closed on an existing directory, wrong symlink, unsafe store path,
  malformed package declaration or incomplete cache instead of deleting or
  replacing operator data.
- Add a root ignore fallback for `.pi/npm/`, contract tests, audit output and
  documented retention/recovery.

## Capabilities

### Modified capabilities

- `factory-operations`: adds a clean, bounded and recoverable package-cache
  boundary to the pinned Pi bootstrap.

## Non-goals

- No Pi, Node, npm or Pi package upgrade.
- No npm install-script approval, user-level Pi configuration, credential,
  model, provider, session or transcript handling.
- No automatic cache pruning or deletion of an existing worktree cache.
- No Rust API, target, CI-lane, release, downstream or `main` change.

## Delivery

Issue #247 owns this tuning slice from
`develop@39fd55666faf54fe237e35cb611c462df064cd38`. It will use the same
planning receipt, review, exact-head CI and `develop` merge flow proven by the
canary.

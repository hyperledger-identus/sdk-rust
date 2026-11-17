## Why

Protected `develop` at `f7e777b8556ba4c82bdc272faf2551e7bfb9d8f8`
failed its latest natural weekly slow run because the isolated UniFFI generator
lock resolves vulnerable `rustix 1.1.4`. The product sponsor also directs the
SDK to retire its Rust 1.89 compiler floor and standardize ordinary SDK
compatibility on the Rust 1.99 line.

The repair must be one atomic compatibility change: upgrade the exact Nix
toolchain and machine policy, move the next unpublished candidate to the
`0.2.x` line, patch the isolated advisory, and preserve truthful downstream
and rollback evidence.

## What changes

- Raise the workspace, release-candidate, primary, and etalon compiler contract
  to exact Rust 1.99.1.
- Move the unpublished candidate policy from `0.1.0-rc.1` / `0.1.x` to
  `0.2.0-rc.1` / `0.2.x`; retain historical 0.1 evidence as history.
- Pin a newer exact `oxalica/rust-overlay` revision that contains Rust 1.99.1
  and update the flake lock.
- Update current support policy, constraints, documentation, candidate
  generators, fixtures, and mutation tests without editing accepted ADRs.
- Resolve the UniFFI generator lock to patched `rustix >= 1.1.6` and prove the
  supply-chain audit.
- Record current NeoPRISM compiler compatibility without mutating NeoPRISM or
  Apollo.

## Capabilities

### Modified capabilities

- `sdk-support-policy`: activates Rust 1.99.1 as the common MSRV, primary, and
  etalon compiler for the next `0.2.x` candidate line.

## Non-goals

- No public Rust API or protocol behavior change.
- No crate publication, release, stable-API claim, `main` promotion, protected
  setting change, or downstream source mutation.
- No removal of target, feature, host, audit, or advisory gates.
- No claim that historical `0.1.x` evidence was produced on Rust 1.99.1.

## Delivery

Issue #596 owns this base repair. The SDK-TS DID Document capability branch is
stacked above it and must stop if this base has a real required-CI failure.

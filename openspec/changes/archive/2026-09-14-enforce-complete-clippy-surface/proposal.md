# Enforce the complete Clippy surface

## Why

The required fast lane intentionally checks the default workspace surface, but
the documented local command also selects every Cargo target and every feature.
On `develop@dbef9923e65d1a8332c4ba38f42532c8a12811f7` that complete command
finds an initial four warnings which the lean fast lane does not see; after
those stopped compilation, complete reruns expose nine more findings in later
targets. The weekly/manual
slow lane currently runs every Nix flake check, but it has no separately named
check that represents this complete Clippy surface.

Issue #268 owns the bounded correction: remove all thirteen warnings, make the
complete command a generated Nix check in the slow lane, and record why the
two production OID4VCI constructor exceptions remain narrow for now.

## What changes

- Fix three nested conditional findings in conformance guards, replace nine
  hand-written no-op wakers with the standard-library waker, and use the
  standard integer multiple predicate in one DID hardening test.
- Add a generated `rust-clippy-all-targets-all-features` Nix check whose Cargo
  selection is the exact workspace/all-target/all-feature warning-denied
  surface.
- Make the weekly/manual slow workflow invoke that named check explicitly on
  Linux and macOS before its complete flake check.
- Record a repository lint-exception policy and convert the two production
  OID4VCI exceptions from open-ended allowances to reasoned expectations.
- Strengthen the offline support-policy validator and mutation suite so the
  complete gate cannot silently lose a target, feature, warning denial, host,
  or slow-lane selector.

## Capabilities

### Modified capabilities

- `nix-tooling`: add a complete Clippy surface to weekly/manual evidence while
  leaving the required fast selector set unchanged.

## Non-goals

- No feature, wire, public API, dependency, compiler, target, MSRV, or nightly
  change.
- No addition of the complete Clippy surface to required per-PR fast CI.
- No broad lint suppression and no redesign of OID4VCI limit constructors.
- No work on issue #7 behavior or issue #168 resource-limit semantics.
- No downstream repository change.

## Delivery

Issue #268 owns this routine repository-local change from exact base
`dbef9923e65d1a8332c4ba38f42532c8a12811f7`. The ready pull request targets
`develop`; publication, release, `main`, and downstream adoption remain out of
scope.

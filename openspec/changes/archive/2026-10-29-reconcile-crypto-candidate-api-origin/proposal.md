# Protect the published crypto API origin

## Why

Issue #586 repaired the package boundary and allowed normal crypto candidate
qualification to reach the public-API comparison. The committed unpublished
`identus-crypto 0.1.0-rc.1` baseline predates four validation methods already
merged for strict DID-key handling. The train is already published and tagged,
so replacing that origin would falsify release history. Exact equality now
incorrectly rejects compatible additive development.

## What Changes

- Pin the published rc.1 API baseline digest and forbid reinitialization while
  that published origin is selected.
- Compare generated current API to the published origin as a multiset:
  additions are compatible, while every removal or changed signature fails.
- Emit a bounded machine-readable compatibility report containing exact
  baseline/current digests and added/removed lines.
- Prove the present delta is exactly the four reviewed methods and their
  cargo-public-api re-export views, then rerun normal candidate qualification.

## Capabilities

### Modified Capabilities

- `release-candidate-trains`: distinguish immutable published API origins from
  prepublication rebaselines.
- `unpublished-crypto-candidate`: require current crypto qualification to retain
  and compare against the published origin rather than overwrite it.

## Impact

This is behavior-neutral release tooling owned by issue #587. It changes no
Rust source, API, wire behavior, feature, dependency, package version, target,
publication state, tag, or downstream repository. It preserves rather than
rewrites published evidence and does not authorize release or slow-run approval.

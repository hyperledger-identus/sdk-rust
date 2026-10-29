# Rebaseline the DID candidate API origin

## Why

After #581 repaired the staged dependency lock, normal DID candidate
qualification reaches the public-API gate and rejects a stale
`identus-did` first-candidate snapshot. The product source already exposes the
generic bounded JSON-wire preflight used by DID document, resolution, and peer
DID paths. The candidate cannot advance while its committed API origin omits
that reviewed surface.

## What changes

- Record a machine-readable exact old/new API-origin rebaseline receipt.
- Prove the diff is additive and limited to `JsonWireLimits`, non-exhaustive
  `JsonWireError`, and `validate_unique_object_names`.
- Replace only the `identus-did` API baseline with generated exact-source
  output; retain the byte-identical resolver HTTP baseline.
- Rerun normal candidate qualification without initialization mode and update
  release-readiness knowledge.

## Capability

### Modified capability

- `release-candidate-trains`: governs a prepublication API-origin rebaseline
  with exact additive evidence and a normal-mode qualification receipt.

## Non-goals

No Rust source or behavior, package/version/feature/dependency, wire contract,
MSRV/target, canonical manifest, publish flag, tag, release, registry write,
downstream mutation, or stable SemVer promise changes.

## Delivery

Issue #582 owns this prerequisite to #388 from protected
`develop@88efe31e3d413e6f51b0cb80f65155281f8feb3c`.

# Qualify SD-JWT VC draft-22 differential oracles

## Why

`identus-sd-jwt-vc` now implements the exact draft-22 profile, but its
differential obligation remains pending. A local round trip and the released
SDK-TS legacy consumer cannot independently prove canonical profile behavior.
Publication therefore remains blocked until two maintained implementations are
frozen, exercised as behavioral oracles, and every disagreement is resolved
against the immutable draft-22 source.

## What Changes

- Freeze OpenWallet Foundation `@sd-jwt/sd-jwt-vc` 0.20.1 and K2GL
  `sd-jwt-vc` 2.1.0 as research-only differential oracles.
- Record exact revisions, package/archive checksums, licenses, toolchains,
  dependency locks, advisory results, source selectors, and upstream test
  receipts without adding either engine to the Cargo production graph.
- Add an immutable offline packet covering protected/media type, `vct` and
  `aka_vcts`, registered-claim placement, holder confirmation, Type Metadata,
  and status references.
- Add a draft-19-to-22 semantic delta and disagreement ledger. The pinned
  draft hash remains normative whenever an oracle is older, incomplete, or
  disagrees.
- Execute the neutral packet through `identus-sd-jwt-vc`, promote stable
  negative and boundary cases into the cross-language catalog, and satisfy the
  differential quality obligation only when the route passes.
- Keep the crate unpublished. Publication remains a separate reviewed release
  decision after all quality and slow-lane evidence is current.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `sd-jwt-vc-draft22`: make the existing conformance-evidence requirement
  executable against two immutable independent behavioral oracles.

## Impact

This is behavior-neutral governance and test evidence. It changes no public
Rust API, wire format, error contract, production dependency, supported target,
or downstream repository. Issue #573 owns the evidence. Issue #572 remains the
only owner of a possible opt-in legacy `vc+sd-jwt` input adapter.

## Why

The dependency portfolio directs agents to replace local OID4VCI form mechanics
with `form_urlencoded 1.2.2`. Exact source review found that its browser-style
parser deliberately preserves malformed percent escapes and decodes invalid
UTF-8 lossily, while the SDK specification requires both to fail. Using only
the compatible serializer would retain the checked length, allocation,
zeroization and strict decoder code, leaving too little payoff for a new
production dependency.

## What Changes

- Require parser-replacement research to prove strict accepted/rejected
  behavior, not merely standards-domain or happy-path parity.
- Correct `form_urlencoded 1.2.2` from `adopt` to `not-adopt` for the current
  OID4VCI boundary and retain the small local codec.
- Record the published artifact checksum, actual VCS revision, feature/cone,
  permissive behavior and objective reconsideration trigger.
- Preserve the existing OID4VCI form bytes, strict failures, resource bounds,
  secret ownership and public API without changing Rust implementation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `dependency-research-readiness`: candidate parser adoption must preserve the
  capability's fail-closed acceptance boundary or explicitly propose a
  reviewed specification change.

## Impact

This is a dependency decision correction. It changes no Cargo dependency,
lockfile, request/response bytes, API, error, target, compiler or downstream
repository. Future agents cannot replace a strict protocol parser with a lossy
browser parser under a general claim of standards reuse.

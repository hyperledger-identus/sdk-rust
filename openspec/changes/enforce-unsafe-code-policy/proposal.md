## Why

`SDK-SEC-001` prohibits unsafe first-party Rust unless a dedicated safety ADR
approves a bounded exception, but `SDK-LIM-008` records that the prohibition is
not uniformly machine-enforced. Six of 17 library roots add a local forbid and
11 do not; library-root attributes also do not govern separate Cargo targets.
The effective policy needs one inherited compiler gate and drift evidence.

## What Changes

- Set `unsafe_code = "forbid"` once in the inherited workspace Rust lints.
- Add a conformance guard that proves the root setting and every current/future
  workspace package's explicit lint inheritance.
- Add deterministic compile-fail probes for every supported authored
  first-party Cargo target class, including proc-macro implementations.
- Record ADR 0087, including a deliberately high-ceremony exception path that
  cannot silently weaken unrelated crates.
- Strengthen `SDK-SEC-001` enforcement and narrow `SDK-LIM-008` to rustc's
  attributed procedural-macro expansion exception, tracked by #189.

## Capabilities

### New Capabilities

- `unsafe-code-policy`: Uniform compiler enforcement and negative evidence for
  the SDK's default prohibition on first-party unsafe Rust.

### Modified Capabilities

- `conformance-crate-structure`: Adds one isolated unsafe-policy guard module
  using the existing manifest helpers.

## Impact

This is a material improvement to security assurance for existing policy. It
changes compiler lint configuration, verification-only conformance code,
governance records and tests. It does not add unsafe code, approve an
exception, lint dependencies, change dependencies/MSRV/targets/features, or
change any public API, wire value, secret, runtime, product or downstream
repository. Rustc does not apply the lint to every external procedural-macro
expansion, so generated-output enforcement remains explicitly limited. Issue
#169 under #166 is the decision authority; #189 owns that focused follow-up.

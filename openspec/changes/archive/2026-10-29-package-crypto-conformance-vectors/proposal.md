# Package crypto conformance vectors

## Why

The exact protected slow run for `develop@6ebd4633c987539689ae180fbccbb629572ca45a`
proved that the `identus-crypto` archive cannot compile its declared tests after
extraction. The compatibility test reads a repository-level fixture above the
crate root, but Cargo packages only crate-owned source. This blocks honest
candidate qualification even though workspace tests pass.

## What Changes

- Move the unchanged crypto compatibility packet into a crate-local,
  package-owned conformance path and make that path the single authority.
- Update the conformance catalog, quality declaration, Nix source filter, and
  tests to consume the canonical crate-local packet.
- Extend candidate evidence so the package listing and extracted test execution
  prove that every declared test input is present without repository files.
- Preserve vector bytes, digest, provenance, algorithms, APIs, errors, package
  versions, and publication state.

## Capabilities

### Modified Capabilities

- `unpublished-crypto-candidate`: require extracted archives to execute their
  declared all-feature test suite using only packaged inputs.
- `source-distribution`: require package-owned test inputs to remain inside the
  package boundary and be visible in Cargo's package listing.

## Impact

This is behavior-neutral source-distribution hardening owned by issue #586. It
changes fixture location and candidate evidence, not cryptographic behavior or
consumer-facing contracts. A new protected merge SHA must receive a fresh slow
run before issue #388 can complete.

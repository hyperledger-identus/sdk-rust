# Make support-policy fixtures deterministic

## Why

Issue #434 records six host-only failures in the support-policy mutation suite.
The production checker remains fail closed and the pinned factory derivation
passes, but test outcomes currently change when an arbitrary host
`nix-instantiate` happens to be discoverable on `PATH`.

## What changes

- Classify Nix mutation fixtures as parser-valid or intentionally parser-invalid.
- Repair the computed-import mutation so it is valid Nix with an explicitly
  bound unresolved import input.
- Exercise parse expectations only with the exact nixpkgs-pinned parser path
  injected by the factory derivation.
- Keep direct host test execution deterministic and independent of ambient Nix.
- Preserve every fail-closed support-policy assertion.

## What does not change

No SDK API, Rust code, support claim, Nix gate, contribution rule, dependency
policy, release artifact, or checker acceptance behavior changes.

## Capabilities

### Modified capabilities

- `ai-software-factory`: require deterministic support-policy mutation evidence
  with explicit parser-validity expectations.

## Authority

Issue #434 and the routine test-harness authority in ADRs 0003 and 0004.

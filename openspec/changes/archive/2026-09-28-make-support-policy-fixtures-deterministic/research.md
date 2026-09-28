# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

`SupportPolicyTests.assert_nix_parses_if_available` discovers
`nix-instantiate` from ambient `PATH`. The pinned `factory-contract`
derivation does not expose that executable, so it skips all parser assertions.
On a host with Nix 2.34.6, six assertions run and fail although the checker
correctly rejects every mutation.

Five mutations place an indented-string token where Nix grammar permits only
an identifier, quoted attribute, interpolation, `in`, or `inherit`; those
fixtures are intentionally parser-invalid. The sixth is syntactically valid,
but `nix-instantiate --parse` performs static name resolution and rejects the
free `localModules` variable before the checker can demonstrate unresolved
graph handling.

## Normative sources

- Issue #434 is the defect and acceptance contract.
- The canonical `ai-software-factory` specification requires reproducible,
  fail-closed factory evidence.
- `nix/checks/factory-contract.nix` owns the pinned execution environment.
- Nix expression syntax and `nix-instantiate --parse` behavior are exercised
  through the nixpkgs-pinned Nix package selected by the repository lock.

The exact repository base is
`f93d160a306c2c572e47751fb04e0d06c620f4c4`; no external donor or consumer
source is used.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep ambient `PATH` discovery | `not-adopt` | Test results depend on arbitrary host installation and version. | Never for required evidence. |
| Remove every Nix parse assertion | `not-adopt` | The checker could accept a mutation that Nix cannot parse, weakening test meaning. | Only if a stronger parser oracle replaces Nix. |
| Inject nixpkgs-pinned `nix-instantiate` into the derivation | `adopt` | Gives required CI one exact parser and keeps direct host runs deterministic. | The factory moves to a different pinned parser contract. |
| Mark grammar-invalid mutations as rejected by Nix and by the checker | `adopt` | Separates fail-closed lexical evidence from semantic bypass evidence. | The fixtures become valid Nix. |
| Bind `localModules` as a function argument | `adopt` | Produces valid Nix while retaining a dynamic unresolved import rejected by policy. | The checker gains a safe bounded representation for dynamic imports. |

## Compatibility and dependency evidence

No Cargo dependency, Rust target, MSRV, public type, wire format, crate feature,
native library, unsafe code, or consumer behavior changes. The Nix package is
already selected by the locked nixpkgs input; exposing its parser only inside
the factory-contract derivation does not add a runtime SDK dependency.

Direct `bash scripts/tests/factory-contract.sh` no longer changes behavior
based on ambient Nix. The pinned derivation exercises both parser-accept and
parser-reject expectations. Rollback restores nondeterministic host discovery
and is therefore undesirable but mechanically safe.

## Security, privacy and maintenance evidence

The production checker is unchanged. Every mutation remains rejected, and
parser-invalid evidence is explicitly double-closed: Nix rejects it and the
checker rejects it. The parser path is a fixed derivation input, not user or
fixture data. No network, secret, credential, telemetry, consumer repository,
or untrusted executable is introduced.

## Rejected or deferred candidates

Ambient version checks, downloading Nix in tests, weakening the checker,
removing mutation vectors, and accepting invalid fixtures as semantic bypasses
are rejected. A standalone Nix parser library is unnecessary for this bounded
harness correction.

## Open questions and blockers

None. If adding the pinned Nix input materially regresses fast-lane time, keep
the exact parser contract in a separate lightweight Nix check rather than
falling back to host discovery.

## Evidence commands

Planning: factory research/constraint readiness and issue-bound preflight.
Implementation: focused six-fixture reproduction, full support-policy suite,
direct factory-contract shell execution, pinned Nix factory-contract build,
factory checks, Nix evaluation, signed/DCO review, and protected exact-head CI.

# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation at revision
`b5d1a4c52d31f3d37564eee9c6edc890fc7b1295` exposes one owned text input,
`Url(String)`; three `u64` time newtypes; static-string component, capability
and error metadata; an enum; two clock ports; and a result alias. `Url` already
enforces `MAX_URL_BYTES = 8_192` before syntax traversal. Serde is enabled only
for `UnixTimestampMillis` and `DurationMillis`; serde derives their numeric
wire behavior from `u64`, but current tests prove a positive round trip rather
than negative, fractional and overflow rejection.

This slice inventories only `identus-core`. It does not imply that transports
or deserializers avoid allocating before validation, nor that other crates have
completed the repository-wide resource-bound audit.

## Normative sources

- [Serde data model](https://serde.rs/data-model.html) defines unsigned integer
  deserialization as a typed numeric input rather than a free-form string.
- [Rust `u64`](https://doc.rust-lang.org/std/primitive.u64.html) fixes the
  representable range to zero through `2^64 - 1`.
- The existing `core-error-conventions` specification is the authoritative SDK
  contract for URL and foundational error values.
- `SDK-SEC-003` and `SDK-LIM-007` are the effective security authority for
  changed boundaries and incomplete inherited audits.

The source revision above and exact workspace version 0.0.0 are pinned
provenance. The license remains Apache-2.0. This is not an SSI protocol or
draft-version decision; no external repository behavior is normative.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Crate-scoped inventory plus focused tests | `adopt` | Produces executable evidence without changing public behavior or adding dependencies. | A new public core input type or serde surface is added. |
| Documentation inventory without tests | `not-adopt` | Would restate inferred `u64` behavior without a regression gate. | Never while focused tests remain cheap. |
| Custom numeric deserializers | `not-adopt` | Existing serde/`u64` semantics already supply the intended range and custom code would create wire risk. | A named consumer requires a different numeric representation. |
| Remove `SDK-LIM-007` | `not-adopt` | One audited crate cannot prove repository-wide completeness. | Issue #168 completes every supported boundary with evidence. |

## Compatibility and dependency evidence

Public and wire compatibility remain unchanged: valid JSON integers continue
to deserialize, and invalid negative, fractional or greater-than-`u64` values
remain rejected by the same generated serde implementation. The facade remains
SDK-owned newtypes; no external types cross it. There is no new feature,
dependency, native code, build script, license, MSRV or target requirement.

The direct dependency cone remains `identus-derive` and `serde`; the resolved
normal cone remains their existing proc-macro and serde support packages. The
tests use the existing `serde_json` development dependency. The SDK continues
to target the repository's Rust 1.98.1 etalon and existing host, WASM, iOS and
Android evidence lanes.

## Security, privacy and maintenance evidence

The URL path has an explicit 8,192-byte accepted/retained and syntax-work
limit, with the existing caveat that callers, transports, decompressors and
generic deserializers may allocate first. Serialized time values have the fixed
`u64` range and constant-size retained state; malformed JSON rejection is
delegated to serde before construction. Monotonic time intentionally has no
serde implementation. Static-string metadata and errors cannot retain runtime
attacker strings, and the clock traits accept no input.

No secrets or PII are used. Tests must not log hostile payloads. Authored unsafe
and native-code evidence is unchanged: workspace `unsafe_code = "forbid"`
applies and core has no FFI/native edge. Supply-chain and maintenance posture
are unchanged because manifests and the lockfile do not change. The crate-level
inventory reduces review ambiguity but does not remove outer-budget duties.
The maintenance, release and security posture is therefore unchanged except
for stronger regression evidence; no publication or support promise is added.

## Rejected or deferred candidates

A validation crate, alternate URL parser, arbitrary-precision integer type and
custom serde visitor are rejected as unnecessary coupling for an evidence-only
slice. Repository-wide automation that infers bounds from source is deferred:
semantic ownership and outer-allocation caveats require human/agent review.

Rollback removes only the evidence document/spec clause and focused tests.
The reconsideration trigger is a new or changed `identus-core` input surface,
consumer wire requirement, serde behavior change, or completion of #168.

## Open questions and blockers

No blocker remains. Pi provider/model choice is personal runtime state and is
not a tracked SDK decision. Harness friction is observation only until a
separate issue authorizes a change.

## Evidence commands

- `git rev-parse HEAD` pins the current implementation revision.
- `rg` and direct source/manifest inspection enumerate core types, serde
  derives, constructors, dependencies, unsafe/native surfaces and consumers.
- Planned commands: focused `identus-core` tests, strict Clippy, factory
  readiness, exact-diff review, target plan and hosted fast CI.
- Unrun at planning time: implementation-dependent tests, Pi canary execution,
  full Nix validation and hosted CI; none is inferred as passing.

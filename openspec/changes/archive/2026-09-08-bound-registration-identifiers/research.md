# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation at pinned base revision
`04266d649d83d27213a11cea88cab064ec5d4811`, inspected for issue
[#209](https://github.com/hyperledger-identus/sdk-rust/issues/209), has one
private `opaque_identifier!` macro generating
`RegistrationIdempotencyKey`, `RegistrationJobId`, `RegistrationActionId`,
`RegistrationSecretHandle`, `RegistrationOperationName`, and
`RegistrationFailureCode`. Their `parse(&str)` clones before `try_new(String)`;
the shared validator checks empty, byte length, surrounding whitespace, and
control characters in bounded short-circuit order. Existing ceilings are 256
or 1,024 bytes and diagnostics redact retained values.

## Normative sources

No external normative source or donor code is required. The behavior is an SDK
resource policy already established by ADR 0015, `SDK-SEC-003`, and issue #168.
Apollo parity, NeoPRISM extraction, Midnight identity, and Oxid are unaffected.
The DID Registration source profile remains pinned to the DIF draft revision
recorded in ADR 0015; this internal allocation order does not change protocol or
draft currency.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Validate borrowed input directly, then allocate | `adopt` | Enforces the existing ceiling before the retained-value allocation with one success copy. | Rust ownership or the macro contract materially changes. |
| Retain delegation through an owned clone | `not-adopt` | Preserves allocation before rejection. | None while pre-allocation bounds remain policy. |
| General validated-newtype crate | `not-adopt` | Cannot replace the private macro ordering and widens the cone without another consumer. | Multiple public crates need the same richer abstraction. |
| Allocation-counting global allocator | `not-adopt` | Adds unsafe global test state for a directly reviewable ordering property. | A safe scoped allocator test facility becomes available and regression evidence is inadequate. |

## Compatibility and dependency evidence

All accepted values, consumer behavior, public signatures, and wire compatibility
are unchanged. Owned construction
continues moving its caller-provided allocation. Outer transports may allocate
before calling `parse`, so `SDK-LIM-007` remains. No dependency, license, native,
FFI, feature, MSRV, target, or public facade changes. The exact Rust version and
features remain the workspace-pinned Rust 1.98.1 configuration. Direct and
resolved dependency cone evidence is unchanged because no package is added. The
Apache-2.0 license and repository provenance remain authoritative; no new
supply-chain input is introduced.

## Security, privacy and maintenance evidence

Direct borrowed validation caps work and rejects before the only retained-value
copy. The validator's length check precedes whitespace and control-character
traversal, and diagnostics retain no caller value. No secret, private key, PII,
unsafe Rust, build script, network operation, or maintenance dependency is added.
Maintenance stays local to the private macro; release policy and the existing
security posture do not change. Supported native, mobile, and WASM target
evidence remains mandatory in Nix.

## Rejected or deferred candidates

Owned delegation, a generalized crate, and a global allocator are not adopted
for the reasons in the candidate table. Outer transport and streaming-deserializer
limits remain deferred to #168 because they are not properties of this value type.

## Open questions and blockers

None. Rollback is a focused revert with no data migration. Stop before merge if
any exact existing ceiling fails, diagnostics expose
a rejection canary, a new dependency becomes necessary, or full gates fail.

## Evidence commands

Pre-implementation commands use `rg`, `git rev-parse HEAD`, `openspec status`,
`openspec validate ... --strict`, and `scripts/factory check --change
bound-registration-identifiers`. At research time the focused tests,
workspace/Nix gates, exact-diff review, and hosted checks are explicitly unrun
and required after implementation.

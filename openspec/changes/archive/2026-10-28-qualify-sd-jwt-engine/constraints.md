# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/489
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep the engine chain/product neutral and
dependency types private. `SDK-COMPAT-001` through `SDK-COMPAT-005` require
stable Rust 1.89.0 for the 0.1.x release line and Rust 1.98.1 primary evidence.
`SDK-SEC-001` through `SDK-SEC-003` prohibit first-party unsafe, secret-bearing
diagnostics, and unbounded new hostile-input paths. `SDK-DELIVERY-001` requires
issue-first spec-driven delivery. `SDK-DEP-001` governs any upstream remediation.
`SDK-LIM-002`, `SDK-LIM-003`, `SDK-LIM-005`, `SDK-LIM-006`, `SDK-LIM-007`, and
`SDK-LIM-010` preserve FFI, target-runtime, product-policy, downstream,
outer-allocation, and advisory/yank limitations.

## Introduced or changed constraints

- RFC 9901 and SD-JWT VC SHALL remain separately versioned capabilities.
- The current SD-JWT VC implementation target is exact draft-22 content, not
  the former draft-19 planning pin; later draft or RFC migration requires an
  explicit profile revision and differential evidence.
- Canonical SD-JWT VC media/protected type is `dc+sd-jwt`. Any legacy
  `vc+sd-jwt` acceptance is an opt-in, time-bounded adapter with explicit
  evidence and cannot weaken canonical issuance.
- Candidate dependency types, errors, JSON values, signing keys, salt sources,
  clocks, policies, and verification results SHALL NOT cross an Identus public
  boundary.
- A production dependency must pass Rust 1.89.0, Rust 1.98.1, dependency-cone,
  advisory/license/source, unsafe/native, target, resource, redaction, and
  normative-parity gates before activation. Research success alone is not
  adoption.
- The base engine owns deterministic format processing; cryptographic signing,
  verification, entropy, time, key resolution, and trust remain injected or
  delegated through existing focused Identus capabilities.

## Introduced or changed limitations

- SD-JWT VC draft-22 remains an Internet-Draft in IESG evaluation, not a final
  RFC or certification target.
- No current Rust candidate is assumed conformant from its description or
  valid-input round trip; rejection classes and work bounds require independent
  evidence.
- Compile-only portable-target results do not prove browser/mobile runtime,
  packaging, FFI, storage, key custody, or production support.
- SDK-TS 8.1.4 is compatibility evidence for migration only and cannot define
  Rust DTOs, errors, security policy, or normative defaults.

## Consumer and product impact

There is no runtime consumer impact in this research change. Future consumers
receive separate base and VC-profile crates behind Rust-idiomatic APIs; legacy
TypeScript compatibility, if required, belongs in a thin adapter. Oxid, Lace,
midnight-identity, and other repositories remain unchanged.

## Activation and rollback

The research decision activates only after exact evidence, accepted ADR,
reviewed implementation issues, signed/DCO PR, and green required CI merge.
Production activation requires a new issue and OpenSpec preflight. Rollback of
this research removes its report, ADR, spec, and roadmap records; no dependency,
public API, wire value, persisted data, or downstream migration exists.

## Evidence

The assessment records immutable source revisions/checksums, standards hashes,
version/features, license/provenance, MSRV, direct and resolved dependency cone,
advisory/yank limits, unsafe/native reach, host and portable targets, protocol
coverage, serialization/disclosure/key-binding behavior, bounds, diagnostics,
secret handling, maintenance, release posture, commands, unrun checks, rollback,
and objective reconsideration triggers.

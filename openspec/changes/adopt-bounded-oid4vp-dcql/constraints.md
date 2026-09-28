# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/429
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`, `002`: OID4VP stays protocol- and chain-neutral; the private
  dependency does not reverse an Identus edge.
- `SDK-COMPAT-001`, `002`, `004`, `005`: Rust/portable target policy is unchanged.
- `SDK-SEC-001`, `002`, `003`: unsafe remains forbidden; all input/work/results
  are bounded and diagnostics are redacted.
- `SDK-REL-001`: `identus-oid4vp` remains unpublished.
- `SDK-LIM-005`, `007`: trust, consent, transport, credential verification, and
  storage remain caller-owned.

## Introduced or changed constraints

Exact `siros-dcql 0.3.0` is private to `identus-oid4vp`. Candidate types and
diagnostics cannot appear in public signatures. The SDK validates stricter
Final structure and all resource/work bounds before invoking candidate logic.

## Introduced or changed limitations

The result proves only that a signature-verified payload contains a structurally
valid DCQL query and that supplied credential descriptors match under the
initial exact-format/no-format-metadata policy. It does not prove a valid full
Authorization Request, verifier trust, credential authenticity, consent, or a
safe response. Scope-based DCQL and format-specific metadata are unsupported.

## Consumer and product impact

Headless wallet consumers gain a portable selection engine without an OIDC
runtime or third-party public types. No Oxid, Midnight, Lace, mobile UI,
transport, storage, or release surface changes in this slice.

## Activation and rollback

Activation requires issue-linked reviewed merge to `develop`. Rollback removes
the additive unpublished API and private dependency. No migration is required.

## Evidence

Issue #429 and ADR 0165 are material authority. Strict facade/differential
tests, dependency/source/license gates, portable compiles, redaction tests,
bounded-work proofs, and exact-diff review are required before delivery.

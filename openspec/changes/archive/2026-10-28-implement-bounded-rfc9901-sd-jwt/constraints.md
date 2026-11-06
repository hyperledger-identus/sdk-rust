# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/568
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 govern inward dependencies, focused reusable
ownership, and effect isolation. SDK-SEC-001 through SDK-SEC-003 govern secret
handling, parser safety, and finite work. SDK-COMPAT-001 through
SDK-COMPAT-005, SDK-DEP-001, SDK-DELIVERY-001, SDK-LIM-002, SDK-LIM-003,
SDK-LIM-005 through SDK-LIM-007, and SDK-LIM-010 remain effective. ADR 0193
authorizes the focused base engine and separates it from the evolving profile.

## Introduced or changed constraints

- `identus-sd-jwt` depends inward on existing generic Identus crates; generic
  credential, presentation, JOSE, crypto, and core crates cannot depend on it.
- RFC 9901 compact parsing, issuer verification, disclosure processing, and
  holder key binding use distinct staged types.
- SHA-256 is the only enabled disclosure digest in this slice. Signature
  algorithms are the existing caller-selected `identus-jose` allowlist.
- All compact, disclosure, JSON, graph, selection, key-binding, retained-output,
  and aggregate-work limits in the design are public and checked at the
  narrowest safe boundary.
- Production dependencies exclude every assessed candidate engine and donor
  DTO, error, runtime, trust, transport, storage, DID, custody, and agent type.
- Public errors and all formatted/serialized diagnostics exclude compact
  credentials, claims, salts, digests, keys, signatures, audience, nonce, and
  issuer/holder identifiers.
- No unsafe code, native dependency, panic on untrusted input, network, file
  system, persistence, executor, global mutable state, or private-key ownership.

## Introduced or changed limitations

- Only RFC 9901 compact serialization and SHA-256 are implemented; JWS JSON
  Serialization and other digest algorithms remain unsupported.
- SD-JWT VC draft-22, `dc+sd-jwt`, `vc+sd-jwt`, OID4VC integration, HTTP,
  trust, replay policy, bindings, publication, and downstream adoption remain
  outside this issue.
- Native/portable compile evidence does not imply browser/mobile runtime,
  packaging, FFI, hardware-backed key, certification, or device support.
- The generic envelope stores opaque SD-JWT bytes; it does not make them
  verified, trusted, authorized, non-replayed, or status-valid.
- Rust staged types/errors are canonical. Language compatibility belongs to
  later thin adapters and cannot reshape this contract.

## Consumer and product impact

Rust consumers gain an additive unreleased RFC 9901 engine suitable for later
profile and language adapters. Existing SDK crates and products remain
behaviorally unchanged. #569 stays blocked until the base quality declaration,
offline E2E, review, protected CI, and merge complete.

## Activation and rollback

Activation requires the exact issue #568 preflight, immutable provenance,
resource/error/API review, conformance/property/fuzz/benchmark/differential/
target evidence, the offline E2E, independent Claude or Codex review, green
protected CI, and an issue-linked PR. Before publication, rollback removes the
additive crate and ledger entry while preserving vectors and decision history.

## Evidence

Evidence authorities are RFC 9901, issue #568, ADR 0193, the parent
qualification archive, the new canonical requirements, quality declaration,
target/dependency/advisory receipts, independent review, and protected CI.

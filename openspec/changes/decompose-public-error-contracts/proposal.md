## Why

`identus-credentials` currently repeats one 44-variant error catalogue across
public constants, `to_identus_error`, `Display`, and domain-split tests. The
outward behavior is safe, but the duplication makes omission and redaction
drift difficult to review before further protocol work expands the SDK.

Issue #271 selects the credentials crate as the smallest representative pilot
for a crate-owned declarative error contract. This change establishes the
mechanism and immutable pre-refactor characterization evidence before any
other crate is considered.

## What Changes

- Add a private, crate-local `ErrorContract` representation and cohesive
  envelope, metadata, status, verification, and verifier catalogues.
- Make the existing `CredentialError` and `CredentialVerificationError`
  bridges and displays consume those catalogues through compile-time
  exhaustive routing.
- Add a checked-in golden characterization of all 47 current public enum
  variants, including local display, code identity and visibility, error kind,
  capability, public message, public `IdentusError` display, and source state.
- Add tests that compare every current variant and public error constant with
  the pre-implementation golden contract and retain redaction canaries.
- Preserve every public enum, variant, derive/attribute, constant, method
  signature and constness, `Display`, `Error::source`, `to_identus_error`, and
  serialized/wire absence exactly.
- Add no crate, dependency, shared error abstraction, protocol error, or
  protocol behavior. Later crates require separate independently revertible
  issues and evidence.

## Capabilities

### New Capabilities

- `public-error-contracts`: crate-owned, declarative and auditable error
  projection with compile-time completeness and golden compatibility evidence,
  piloted only in `identus-credentials`.

### Modified Capabilities

None. Credential, verification, status, core-error and protocol behavior is
characterized and preserved rather than changed.

## Impact

- **Repository/base:** `hyperledger-identus/sdk-rust` on
  `develop@707a5a22c3fad18724d5c5cac953e7387f7e49d8`.
- **Issue/decision:** issue #271 and ADR 0116.
- **Owner:** `identus-credentials`; `identus-core` remains an unchanged
  dependency and owner of `IdentusError`.
- **Code:** private credentials error-contract/catalogue modules and existing
  credentials error implementations/tests only.
- **Public and wire API:** no intended change. The crate remains unpublished
  and `0.0.0`, but the pilot treats its current surface as a strict
  characterization boundary.
- **Dependencies/targets:** no manifest, feature, compiler, target, FFI,
  serialization, protocol, storage, or consumer change.
- **Isolation:** no consumer repository is inspected or changed. Issue #7,
  issue #168, OID4VCI behavior, release and publication remain out of scope.

## ADDED Requirements

### Requirement: Credentials owns a private declarative error contract

`identus-credentials` SHALL own a private compile-time `ErrorContract` and
private catalogue records for its existing error variants. A record SHALL
contain the stable `ErrorCode`, `ErrorKind`, `CapabilityId`, local display text,
and public message needed by the existing surfaces. Production use of the
record SHALL allocate no data and SHALL introduce no crate, dependency,
feature, runtime, unsafe/native code, serialization, FFI, or public API.

#### Scenario: narrow dependency ownership is preserved

- **WHEN** the credentials pilot is built under its current feature and target matrix
- **THEN** its resolved dependency cone and public item inventory SHALL remain unchanged and the contract/catalogue types SHALL not be publicly reachable

### Requirement: Catalogue boundaries follow credential invariants

The private catalogue SHALL make envelope/artifact, metadata/schema, status,
verification-report, and verification-execution mappings independently
discoverable. Each current error variant SHALL belong to exactly one such
group. The catalogue SHALL NOT import OID4VCI, chain, product, storage, trust,
transport, consumer, or shared workspace-error policy.

#### Scenario: one domain can be reviewed independently

- **WHEN** a reviewer inspects the status error catalogue
- **THEN** every status-owned credentials mapping SHALL be visible together without requiring review of envelope, metadata, report, verifier, or protocol mappings

### Requirement: Every public error variant routes exhaustively

`CredentialError` and `CredentialVerificationError` SHALL each use one private,
wildcard-free exhaustive router from every enum variant to exactly one
`ErrorContract`. `Display` and `to_identus_error` SHALL obtain their values from
that record. Adding an enum variant without a contract SHALL fail compilation.

#### Scenario: incomplete mapping is a compile failure

- **WHEN** a compile probe adds a temporary unmapped variant to either pilot enum or removes its routing arm
- **THEN** compilation SHALL fail because the exhaustive contract router is incomplete

### Requirement: Pre-refactor golden behavior remains exact

The planning golden SHALL remain the independent compatibility oracle. The
fixture `golden/credentials-error-contract-v1.csv` was captured from
`develop@353030a7f263b9a1fba9deac0312ed228e61d761`. A test SHALL parse its fixed schema,
require exactly 47 unique `(error_type, variant)` rows, and compare every
current variant's constant name/visibility, code, kind, capability, local
display, public message, complete public display, and error-source state.
Malformed, missing, duplicate, extra, or behavior-mismatched rows SHALL fail.

#### Scenario: all current credential errors match the golden

- **WHEN** all 44 `CredentialError` and three `CredentialVerificationError` variants are characterized through the refactored implementation
- **THEN** all 47 rows SHALL match the planning fixture exactly, including the intentionally different local and public messages

#### Scenario: same-change regeneration cannot bless drift

- **WHEN** implementation behavior differs from the planning fixture
- **THEN** the test SHALL fail and no build step SHALL rewrite or regenerate the checked-in golden automatically

### Requirement: Existing public and wire surface is unchanged

The pilot SHALL preserve both public enum names, variants, declaration order,
derives, non-exhaustive attributes, documentation, re-exports, trait
implementations, public error-code constant names/types/values/paths, method
receivers, return types and method constness. It SHALL preserve
`CredentialError::to_identus_error` as non-const and
`CredentialVerificationError::to_identus_error` as `const`. It SHALL add no
Serde implementation, error wire model, retryability or structured metadata,
binding annotation, or public catalogue/introspection API.

#### Scenario: public API comparison is empty

- **WHEN** the post-refactor credentials public API is compared with the exact base
- **THEN** the comparison SHALL report no added, removed, renamed, retyped, visibility-changed, trait-changed, or constness-changed public item

#### Scenario: no wire contract is invented

- **WHEN** the refactored error types are inspected for serialization and generated bindings
- **THEN** they SHALL remain absent from Serde and binding surfaces exactly as at the base

### Requirement: Redaction and source behavior are preserved

Every catalogue text field SHALL be `&'static str`. No caller-controlled,
credential, identifier, status, proof, endpoint, cause, secret, or internal
runtime value SHALL enter local `Display`, public message, complete public
display, debug-derived variant identity, or the golden. Both pilot enums SHALL
continue returning no source from `std::error::Error::source`.

#### Scenario: caller canaries remain absent

- **WHEN** invalid credential, metadata, status and verification inputs contain known caller-controlled canaries and their errors are formatted locally and through `IdentusError`
- **THEN** no canary SHALL appear and the exact golden static strings SHALL be returned

#### Scenario: error source semantics stay empty

- **WHEN** every pilot variant is viewed through `dyn std::error::Error`
- **THEN** `source()` SHALL return `None`

### Requirement: Pilot scope does not imply workspace standardization

The implementation SHALL migrate only `identus-credentials`. Other SDK error
catalogues SHALL remain unchanged until independently issue-linked,
characterized and reviewed. No shared runtime error crate, public trait, derive
macro, cross-crate catalogue, OID4VCI feature, inherited-input remediation, or
consumer adoption SHALL be delivered by this change.

#### Scenario: unrelated error surfaces remain untouched

- **WHEN** the implementation diff is reviewed against its base
- **THEN** no other crate's error source or behavior, issue #7 behavior, issue #168 behavior, or consumer repository SHALL have changed

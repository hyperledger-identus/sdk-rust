# Core Error And Result Conventions

`identus-core` owns the first shared error and result DTO conventions for the
workspace. The goal is to keep crate-specific behavior type-safe while giving
bindings a stable, redaction-safe envelope that can be exposed consistently to
TypeScript, Swift, Kotlin, React, React Native, Node, and WASM packages.

## Public Types

| Type | Purpose |
|---|---|
| `CapabilityId` | Stable capability identifier used by errors, fixtures, and binding DTOs. |
| `ErrorCode` | Stable typed error code that can cross Rust and wrapper boundaries. |
| `ErrorKind` | Coarse error family for policy decisions and binding mapping. |
| `RedactionPolicy` | Diagnostic exposure policy: public, internal, or secret. |
| `IdentusError` | Redaction-safe SDK error with stable code, kind, capability, and public message. |
| `IdentusResult<T>` | Standard result alias for core-facing APIs. |
| `ResultEnvelope<T>` | Binding-friendly success/error envelope. |
| `ErrorEnvelope` | Binding-facing error DTO with stable strings only. |

## Rules

- `Display` for `IdentusError` renders only the stable error code and public
  message.
- Secret or internal context must stay in adapter-local diagnostics and must not
  be stored in `IdentusError`.
- Capability ids and error codes must be stable strings because conformance
  fixtures and wrappers use them as compatibility contracts.
- Domain crates should return `IdentusResult<T>` at public crate boundaries once
  behavior moves beyond parser-local errors.
- Bindings should expose `ErrorEnvelope` or generated equivalents, not
  crate-specific Rust error internals.
- Adapter implementations may attach private diagnostics in local logs, but
  must map public failures back into typed, redaction-safe core errors.

## Catalog Fixture

`fixtures/conformance/static-model/typed-error-catalog.json` is the
machine-readable compatibility catalog for stable error codes that have crossed
from domain crates into the core error surface. The fixture records the owning
capability, local Rust error type, core DTO, source variant, public message, and
binding targets for each code.

The catalog is a compatibility contract for generated bindings and conformance
tests. Updating or removing any cataloged code requires a task with acceptance
criteria, source and documentation updates, and an explicit compatibility
decision for TypeScript, Swift, Kotlin, Node, WASM, React, and React Native
consumers.

## Initial Error Families

`ErrorKind` starts with families needed by current fixtures and planned
capability contracts: invalid input, unsupported feature, not found, conflict,
policy violation, verification failure, transport, storage, cryptography, trust,
and internal invariant failure.

New families require acceptance criteria that explain the capability contract,
binding mapping, and redaction policy.

## First Adopter

`identus-did` is the first domain crate to bridge local parser errors into the
core convention. DID and DID URL parsing keep their crate-specific
`DidParseError` for idiomatic Rust `FromStr` compatibility, while
`parse_with_core_error`, `DidParseError::to_identus_error`, and
`DidParseError::to_error_envelope` expose stable typed codes for bindings and
conformance fixtures.

Initial DID parser codes:

- `missing_did_scheme`
- `invalid_did_method`
- `invalid_did_method_specific_id`
- `did_url_components_not_allowed`
- `invalid_did_url_component`

## Second Adopter

`identus-messaging` bridges local DIDComm parser and addressing errors into the
same core convention. DIDComm message type, message id, thread id, and addressed
message constructors keep `DidCommParseError` for idiomatic Rust compatibility,
while `parse_with_core_error`, `addressed_with_core_error`,
`DidCommParseError::to_identus_error`, and
`DidCommParseError::to_error_envelope` expose stable typed codes for bindings
and transcript replay.

Initial DIDComm parser codes:

- `missing_didcomm_prefix`
- `missing_didcomm_path_segment`
- `invalid_didcomm_path_segment`
- `invalid_didcomm_version`
- `unexpected_didcomm_path_segment`
- `invalid_didcomm_identifier`
- `missing_didcomm_recipient`

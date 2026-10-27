//! Finite public resource profile for PRISM DID operations.

/// Maximum raw protobuf bytes in one PRISM operation.
pub const MAX_OPERATION_BYTES: usize = 96_000;
/// Maximum bytes in a long-form PRISM DID under the raw-operation budget.
pub const MAX_LONG_FORM_DID_BYTES: usize =
    "did:prism:".len() + 64 + 1 + (MAX_OPERATION_BYTES * 4).div_ceil(3);
/// Maximum raw protobuf bytes in one signed PRISM operation envelope.
pub const MAX_SIGNED_OPERATION_BYTES: usize = 128 * 1_024;
/// Maximum estimated native DID-document projection size.
pub const MAX_PROJECTED_DOCUMENT_BYTES: usize = 32 * 1_024 * 1_024;
/// Maximum bytes in the signing key identifier carried by an envelope.
pub const MAX_SIGNED_WITH_BYTES: usize = 50;
/// Maximum bytes in a supported DER or fixed-width signature.
pub const MAX_SIGNATURE_BYTES: usize = 128;
/// Maximum active verification methods in one PRISM state.
pub const MAX_VERIFICATION_METHODS: usize = 50;
/// Maximum active services in one PRISM state.
pub const MAX_SERVICES: usize = 50;
/// Maximum retired key or service identifiers retained by one materialized state.
pub const MAX_RETIRED_IDENTIFIERS_PER_KIND: usize = 4_096;
/// Maximum ASCII fragment characters in a key or service identifier.
pub const MAX_ID_CHARACTERS: usize = 50;
/// Maximum ASCII grammar characters in a service type wire value.
pub const MAX_TYPE_CHARACTERS: usize = 100;
/// Maximum Unicode scalar values in a service endpoint wire value.
pub const MAX_SERVICE_ENDPOINT_CHARACTERS: usize = 300;
/// Maximum UTF-8 bytes in a 300-character service endpoint wire value.
pub const MAX_SERVICE_ENDPOINT_BYTES: usize = MAX_SERVICE_ENDPOINT_CHARACTERS * 4;
/// Maximum additional context URI values. DID Core context makes the projected total 128.
pub const MAX_CONTEXTS: usize = 127;

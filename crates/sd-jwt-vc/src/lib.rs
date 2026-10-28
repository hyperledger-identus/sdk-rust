//! Bounded SD-JWT VC semantics pinned to draft-22.
//!
//! RFC 9901 mechanics come from `identus-sd-jwt`. This crate assigns the
//! canonical `dc+sd-jwt` profile meaning while keeping retrieval, trust,
//! issuer authorization, status acceptance and product policy outside.

#![forbid(unsafe_code)]

mod error;
mod issuer_evidence;
mod limits;
mod profile;
mod resource;
mod status;
mod type_metadata;
mod verifier;

pub use error::SdJwtVcError;
pub use issuer_evidence::{IssuerEvidence, IssuerEvidenceKind, IssuerMetadataEvidence};
pub use limits::SdJwtVcLimits;
pub use profile::{
    Draft22SdJwtVc, Draft22VerificationReceipt, IssuedDraft22SdJwtVc, ProfileValidationInputs,
    issue_draft22, parse_draft22_envelope,
};
pub use status::{StatusListBinding, StatusListEvidence, correlate_status_evidence};
pub use type_metadata::{
    ClaimDisclosureRule, ClaimMetadata, MetadataClaimPath, MetadataClaimPathSegment, TypeMetadata,
    TypeMetadataChain,
};
pub use verifier::{Draft22CredentialVerifier, Draft22VerifierEvidence};

use identus_core::Component;

pub const PROTECTED_TYPE: &str = "dc+sd-jwt";
pub const MEDIA_TYPE: &str = "application/dc+sd-jwt";
pub const ENVELOPE_FORMAT: &str = "dc+sd-jwt";
pub const DRAFT_NAME: &str = "draft-ietf-oauth-sd-jwt-vc-22";
pub const DRAFT_DATE: &str = "2026-10-27";
pub const DRAFT_SOURCE: &str = "https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt";
pub const DRAFT_SHA256: &str = "da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124";

pub const COMPONENT: Component = Component {
    name: "identus-sd-jwt-vc",
    summary: "Bounded canonical SD-JWT VC draft-22 profile over RFC 9901.",
};

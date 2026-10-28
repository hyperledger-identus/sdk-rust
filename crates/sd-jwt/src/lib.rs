//! Bounded RFC 9901 compact SD-JWT mechanics over Identus capabilities.
//!
//! Parsed, issuer-verified, and disclosure-verified values are distinct.
//! Trust, replay, status, and authorization remain caller policy.

#![forbid(unsafe_code)]

mod engine;
mod error;
mod hash;
mod json;
mod limits;
mod types;

pub use engine::{
    DisclosureVerifiedSdJwt, EncodedDisclosure, IssuedSdJwt, IssuerVerifiedSdJwt,
    KeyBindingExpectations, SdJwtPresentation, UnverifiedSdJwt, VerifiedSdJwt, issue,
    issue_with_hasher, parse_envelope,
};
pub use error::SdJwtError;
pub use hash::{DisclosureHasher, Sha256DisclosureHasher};
pub use limits::SdJwtLimits;
pub use types::{
    ClaimPath, DecoyLocation, IssuancePlan, PathSegment, SelectionPlan, VerificationReceipt,
    map_claim_path, map_disclosure_plan,
};

use identus_core::Component;

/// Metadata for the unpublished RFC 9901 crate.
pub const COMPONENT: Component = Component {
    name: "identus-sd-jwt",
    summary: "Bounded RFC 9901 compact issuance, disclosure and key binding.",
};

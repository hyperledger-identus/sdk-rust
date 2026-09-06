//! Bounded OpenID for Verifiable Credential Issuance protocol semantics.
//!
//! This first slice validates only the Credential Offer invocation transport.
//! It does not interpret offer members, perform network access, or establish
//! issuer trust.

#![forbid(unsafe_code)]

mod error;
mod json;
mod limits;
mod transport;

pub use error::{CAPABILITY, CredentialOfferError, error_code};
pub use limits::CredentialOfferLimits;
pub use transport::{CredentialOfferReference, CredentialOfferRequest, EmbeddedCredentialOffer};

use identus_core::Component;

/// Metadata for the `identus-oid4vci` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vci",
    summary: "Bounded OID4VCI Final protocol semantics.",
};

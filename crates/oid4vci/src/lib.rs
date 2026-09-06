//! Bounded OpenID for Verifiable Credential Issuance protocol semantics.
//!
//! The crate distinguishes Credential Offer invocation transport from bounded
//! core-member semantics. It performs no network access and establishes no
//! issuer trust.

#![forbid(unsafe_code)]

mod error;
mod grants;
mod json;
mod limits;
mod semantic;
mod transport;

pub use error::{CAPABILITY, CredentialOfferError, error_code};
pub use grants::{
    AuthorizationCodeGrant, AuthorizationServerIdentifier, CredentialOfferWithGrants, IssuerState,
    PreAuthorizedCode, PreAuthorizedCodeGrant, TransactionCodeDescription,
    TransactionCodeInputMode, TransactionCodeRequirements,
};
pub use limits::{
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferSemanticLimits,
    MAX_CONFIGURABLE_JSON_DEPTH,
};
pub use semantic::{CredentialConfigurationId, CredentialIssuerIdentifier, CredentialOffer};
pub use transport::{CredentialOfferReference, CredentialOfferRequest, EmbeddedCredentialOffer};

use identus_core::Component;

/// Metadata for the `identus-oid4vci` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vci",
    summary: "Bounded OID4VCI Final protocol semantics.",
};

//! Bounded OpenID for Verifiable Credential Issuance protocol semantics.
//!
//! The crate distinguishes Credential Offer invocation transport, offer/grant
//! semantics, unsigned Credential Issuer Metadata, exact cross-document
//! agreement, a partial Authorization Server Metadata core, and explicit
//! Pre-Authorized Code server binding. It performs no network access, does not
//! construct endpoint messages or validate complete RFC 8414 metadata, and
//! establishes no issuer or server trust.

#![forbid(unsafe_code)]

mod authorization_metadata;
mod error;
mod grants;
mod json;
mod limits;
mod metadata;
mod pre_authorized_server;
mod semantic;
mod transport;

pub use authorization_metadata::{
    AUTHORIZATION_CODE_GRANT_TYPE, AuthorizationEndpoint, AuthorizationServerMetadataCore,
    GrantTypeIdentifier, IMPLICIT_GRANT_TYPE, PRE_AUTHORIZED_CODE_GRANT_TYPE, TokenEndpoint,
};
pub use error::{CAPABILITY, CredentialOfferError, error_code};
pub use grants::{
    AuthorizationCodeGrant, AuthorizationServerIdentifier, CredentialOfferWithGrants, IssuerState,
    PreAuthorizedCode, PreAuthorizedCodeGrant, TransactionCodeDescription,
    TransactionCodeInputMode, TransactionCodeRequirements,
};
pub use limits::{
    AuthorizationServerMetadataLimits, CredentialIssuerMetadataLimits, CredentialOfferGrantLimits,
    CredentialOfferLimits, CredentialOfferSemanticLimits, MAX_CONFIGURABLE_JSON_DEPTH,
};
pub use metadata::{
    CredentialConfigurationSummary, CredentialEndpoint, CredentialFormatIdentifier,
    CredentialIssuerMetadata, CredentialOfferWithMetadata,
};
pub use pre_authorized_server::CredentialOfferWithPreAuthorizedServer;
pub use semantic::{CredentialConfigurationId, CredentialIssuerIdentifier, CredentialOffer};
pub use transport::{CredentialOfferReference, CredentialOfferRequest, EmbeddedCredentialOffer};

use identus_core::Component;

/// Metadata for the `identus-oid4vci` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vci",
    summary: "Bounded OID4VCI Final protocol semantics.",
};

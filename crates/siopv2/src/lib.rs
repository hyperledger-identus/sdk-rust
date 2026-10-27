//! Experimental bounded SIOPv2 draft-13 DID-subject protocol core.
//!
//! This crate verifies a caller-authorized signed request, constructs a
//! `subject_signed_id_token` through an injected JOSE signer, emits exact
//! `direct_post` data, and models a pure caller-driven lifecycle. It performs
//! no networking, DID resolution, trust, custody, persistence, UI, or consent.

#![forbid(unsafe_code)]

mod error;
mod flow;
mod limits;
mod request;
mod response;

pub use error::Siopv2Error;
pub use flow::{EventId, SiopFlow};
pub use limits::Siopv2Limits;
pub use request::{REQUEST_OBJECT_JWT_TYPE, UnverifiedSiopRequest, ValidatedSiopRequest};
pub use response::{
    DIRECT_POST_MEDIA_TYPE, DirectPostResponse, ID_TOKEN_JWT_TYPE, SiopResponsePlan,
};

use identus_core::Component;

/// Audience identifier defined by SIOPv2 draft 13.
pub const SIOPV2_AUDIENCE: &str = "https://self-issued.me/v2";

/// Metadata for the experimental `identus-siopv2` crate.
pub const COMPONENT: Component = Component {
    name: "identus-siopv2",
    summary: "Bounded experimental SIOPv2 draft-13 DID-subject protocol core.",
};

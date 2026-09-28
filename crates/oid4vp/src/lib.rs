//! Bounded OpenID for Verifiable Presentations protocol foundations.
//!
//! The crate classifies a by-reference Authorization Request, describes
//! bounded runtime-neutral GET/POST retrieval, binds its response, and verifies
//! a signed compact JAR through [`identus_jose`]. It does not execute HTTP,
//! authorize a verifier key, interpret DCQL, select credentials, record
//! consent, or construct a response.

#![forbid(unsafe_code)]

mod error;
mod invocation;
mod limits;
mod request_object;
mod request_object_json;
mod request_uri;

pub use error::{CAPABILITY, Oid4vpError, error_code};
pub use invocation::{
    AuthorizationRequestInvocation, ReferencedAuthorizationRequest, RequestUriMethod,
};
pub use limits::{
    AuthorizationRequestInvocationLimits, RequestObjectValidationLimits, RequestUriRetrievalLimits,
};
pub use request_object::{REQUEST_OBJECT_JWT_TYPE, UnverifiedRequestObject, VerifiedRequestObject};
pub use request_uri::{
    FORM_MEDIA_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RequestUriRetrievalInput,
    RequestUriRetrievalRequest,
};

use identus_core::Component;

/// Metadata for the `identus-oid4vp` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vp",
    summary: "Bounded OID4VP Final ingress, retrieval, and signed JAR validation.",
};

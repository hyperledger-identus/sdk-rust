//! Bounded OpenID for Verifiable Presentations protocol ingress.
//!
//! This first slice classifies only an OpenID4VP 1.0 Authorization Request
//! invocation whose Request Object is supplied by HTTPS reference. It does not
//! retrieve or verify that object, interpret DCQL, select credentials, record
//! consent, or construct a response.

#![forbid(unsafe_code)]

mod error;
mod invocation;
mod limits;

pub use error::{CAPABILITY, Oid4vpError, error_code};
pub use invocation::{
    AuthorizationRequestInvocation, ReferencedAuthorizationRequest, RequestUriMethod,
};
pub use limits::AuthorizationRequestInvocationLimits;

use identus_core::Component;

/// Metadata for the `identus-oid4vp` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vp",
    summary: "Bounded OID4VP Final protocol ingress.",
};

//! Bounded OpenID for Verifiable Presentations protocol foundations.
//!
//! The crate classifies a by-reference Authorization Request, describes
//! bounded runtime-neutral GET/POST retrieval, binds its response, and verifies
//! a signed compact JAR through [`identus_jose`], validates audience/time/replay,
//! binds the exact verified key to the Final client identifier through a
//! caller-owned trust port, and validates/evaluates DCQL through a bounded
//! SDK-owned facade, then composes one Final `vp_token` request state and
//! bounded mutually exclusive fragment, query, form-post, plain direct-post,
//! explicit protocol-error, or engine-neutral encrypted response material.
//! Direct-post material can consume one bounded endpoint acknowledgement.
//! It does not execute HTTP, bundle a DID/PKI/Federation/attestation trust
//! engine, record consent, create presentations, verify credentials, or
//! translate internal failures into wire errors.

#![forbid(unsafe_code)]

mod authorization_request;
mod dcql;
mod direct_post;
mod direct_post_acknowledgement;
mod encrypted_direct_post;
mod error;
mod invocation;
mod limits;
mod redirect_response;
mod request_object;
mod request_object_json;
mod request_object_security;
mod request_uri;
mod verifier_authorization;

pub use authorization_request::{
    AuthorizationResponseMode, AuthorizationResponseType, ValidatedAuthorizationRequest,
};
pub use dcql::{
    DcqlCandidate, DcqlClaimError, DcqlCombination, DcqlCombinationMember, DcqlCredential,
    DcqlDropped, DcqlEvaluation, DcqlPathComponent, DcqlQueryMatch, DcqlSelectedClaim,
    ValidatedDcqlQuery,
};
pub use direct_post::{
    DIRECT_POST_HTTP_METHOD, DIRECT_POST_MEDIA_TYPE, DirectPostErrorCode, DirectPostErrorDetails,
    DirectPostErrorResponse, DirectPostPresentation, DirectPostQueryResult, DirectPostResponse,
};
pub use direct_post_acknowledgement::{DirectPostAcknowledgement, DirectPostNavigationCandidate};
pub use encrypted_direct_post::EncryptedDirectPostResponse;
pub use error::{CAPABILITY, Oid4vpError, error_code};
pub use invocation::{
    AuthorizationRequestInvocation, ReferencedAuthorizationRequest, RequestUriMethod,
};
pub use limits::{
    AuthorizationRequestInvocationLimits, AuthorizationRequestValidationLimits, DcqlLimits,
    DirectPostAcknowledgementLimits, DirectPostErrorResponseLimits, DirectPostResponseLimits,
    RedirectResponseLimits, RequestObjectValidationLimits, RequestUriRetrievalLimits,
};
pub use redirect_response::{RedirectResponse, RedirectResponseKind};
pub use request_object::{REQUEST_OBJECT_JWT_TYPE, UnverifiedRequestObject, VerifiedRequestObject};
pub use request_object_security::{
    RequestObjectReplayFailure, RequestObjectReplayFuture, RequestObjectReplayGuard,
    RequestObjectReplayInput, RequestObjectSecurityEvidence, RequestObjectSecurityPolicy,
    STATIC_DISCOVERY_AUDIENCE,
};
pub use request_uri::{
    FORM_MEDIA_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RequestUriRetrievalInput,
    RequestUriRetrievalRequest,
};
pub use verifier_authorization::{
    ClientIdentifierScheme, VerifierAuthorizationEvidence, VerifierAuthorizationLimits,
    VerifierInfo, VerifierMetadata, VerifierResponseDestination, VerifierTrustFailure,
    VerifierTrustFuture, VerifierTrustInput, VerifierTrustPolicy,
};

use identus_core::Component;

/// Metadata for the `identus-oid4vp` crate.
pub const COMPONENT: Component = Component {
    name: "identus-oid4vp",
    summary: "Bounded OID4VP Final ingress, JAR, DCQL, and direct-post responses.",
};

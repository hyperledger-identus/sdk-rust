use std::{
    fmt,
    io::{self, Write},
    str,
};

use identus_jose::Oid4vciProofJwt;
use zeroize::Zeroizing;

use crate::{
    CredentialEndpoint, CredentialOfferError, CredentialOfferWithMetadata,
    JwtCredentialRequestLimits, TokenResponseCore, TokenResponseWithAuthorizationDetails,
};

/// HTTP method required for a Credential Endpoint request.
pub const CREDENTIAL_REQUEST_HTTP_METHOD: &str = "POST";

/// Media type required for an unencrypted Credential Request.
pub const CREDENTIAL_REQUEST_MEDIA_TYPE: &str = "application/json";

const BEARER_PREFIX: &str = "Bearer ";

/// A bounded unencrypted Final Credential Request using JWT key proofs.
///
/// This value contains an access token and proof material. Keep its sensitive
/// values out of logs, URLs, telemetry, caches, and long-lived storage.
pub struct JwtCredentialRequest {
    credential_endpoint: CredentialEndpoint,
    authorization: Zeroizing<String>,
    json_body: Zeroizing<Vec<u8>>,
    proof_count: usize,
}

impl JwtCredentialRequest {
    /// Borrow the validated HTTPS Credential Endpoint.
    pub const fn credential_endpoint(&self) -> &CredentialEndpoint {
        &self.credential_endpoint
    }

    /// Return the HTTP method required for this request.
    pub const fn http_method(&self) -> &'static str {
        CREDENTIAL_REQUEST_HTTP_METHOD
    }

    /// Return the media type required for this unencrypted request.
    pub const fn media_type(&self) -> &'static str {
        CREDENTIAL_REQUEST_MEDIA_TYPE
    }

    /// Return the number of JWT proofs in the body.
    pub const fn proof_count(&self) -> usize {
        self.proof_count
    }

    /// Return the complete Authorization field-value byte count.
    pub fn authorization_len(&self) -> usize {
        self.authorization.len()
    }

    /// Return the complete JSON body byte count.
    pub fn json_body_len(&self) -> usize {
        self.json_body.len()
    }

    /// Borrow the exact Authorization field value for immediate transport.
    ///
    /// Keep this bearer credential out of logs, URLs, telemetry, caches, and
    /// long-lived storage.
    pub fn expose_sensitive_authorization(&self) -> &str {
        &self.authorization
    }

    /// Borrow the exact JSON body for immediate transport.
    ///
    /// The body contains holder proofs. Keep it out of logs, telemetry, caches,
    /// and unrelated storage.
    pub fn expose_sensitive_json_body(&self) -> &str {
        str::from_utf8(&self.json_body).expect("serde_json emitted UTF-8")
    }
}

impl fmt::Debug for JwtCredentialRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwtCredentialRequest")
            .field("authorization_bytes", &self.authorization.len())
            .field("json_body_bytes", &self.json_body.len())
            .field("proof_count", &self.proof_count)
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithMetadata {
    /// Construct the bounded configuration-ID request for holder-produced JWT proofs.
    pub fn try_create_jwt_credential_request(
        &self,
        token_response: &TokenResponseCore,
        offered_configuration_index: usize,
        proofs: &[Oid4vciProofJwt],
        limits: JwtCredentialRequestLimits,
    ) -> Result<JwtCredentialRequest, CredentialOfferError> {
        let configuration = self
            .credential_offer()
            .credential_offer()
            .credential_configuration_ids()
            .get(offered_configuration_index)
            .ok_or(CredentialOfferError::CredentialRequestConfigurationMissing)?;

        if token_response.authorization_details_present() {
            return Err(CredentialOfferError::CredentialRequestAuthorizationDetailsUnsupported);
        }

        self.try_create_jwt_credential_request_with_selector(
            token_response,
            CredentialSelector::Configuration(configuration.as_str()),
            proofs,
            limits,
        )
    }

    /// Construct a bounded request for an authorized Credential Dataset identifier.
    ///
    /// Both indices address validated, source-ordered Token Response state. The
    /// selected Authorization Detail must reference a configuration from this
    /// matched Credential Offer.
    pub fn try_create_authorized_jwt_credential_request(
        &self,
        token_response: &TokenResponseWithAuthorizationDetails,
        authorization_detail_index: usize,
        credential_identifier_index: usize,
        proofs: &[Oid4vciProofJwt],
        limits: JwtCredentialRequestLimits,
    ) -> Result<JwtCredentialRequest, CredentialOfferError> {
        let detail = token_response
            .credential_authorization_details()
            .get(authorization_detail_index)
            .ok_or(CredentialOfferError::CredentialRequestAuthorizationDetailMissing)?;
        let identifier = detail
            .credential_identifiers()
            .nth(credential_identifier_index)
            .ok_or(CredentialOfferError::CredentialRequestIdentifierMissing)?;
        let configuration_is_offered = self
            .credential_offer()
            .credential_offer()
            .credential_configuration_ids()
            .iter()
            .any(|configuration| configuration.as_str() == detail.credential_configuration_id());
        if !configuration_is_offered {
            return Err(CredentialOfferError::CredentialRequestAuthorizationConfigurationMismatch);
        }

        self.try_create_jwt_credential_request_with_selector(
            token_response.token_response_core(),
            CredentialSelector::AuthorizedDataset(identifier),
            proofs,
            limits,
        )
    }

    fn try_create_jwt_credential_request_with_selector(
        &self,
        token_response: &TokenResponseCore,
        selector: CredentialSelector<'_>,
        proofs: &[Oid4vciProofJwt],
        limits: JwtCredentialRequestLimits,
    ) -> Result<JwtCredentialRequest, CredentialOfferError> {
        if !token_response.token_type().eq_ignore_ascii_case("Bearer") {
            return Err(CredentialOfferError::CredentialRequestTokenTypeUnsupported);
        }
        let access_token = token_response.expose_sensitive_access_token();
        if !is_rfc6750_b64token(access_token) {
            return Err(CredentialOfferError::InvalidCredentialRequestBearerToken);
        }
        if proofs.is_empty() {
            return Err(CredentialOfferError::CredentialRequestProofsRequired);
        }
        if proofs.len() > limits.max_proofs() {
            return Err(CredentialOfferError::TooManyCredentialRequestProofs);
        }
        if proofs
            .iter()
            .any(|proof| proof.compact().len() > limits.max_proof_bytes())
        {
            return Err(CredentialOfferError::CredentialRequestProofTooLarge);
        }

        let authorization_len = BEARER_PREFIX
            .len()
            .checked_add(access_token.len())
            .ok_or(CredentialOfferError::CredentialRequestAuthorizationTooLarge)?;
        if authorization_len > limits.max_authorization_bytes() {
            return Err(CredentialOfferError::CredentialRequestAuthorizationTooLarge);
        }
        let mut authorization = Zeroizing::new(String::with_capacity(authorization_len));
        authorization.push_str(BEARER_PREFIX);
        authorization.push_str(access_token);

        let mut body = BoundedJsonBody::new(limits.max_json_body_bytes());
        match selector {
            CredentialSelector::Configuration(configuration) => {
                body.push_static(b"{\"credential_configuration_id\":")?;
                body.push_json_string(configuration)?;
            }
            CredentialSelector::AuthorizedDataset(identifier) => {
                body.push_static(b"{\"credential_identifier\":")?;
                body.push_json_string(identifier)?;
            }
        }
        body.push_static(b",\"proofs\":{\"jwt\":[")?;
        for (index, proof) in proofs.iter().enumerate() {
            if index != 0 {
                body.push_static(b",")?;
            }
            body.push_json_string(proof.compact())?;
        }
        body.push_static(b"]}}")?;

        Ok(JwtCredentialRequest {
            credential_endpoint: self
                .credential_issuer_metadata()
                .credential_endpoint()
                .duplicate(),
            authorization,
            json_body: body.into_bytes(),
            proof_count: proofs.len(),
        })
    }
}

enum CredentialSelector<'a> {
    Configuration(&'a str),
    AuthorizedDataset(&'a str),
}

fn is_rfc6750_b64token(value: &str) -> bool {
    let bytes = value.as_bytes();
    let content_len = bytes
        .iter()
        .position(|byte| *byte == b'=')
        .unwrap_or(bytes.len());
    content_len != 0
        && bytes[..content_len].iter().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'+' | b'/')
        })
        && bytes[content_len..].iter().all(|byte| *byte == b'=')
}

struct BoundedJsonBody {
    bytes: Zeroizing<Vec<u8>>,
    max_bytes: usize,
    overflowed: bool,
}

impl BoundedJsonBody {
    fn new(max_bytes: usize) -> Self {
        Self {
            bytes: Zeroizing::new(Vec::with_capacity(max_bytes.min(4_096))),
            max_bytes,
            overflowed: false,
        }
    }

    fn push_static(&mut self, value: &[u8]) -> Result<(), CredentialOfferError> {
        self.write_all(value)
            .map_err(|_| CredentialOfferError::CredentialRequestBodyTooLarge)
    }

    fn push_json_string(&mut self, value: &str) -> Result<(), CredentialOfferError> {
        if serde_json::to_writer(&mut *self, value).is_err() {
            debug_assert!(self.overflowed);
            return Err(CredentialOfferError::CredentialRequestBodyTooLarge);
        }
        Ok(())
    }

    fn into_bytes(self) -> Zeroizing<Vec<u8>> {
        self.bytes
    }
}

impl io::Write for BoundedJsonBody {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let Some(new_len) = self.bytes.len().checked_add(buffer.len()) else {
            self.overflowed = true;
            return Err(io::Error::other("credential request body limit exceeded"));
        };
        if new_len > self.max_bytes {
            self.overflowed = true;
            return Err(io::Error::other("credential request body limit exceeded"));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

use std::fmt;

use zeroize::Zeroizing;

use crate::{
    AuthorizationServerMetadataCore, CredentialConfigurationId, CredentialIssuerMetadata,
    CredentialOfferError, CredentialOfferWithPreAuthorizedTokenInput,
    PRE_AUTHORIZED_CODE_GRANT_TYPE, PreAuthorizedTokenRequestLimits, TokenEndpoint,
    form::{append_pair, pair_len},
};

/// HTTP method required for a Token Endpoint request.
pub const TOKEN_REQUEST_HTTP_METHOD: &str = "POST";

/// Media type required for the constructed Token Request body.
pub const TOKEN_REQUEST_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

/// A bounded mandatory Pre-Authorized Code Token Request.
///
/// This value contains bearer-adjacent material. Its form body must not be
/// logged, placed in a URL, sent to telemetry, cached, generically serialized,
/// or retained longer than the transport operation requires.
pub struct PreAuthorizedTokenRequest {
    token_endpoint: TokenEndpoint,
    credential_issuer_metadata: CredentialIssuerMetadata,
    authorization_server_metadata: AuthorizationServerMetadataCore,
    offered_configurations: Vec<CredentialConfigurationId>,
    form_body: Zeroizing<String>,
    transaction_code_present: bool,
}

impl PreAuthorizedTokenRequest {
    pub(crate) fn into_response_parts(
        self,
    ) -> (
        CredentialIssuerMetadata,
        AuthorizationServerMetadataCore,
        Vec<CredentialConfigurationId>,
        bool,
    ) {
        let Self {
            token_endpoint: _,
            credential_issuer_metadata,
            authorization_server_metadata,
            offered_configurations,
            form_body,
            transaction_code_present,
        } = self;
        drop(form_body);
        (
            credential_issuer_metadata,
            authorization_server_metadata,
            offered_configurations,
            transaction_code_present,
        )
    }

    /// Borrow the validated HTTPS Token Endpoint.
    pub const fn token_endpoint(&self) -> &TokenEndpoint {
        &self.token_endpoint
    }

    /// Borrow the exact Credential Issuer Metadata matched before construction.
    pub const fn credential_issuer_metadata(&self) -> &CredentialIssuerMetadata {
        &self.credential_issuer_metadata
    }

    /// Borrow the exact selected Authorization Server Metadata.
    pub const fn authorization_server_metadata(&self) -> &AuthorizationServerMetadataCore {
        &self.authorization_server_metadata
    }

    /// Borrow the ordered Credential Configuration IDs from the matched offer.
    pub fn offered_credential_configurations(&self) -> &[CredentialConfigurationId] {
        &self.offered_configurations
    }

    /// Return the HTTP method required for this request.
    pub const fn http_method(&self) -> &'static str {
        TOKEN_REQUEST_HTTP_METHOD
    }

    /// Return the media type required for this request body.
    pub const fn media_type(&self) -> &'static str {
        TOKEN_REQUEST_MEDIA_TYPE
    }

    /// Return the exact encoded form-body byte count.
    pub fn form_body_len(&self) -> usize {
        self.form_body.len()
    }

    /// Report whether this request contains a Transaction Code.
    pub const fn transaction_code_present(&self) -> bool {
        self.transaction_code_present
    }

    /// Borrow the exact secret-bearing form body for immediate transport.
    ///
    /// The returned value contains the Pre-Authorized Code and may contain a
    /// Transaction Code. Keep it out of logs, URLs, telemetry, caches, generic
    /// serializers, and long-lived storage.
    pub fn expose_sensitive_form_body(&self) -> &str {
        &self.form_body
    }
}

impl fmt::Debug for PreAuthorizedTokenRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreAuthorizedTokenRequest")
            .field("form_body_bytes", &self.form_body.len())
            .field("transaction_code_present", &self.transaction_code_present)
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithPreAuthorizedTokenInput {
    /// Consume this prepared input state and construct the bounded mandatory
    /// Pre-Authorized Code Token Request form.
    pub fn try_into_pre_authorized_token_request(
        self,
        limits: PreAuthorizedTokenRequestLimits,
    ) -> Result<PreAuthorizedTokenRequest, CredentialOfferError> {
        let server = self.credential_offer_with_pre_authorized_server();
        let token_endpoint = server
            .authorization_server_metadata()
            .token_endpoint()
            .ok_or(CredentialOfferError::TokenEndpointRequired)?
            .duplicate();
        let pre_authorized_code = server
            .credential_offer_with_metadata()
            .credential_offer()
            .pre_authorized_code()
            .ok_or(CredentialOfferError::PreAuthorizedCodeGrantMissing)?
            .pre_authorized_code()
            .as_str();
        let transaction_code = self.transaction_code();
        let form_body_len = request_body_len(pre_authorized_code, transaction_code)
            .ok_or(CredentialOfferError::PreAuthorizedTokenRequestTooLarge)?;
        if form_body_len > limits.max_form_body_bytes() {
            return Err(CredentialOfferError::PreAuthorizedTokenRequestTooLarge);
        }

        let mut form_body = Zeroizing::new(String::with_capacity(form_body_len));
        append_pair(&mut form_body, "grant_type", PRE_AUTHORIZED_CODE_GRANT_TYPE);
        form_body.push('&');
        append_pair(&mut form_body, "pre-authorized_code", pre_authorized_code);
        if let Some(value) = transaction_code {
            form_body.push('&');
            append_pair(&mut form_body, "tx_code", value);
        }
        debug_assert_eq!(form_body.len(), form_body_len);

        let transaction_code_present = transaction_code.is_some();
        let (server, transaction_code) = self.into_parts();
        drop(transaction_code);
        let offered_configurations = server
            .credential_offer_with_metadata()
            .credential_offer()
            .credential_offer()
            .credential_configuration_ids()
            .iter()
            .map(CredentialConfigurationId::duplicate)
            .collect();
        let (matched_offer, authorization_server_metadata) = server.into_parts();
        let (grant_offer, credential_issuer_metadata) = matched_offer.into_parts();
        drop(grant_offer);

        Ok(PreAuthorizedTokenRequest {
            token_endpoint,
            credential_issuer_metadata,
            authorization_server_metadata,
            offered_configurations,
            form_body,
            transaction_code_present,
        })
    }
}

fn request_body_len(pre_authorized_code: &str, transaction_code: Option<&str>) -> Option<usize> {
    pair_len("grant_type", PRE_AUTHORIZED_CODE_GRANT_TYPE)?
        .checked_add(1)?
        .checked_add(pair_len("pre-authorized_code", pre_authorized_code)?)?
        .checked_add(match transaction_code {
            Some(value) => 1usize.checked_add(pair_len("tx_code", value)?)?,
            None => 0,
        })
}

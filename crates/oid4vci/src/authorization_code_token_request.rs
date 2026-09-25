use std::fmt;

use zeroize::Zeroizing;

use crate::{
    AUTHORIZATION_CODE_GRANT_TYPE, AuthorizationCodeTokenRequestLimits,
    AuthorizationResponseIssuerIdentification, AuthorizationServerMetadataCore,
    CorrelatedAuthorizationCode, CredentialConfigurationId, CredentialIssuerMetadata,
    CredentialOfferError, TOKEN_REQUEST_HTTP_METHOD, TOKEN_REQUEST_MEDIA_TYPE, TokenEndpoint,
    form::{append_pair, pair_len},
};

/// A bounded OAuth Authorization Code Token Request for an unauthenticated
/// public client.
///
/// The form body contains a one-time authorization code and PKCE verifier. It
/// must not be logged, placed in a URL, sent to telemetry, cached, generically
/// serialized, or retained longer than the transport operation requires.
pub struct AuthorizationCodeTokenRequest {
    token_endpoint: TokenEndpoint,
    credential_issuer_metadata: CredentialIssuerMetadata,
    authorization_server_metadata: AuthorizationServerMetadataCore,
    selected_configuration: CredentialConfigurationId,
    form_body: Zeroizing<String>,
    issuer_identification: AuthorizationResponseIssuerIdentification,
}

impl AuthorizationCodeTokenRequest {
    /// Borrow the validated Token Endpoint selected before authorization.
    pub const fn token_endpoint(&self) -> &TokenEndpoint {
        &self.token_endpoint
    }

    /// Borrow the retained validated Credential Issuer Metadata.
    pub const fn credential_issuer_metadata(&self) -> &CredentialIssuerMetadata {
        &self.credential_issuer_metadata
    }

    /// Borrow the retained selected Authorization Server Metadata.
    pub const fn authorization_server_metadata(&self) -> &AuthorizationServerMetadataCore {
        &self.authorization_server_metadata
    }

    /// Borrow the selected offered Credential Configuration ID.
    pub fn selected_credential_configuration(&self) -> &CredentialConfigurationId {
        &self.selected_configuration
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

    /// Return the unchanged issuer-identification evidence from correlation.
    pub const fn issuer_identification(&self) -> AuthorizationResponseIssuerIdentification {
        self.issuer_identification
    }

    /// Borrow the exact secret-bearing form body for immediate transport.
    ///
    /// Keep the returned authorization code and PKCE verifier out of logs,
    /// URLs, telemetry, caches, generic serializers and long-lived storage.
    pub fn expose_sensitive_form_body(&self) -> &str {
        &self.form_body
    }
}

impl fmt::Debug for AuthorizationCodeTokenRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationCodeTokenRequest")
            .field("form_body_bytes", &self.form_body.len())
            .field("issuer_identification", &self.issuer_identification)
            .finish_non_exhaustive()
    }
}

impl CorrelatedAuthorizationCode {
    /// Consume this correlated success and construct a bounded Authorization
    /// Code Token Request for an unauthenticated public client.
    pub fn try_into_public_client_token_request(
        self,
        limits: AuthorizationCodeTokenRequestLimits,
    ) -> Result<AuthorizationCodeTokenRequest, CredentialOfferError> {
        let (authorization_request, code, issuer_identification) = self.into_token_request_parts();
        let (server, selected_configuration_index, client_id, redirect_uri, code_verifier) =
            authorization_request
                .into_input()
                .into_public_client_token_request_parts();
        let token_endpoint = server
            .authorization_server_metadata()
            .token_endpoint()
            .ok_or(CredentialOfferError::TokenEndpointRequired)?;
        if token_endpoint.as_str().len() > limits.max_token_endpoint_bytes() {
            return Err(CredentialOfferError::AuthorizationCodeTokenEndpointTooLarge);
        }
        let token_endpoint = token_endpoint.duplicate();

        let form_body_len = request_body_len(
            &code,
            redirect_uri.as_str(),
            client_id.as_str(),
            code_verifier.as_str(),
        )
        .ok_or(CredentialOfferError::AuthorizationCodeTokenRequestTooLarge)?;
        if form_body_len > limits.max_form_body_bytes() {
            return Err(CredentialOfferError::AuthorizationCodeTokenRequestTooLarge);
        }

        let mut form_body = Zeroizing::new(String::with_capacity(form_body_len));
        append_pair(&mut form_body, "grant_type", AUTHORIZATION_CODE_GRANT_TYPE);
        form_body.push('&');
        append_pair(&mut form_body, "code", &code);
        form_body.push('&');
        append_pair(&mut form_body, "redirect_uri", redirect_uri.as_str());
        form_body.push('&');
        append_pair(&mut form_body, "client_id", client_id.as_str());
        form_body.push('&');
        append_pair(&mut form_body, "code_verifier", code_verifier.as_str());
        debug_assert_eq!(form_body.len(), form_body_len);

        let selected_configuration = server
            .credential_offer_with_metadata()
            .credential_offer()
            .credential_offer()
            .credential_configuration_ids()
            .get(selected_configuration_index)
            .ok_or(CredentialOfferError::AuthorizationRequestConfigurationMissing)?
            .duplicate();
        let (matched_offer, authorization_server_metadata) = server.into_parts();
        let (_, credential_issuer_metadata) = matched_offer.into_parts();

        Ok(AuthorizationCodeTokenRequest {
            token_endpoint,
            credential_issuer_metadata,
            authorization_server_metadata,
            selected_configuration,
            form_body,
            issuer_identification,
        })
    }
}

fn request_body_len(
    code: &str,
    redirect_uri: &str,
    client_id: &str,
    code_verifier: &str,
) -> Option<usize> {
    pair_len("grant_type", AUTHORIZATION_CODE_GRANT_TYPE)?
        .checked_add(1)?
        .checked_add(pair_len("code", code)?)?
        .checked_add(1)?
        .checked_add(pair_len("redirect_uri", redirect_uri)?)?
        .checked_add(1)?
        .checked_add(pair_len("client_id", client_id)?)?
        .checked_add(1)?
        .checked_add(pair_len("code_verifier", code_verifier)?)
}

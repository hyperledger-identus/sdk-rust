use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, CredentialOfferWithPreAuthorizedTokenInput,
    PRE_AUTHORIZED_CODE_GRANT_TYPE, PreAuthorizedTokenRequestLimits, TokenEndpoint,
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
    form_body: Zeroizing<String>,
    transaction_code_present: bool,
}

impl PreAuthorizedTokenRequest {
    /// Borrow the validated HTTPS Token Endpoint.
    pub const fn token_endpoint(&self) -> &TokenEndpoint {
        &self.token_endpoint
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
            .ok_or(CredentialOfferError::TokenEndpointRequired)?;
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

        Ok(PreAuthorizedTokenRequest {
            token_endpoint: token_endpoint.duplicate(),
            form_body,
            transaction_code_present: transaction_code.is_some(),
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

fn pair_len(name: &str, value: &str) -> Option<usize> {
    encoded_len(name)?
        .checked_add(1)?
        .checked_add(encoded_len(value)?)
}

fn encoded_len(value: &str) -> Option<usize> {
    value.as_bytes().iter().try_fold(0usize, |length, byte| {
        length.checked_add(if is_form_literal(*byte) || *byte == b' ' {
            1
        } else {
            3
        })
    })
}

fn append_pair(output: &mut String, name: &str, value: &str) {
    append_encoded(output, name);
    output.push('=');
    append_encoded(output, value);
}

fn append_encoded(output: &mut String, value: &str) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    for byte in value.bytes() {
        if is_form_literal(byte) {
            output.push(char::from(byte));
        } else if byte == b' ' {
            output.push('+');
        } else {
            output.push('%');
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
}

const fn is_form_literal(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'*' | b'-' | b'.' | b'_')
}

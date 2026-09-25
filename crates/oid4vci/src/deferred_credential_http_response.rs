use std::fmt;

use crate::{
    CredentialOfferError, DeferredCredentialHttpResponseLimits, DeferredCredentialRequest,
    DeferredCredentialResponseCore, ImmediateCredentialResponseCore,
    http_field::is_application_json,
};

/// A successful unencrypted Final response to a Deferred Credential Request.
///
/// This value proves bounded response syntax and, for a pending response,
/// transaction correlation. It does not prove transport origin, credential
/// validity, freshness, retry safety, terminal use, trust, or storage safety.
pub enum DeferredCredentialOutcome {
    /// The issuer returned one or more credentials with HTTP status 200.
    Issued(ImmediateCredentialResponseCore),
    /// The issuer kept the transaction pending with HTTP status 202.
    Pending(DeferredCredentialResponseCore),
}

impl DeferredCredentialOutcome {
    /// Borrow the immediate response when issuance completed.
    pub const fn issued(&self) -> Option<&ImmediateCredentialResponseCore> {
        match self {
            Self::Issued(response) => Some(response),
            Self::Pending(_) => None,
        }
    }

    /// Borrow the deferred response when issuance remains pending.
    pub const fn pending(&self) -> Option<&DeferredCredentialResponseCore> {
        match self {
            Self::Issued(_) => None,
            Self::Pending(response) => Some(response),
        }
    }
}

impl fmt::Debug for DeferredCredentialOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Issued(response) => formatter
                .debug_tuple("DeferredCredentialOutcome::Issued")
                .field(response)
                .finish(),
            Self::Pending(response) => formatter
                .debug_tuple("DeferredCredentialOutcome::Pending")
                .field(response)
                .finish(),
        }
    }
}

impl DeferredCredentialRequest {
    /// Validate an unencrypted Final Deferred Credential HTTP response.
    ///
    /// The caller remains responsible for HTTP execution and origin, network
    /// and cache policy, access-token handling, interval scheduling, retry and
    /// invalidation behavior, credential verification, trust, and storage.
    pub fn validate_response(
        &self,
        status_code: u16,
        content_type: &str,
        body: &str,
        limits: DeferredCredentialHttpResponseLimits,
    ) -> Result<DeferredCredentialOutcome, CredentialOfferError> {
        parse_deferred_success_response(
            self.transaction_id(),
            status_code,
            content_type,
            body,
            limits,
        )
    }
}

pub(crate) fn parse_deferred_success_response(
    transaction_id: &str,
    status_code: u16,
    content_type: &str,
    body: &str,
    limits: DeferredCredentialHttpResponseLimits,
) -> Result<DeferredCredentialOutcome, CredentialOfferError> {
    if status_code != 200 && status_code != 202 {
        return Err(CredentialOfferError::InvalidDeferredCredentialHttpStatus);
    }
    if content_type.len() > limits.max_content_type_bytes() {
        return Err(CredentialOfferError::DeferredCredentialContentTypeTooLarge);
    }
    if !is_application_json(content_type.as_bytes()) {
        return Err(CredentialOfferError::InvalidDeferredCredentialContentType);
    }

    if status_code == 200 {
        return ImmediateCredentialResponseCore::parse(body, limits.immediate_response_limits())
            .map(DeferredCredentialOutcome::Issued);
    }

    let response = DeferredCredentialResponseCore::parse(body, limits.deferred_response_limits())?;
    if response.transaction_id().expose_sensitive_transaction_id() != transaction_id {
        return Err(CredentialOfferError::DeferredCredentialTransactionMismatch);
    }
    Ok(DeferredCredentialOutcome::Pending(response))
}

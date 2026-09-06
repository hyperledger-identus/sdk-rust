use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, CredentialOfferWithPreAuthorizedServer, TransactionCodeInputLimits,
};

/// A Pre-Authorized Code flow whose optional Transaction Code input agrees
/// with the Credential Offer.
///
/// This state proves input presence and resource bounds only. It does not
/// prove that the input is correct or that any Token Request is ready or safe
/// to execute.
pub struct CredentialOfferWithPreAuthorizedTokenInput {
    offer: CredentialOfferWithPreAuthorizedServer,
    transaction_code: Option<Zeroizing<String>>,
}

impl CredentialOfferWithPreAuthorizedTokenInput {
    pub(crate) fn transaction_code(&self) -> Option<&str> {
        self.transaction_code.as_deref().map(String::as_str)
    }

    /// Borrow the bound Pre-Authorized Code server state.
    pub const fn credential_offer_with_pre_authorized_server(
        &self,
    ) -> &CredentialOfferWithPreAuthorizedServer {
        &self.offer
    }

    /// Report whether the Credential Offer required and the caller supplied a
    /// Transaction Code.
    pub const fn transaction_code_present(&self) -> bool {
        self.transaction_code.is_some()
    }
}

impl fmt::Debug for CredentialOfferWithPreAuthorizedTokenInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferWithPreAuthorizedTokenInput")
            .field("transaction_code_present", &self.transaction_code.is_some())
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithPreAuthorizedServer {
    /// Consume this bound server state and optional caller-owned Transaction
    /// Code after validating presence and resource bounds.
    pub fn try_with_transaction_code_input(
        self,
        transaction_code: Option<String>,
        limits: TransactionCodeInputLimits,
    ) -> Result<CredentialOfferWithPreAuthorizedTokenInput, CredentialOfferError> {
        let transaction_code = transaction_code.map(Zeroizing::new);
        let transaction_code_required = self
            .credential_offer_with_metadata()
            .credential_offer()
            .pre_authorized_code()
            .ok_or(CredentialOfferError::PreAuthorizedCodeGrantMissing)?
            .transaction_code()
            .is_some();

        match (transaction_code_required, transaction_code.as_ref()) {
            (true, None) => return Err(CredentialOfferError::TransactionCodeInputRequired),
            (false, Some(_)) => return Err(CredentialOfferError::TransactionCodeInputUnexpected),
            _ => {}
        }

        if let Some(value) = transaction_code.as_ref() {
            if value.is_empty() {
                return Err(CredentialOfferError::TransactionCodeInputEmpty);
            }
            if value.len() > limits.max_transaction_code_bytes() {
                return Err(CredentialOfferError::TransactionCodeInputTooLarge);
            }
        }

        Ok(CredentialOfferWithPreAuthorizedTokenInput {
            offer: self,
            transaction_code,
        })
    }
}

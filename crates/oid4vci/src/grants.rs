use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialOffer, CredentialOfferError, CredentialOfferGrantLimits,
    json::{
        AuthorizationCodeGrantFields, PreAuthorizedCodeGrantFields, TransactionCodeFields,
        parse_credential_offer_grant_fields,
    },
    semantic::is_valid_https_identifier,
};

/// A validated opaque state carried by an Authorization Code grant.
pub struct IssuerState {
    value: Zeroizing<String>,
}

impl IssuerState {
    /// Borrow the exact decoded issuer state.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for IssuerState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuerState")
            .finish_non_exhaustive()
    }
}

/// A syntactically validated RFC 8414 Authorization Server identifier.
pub struct AuthorizationServerIdentifier {
    value: Zeroizing<String>,
}

impl AuthorizationServerIdentifier {
    fn try_from_value(value: Zeroizing<String>) -> Result<Self, CredentialOfferError> {
        if !is_valid_https_identifier(value.as_str()) {
            return Err(CredentialOfferError::UnsafeAuthorizationServer);
        }
        Ok(Self { value })
    }

    /// Borrow the exact decoded Authorization Server identifier.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for AuthorizationServerIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationServerIdentifier")
            .finish_non_exhaustive()
    }
}

/// A validated Authorization Code grant advertised by a Credential Offer.
pub struct AuthorizationCodeGrant {
    issuer_state: Option<IssuerState>,
    authorization_server: Option<AuthorizationServerIdentifier>,
}

impl AuthorizationCodeGrant {
    /// Borrow the optional opaque issuer state.
    pub const fn issuer_state(&self) -> Option<&IssuerState> {
        self.issuer_state.as_ref()
    }

    /// Borrow the optional syntactically validated Authorization Server hint.
    pub const fn authorization_server(&self) -> Option<&AuthorizationServerIdentifier> {
        self.authorization_server.as_ref()
    }
}

impl fmt::Debug for AuthorizationCodeGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationCodeGrant")
            .field("issuer_state_present", &self.issuer_state.is_some())
            .field(
                "authorization_server_present",
                &self.authorization_server.is_some(),
            )
            .finish_non_exhaustive()
    }
}

/// A validated opaque Pre-Authorized Code.
pub struct PreAuthorizedCode {
    value: Zeroizing<String>,
}

impl PreAuthorizedCode {
    /// Borrow the exact decoded Pre-Authorized Code.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for PreAuthorizedCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreAuthorizedCode")
            .finish_non_exhaustive()
    }
}

/// The input character set advertised for a Transaction Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransactionCodeInputMode {
    /// Decimal digits only.
    Numeric,
    /// Any characters accepted by the Authorization Server.
    Text,
}

/// Human-facing guidance for acquiring a Transaction Code.
pub struct TransactionCodeDescription {
    value: Zeroizing<String>,
}

impl TransactionCodeDescription {
    /// Borrow the exact decoded guidance string.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for TransactionCodeDescription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransactionCodeDescription")
            .finish_non_exhaustive()
    }
}

/// Validated requirements for a Transaction Code requested by an offer.
pub struct TransactionCodeRequirements {
    input_mode: Option<TransactionCodeInputMode>,
    length: Option<usize>,
    description: Option<TransactionCodeDescription>,
}

impl TransactionCodeRequirements {
    /// Return the input mode stated by the issuer, if any.
    pub const fn input_mode(&self) -> Option<TransactionCodeInputMode> {
        self.input_mode
    }

    /// Return the stated input mode or the Final default of `numeric`.
    pub const fn effective_input_mode(&self) -> TransactionCodeInputMode {
        match self.input_mode {
            Some(mode) => mode,
            None => TransactionCodeInputMode::Numeric,
        }
    }

    /// Return the optional advertised Transaction Code length.
    pub const fn length(&self) -> Option<usize> {
        self.length
    }

    /// Borrow the optional human-facing guidance.
    pub const fn description(&self) -> Option<&TransactionCodeDescription> {
        self.description.as_ref()
    }
}

impl fmt::Debug for TransactionCodeRequirements {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransactionCodeRequirements")
            .field("input_mode", &self.input_mode)
            .field("length", &self.length)
            .field("description_present", &self.description.is_some())
            .finish_non_exhaustive()
    }
}

/// A validated Pre-Authorized Code grant advertised by a Credential Offer.
pub struct PreAuthorizedCodeGrant {
    pre_authorized_code: PreAuthorizedCode,
    transaction_code: Option<TransactionCodeRequirements>,
    authorization_server: Option<AuthorizationServerIdentifier>,
}

impl PreAuthorizedCodeGrant {
    /// Borrow the opaque Pre-Authorized Code.
    pub const fn pre_authorized_code(&self) -> &PreAuthorizedCode {
        &self.pre_authorized_code
    }

    /// Borrow Transaction Code requirements when the offer requires one.
    pub const fn transaction_code(&self) -> Option<&TransactionCodeRequirements> {
        self.transaction_code.as_ref()
    }

    /// Borrow the optional syntactically validated Authorization Server hint.
    pub const fn authorization_server(&self) -> Option<&AuthorizationServerIdentifier> {
        self.authorization_server.as_ref()
    }
}

impl fmt::Debug for PreAuthorizedCodeGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreAuthorizedCodeGrant")
            .field("transaction_code_present", &self.transaction_code.is_some())
            .field(
                "authorization_server_present",
                &self.authorization_server.is_some(),
            )
            .finish_non_exhaustive()
    }
}

/// A Credential Offer whose two Final known grant shapes are validated.
pub struct CredentialOfferWithGrants {
    offer: CredentialOffer,
    authorization_code: Option<AuthorizationCodeGrant>,
    pre_authorized_code: Option<PreAuthorizedCodeGrant>,
}

impl CredentialOfferWithGrants {
    /// Borrow the prior core-semantic offer and its exact retained JSON.
    pub const fn credential_offer(&self) -> &CredentialOffer {
        &self.offer
    }

    /// Borrow the advertised Authorization Code grant, if present.
    pub const fn authorization_code(&self) -> Option<&AuthorizationCodeGrant> {
        self.authorization_code.as_ref()
    }

    /// Borrow the advertised Pre-Authorized Code grant, if present.
    pub const fn pre_authorized_code(&self) -> Option<&PreAuthorizedCodeGrant> {
        self.pre_authorized_code.as_ref()
    }

    /// Borrow the exact decoded JSON retained from transport validation.
    pub fn as_json(&self) -> &str {
        self.offer.as_json()
    }
}

impl fmt::Debug for CredentialOfferWithGrants {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferWithGrants")
            .field(
                "authorization_code_present",
                &self.authorization_code.is_some(),
            )
            .field(
                "pre_authorized_code_present",
                &self.pre_authorized_code.is_some(),
            )
            .finish_non_exhaustive()
    }
}

impl CredentialOffer {
    /// Consume this core offer and validate its two Final known grant shapes.
    pub fn try_into_grants(
        self,
        limits: CredentialOfferGrantLimits,
    ) -> Result<CredentialOfferWithGrants, CredentialOfferError> {
        let fields = parse_credential_offer_grant_fields(
            self.as_json().as_bytes(),
            self.transport_limits(),
            limits,
        )?;
        let authorization_code = fields
            .authorization_code
            .map(authorization_code_from_fields)
            .transpose()?;
        let pre_authorized_code = fields
            .pre_authorized_code
            .map(pre_authorized_code_from_fields)
            .transpose()?;
        Ok(CredentialOfferWithGrants {
            offer: self,
            authorization_code,
            pre_authorized_code,
        })
    }
}

fn authorization_code_from_fields(
    fields: AuthorizationCodeGrantFields,
) -> Result<AuthorizationCodeGrant, CredentialOfferError> {
    Ok(AuthorizationCodeGrant {
        issuer_state: fields.issuer_state.map(|value| IssuerState { value }),
        authorization_server: fields
            .authorization_server
            .map(AuthorizationServerIdentifier::try_from_value)
            .transpose()?,
    })
}

fn pre_authorized_code_from_fields(
    fields: PreAuthorizedCodeGrantFields,
) -> Result<PreAuthorizedCodeGrant, CredentialOfferError> {
    Ok(PreAuthorizedCodeGrant {
        pre_authorized_code: PreAuthorizedCode {
            value: fields.pre_authorized_code,
        },
        transaction_code: fields
            .transaction_code
            .map(transaction_code_from_fields)
            .transpose()?,
        authorization_server: fields
            .authorization_server
            .map(AuthorizationServerIdentifier::try_from_value)
            .transpose()?,
    })
}

fn transaction_code_from_fields(
    fields: TransactionCodeFields,
) -> Result<TransactionCodeRequirements, CredentialOfferError> {
    let input_mode = fields
        .input_mode
        .map(|value| match value.as_str() {
            "numeric" => Ok(TransactionCodeInputMode::Numeric),
            "text" => Ok(TransactionCodeInputMode::Text),
            _ => Err(CredentialOfferError::InvalidTransactionCodeMode),
        })
        .transpose()?;
    Ok(TransactionCodeRequirements {
        input_mode,
        length: fields.length,
        description: fields
            .description
            .map(|value| TransactionCodeDescription { value }),
    })
}

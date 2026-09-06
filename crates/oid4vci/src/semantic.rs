use std::fmt;

use uriparse::URI;
use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, CredentialOfferSemanticLimits, EmbeddedCredentialOffer,
    json::parse_credential_offer_fields,
};

/// A syntactically validated OID4VCI Credential Issuer Identifier.
pub struct CredentialIssuerIdentifier {
    value: Zeroizing<String>,
}

impl CredentialIssuerIdentifier {
    fn try_from_value(value: Zeroizing<String>) -> Result<Self, CredentialOfferError> {
        let bytes = value.as_bytes();
        if !bytes
            .get(..8)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(b"https://"))
        {
            return Err(CredentialOfferError::UnsafeCredentialIssuer);
        }
        let authority = &bytes[8..bytes[8..]
            .iter()
            .position(|byte| matches!(byte, b'/' | b'?' | b'#'))
            .map_or(bytes.len(), |offset| 8 + offset)];
        if authority.is_empty() || authority.first() == Some(&b':') {
            return Err(CredentialOfferError::UnsafeCredentialIssuer);
        }
        let parsed = URI::try_from(value.as_str())
            .map_err(|_| CredentialOfferError::UnsafeCredentialIssuer)?;
        if !parsed.scheme().as_str().eq_ignore_ascii_case("https")
            || parsed.host().is_none()
            || parsed.has_username()
            || parsed.has_password()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(CredentialOfferError::UnsafeCredentialIssuer);
        }
        Ok(Self { value })
    }

    /// Borrow the exact decoded Credential Issuer Identifier.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for CredentialIssuerIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialIssuerIdentifier")
            .finish_non_exhaustive()
    }
}

/// One decoded Credential Configuration ID from a validated offer.
pub struct CredentialConfigurationId {
    value: Zeroizing<String>,
}

impl CredentialConfigurationId {
    /// Borrow the exact decoded Credential Configuration ID.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for CredentialConfigurationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialConfigurationId")
            .finish_non_exhaustive()
    }
}

/// An embedded Credential Offer with validated OID4VCI 1.0 Final core members.
pub struct CredentialOffer {
    embedded: EmbeddedCredentialOffer,
    credential_issuer: CredentialIssuerIdentifier,
    credential_configuration_ids: Vec<CredentialConfigurationId>,
    grants_present: bool,
}

impl CredentialOffer {
    /// Consume validated transport JSON and validate its core semantic members.
    pub fn try_from_embedded(
        embedded: EmbeddedCredentialOffer,
        limits: CredentialOfferSemanticLimits,
    ) -> Result<Self, CredentialOfferError> {
        let fields = parse_credential_offer_fields(
            embedded.as_json().as_bytes(),
            embedded.limits(),
            limits,
        )?;
        let credential_issuer =
            CredentialIssuerIdentifier::try_from_value(fields.credential_issuer)?;
        let credential_configuration_ids = fields
            .credential_configuration_ids
            .into_iter()
            .map(|value| CredentialConfigurationId { value })
            .collect();
        Ok(Self {
            embedded,
            credential_issuer,
            credential_configuration_ids,
            grants_present: fields.grants_present,
        })
    }

    /// Borrow the validated Credential Issuer Identifier.
    pub const fn credential_issuer(&self) -> &CredentialIssuerIdentifier {
        &self.credential_issuer
    }

    /// Borrow the ordered, duplicate-free Credential Configuration IDs.
    pub fn credential_configuration_ids(&self) -> &[CredentialConfigurationId] {
        &self.credential_configuration_ids
    }

    /// Report whether the opaque `grants` object was present.
    pub const fn grants_present(&self) -> bool {
        self.grants_present
    }

    /// Borrow the exact decoded JSON retained from transport validation.
    pub fn as_json(&self) -> &str {
        self.embedded.as_json()
    }
}

impl fmt::Debug for CredentialOffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOffer")
            .field(
                "credential_configuration_ids_len",
                &self.credential_configuration_ids.len(),
            )
            .field("grants_present", &self.grants_present)
            .finish_non_exhaustive()
    }
}

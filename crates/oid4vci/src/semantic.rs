use std::fmt;

use fluent_uri::Uri as ParsedUri;
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
    pub(crate) fn try_from_value(value: Zeroizing<String>) -> Result<Self, CredentialOfferError> {
        if !is_valid_https_identifier(value.as_str()) {
            return Err(CredentialOfferError::UnsafeCredentialIssuer);
        }
        Ok(Self { value })
    }

    /// Borrow the exact decoded Credential Issuer Identifier.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

pub(crate) fn is_valid_https_identifier(value: &str) -> bool {
    is_valid_https_uri(value, false)
}

pub(crate) fn is_valid_https_endpoint(value: &str) -> bool {
    is_valid_https_uri(value, true)
}

fn is_valid_https_uri(value: &str, allow_query: bool) -> bool {
    let bytes = value.as_bytes();
    if !bytes
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(b"https://"))
    {
        return false;
    }
    let authority_end = bytes[8..]
        .iter()
        .position(|byte| matches!(byte, b'/' | b'?' | b'#'))
        .map_or(bytes.len(), |offset| 8 + offset);
    let authority = &bytes[8..authority_end];
    if authority.is_empty() || authority.first() == Some(&b':') {
        return false;
    }
    ParsedUri::parse(value).is_ok_and(|parsed| has_safe_https_components(&parsed, allow_query))
}

fn has_safe_https_components(parsed: &ParsedUri<&str>, allow_query: bool) -> bool {
    let Some(authority) = parsed.authority() else {
        return false;
    };
    parsed.scheme().as_str().eq_ignore_ascii_case("https")
        && !authority.host().is_empty()
        && authority.userinfo().is_none()
        && (allow_query || parsed.query().is_none())
        && parsed.fragment().is_none()
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

    pub(crate) const fn transport_limits(&self) -> crate::CredentialOfferLimits {
        self.embedded.limits()
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

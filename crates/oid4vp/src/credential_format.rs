//! SDK-owned credential-format identities and bounded capability ports.

use std::fmt;

use serde_json::Value;
use zeroize::Zeroizing;

use crate::Oid4vpError;

/// Credential format identifiers recognized by OpenID4VP Final.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CredentialFormat {
    /// SD-JWT VC using the `dc+sd-jwt` identifier.
    DcSdJwt,
    /// ISO mdoc using the `mso_mdoc` identifier.
    MsoMdoc,
    /// JWT-encoded W3C Verifiable Credential.
    JwtVcJson,
    /// Linked-data W3C Verifiable Credential.
    LdpVc,
    /// A bounded extension format whose semantics are caller-owned.
    Extension(Zeroizing<String>),
}

impl CredentialFormat {
    pub(crate) fn parse(value: &str) -> Self {
        match value {
            "dc+sd-jwt" => Self::DcSdJwt,
            "mso_mdoc" => Self::MsoMdoc,
            "jwt_vc_json" => Self::JwtVcJson,
            "ldp_vc" => Self::LdpVc,
            value => Self::Extension(Zeroizing::new(value.to_owned())),
        }
    }

    /// Construct one bounded extension identifier for a caller-owned format.
    pub fn extension(
        identifier: impl Into<String>,
        max_identifier_bytes: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_identifier_bytes == 0 {
            return Err(Oid4vpError::InvalidLimits);
        }
        let identifier = Zeroizing::new(identifier.into());
        if identifier.is_empty() || identifier.len() > max_identifier_bytes {
            return Err(Oid4vpError::InvalidDcqlQuery);
        }
        Ok(Self::Extension(identifier))
    }

    /// Explicitly reveal the exact wire identifier.
    pub fn expose_sensitive_identifier(&self) -> &str {
        match self {
            Self::DcSdJwt => "dc+sd-jwt",
            Self::MsoMdoc => "mso_mdoc",
            Self::JwtVcJson => "jwt_vc_json",
            Self::LdpVc => "ldp_vc",
            Self::Extension(value) => value,
        }
    }
}

impl fmt::Debug for CredentialFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DcSdJwt => formatter.write_str("DcSdJwt"),
            Self::MsoMdoc => formatter.write_str("MsoMdoc"),
            Self::JwtVcJson => formatter.write_str("JwtVcJson"),
            Self::LdpVc => formatter.write_str("LdpVc"),
            Self::Extension(value) => formatter
                .debug_struct("Extension")
                .field("identifier_len", &value.len())
                .finish(),
        }
    }
}

/// Static outcome returned by a credential-format capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialFormatCapabilityError {
    /// Metadata has the wrong shape or value for the format.
    InvalidMetadata,
    /// Metadata requests a valid but unsupported profile feature.
    UnsupportedMetadata,
}

/// Caller-owned validation for one credential format.
///
/// Implementations must be deterministic, effect-free, and keep foreign DTOs
/// and diagnostic values behind this static SDK boundary.
pub trait CredentialFormatCapability {
    /// Exact credential format implemented by this capability.
    fn format(&self) -> CredentialFormat;

    /// Validate already resource-bounded DCQL metadata.
    fn validate_metadata(&self, metadata: &Value) -> Result<(), CredentialFormatCapabilityError>;

    /// Whether this format can bind the named transaction-data type.
    fn supports_transaction_type(&self, transaction_type: &str) -> bool;
}

pub(crate) fn validate_format_metadata(
    format: &CredentialFormat,
    metadata: &Value,
    capabilities: &[&dyn CredentialFormatCapability],
) -> Result<(), Oid4vpError> {
    let Some(object) = metadata.as_object() else {
        return Err(Oid4vpError::InvalidDcqlQuery);
    };
    if object.is_empty() {
        return Ok(());
    }

    let mut matching = capabilities.iter().filter(|capability| {
        capability.format().expose_sensitive_identifier() == format.expose_sensitive_identifier()
    });
    let capability = matching.next().ok_or(Oid4vpError::InvalidDcqlQuery)?;
    if matching.next().is_some() {
        return Err(Oid4vpError::InvalidDcqlQuery);
    }
    capability
        .validate_metadata(metadata)
        .map_err(|_| Oid4vpError::InvalidDcqlQuery)
}

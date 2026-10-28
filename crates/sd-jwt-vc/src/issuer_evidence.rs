use std::fmt;

use identus_crypto::PublicKeyJwk;
use identus_jose::ProtectedHeader;
use uriparse::URIReference;

use crate::{SdJwtVcError, SdJwtVcLimits, resource::bounded_json_size};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IssuerEvidenceKind {
    InlineX509,
    IssuerMetadataInlineJwks,
    IssuerMetadataJwksUri,
}

#[derive(Clone)]
enum KeySetEvidence {
    Inline(Vec<PublicKeyJwk>),
    Uri {
        uri: String,
        resolved_keys: Vec<PublicKeyJwk>,
    },
}

/// Already retrieved, bounded JWT VC Issuer Metadata evidence.
#[derive(Clone)]
pub struct IssuerMetadataEvidence {
    issuer: String,
    key_set: KeySetEvidence,
    encoded_bytes: usize,
}

impl IssuerMetadataEvidence {
    pub fn inline_jwks(
        issuer: &str,
        keys: Vec<PublicKeyJwk>,
        encoded_bytes: usize,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        validate_issuer(issuer, limits)?;
        let retained_bytes = retained_key_set_bytes(issuer, None, &keys, limits)?;
        if keys.is_empty()
            || keys.len() > limits.jwk_set_keys
            || encoded_bytes == 0
            || encoded_bytes > limits.issuer_metadata_bytes
        {
            return Err(SdJwtVcError::ResourceLimit);
        }
        Ok(Self {
            issuer: issuer.to_owned(),
            key_set: KeySetEvidence::Inline(keys),
            encoded_bytes: encoded_bytes.max(retained_bytes),
        })
    }

    pub fn jwks_uri(
        issuer: &str,
        jwks_uri: &str,
        resolved_keys: Vec<PublicKeyJwk>,
        encoded_bytes: usize,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        validate_issuer(issuer, limits)?;
        validate_https_uri(jwks_uri, limits.issuer_identifier_bytes)?;
        let retained_bytes =
            retained_key_set_bytes(issuer, Some(jwks_uri), &resolved_keys, limits)?;
        if resolved_keys.is_empty()
            || resolved_keys.len() > limits.jwk_set_keys
            || encoded_bytes == 0
            || encoded_bytes > limits.issuer_metadata_bytes
        {
            return Err(SdJwtVcError::ResourceLimit);
        }
        Ok(Self {
            issuer: issuer.to_owned(),
            key_set: KeySetEvidence::Uri {
                uri: jwks_uri.to_owned(),
                resolved_keys,
            },
            encoded_bytes: encoded_bytes.max(retained_bytes),
        })
    }

    pub(crate) fn correlate(
        &self,
        issuer: &str,
        verification_key: &PublicKeyJwk,
    ) -> Result<IssuerEvidenceKind, SdJwtVcError> {
        if self.issuer != issuer {
            return Err(SdJwtVcError::EvidenceMismatch);
        }
        match &self.key_set {
            KeySetEvidence::Inline(keys) => {
                if !keys.iter().any(|candidate| candidate == verification_key) {
                    return Err(SdJwtVcError::EvidenceMismatch);
                }
                Ok(IssuerEvidenceKind::IssuerMetadataInlineJwks)
            }
            KeySetEvidence::Uri { uri, resolved_keys } => {
                let _ = uri.len();
                if !resolved_keys
                    .iter()
                    .any(|candidate| candidate == verification_key)
                {
                    return Err(SdJwtVcError::EvidenceMismatch);
                }
                Ok(IssuerEvidenceKind::IssuerMetadataJwksUri)
            }
        }
    }
}

fn retained_key_set_bytes(
    issuer: &str,
    uri: Option<&str>,
    keys: &[PublicKeyJwk],
    limits: SdJwtVcLimits,
) -> Result<usize, SdJwtVcError> {
    if keys.is_empty() || keys.len() > limits.jwk_set_keys {
        return Err(SdJwtVcError::ResourceLimit);
    }
    let key_bytes = bounded_json_size(keys, limits.issuer_metadata_bytes)?;
    issuer
        .len()
        .checked_add(uri.map_or(0, str::len))
        .and_then(|bytes| bytes.checked_add(key_bytes))
        .filter(|bytes| *bytes <= limits.issuer_metadata_bytes)
        .ok_or(SdJwtVcError::ResourceLimit)
}

impl fmt::Debug for IssuerMetadataEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuerMetadataEvidence")
            .field("issuer_bytes", &self.issuer.len())
            .field("encoded_bytes", &self.encoded_bytes)
            .field(
                "kind",
                &match self.key_set {
                    KeySetEvidence::Inline(_) => IssuerEvidenceKind::IssuerMetadataInlineJwks,
                    KeySetEvidence::Uri { .. } => IssuerEvidenceKind::IssuerMetadataJwksUri,
                },
            )
            .finish()
    }
}

/// Caller-produced structural discovery evidence. It is never a trust result.
#[derive(Clone)]
pub enum IssuerEvidence {
    InlineX509 {
        certificate_chain: Vec<String>,
        extracted_key: PublicKeyJwk,
    },
    Metadata(IssuerMetadataEvidence),
}

impl IssuerEvidence {
    pub(crate) fn correlate(
        &self,
        header: &ProtectedHeader,
        verification_key: &PublicKeyJwk,
        issuer: &str,
        limits: SdJwtVcLimits,
    ) -> Result<IssuerEvidenceKind, SdJwtVcError> {
        match self {
            Self::InlineX509 {
                certificate_chain,
                extracted_key,
            } => {
                let chain_bytes =
                    bounded_json_size(certificate_chain, limits.issuer_metadata_bytes)?;
                let key_bytes = bounded_json_size(extracted_key, limits.issuer_metadata_bytes)?;
                if certificate_chain.is_empty()
                    || chain_bytes
                        .checked_add(key_bytes)
                        .is_none_or(|bytes| bytes > limits.issuer_metadata_bytes)
                {
                    return Err(SdJwtVcError::ResourceLimit);
                }
                if header.certificate_chain() != Some(certificate_chain.as_slice())
                    || extracted_key != verification_key
                {
                    return Err(SdJwtVcError::EvidenceMismatch);
                }
                Ok(IssuerEvidenceKind::InlineX509)
            }
            Self::Metadata(metadata) => {
                metadata.validate_active_limits(limits)?;
                metadata.correlate(issuer, verification_key)
            }
        }
    }
}

impl IssuerMetadataEvidence {
    fn validate_active_limits(&self, limits: SdJwtVcLimits) -> Result<(), SdJwtVcError> {
        validate_issuer(&self.issuer, limits)?;
        if self.encoded_bytes > limits.issuer_metadata_bytes {
            return Err(SdJwtVcError::ResourceLimit);
        }
        match &self.key_set {
            KeySetEvidence::Inline(keys) => {
                retained_key_set_bytes(&self.issuer, None, keys, limits)?;
            }
            KeySetEvidence::Uri { uri, resolved_keys } => {
                validate_https_uri(uri, limits.issuer_identifier_bytes)?;
                retained_key_set_bytes(&self.issuer, Some(uri), resolved_keys, limits)?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for IssuerEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InlineX509 {
                certificate_chain, ..
            } => formatter
                .debug_struct("IssuerEvidence")
                .field("kind", &IssuerEvidenceKind::InlineX509)
                .field("certificate_count", &certificate_chain.len())
                .finish(),
            Self::Metadata(value) => value.fmt(formatter),
        }
    }
}

fn validate_issuer(value: &str, limits: SdJwtVcLimits) -> Result<(), SdJwtVcError> {
    if value.is_empty()
        || value.len() > limits.issuer_identifier_bytes
        || value.chars().any(char::is_control)
    {
        return Err(SdJwtVcError::IssuerEvidence);
    }
    Ok(())
}

fn validate_https_uri(value: &str, maximum: usize) -> Result<(), SdJwtVcError> {
    if value.len() > maximum {
        return Err(SdJwtVcError::ResourceLimit);
    }
    let uri = URIReference::try_from(value).map_err(|_| SdJwtVcError::IssuerEvidence)?;
    if uri.scheme().map(|scheme| scheme.as_str()) != Some("https") || uri.host().is_none() {
        return Err(SdJwtVcError::IssuerEvidence);
    }
    Ok(())
}

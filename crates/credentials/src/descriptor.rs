use std::{fmt, str::FromStr};

use crate::CredentialError;

/// Maximum encoded length of an entity identifier.
pub const MAX_CREDENTIAL_ENTITY_ID_BYTES: usize = 1_024;
/// Maximum encoded length of a credential type.
pub const MAX_CREDENTIAL_TYPE_BYTES: usize = 256;
/// Maximum encoded length of a schema identifier.
pub const MAX_CREDENTIAL_SCHEMA_ID_BYTES: usize = 1_024;
/// Maximum encoded length of a schema version.
pub const MAX_CREDENTIAL_SCHEMA_VERSION_BYTES: usize = 128;
/// Maximum encoded length of a claim identifier.
pub const MAX_CREDENTIAL_CLAIM_ID_BYTES: usize = 128;
/// Maximum encoded length of a claim value-type hint.
pub const MAX_CREDENTIAL_CLAIM_VALUE_TYPE_BYTES: usize = 128;
/// Maximum encoded length of one claim-path segment.
pub const MAX_CREDENTIAL_CLAIM_PATH_SEGMENT_BYTES: usize = 128;

fn validate_descriptor_text(
    value: &str,
    maximum: usize,
    error: CredentialError,
) -> Result<(), CredentialError> {
    if value.is_empty()
        || value.len() > maximum
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(error);
    }
    Ok(())
}

macro_rules! descriptor_type {
    ($(#[$meta:meta])* $name:ident, $maximum:ident, $error:ident) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Parse and own a descriptor after validating the borrowed input.
            pub fn parse(value: &str) -> Result<Self, CredentialError> {
                validate_descriptor_text(value, $maximum, CredentialError::$error)?;
                Ok(Self(value.to_owned()))
            }

            /// Return the exact validated descriptor text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = CredentialError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }
    };
}

/// Bounded format-neutral identifier for a credential issuer or subject.
///
/// Concrete adapters remain responsible for URL, DID, or profile grammar.
#[must_use]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CredentialEntityId(String);

impl CredentialEntityId {
    /// Parse and own an identifier after validating the borrowed input.
    pub fn parse(value: &str) -> Result<Self, CredentialError> {
        validate_descriptor_text(
            value,
            MAX_CREDENTIAL_ENTITY_ID_BYTES,
            CredentialError::InvalidEntityIdentifier,
        )?;
        Ok(Self(value.to_owned()))
    }

    /// Return the exact validated identifier through an explicit accessor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for CredentialEntityId {
    type Err = CredentialError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Debug for CredentialEntityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialEntityId")
            .field("length", &self.0.len())
            .finish_non_exhaustive()
    }
}

descriptor_type!(
    /// Bounded format-neutral credential type term or URL.
    CredentialType,
    MAX_CREDENTIAL_TYPE_BYTES,
    InvalidCredentialType
);

descriptor_type!(
    /// Bounded format-neutral schema identifier.
    CredentialSchemaId,
    MAX_CREDENTIAL_SCHEMA_ID_BYTES,
    InvalidSchemaIdentifier
);

descriptor_type!(
    /// Bounded optional schema-version spelling.
    CredentialSchemaVersion,
    MAX_CREDENTIAL_SCHEMA_VERSION_BYTES,
    InvalidSchemaVersion
);

descriptor_type!(
    /// Bounded claim identifier unique within a schema.
    CredentialClaimId,
    MAX_CREDENTIAL_CLAIM_ID_BYTES,
    InvalidClaimIdentifier
);

descriptor_type!(
    /// Bounded format-owned hint describing a claim value type.
    CredentialClaimValueType,
    MAX_CREDENTIAL_CLAIM_VALUE_TYPE_BYTES,
    InvalidClaimValueType
);

descriptor_type!(
    /// One bounded format-neutral segment in a claim path.
    CredentialClaimPathSegment,
    MAX_CREDENTIAL_CLAIM_PATH_SEGMENT_BYTES,
    InvalidClaimPathSegment
);

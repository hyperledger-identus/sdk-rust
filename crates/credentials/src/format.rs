use std::{fmt, str::FromStr};

use crate::CredentialError;

/// Maximum encoded length of a credential format identifier.
pub const MAX_CREDENTIAL_FORMAT_BYTES: usize = 128;

/// Open, validated identifier owned by a credential-format adapter.
///
/// The SDK validates only a small resource-safe token grammar. It does not
/// maintain a closed registry or claim support for a syntactically valid
/// format.
#[must_use]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CredentialFormat(String);

impl CredentialFormat {
    /// Parse and own a format identifier after validating the borrowed input.
    pub fn parse(value: &str) -> Result<Self, CredentialError> {
        let bytes = value.as_bytes();
        let Some((first, rest)) = bytes.split_first() else {
            return Err(CredentialError::InvalidFormat);
        };

        if bytes.len() > MAX_CREDENTIAL_FORMAT_BYTES
            || !first.is_ascii_alphanumeric()
            || !rest.iter().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-' | b':')
            })
        {
            return Err(CredentialError::InvalidFormat);
        }

        Ok(Self(value.to_owned()))
    }

    /// Return the exact validated identifier spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for CredentialFormat {
    type Err = CredentialError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Debug for CredentialFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialFormat")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for CredentialFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

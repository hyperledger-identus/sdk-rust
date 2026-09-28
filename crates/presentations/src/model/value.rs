use std::{fmt, str::FromStr};

use crate::PresentationError;

use super::{
    MAX_PRESENTATION_CHALLENGE_BYTES, MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES,
    MAX_PRESENTATION_PURPOSE_BYTES, MAX_PRESENTATION_QUERY_ID_BYTES,
};

fn valid_text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

/// Bounded identifier correlating one credential query with its candidates.
#[must_use]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentationQueryId(String);

impl PresentationQueryId {
    /// Parse and own a query identifier after validating borrowed input.
    pub fn parse(value: &str) -> Result<Self, PresentationError> {
        let bytes = value.as_bytes();
        let valid = !bytes.is_empty()
            && bytes.len() <= MAX_PRESENTATION_QUERY_ID_BYTES
            && bytes[0].is_ascii_alphanumeric()
            && bytes[1..].iter().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':')
            });
        if !valid {
            return Err(PresentationError::InvalidQueryId);
        }
        Ok(Self(value.to_owned()))
    }

    /// Return the exact validated query identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for PresentationQueryId {
    type Err = PresentationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Debug for PresentationQueryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationQueryId")
            .field("length", &self.0.len())
            .finish_non_exhaustive()
    }
}

/// Bounded human-readable purpose supplied with a presentation request.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationPurpose(String);

impl PresentationPurpose {
    /// Parse and own exact purpose text after validating borrowed input.
    pub fn parse(value: &str) -> Result<Self, PresentationError> {
        if !valid_text(value, MAX_PRESENTATION_PURPOSE_BYTES) {
            return Err(PresentationError::InvalidPurpose);
        }
        Ok(Self(value.to_owned()))
    }

    /// Return the exact validated purpose text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for PresentationPurpose {
    type Err = PresentationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Debug for PresentationPurpose {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationPurpose")
            .field("length", &self.0.len())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, PartialEq, Eq)]
enum OpaquePresentationValue {
    Text(String),
    Bytes(Vec<u8>),
}

impl OpaquePresentationValue {
    fn from_text(
        value: &str,
        maximum: usize,
        error: PresentationError,
    ) -> Result<Self, PresentationError> {
        if !valid_text(value, maximum) {
            return Err(error);
        }
        Ok(Self::Text(value.to_owned()))
    }

    fn from_bytes(
        value: Vec<u8>,
        maximum: usize,
        error: PresentationError,
    ) -> Result<Self, PresentationError> {
        if value.is_empty() || value.len() > maximum {
            return Err(error);
        }
        Ok(Self::Bytes(value))
    }

    fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value.as_str()),
            Self::Bytes(_) => None,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Text(value) => value.as_bytes(),
            Self::Bytes(value) => value,
        }
    }

    const fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Bytes(_) => "bytes",
        }
    }
}

macro_rules! opaque_value_type {
    ($(#[$meta:meta])* $name:ident, $maximum:ident, $error:ident) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name(OpaquePresentationValue);

        impl $name {
            /// Preserve exact validated text.
            pub fn from_text(value: &str) -> Result<Self, PresentationError> {
                Ok(Self(OpaquePresentationValue::from_text(
                    value,
                    $maximum,
                    PresentationError::$error,
                )?))
            }

            /// Preserve an exact transferred byte vector.
            pub fn from_bytes(value: Vec<u8>) -> Result<Self, PresentationError> {
                Ok(Self(OpaquePresentationValue::from_bytes(
                    value,
                    $maximum,
                    PresentationError::$error,
                )?))
            }

            /// Return text when the value was constructed as text.
            pub fn as_text(&self) -> Option<&str> {
                self.0.as_text()
            }

            /// Return exact bytes for either representation.
            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("kind", &self.0.kind())
                    .field("length", &self.0.as_bytes().len())
                    .finish_non_exhaustive()
            }
        }
    };
}

opaque_value_type!(
    /// Opaque bounded replay-binding input supplied by a protocol adapter.
    PresentationChallenge,
    MAX_PRESENTATION_CHALLENGE_BYTES,
    InvalidChallenge
);

opaque_value_type!(
    /// Opaque bounded local reference to one stored credential.
    PresentationCredentialHandle,
    MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES,
    InvalidCredentialHandle
);

/// Generic disclosure intent for one requested claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PresentationClaimIntent {
    /// Reveal a claim through the selected credential format.
    Reveal,
    /// Prove a format-owned predicate without defining its parameters here.
    Predicate,
}

impl PresentationClaimIntent {
    /// Return the stable machine spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reveal => "reveal",
            Self::Predicate => "predicate",
        }
    }

    /// Parse a stable machine spelling without allocation.
    pub fn parse(value: &str) -> Result<Self, PresentationError> {
        match value {
            "reveal" => Ok(Self::Reveal),
            "predicate" => Ok(Self::Predicate),
            _ => Err(PresentationError::InvalidClaimIntent),
        }
    }
}

impl FromStr for PresentationClaimIntent {
    type Err = PresentationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PresentationClaimIntent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

use std::fmt;

use crate::{Error, error::RegistrationError};

use super::{MAX_REGISTRATION_ID_BYTES, MAX_REGISTRATION_OPAQUE_ID_BYTES};

macro_rules! opaque_identifier {
    ($(#[$meta:meta])* $name:ident, $limit:expr) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Validate and retain an owned identifier.
            pub fn try_new(value: String) -> Result<Self, Error> {
                validate_identifier(&value, $limit)?;
                Ok(Self(value))
            }

            /// Parse and retain a borrowed identifier.
            pub fn parse(value: &str) -> Result<Self, Error> {
                validate_identifier(value, $limit)?;
                Ok(Self(value.to_owned()))
            }

            /// Borrow the validated identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume the value and return its string representation.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&"<redacted>").finish()
            }
        }
    };
}

opaque_identifier!(
    /// A retry key that identifies one canonical mutation request.
    RegistrationIdempotencyKey,
    MAX_REGISTRATION_ID_BYTES
);
opaque_identifier!(
    /// An opaque, method-owned identifier for a non-terminal registration job.
    RegistrationJobId,
    MAX_REGISTRATION_OPAQUE_ID_BYTES
);
opaque_identifier!(
    /// A bounded identifier correlating one client-managed action.
    RegistrationActionId,
    MAX_REGISTRATION_ID_BYTES
);
opaque_identifier!(
    /// An opaque reference into an adapter-owned custody system.
    RegistrationSecretHandle,
    MAX_REGISTRATION_OPAQUE_ID_BYTES
);
opaque_identifier!(
    /// A bounded method-specific update or action name.
    RegistrationOperationName,
    MAX_REGISTRATION_ID_BYTES
);
opaque_identifier!(
    /// A bounded, stable standard or method-defined registration failure code.
    RegistrationFailureCode,
    MAX_REGISTRATION_ID_BYTES
);

/// Stable generic registration failure classifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DidRegistrationErrorKind {
    /// The exact DID method is not registered.
    MethodNotSupported,
    /// The method is known but has no registration capability.
    FeatureNotSupported,
    /// One idempotency key was reused with different canonical input.
    Conflict,
    /// An explicit cancellation completed before irreversible work.
    Cancelled,
    /// Registration failed unexpectedly.
    InternalError,
}

impl DidRegistrationErrorKind {
    /// Return the stable failure code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MethodNotSupported => "methodNotSupported",
            Self::FeatureNotSupported => "featureNotSupported",
            Self::Conflict => "conflict",
            Self::Cancelled => "cancelled",
            Self::InternalError => "internalError",
        }
    }
}

impl RegistrationFailureCode {
    /// Construct one infallible standard failure code.
    #[must_use]
    pub fn standard(kind: DidRegistrationErrorKind) -> Self {
        Self(kind.as_str().to_owned())
    }

    /// Classify this code when it is one of the SDK standard values.
    #[must_use]
    pub fn kind(&self) -> Option<DidRegistrationErrorKind> {
        Some(match self.as_str() {
            "methodNotSupported" => DidRegistrationErrorKind::MethodNotSupported,
            "featureNotSupported" => DidRegistrationErrorKind::FeatureNotSupported,
            "conflict" => DidRegistrationErrorKind::Conflict,
            "cancelled" => DidRegistrationErrorKind::Cancelled,
            "internalError" => DidRegistrationErrorKind::InternalError,
            _ => return None,
        })
    }
}
fn validate_identifier(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(invalid(RegistrationError::InvalidString));
    }
    Ok(())
}
const fn invalid(reason: RegistrationError) -> Error {
    Error::InvalidRegistration(reason)
}

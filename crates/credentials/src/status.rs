use std::{fmt, str::FromStr};

use identus_core::{DurationMillis, UnixTimestampMillis};

use crate::{CredentialError, schema::has_duplicates};

/// Maximum encoded length of a credential-status method identifier.
pub const MAX_CREDENTIAL_STATUS_METHOD_BYTES: usize = 256;
/// Maximum encoded length of a credential-status purpose identifier.
pub const MAX_CREDENTIAL_STATUS_PURPOSE_BYTES: usize = 256;
/// Maximum encoded length of an opaque credential-status reference.
pub const MAX_CREDENTIAL_STATUS_REFERENCE_BYTES: usize = 2_048;
/// Maximum encoded length of an opaque credential-status handle.
pub const MAX_CREDENTIAL_STATUS_HANDLE_BYTES: usize = 1_024;
/// Maximum encoded length of an opaque credential-status revision.
pub const MAX_CREDENTIAL_STATUS_REVISION_BYTES: usize = 256;
/// Maximum encoded length of an opaque observed credential-status value.
pub const MAX_CREDENTIAL_STATUS_VALUE_BYTES: usize = 256;
/// Maximum number of complete status bindings carried by one collection.
pub const MAX_CREDENTIAL_STATUS_BINDINGS: usize = 16;
/// Maximum number of methods or purposes in one query allow-list.
pub const MAX_CREDENTIAL_STATUS_REQUIREMENT_VALUES: usize = 16;

fn validate_status_text(
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

macro_rules! status_identifier {
    ($(#[$meta:meta])* $name:ident, $maximum:ident, $error:ident) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Parse and own an identifier after validating the borrowed input.
            pub fn parse(value: &str) -> Result<Self, CredentialError> {
                validate_status_text(value, $maximum, CredentialError::$error)?;
                Ok(Self(value.to_owned()))
            }

            /// Return the exact validated identifier.
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

status_identifier!(
    /// Open bounded identifier for a credential-status method or entry type.
    CredentialStatusMethod,
    MAX_CREDENTIAL_STATUS_METHOD_BYTES,
    InvalidStatusMethod
);

status_identifier!(
    /// Open bounded identifier for why a credential-status entry is processed.
    CredentialStatusPurpose,
    MAX_CREDENTIAL_STATUS_PURPOSE_BYTES,
    InvalidStatusPurpose
);

#[derive(Clone, PartialEq, Eq, Hash)]
enum StatusOpaque {
    Text(String),
    Bytes(Vec<u8>),
}

impl StatusOpaque {
    fn from_text(
        value: &str,
        maximum: usize,
        error: CredentialError,
    ) -> Result<Self, CredentialError> {
        validate_status_text(value, maximum, error)?;
        Ok(Self::Text(value.to_owned()))
    }

    fn from_bytes(
        value: Vec<u8>,
        maximum: usize,
        error: CredentialError,
    ) -> Result<Self, CredentialError> {
        if value.is_empty() || value.len() > maximum {
            return Err(error);
        }
        Ok(Self::Bytes(value))
    }

    fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            Self::Bytes(_) => None,
        }
    }

    fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Text(_) => None,
            Self::Bytes(value) => Some(value),
        }
    }

    fn representation(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Bytes(_) => "bytes",
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Text(value) => value.len(),
            Self::Bytes(value) => value.len(),
        }
    }
}

macro_rules! status_opaque_type {
    ($(#[$meta:meta])* $name:ident, $maximum:ident, $error:ident) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(StatusOpaque);

        impl $name {
            /// Validate and own an exact textual value.
            pub fn from_text(value: &str) -> Result<Self, CredentialError> {
                StatusOpaque::from_text(value, $maximum, CredentialError::$error).map(Self)
            }

            /// Validate and retain an exact transferred byte vector.
            pub fn from_bytes(value: Vec<u8>) -> Result<Self, CredentialError> {
                StatusOpaque::from_bytes(value, $maximum, CredentialError::$error).map(Self)
            }

            /// Return text when this value was constructed from text.
            pub fn as_text(&self) -> Option<&str> {
                self.0.as_text()
            }

            /// Return bytes when this value was constructed from bytes.
            pub fn as_bytes(&self) -> Option<&[u8]> {
                self.0.as_bytes()
            }

            /// Return the encoded byte length without exposing the value.
            pub fn encoded_len(&self) -> usize {
                self.0.len()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("representation", &self.0.representation())
                    .field("length", &self.0.len())
                    .finish_non_exhaustive()
            }
        }
    };
}

status_opaque_type!(
    /// Opaque location or domain used to retrieve credential status.
    CredentialStatusReference,
    MAX_CREDENTIAL_STATUS_REFERENCE_BYTES,
    InvalidStatusReference
);

status_opaque_type!(
    /// Opaque entry index, commitment, or credential-bound status handle.
    CredentialStatusHandle,
    MAX_CREDENTIAL_STATUS_HANDLE_BYTES,
    InvalidStatusHandle
);

status_opaque_type!(
    /// Opaque method-owned registry or evidence revision.
    CredentialStatusRevision,
    MAX_CREDENTIAL_STATUS_REVISION_BYTES,
    InvalidStatusRevision
);

status_opaque_type!(
    /// Opaque method- and purpose-specific observed status value.
    CredentialStatusValue,
    MAX_CREDENTIAL_STATUS_VALUE_BYTES,
    InvalidStatusValue
);

/// One complete format-neutral credential-status binding.
#[must_use]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CredentialStatusBinding {
    method: CredentialStatusMethod,
    purpose: CredentialStatusPurpose,
    reference: CredentialStatusReference,
    handle: CredentialStatusHandle,
}

impl CredentialStatusBinding {
    /// Construct a complete binding from already validated role values.
    pub fn new(
        method: CredentialStatusMethod,
        purpose: CredentialStatusPurpose,
        reference: CredentialStatusReference,
        handle: CredentialStatusHandle,
    ) -> Self {
        Self {
            method,
            purpose,
            reference,
            handle,
        }
    }

    /// Return the status method identifier.
    pub const fn method(&self) -> &CredentialStatusMethod {
        &self.method
    }

    /// Return the status purpose identifier.
    pub const fn purpose(&self) -> &CredentialStatusPurpose {
        &self.purpose
    }

    /// Return the opaque status location.
    pub const fn reference(&self) -> &CredentialStatusReference {
        &self.reference
    }

    /// Return the opaque credential-bound handle.
    pub const fn handle(&self) -> &CredentialStatusHandle {
        &self.handle
    }
}

impl fmt::Debug for CredentialStatusBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialStatusBinding")
            .field("method", &self.method)
            .field("purpose", &self.purpose)
            .field("reference", &self.reference)
            .field("handle", &self.handle)
            .finish()
    }
}

/// Non-empty bounded collection of exact-unique status bindings.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialStatusBindings(Vec<CredentialStatusBinding>);

impl CredentialStatusBindings {
    /// Retain complete bindings after enforcing count and uniqueness bounds.
    pub fn new(bindings: Vec<CredentialStatusBinding>) -> Result<Self, CredentialError> {
        if bindings.is_empty() || bindings.len() > MAX_CREDENTIAL_STATUS_BINDINGS {
            return Err(CredentialError::InvalidStatusBindingCollection);
        }
        if has_duplicates(&bindings) {
            return Err(CredentialError::DuplicateStatusBinding);
        }
        Ok(Self(bindings))
    }

    /// Borrow the exact complete bindings.
    pub fn as_slice(&self) -> &[CredentialStatusBinding] {
        &self.0
    }

    /// Return the number of complete bindings.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether this validated collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for CredentialStatusBindings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialStatusBindings")
            .field("binding_count", &self.0.len())
            .finish_non_exhaustive()
    }
}

/// Method-neutral freshness criteria carried to a status adapter.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialStatusFreshness {
    minimum_revision: Option<CredentialStatusRevision>,
    maximum_age: Option<DurationMillis>,
}

impl CredentialStatusFreshness {
    /// Construct one or both freshness criteria without interpreting them.
    pub fn new(
        minimum_revision: Option<CredentialStatusRevision>,
        maximum_age: Option<DurationMillis>,
    ) -> Result<Self, CredentialError> {
        if minimum_revision.is_none() && maximum_age.is_none() {
            return Err(CredentialError::InvalidStatusFreshness);
        }
        Ok(Self {
            minimum_revision,
            maximum_age,
        })
    }

    /// Return the optional adapter-owned minimum revision.
    pub const fn minimum_revision(&self) -> Option<&CredentialStatusRevision> {
        self.minimum_revision.as_ref()
    }

    /// Return the optional maximum evidence age.
    pub const fn maximum_age(&self) -> Option<DurationMillis> {
        self.maximum_age
    }
}

impl fmt::Debug for CredentialStatusFreshness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialStatusFreshness")
            .field("minimum_revision_present", &self.minimum_revision.is_some())
            .field("maximum_age", &self.maximum_age)
            .finish()
    }
}

fn validate_allow_list<T: PartialEq>(
    values: &Option<Vec<T>>,
    duplicate_error: CredentialError,
) -> Result<(), CredentialError> {
    let Some(values) = values else {
        return Ok(());
    };
    if values.is_empty() || values.len() > MAX_CREDENTIAL_STATUS_REQUIREMENT_VALUES {
        return Err(CredentialError::InvalidStatusRequirements);
    }
    if has_duplicates(values) {
        return Err(duplicate_error);
    }
    Ok(())
}

/// Structural restrictions carried with a complete credential-status query.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialStatusRequirements {
    accepted_methods: Option<Vec<CredentialStatusMethod>>,
    accepted_purposes: Option<Vec<CredentialStatusPurpose>>,
    freshness: Option<CredentialStatusFreshness>,
}

impl CredentialStatusRequirements {
    /// Construct bounded allow-lists and optional freshness requirements.
    pub fn new(
        accepted_methods: Option<Vec<CredentialStatusMethod>>,
        accepted_purposes: Option<Vec<CredentialStatusPurpose>>,
        freshness: Option<CredentialStatusFreshness>,
    ) -> Result<Self, CredentialError> {
        validate_allow_list(&accepted_methods, CredentialError::DuplicateStatusMethod)?;
        validate_allow_list(&accepted_purposes, CredentialError::DuplicateStatusPurpose)?;
        Ok(Self {
            accepted_methods,
            accepted_purposes,
            freshness,
        })
    }

    /// Construct requirements with no generic method, purpose, or freshness restriction.
    pub const fn unrestricted() -> Self {
        Self {
            accepted_methods: None,
            accepted_purposes: None,
            freshness: None,
        }
    }

    /// Borrow the accepted methods, or `None` when unrestricted.
    pub fn accepted_methods(&self) -> Option<&[CredentialStatusMethod]> {
        self.accepted_methods.as_deref()
    }

    /// Borrow the accepted purposes, or `None` when unrestricted.
    pub fn accepted_purposes(&self) -> Option<&[CredentialStatusPurpose]> {
        self.accepted_purposes.as_deref()
    }

    /// Return the optional freshness criteria.
    pub const fn freshness(&self) -> Option<&CredentialStatusFreshness> {
        self.freshness.as_ref()
    }
}

impl fmt::Debug for CredentialStatusRequirements {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialStatusRequirements")
            .field(
                "accepted_method_count",
                &self.accepted_methods.as_ref().map(Vec::len),
            )
            .field(
                "accepted_purpose_count",
                &self.accepted_purposes.as_ref().map(Vec::len),
            )
            .field("freshness", &self.freshness)
            .finish()
    }
}

/// One executable-shaped query with a complete binding and structural requirements.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialStatusQuery {
    binding: CredentialStatusBinding,
    requirements: CredentialStatusRequirements,
}

impl CredentialStatusQuery {
    /// Construct a query whose binding satisfies its structural allow-lists.
    pub fn new(
        binding: CredentialStatusBinding,
        requirements: CredentialStatusRequirements,
    ) -> Result<Self, CredentialError> {
        if requirements
            .accepted_methods()
            .is_some_and(|methods| !methods.contains(binding.method()))
            || requirements
                .accepted_purposes()
                .is_some_and(|purposes| !purposes.contains(binding.purpose()))
        {
            return Err(CredentialError::StatusQueryMismatch);
        }
        Ok(Self {
            binding,
            requirements,
        })
    }

    /// Return the complete status binding to query.
    pub const fn binding(&self) -> &CredentialStatusBinding {
        &self.binding
    }

    /// Return the structural query requirements.
    pub const fn requirements(&self) -> &CredentialStatusRequirements {
        &self.requirements
    }
}

impl fmt::Debug for CredentialStatusQuery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialStatusQuery")
            .field("binding", &self.binding)
            .field("requirements", &self.requirements)
            .finish()
    }
}

/// One attributable method-specific credential-status observation.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialStatusEvidence {
    binding: CredentialStatusBinding,
    value: CredentialStatusValue,
    revision: Option<CredentialStatusRevision>,
    observed_at: Option<UnixTimestampMillis>,
    expires_at: Option<UnixTimestampMillis>,
}

impl CredentialStatusEvidence {
    /// Construct evidence while rejecting a reversed known time interval.
    pub fn new(
        binding: CredentialStatusBinding,
        value: CredentialStatusValue,
        revision: Option<CredentialStatusRevision>,
        observed_at: Option<UnixTimestampMillis>,
        expires_at: Option<UnixTimestampMillis>,
    ) -> Result<Self, CredentialError> {
        if matches!((observed_at, expires_at), (Some(observed), Some(expires)) if observed > expires)
        {
            return Err(CredentialError::InvalidStatusEvidenceRange);
        }
        Ok(Self {
            binding,
            value,
            revision,
            observed_at,
            expires_at,
        })
    }

    /// Return the complete binding described by this observation.
    pub const fn binding(&self) -> &CredentialStatusBinding {
        &self.binding
    }

    /// Return the opaque method-specific observed value.
    pub const fn value(&self) -> &CredentialStatusValue {
        &self.value
    }

    /// Return the optional method-owned revision.
    pub const fn revision(&self) -> Option<&CredentialStatusRevision> {
        self.revision.as_ref()
    }

    /// Return the optional observation timestamp.
    pub const fn observed_at(&self) -> Option<UnixTimestampMillis> {
        self.observed_at
    }

    /// Return the optional evidence-expiry timestamp.
    pub const fn expires_at(&self) -> Option<UnixTimestampMillis> {
        self.expires_at
    }
}

impl fmt::Debug for CredentialStatusEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialStatusEvidence")
            .field("binding", &self.binding)
            .field("value", &self.value)
            .field("revision_present", &self.revision.is_some())
            .field("observed_at_present", &self.observed_at.is_some())
            .field("expires_at_present", &self.expires_at.is_some())
            .finish()
    }
}

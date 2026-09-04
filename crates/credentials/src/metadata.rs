use std::fmt;

use identus_core::UnixTimestampMillis;

use crate::{
    CredentialEntityId, CredentialError, CredentialSchemaDescriptor, CredentialType,
    MAX_CREDENTIAL_TYPES, schema::contains_duplicates,
};

/// Maximum number of identified subjects in normalized credential metadata.
pub const MAX_CREDENTIAL_SUBJECTS: usize = 16;
/// Maximum number of schemas in normalized credential metadata.
pub const MAX_CREDENTIAL_SCHEMAS: usize = 16;

/// Bounded descriptive metadata projected by a credential-format adapter.
///
/// Construction proves only internal shape. It does not prove that this
/// metadata was parsed from, secured by, or verified against any credential.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct CredentialMetadata {
    issuer: CredentialEntityId,
    subjects: Vec<CredentialEntityId>,
    credential_types: Vec<CredentialType>,
    schemas: Vec<CredentialSchemaDescriptor>,
    valid_from: Option<UnixTimestampMillis>,
    valid_until: Option<UnixTimestampMillis>,
}

impl CredentialMetadata {
    /// Construct metadata while enforcing collection and validity invariants.
    pub fn new(
        issuer: CredentialEntityId,
        subjects: Vec<CredentialEntityId>,
        credential_types: Vec<CredentialType>,
        schemas: Vec<CredentialSchemaDescriptor>,
        valid_from: Option<UnixTimestampMillis>,
        valid_until: Option<UnixTimestampMillis>,
    ) -> Result<Self, CredentialError> {
        if subjects.len() > MAX_CREDENTIAL_SUBJECTS
            || credential_types.is_empty()
            || credential_types.len() > MAX_CREDENTIAL_TYPES
            || schemas.len() > MAX_CREDENTIAL_SCHEMAS
        {
            return Err(CredentialError::InvalidDescriptorCollection);
        }
        if contains_duplicates(&subjects) {
            return Err(CredentialError::DuplicateCredentialSubject);
        }
        if contains_duplicates(&credential_types) {
            return Err(CredentialError::DuplicateCredentialType);
        }
        if schemas.iter().enumerate().any(|(index, schema)| {
            schemas[..index]
                .iter()
                .any(|previous| previous.id() == schema.id())
        }) {
            return Err(CredentialError::DuplicateSchemaIdentifier);
        }
        if matches!((valid_from, valid_until), (Some(from), Some(until)) if from > until) {
            return Err(CredentialError::InvalidValidityRange);
        }

        Ok(Self {
            issuer,
            subjects,
            credential_types,
            schemas,
            valid_from,
            valid_until,
        })
    }

    /// Return the issuer identifier through an explicit accessor.
    pub const fn issuer(&self) -> &CredentialEntityId {
        &self.issuer
    }

    /// Borrow zero or more unique subject identifiers.
    pub fn subjects(&self) -> &[CredentialEntityId] {
        &self.subjects
    }

    /// Borrow the unique credential types.
    pub fn credential_types(&self) -> &[CredentialType] {
        &self.credential_types
    }

    /// Borrow the unique schema descriptors.
    pub fn schemas(&self) -> &[CredentialSchemaDescriptor] {
        &self.schemas
    }

    /// Return the optional normalized validity start.
    pub const fn valid_from(&self) -> Option<UnixTimestampMillis> {
        self.valid_from
    }

    /// Return the optional normalized validity end.
    pub const fn valid_until(&self) -> Option<UnixTimestampMillis> {
        self.valid_until
    }
}

impl fmt::Debug for CredentialMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialMetadata")
            .field("issuer_present", &true)
            .field("subject_count", &self.subjects.len())
            .field("credential_type_count", &self.credential_types.len())
            .field("schema_count", &self.schemas.len())
            .field("valid_from_present", &self.valid_from.is_some())
            .field("valid_until_present", &self.valid_until.is_some())
            .finish_non_exhaustive()
    }
}

use std::{fmt, str::FromStr};

use crate::{
    CredentialClaimId, CredentialClaimPathSegment, CredentialClaimValueType, CredentialError,
    CredentialSchemaId, CredentialSchemaVersion, CredentialType,
};

/// Maximum number of segments in one credential claim path.
pub const MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS: usize = 16;
/// Maximum number of claim descriptors in one schema.
pub const MAX_CREDENTIAL_SCHEMA_CLAIMS: usize = 64;
/// Maximum number of credential types in metadata or one schema.
pub const MAX_CREDENTIAL_TYPES: usize = 16;

fn has_duplicates<T: PartialEq>(values: &[T]) -> bool {
    values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].contains(value))
}

/// How a claim can participate in disclosure or proof generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CredentialClaimDisclosure {
    /// The claim is carried in clear form.
    Public,
    /// The holder can reveal the claim selectively.
    Selective,
    /// The credential carries a commitment to holder-retained material.
    Committed,
    /// Only predicates over the claim are intended to be disclosed.
    PredicateOnly,
}

impl CredentialClaimDisclosure {
    /// Return the stable machine spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Selective => "selective",
            Self::Committed => "committed",
            Self::PredicateOnly => "predicate_only",
        }
    }

    /// Parse a stable machine spelling without allocation.
    pub fn parse(value: &str) -> Result<Self, CredentialError> {
        match value {
            "public" => Ok(Self::Public),
            "selective" => Ok(Self::Selective),
            "committed" => Ok(Self::Committed),
            "predicate_only" => Ok(Self::PredicateOnly),
            _ => Err(CredentialError::InvalidClaimDisclosure),
        }
    }
}

impl FromStr for CredentialClaimDisclosure {
    type Err = CredentialError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for CredentialClaimDisclosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Non-empty bounded sequence locating a claim in a format-owned structure.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CredentialClaimPath(Vec<CredentialClaimPathSegment>);

impl CredentialClaimPath {
    /// Construct a path from validated segments.
    pub fn new(segments: Vec<CredentialClaimPathSegment>) -> Result<Self, CredentialError> {
        if segments.is_empty() || segments.len() > MAX_CREDENTIAL_CLAIM_PATH_SEGMENTS {
            return Err(CredentialError::InvalidClaimPath);
        }
        Ok(Self(segments))
    }

    /// Borrow the exact validated segments.
    pub fn segments(&self) -> &[CredentialClaimPathSegment] {
        &self.0
    }
}

/// Value-free description of one claim in a credential schema.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialClaimDescriptor {
    id: CredentialClaimId,
    path: CredentialClaimPath,
    disclosure: CredentialClaimDisclosure,
    required: bool,
    value_type: Option<CredentialClaimValueType>,
}

impl CredentialClaimDescriptor {
    /// Construct a claim descriptor from already validated values.
    pub fn new(
        id: CredentialClaimId,
        path: CredentialClaimPath,
        disclosure: CredentialClaimDisclosure,
        required: bool,
        value_type: Option<CredentialClaimValueType>,
    ) -> Self {
        Self {
            id,
            path,
            disclosure,
            required,
            value_type,
        }
    }

    /// Return the claim identifier.
    pub const fn id(&self) -> &CredentialClaimId {
        &self.id
    }

    /// Return the segmented claim path.
    pub const fn path(&self) -> &CredentialClaimPath {
        &self.path
    }

    /// Return the disclosure capability.
    pub const fn disclosure(&self) -> CredentialClaimDisclosure {
        self.disclosure
    }

    /// Whether a conforming credential is expected to contain this claim.
    pub const fn required(&self) -> bool {
        self.required
    }

    /// Return the optional format-owned value-type hint.
    pub const fn value_type(&self) -> Option<&CredentialClaimValueType> {
        self.value_type.as_ref()
    }
}

/// Bounded format-neutral credential schema and claim-shape descriptor.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialSchemaDescriptor {
    id: CredentialSchemaId,
    version: Option<CredentialSchemaVersion>,
    credential_types: Vec<CredentialType>,
    claims: Vec<CredentialClaimDescriptor>,
}

impl CredentialSchemaDescriptor {
    /// Construct a schema while enforcing bounds and uniqueness.
    pub fn new(
        id: CredentialSchemaId,
        version: Option<CredentialSchemaVersion>,
        credential_types: Vec<CredentialType>,
        claims: Vec<CredentialClaimDescriptor>,
    ) -> Result<Self, CredentialError> {
        if credential_types.is_empty()
            || credential_types.len() > MAX_CREDENTIAL_TYPES
            || claims.len() > MAX_CREDENTIAL_SCHEMA_CLAIMS
        {
            return Err(CredentialError::InvalidDescriptorCollection);
        }
        if has_duplicates(&credential_types) {
            return Err(CredentialError::DuplicateCredentialType);
        }
        if claims.iter().enumerate().any(|(index, claim)| {
            claims[..index]
                .iter()
                .any(|previous| previous.id() == claim.id())
        }) {
            return Err(CredentialError::DuplicateClaimIdentifier);
        }
        if claims.iter().enumerate().any(|(index, claim)| {
            claims[..index]
                .iter()
                .any(|previous| previous.path() == claim.path())
        }) {
            return Err(CredentialError::DuplicateClaimPath);
        }

        Ok(Self {
            id,
            version,
            credential_types,
            claims,
        })
    }

    /// Return the schema identifier.
    pub const fn id(&self) -> &CredentialSchemaId {
        &self.id
    }

    /// Return the optional schema version.
    pub const fn version(&self) -> Option<&CredentialSchemaVersion> {
        self.version.as_ref()
    }

    /// Borrow the unique credential types.
    pub fn credential_types(&self) -> &[CredentialType] {
        &self.credential_types
    }

    /// Borrow the unique claim descriptors.
    pub fn claims(&self) -> &[CredentialClaimDescriptor] {
        &self.claims
    }
}

pub(crate) fn contains_duplicates<T: PartialEq>(values: &[T]) -> bool {
    has_duplicates(values)
}

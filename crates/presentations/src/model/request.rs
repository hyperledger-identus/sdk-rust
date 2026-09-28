use std::fmt;

use identus_credentials::{
    CredentialClaimPath, CredentialEntityId, CredentialFormat, CredentialSchemaId, CredentialType,
};

use crate::PresentationError;

use super::{
    MAX_PRESENTATION_FILTER_VALUES, MAX_PRESENTATION_QUERY_CLAIMS,
    MAX_PRESENTATION_REQUEST_QUERIES, PresentationChallenge, PresentationClaimIntent,
    PresentationPurpose, PresentationQueryId, has_duplicates,
};

/// Value-free request for one credential claim path.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationClaimRequest {
    path: CredentialClaimPath,
    intent: PresentationClaimIntent,
    required: bool,
}

impl PresentationClaimRequest {
    /// Construct a claim request from an already validated credential path.
    pub const fn new(
        path: CredentialClaimPath,
        intent: PresentationClaimIntent,
        required: bool,
    ) -> Self {
        Self {
            path,
            intent,
            required,
        }
    }

    /// Return the requested credential claim path.
    pub const fn path(&self) -> &CredentialClaimPath {
        &self.path
    }

    /// Return the generic disclosure intent.
    pub const fn intent(&self) -> PresentationClaimIntent {
        self.intent
    }

    /// Whether a candidate must cover this path.
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for PresentationClaimRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationClaimRequest")
            .field("path_segments", &self.path.segments().len())
            .field("intent", &self.intent)
            .field("required", &self.required)
            .finish_non_exhaustive()
    }
}

/// Optional bounded descriptor filters for one credential query.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCredentialFilters {
    accepted_issuers: Option<Vec<CredentialEntityId>>,
    accepted_types: Option<Vec<CredentialType>>,
    accepted_schemas: Option<Vec<CredentialSchemaId>>,
}

impl PresentationCredentialFilters {
    /// Construct filters; absent means unrestricted and present means 1–16
    /// unique values.
    pub fn new(
        accepted_issuers: Option<Vec<CredentialEntityId>>,
        accepted_types: Option<Vec<CredentialType>>,
        accepted_schemas: Option<Vec<CredentialSchemaId>>,
    ) -> Result<Self, PresentationError> {
        validate_optional_filter(accepted_issuers.as_deref())?;
        validate_optional_filter(accepted_types.as_deref())?;
        validate_optional_filter(accepted_schemas.as_deref())?;

        if accepted_issuers.as_deref().is_some_and(has_duplicates) {
            return Err(PresentationError::DuplicateIssuerFilter);
        }
        if accepted_types.as_deref().is_some_and(has_duplicates) {
            return Err(PresentationError::DuplicateTypeFilter);
        }
        if accepted_schemas.as_deref().is_some_and(has_duplicates) {
            return Err(PresentationError::DuplicateSchemaFilter);
        }

        Ok(Self {
            accepted_issuers,
            accepted_types,
            accepted_schemas,
        })
    }

    /// Construct an unrestricted filter set.
    pub const fn unrestricted() -> Self {
        Self {
            accepted_issuers: None,
            accepted_types: None,
            accepted_schemas: None,
        }
    }

    /// Borrow the accepted issuers; absence means unrestricted.
    pub fn accepted_issuers(&self) -> Option<&[CredentialEntityId]> {
        self.accepted_issuers.as_deref()
    }

    /// Borrow the accepted credential types; absence means unrestricted.
    pub fn accepted_types(&self) -> Option<&[CredentialType]> {
        self.accepted_types.as_deref()
    }

    /// Borrow the accepted schema identifiers; absence means unrestricted.
    pub fn accepted_schemas(&self) -> Option<&[CredentialSchemaId]> {
        self.accepted_schemas.as_deref()
    }
}

impl fmt::Debug for PresentationCredentialFilters {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCredentialFilters")
            .field(
                "issuer_count",
                &self.accepted_issuers.as_ref().map(Vec::len),
            )
            .field("type_count", &self.accepted_types.as_ref().map(Vec::len))
            .field(
                "schema_count",
                &self.accepted_schemas.as_ref().map(Vec::len),
            )
            .finish_non_exhaustive()
    }
}

fn validate_optional_filter<T>(values: Option<&[T]>) -> Result<(), PresentationError> {
    if values
        .is_some_and(|values| values.is_empty() || values.len() > MAX_PRESENTATION_FILTER_VALUES)
    {
        return Err(PresentationError::InvalidQueryFilters);
    }
    Ok(())
}

/// Bounded format-aware request for matching credentials.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCredentialQuery {
    id: PresentationQueryId,
    format: CredentialFormat,
    multiple: bool,
    requires_holder_binding: bool,
    filters: PresentationCredentialFilters,
    claims: Vec<PresentationClaimRequest>,
}

impl PresentationCredentialQuery {
    /// Construct a query while enforcing claim bounds and path uniqueness.
    pub fn new(
        id: PresentationQueryId,
        format: CredentialFormat,
        multiple: bool,
        requires_holder_binding: bool,
        filters: PresentationCredentialFilters,
        claims: Vec<PresentationClaimRequest>,
    ) -> Result<Self, PresentationError> {
        if claims.len() > MAX_PRESENTATION_QUERY_CLAIMS {
            return Err(PresentationError::InvalidQueryClaims);
        }
        if claims.iter().enumerate().any(|(index, claim)| {
            claims[..index]
                .iter()
                .any(|previous| previous.path() == claim.path())
        }) {
            return Err(PresentationError::DuplicateQueryClaim);
        }
        Ok(Self {
            id,
            format,
            multiple,
            requires_holder_binding,
            filters,
            claims,
        })
    }

    /// Return the query identifier.
    pub const fn id(&self) -> &PresentationQueryId {
        &self.id
    }

    /// Return the requested credential format.
    pub const fn format(&self) -> &CredentialFormat {
        &self.format
    }

    /// Whether the query accepts more than one matching credential.
    pub const fn multiple(&self) -> bool {
        self.multiple
    }

    /// Whether the requested presentation requires cryptographic holder binding.
    pub const fn requires_holder_binding(&self) -> bool {
        self.requires_holder_binding
    }

    /// Return the descriptor filters.
    pub const fn filters(&self) -> &PresentationCredentialFilters {
        &self.filters
    }

    /// Borrow the unique requested claim paths and intents.
    pub fn claims(&self) -> &[PresentationClaimRequest] {
        &self.claims
    }
}

impl fmt::Debug for PresentationCredentialQuery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCredentialQuery")
            .field("id", &self.id)
            .field("format", &self.format)
            .field("multiple", &self.multiple)
            .field("requires_holder_binding", &self.requires_holder_binding)
            .field("filters", &self.filters)
            .field("claim_count", &self.claims.len())
            .finish_non_exhaustive()
    }
}

/// Bounded semantic presentation request, independent of protocol wire data.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationRequest {
    verifier: CredentialEntityId,
    purpose: Option<PresentationPurpose>,
    challenge: Option<PresentationChallenge>,
    queries: Vec<PresentationCredentialQuery>,
}

impl PresentationRequest {
    /// Construct a request with 1–16 uniquely identified credential queries.
    pub fn new(
        verifier: CredentialEntityId,
        purpose: Option<PresentationPurpose>,
        challenge: Option<PresentationChallenge>,
        queries: Vec<PresentationCredentialQuery>,
    ) -> Result<Self, PresentationError> {
        if queries.is_empty() || queries.len() > MAX_PRESENTATION_REQUEST_QUERIES {
            return Err(PresentationError::InvalidRequestQueries);
        }
        if queries.iter().enumerate().any(|(index, query)| {
            queries[..index]
                .iter()
                .any(|previous| previous.id() == query.id())
        }) {
            return Err(PresentationError::DuplicateQueryId);
        }
        Ok(Self {
            verifier,
            purpose,
            challenge,
            queries,
        })
    }

    /// Return the verifier identifier.
    pub const fn verifier(&self) -> &CredentialEntityId {
        &self.verifier
    }

    /// Return the optional presentation purpose.
    pub const fn purpose(&self) -> Option<&PresentationPurpose> {
        self.purpose.as_ref()
    }

    /// Return the optional opaque replay challenge.
    pub const fn challenge(&self) -> Option<&PresentationChallenge> {
        self.challenge.as_ref()
    }

    /// Borrow the ordered unique credential queries.
    pub fn queries(&self) -> &[PresentationCredentialQuery] {
        &self.queries
    }

    pub(super) fn query(&self, id: &PresentationQueryId) -> Option<&PresentationCredentialQuery> {
        self.queries.iter().find(|query| query.id() == id)
    }
}

impl fmt::Debug for PresentationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationRequest")
            .field("has_purpose", &self.purpose.is_some())
            .field("has_challenge", &self.challenge.is_some())
            .field("query_count", &self.queries.len())
            .finish_non_exhaustive()
    }
}

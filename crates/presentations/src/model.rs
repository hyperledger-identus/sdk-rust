use std::{fmt, str::FromStr};

use identus_credentials::{
    CredentialClaimPath, CredentialEntityId, CredentialFormat, CredentialSchemaId, CredentialType,
};

use crate::PresentationError;

/// Maximum encoded length of a presentation query identifier.
pub const MAX_PRESENTATION_QUERY_ID_BYTES: usize = 128;
/// Maximum encoded length of a presentation purpose.
pub const MAX_PRESENTATION_PURPOSE_BYTES: usize = 2_048;
/// Maximum encoded length of a presentation replay challenge.
pub const MAX_PRESENTATION_CHALLENGE_BYTES: usize = 1_024;
/// Maximum encoded length of a local credential handle.
pub const MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES: usize = 1_024;
/// Maximum number of values in one present query filter.
pub const MAX_PRESENTATION_FILTER_VALUES: usize = 16;
/// Maximum number of claim requests in one credential query.
pub const MAX_PRESENTATION_QUERY_CLAIMS: usize = 64;
/// Maximum number of credential queries in one presentation request.
pub const MAX_PRESENTATION_REQUEST_QUERIES: usize = 16;
/// Maximum number of satisfiable claims recorded by one candidate.
pub const MAX_PRESENTATION_CANDIDATE_CLAIMS: usize = 64;
/// Maximum number of credential candidates in one validated set.
pub const MAX_PRESENTATION_CANDIDATES: usize = 64;

fn valid_text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

fn has_duplicates<T: PartialEq>(values: &[T]) -> bool {
    values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].contains(value))
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

    fn query(&self, id: &PresentationQueryId) -> Option<&PresentationCredentialQuery> {
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

/// One local credential reported as structurally able to satisfy a query.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCredentialCandidate {
    query_id: PresentationQueryId,
    credential_handle: PresentationCredentialHandle,
    format: CredentialFormat,
    satisfiable_claims: Vec<CredentialClaimPath>,
}

impl PresentationCredentialCandidate {
    /// Construct a candidate with a bounded unique set of satisfiable paths.
    pub fn new(
        query_id: PresentationQueryId,
        credential_handle: PresentationCredentialHandle,
        format: CredentialFormat,
        satisfiable_claims: Vec<CredentialClaimPath>,
    ) -> Result<Self, PresentationError> {
        if satisfiable_claims.len() > MAX_PRESENTATION_CANDIDATE_CLAIMS {
            return Err(PresentationError::InvalidCandidateClaims);
        }
        if has_duplicates(&satisfiable_claims) {
            return Err(PresentationError::DuplicateCandidateClaim);
        }
        Ok(Self {
            query_id,
            credential_handle,
            format,
            satisfiable_claims,
        })
    }

    /// Return the query this candidate addresses.
    pub const fn query_id(&self) -> &PresentationQueryId {
        &self.query_id
    }

    /// Return the opaque local credential handle.
    pub const fn credential_handle(&self) -> &PresentationCredentialHandle {
        &self.credential_handle
    }

    /// Return the credential format reported by the candidate source.
    pub const fn format(&self) -> &CredentialFormat {
        &self.format
    }

    /// Borrow the unique requested paths this candidate can satisfy.
    pub fn satisfiable_claims(&self) -> &[CredentialClaimPath] {
        &self.satisfiable_claims
    }
}

impl fmt::Debug for PresentationCredentialCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCredentialCandidate")
            .field("query_id", &self.query_id)
            .field("credential_handle", &self.credential_handle)
            .field("format", &self.format)
            .field("claim_count", &self.satisfiable_claims.len())
            .finish_non_exhaustive()
    }
}

/// Bounded candidates proven structurally consistent with one request.
#[must_use]
#[derive(Clone, PartialEq, Eq)]
pub struct PresentationCandidateSet(Vec<PresentationCredentialCandidate>);

impl PresentationCandidateSet {
    /// Validate and retain a candidate vector against its presentation request.
    pub fn new(
        request: &PresentationRequest,
        candidates: Vec<PresentationCredentialCandidate>,
    ) -> Result<Self, PresentationError> {
        if candidates.len() > MAX_PRESENTATION_CANDIDATES {
            return Err(PresentationError::InvalidCandidates);
        }
        if candidates.iter().enumerate().any(|(index, candidate)| {
            candidates[..index].iter().any(|previous| {
                previous.query_id() == candidate.query_id()
                    && previous.credential_handle() == candidate.credential_handle()
            })
        }) {
            return Err(PresentationError::DuplicateCandidate);
        }

        for candidate in &candidates {
            let query = request
                .query(candidate.query_id())
                .ok_or(PresentationError::UnknownCandidateQuery)?;
            if candidate.format() != query.format() {
                return Err(PresentationError::CandidateFormatMismatch);
            }
            if candidate.satisfiable_claims().iter().any(|candidate_path| {
                !query
                    .claims()
                    .iter()
                    .any(|claim| claim.path() == candidate_path)
            }) {
                return Err(PresentationError::CandidateUnrequestedClaim);
            }
            if query.claims().iter().any(|claim| {
                claim.required()
                    && !candidate
                        .satisfiable_claims()
                        .iter()
                        .any(|candidate_path| candidate_path == claim.path())
            }) {
                return Err(PresentationError::CandidateMissingRequiredClaim);
            }
        }
        Ok(Self(candidates))
    }

    /// Borrow the ordered validated candidates.
    pub fn as_slice(&self) -> &[PresentationCredentialCandidate] {
        &self.0
    }

    /// Consume the set and return its candidate vector.
    pub fn into_vec(self) -> Vec<PresentationCredentialCandidate> {
        self.0
    }
}

impl fmt::Debug for PresentationCandidateSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PresentationCandidateSet")
            .field("candidate_count", &self.0.len())
            .finish_non_exhaustive()
    }
}

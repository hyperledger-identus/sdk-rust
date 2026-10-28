use std::fmt;

use identus_credentials::CredentialClaimPath;
use identus_presentations::{PresentationClaimIntent, PresentationDisclosurePlan};

use crate::{SdJwtError, SdJwtLimits};

/// One validated object member or array position.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathSegment {
    Member(String),
    Index(usize),
}

/// A bounded claim location in issuer JSON.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClaimPath(Vec<PathSegment>);

impl ClaimPath {
    pub fn new(segments: Vec<PathSegment>, limits: SdJwtLimits) -> Result<Self, SdJwtError> {
        limits.validate()?;
        if segments.is_empty()
            || segments.len() > limits.json_depth
            || segments.iter().any(|part| match part {
                PathSegment::Member(name) => {
                    name.is_empty()
                        || name.len() > 1_024
                        || matches!(name.as_str(), "_sd" | "_sd_alg" | "...")
                }
                PathSegment::Index(_) => false,
            })
        {
            return Err(SdJwtError::InvalidSelection);
        }
        Ok(Self(segments))
    }
    pub fn segments(&self) -> &[PathSegment] {
        &self.0
    }
}

impl fmt::Debug for ClaimPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClaimPath")
            .field("segments", &self.0.len())
            .finish()
    }
}

/// One object container where an issuer inserts a decoy digest.
pub enum DecoyLocation {
    Root,
    Path(ClaimPath),
}

impl DecoyLocation {
    pub fn root() -> Self {
        Self::Root
    }

    pub fn at(path: ClaimPath) -> Self {
        Self::Path(path)
    }
}

impl fmt::Debug for DecoyLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecoyLocation")
            .field("root", &matches!(self, Self::Root))
            .finish()
    }
}

/// Issuer-owned choices. Decoy locations identify object containers.
pub struct IssuancePlan {
    pub disclose: Vec<ClaimPath>,
    pub decoy_locations: Vec<DecoyLocation>,
    pub salt_bytes: usize,
}

/// Profile-neutral protected-header choices for issuance.
///
/// The RFC engine validates the value as a bounded JOSE protected `typ` but
/// assigns it no media-type or credential-profile meaning.
#[derive(Clone, Copy, Debug, Default)]
pub struct IssuanceOptions<'a> {
    pub protected_type: Option<&'a str>,
}

impl fmt::Debug for IssuancePlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssuancePlan")
            .field("disclosures", &self.disclose.len())
            .field("decoys", &self.decoy_locations.len())
            .finish()
    }
}

/// A selected set of claim locations.
pub struct SelectionPlan {
    pub paths: Vec<ClaimPath>,
}
impl fmt::Debug for SelectionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SelectionPlan")
            .field("paths", &self.paths.len())
            .finish()
    }
}

/// Map one generic, value-free path as object members.
///
/// The generic credential path vocabulary does not distinguish object members
/// from array positions. Typed [`ClaimPath`] values remain available when an
/// SD-JWT adapter needs an array index.
pub fn map_claim_path(
    path: &CredentialClaimPath,
    limits: SdJwtLimits,
) -> Result<ClaimPath, SdJwtError> {
    let parts = path
        .segments()
        .iter()
        .map(|segment| PathSegment::Member(segment.as_str().to_owned()))
        .collect();
    ClaimPath::new(parts, limits)
}

/// Map reveal intents for exactly one selected generic credential.
pub fn map_disclosure_plan(
    plan: &PresentationDisclosurePlan,
    selection_index: usize,
    limits: SdJwtLimits,
) -> Result<SelectionPlan, SdJwtError> {
    limits.validate()?;
    let selection = plan
        .as_slice()
        .get(selection_index)
        .ok_or(SdJwtError::InvalidSelection)?;
    if selection.selected_claims().len() > limits.selected_paths {
        return Err(SdJwtError::ResourceLimit);
    }
    let mut paths = Vec::new();
    for claim in selection.selected_claims() {
        if claim.intent() != PresentationClaimIntent::Reveal {
            return Err(SdJwtError::InvalidSelection);
        }
        paths.push(map_claim_path(claim.path(), limits)?);
    }
    Ok(SelectionPlan { paths })
}

/// Static evidence only; never contains compact material or comparison values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerificationReceipt {
    pub issuer_signature: bool,
    pub disclosures_processed: bool,
    pub holder_binding: bool,
    pub disclosure_count: usize,
    pub algorithm: identus_jose::JwsAlgorithm,
}

use std::{collections::BTreeSet, fmt};

use identus_sd_jwt::{ClaimPath, DisclosureVerifiedSdJwt, PathSegment, SdJwtLimits};
use serde_json::Value;

use crate::{SdJwtVcError, SdJwtVcLimits, profile::validate_type};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimDisclosureRule {
    Always,
    Allowed,
    Never,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetadataClaimPathSegment {
    Member(String),
    Index(usize),
    AnyIndex,
}

impl fmt::Debug for MetadataClaimPathSegment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Member(value) => formatter.debug_tuple("Member").field(&value.len()).finish(),
            Self::Index(_) => formatter.write_str("Index"),
            Self::AnyIndex => formatter.write_str("AnyIndex"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataClaimPath(Vec<MetadataClaimPathSegment>);

impl MetadataClaimPath {
    pub fn new(segments: Vec<String>, limits: SdJwtVcLimits) -> Result<Self, SdJwtVcError> {
        Self::from_segments(
            segments
                .into_iter()
                .map(MetadataClaimPathSegment::Member)
                .collect(),
            limits,
        )
    }

    pub fn from_segments(
        segments: Vec<MetadataClaimPathSegment>,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        if segments.is_empty()
            || segments.len() > limits.claim_path_segments
            || segments.iter().any(|segment| {
                matches!(segment, MetadataClaimPathSegment::Member(value) if value.is_empty()
                    || value.len() > limits.credential_type_bytes
                    || value.chars().any(char::is_control))
            })
        {
            return Err(SdJwtVcError::TypeMetadata);
        }
        Ok(Self(segments))
    }

    pub fn segments(&self) -> &[MetadataClaimPathSegment] {
        &self.0
    }
}

impl fmt::Debug for MetadataClaimPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MetadataClaimPath")
            .field("segments", &self.0.len())
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimMetadata {
    path: MetadataClaimPath,
    mandatory: bool,
    disclosure: ClaimDisclosureRule,
}

impl ClaimMetadata {
    pub fn new(
        path: Vec<String>,
        mandatory: bool,
        disclosure: ClaimDisclosureRule,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        Ok(Self {
            path: MetadataClaimPath::new(path, limits)?,
            mandatory,
            disclosure,
        })
    }

    pub fn from_path(
        path: Vec<MetadataClaimPathSegment>,
        mandatory: bool,
        disclosure: ClaimDisclosureRule,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        Ok(Self {
            path: MetadataClaimPath::from_segments(path, limits)?,
            mandatory,
            disclosure,
        })
    }
}

#[derive(Clone)]
pub struct TypeMetadata {
    vct: String,
    extends: Option<String>,
    claims: Vec<ClaimMetadata>,
    encoded_bytes: usize,
}

impl TypeMetadata {
    pub fn new(
        vct: &str,
        extends: Option<&str>,
        claims: Vec<ClaimMetadata>,
        encoded_bytes: usize,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        validate_type(vct, limits)?;
        if let Some(parent) = extends {
            validate_type(parent, limits)?;
            if parent == vct {
                return Err(SdJwtVcError::TypeMetadataCycle);
            }
        }
        if encoded_bytes == 0
            || encoded_bytes > limits.type_metadata_bytes
            || claims.len() > limits.claim_metadata_entries
        {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let retained_bytes = retained_bytes(vct, extends, &claims)?;
        if retained_bytes > limits.type_metadata_bytes {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let mut paths = BTreeSet::new();
        if claims.iter().any(|claim| !paths.insert(claim.path.clone())) {
            return Err(SdJwtVcError::TypeMetadata);
        }
        Ok(Self {
            vct: vct.to_owned(),
            extends: extends.map(str::to_owned),
            claims,
            encoded_bytes: encoded_bytes.max(retained_bytes),
        })
    }
}

impl fmt::Debug for TypeMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypeMetadata")
            .field("vct_bytes", &self.vct.len())
            .field("extends", &self.extends.is_some())
            .field("claims", &self.claims.len())
            .field("encoded_bytes", &self.encoded_bytes)
            .finish()
    }
}

/// A leaf-first, finite and acyclic Type Metadata inheritance chain.
#[derive(Clone, Debug)]
pub struct TypeMetadataChain(Vec<TypeMetadata>);

impl TypeMetadataChain {
    pub fn new(values: Vec<TypeMetadata>, limits: SdJwtVcLimits) -> Result<Self, SdJwtVcError> {
        if values.is_empty() || values.len() > limits.type_extension_depth {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let total_bytes = values.iter().try_fold(0_usize, |total, value| {
            total.checked_add(value.encoded_bytes)
        });
        if total_bytes.is_none_or(|total| total > limits.type_metadata_bytes) {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let mut seen = BTreeSet::new();
        for (index, value) in values.iter().enumerate() {
            if !seen.insert(value.vct.as_str()) {
                return Err(SdJwtVcError::TypeMetadataCycle);
            }
            if value
                .extends
                .as_deref()
                .is_some_and(|parent| seen.contains(parent))
            {
                return Err(SdJwtVcError::TypeMetadataCycle);
            }
            let next = values.get(index + 1);
            match (value.extends.as_deref(), next) {
                (Some(parent), Some(next)) if parent == next.vct => {}
                (None, None) => {}
                _ => return Err(SdJwtVcError::TypeMetadata),
            }
        }
        Ok(Self(values))
    }

    pub(crate) fn validate_for(
        &self,
        vct: &str,
        base: &DisclosureVerifiedSdJwt,
        limits: SdJwtVcLimits,
    ) -> Result<(), SdJwtVcError> {
        self.validate_active_limits(limits)?;
        if self.0.first().map(|value| value.vct.as_str()) != Some(vct) {
            return Err(SdJwtVcError::EvidenceMismatch);
        }
        let required_work = self.0.iter().try_fold(0_usize, |total, metadata| {
            metadata
                .claims
                .iter()
                .try_fold(total.checked_add(1)?, |work, claim| {
                    work.checked_add(2 + claim.path.segments().len() * 2)
                })
        });
        if required_work.is_none_or(|work| work > limits.work_units) {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let mut work_left = limits
            .work_units
            .checked_sub(required_work.unwrap_or_default())
            .ok_or(SdJwtVcError::ResourceLimit)?;
        for metadata in self.0.iter().rev() {
            for claim in &metadata.claims {
                let observation = observe_path(base, claim.path.segments(), &mut work_left)?;
                let present = observation.present;
                if claim.mandatory && !present && claim.disclosure == ClaimDisclosureRule::Never {
                    return Err(SdJwtVcError::TypeMetadataRule);
                }
                match claim.disclosure {
                    ClaimDisclosureRule::Always
                        if present && !observation.all_present_disclosed =>
                    {
                        return Err(SdJwtVcError::TypeMetadataRule);
                    }
                    ClaimDisclosureRule::Never if observation.any_disclosed => {
                        return Err(SdJwtVcError::TypeMetadataRule);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn validate_active_limits(&self, limits: SdJwtVcLimits) -> Result<(), SdJwtVcError> {
        if self.0.is_empty() || self.0.len() > limits.type_extension_depth {
            return Err(SdJwtVcError::ResourceLimit);
        }
        let mut total_bytes = 0_usize;
        for metadata in &self.0 {
            validate_type(&metadata.vct, limits)?;
            if let Some(parent) = metadata.extends.as_deref() {
                validate_type(parent, limits)?;
            }
            if metadata.claims.len() > limits.claim_metadata_entries {
                return Err(SdJwtVcError::ResourceLimit);
            }
            for claim in &metadata.claims {
                if claim.path.segments().is_empty()
                    || claim.path.segments().len() > limits.claim_path_segments
                    || claim.path.segments().iter().any(|segment| {
                        matches!(segment, MetadataClaimPathSegment::Member(value) if value.is_empty()
                            || value.len() > limits.credential_type_bytes
                            || value.chars().any(char::is_control))
                    })
                {
                    return Err(SdJwtVcError::ResourceLimit);
                }
            }
            total_bytes = total_bytes
                .checked_add(metadata.encoded_bytes)
                .ok_or(SdJwtVcError::ResourceLimit)?;
        }
        if total_bytes > limits.type_metadata_bytes {
            return Err(SdJwtVcError::ResourceLimit);
        }
        Ok(())
    }
}

fn retained_bytes(
    vct: &str,
    extends: Option<&str>,
    claims: &[ClaimMetadata],
) -> Result<usize, SdJwtVcError> {
    claims.iter().try_fold(
        vct.len()
            .checked_add(extends.map_or(0, str::len))
            .ok_or(SdJwtVcError::ResourceLimit)?,
        |total, claim| {
            claim.path.segments().iter().try_fold(
                total.checked_add(2).ok_or(SdJwtVcError::ResourceLimit)?,
                |bytes, segment| {
                    let segment_bytes = match segment {
                        MetadataClaimPathSegment::Member(value) => value.len(),
                        MetadataClaimPathSegment::Index(value) => value.to_string().len(),
                        MetadataClaimPathSegment::AnyIndex => 1,
                    };
                    bytes
                        .checked_add(segment_bytes)
                        .ok_or(SdJwtVcError::ResourceLimit)
                },
            )
        },
    )
}

#[derive(Default)]
struct PathObservation {
    present: bool,
    any_disclosed: bool,
    all_present_disclosed: bool,
}

fn observe_path(
    base: &DisclosureVerifiedSdJwt,
    path: &[MetadataClaimPathSegment],
    work_left: &mut usize,
) -> Result<PathObservation, SdJwtVcError> {
    let mut concrete = Vec::with_capacity(path.len());
    let mut disclosed = Vec::new();
    collect_path_observations(
        base,
        base.claims(),
        path,
        &mut concrete,
        &mut disclosed,
        work_left,
    )?;
    Ok(PathObservation {
        present: !disclosed.is_empty(),
        any_disclosed: disclosed.iter().any(|value| *value),
        all_present_disclosed: !disclosed.is_empty() && disclosed.iter().all(|value| *value),
    })
}

fn collect_path_observations(
    base: &DisclosureVerifiedSdJwt,
    current: &Value,
    path: &[MetadataClaimPathSegment],
    concrete: &mut Vec<PathSegment>,
    disclosed: &mut Vec<bool>,
    work_left: &mut usize,
) -> Result<(), SdJwtVcError> {
    *work_left = work_left
        .checked_sub(1)
        .ok_or(SdJwtVcError::ResourceLimit)?;
    let Some((segment, remaining)) = path.split_first() else {
        let selected = ClaimPath::new(concrete.clone(), SdJwtLimits::default())
            .is_ok_and(|path| base.was_disclosed(&path));
        disclosed.push(selected);
        return Ok(());
    };
    match segment {
        MetadataClaimPathSegment::Member(name) => {
            if let Some(next) = current.as_object().and_then(|object| object.get(name)) {
                concrete.push(PathSegment::Member(name.clone()));
                collect_path_observations(base, next, remaining, concrete, disclosed, work_left)?;
                concrete.pop();
            }
        }
        MetadataClaimPathSegment::Index(index) => {
            if let Some(next) = current.as_array().and_then(|array| array.get(*index)) {
                concrete.push(PathSegment::Index(*index));
                collect_path_observations(base, next, remaining, concrete, disclosed, work_left)?;
                concrete.pop();
            }
        }
        MetadataClaimPathSegment::AnyIndex => {
            if let Some(array) = current.as_array() {
                for (index, next) in array.iter().enumerate() {
                    concrete.push(PathSegment::Index(index));
                    collect_path_observations(
                        base, next, remaining, concrete, disclosed, work_left,
                    )?;
                    concrete.pop();
                }
            }
        }
    }
    Ok(())
}

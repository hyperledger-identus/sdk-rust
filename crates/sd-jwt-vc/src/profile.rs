use std::{collections::BTreeSet, fmt};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use identus_credentials::{CredentialEnvelope, CredentialFormat, CredentialPayload};
use identus_crypto::{PublicKeyJwk, SecureRandom};
use identus_jose::JwsSigner;
use identus_sd_jwt::{
    ClaimPath, DisclosureVerifiedSdJwt, IssuanceOptions, IssuancePlan, PathSegment, SdJwtLimits,
    UnverifiedSdJwt, issue_with_options,
};
use serde_json::{Map, Value};
use uriparse::URIReference;

use crate::{
    ENVELOPE_FORMAT, IssuerEvidence, IssuerEvidenceKind, PROTECTED_TYPE, SdJwtVcError,
    SdJwtVcLimits, StatusListBinding, StatusListEvidence, TypeMetadataChain,
    correlate_status_evidence, resource::bounded_json_bytes, resource::bounded_json_size,
};

const NON_SELECTIVE: [&str; 8] = [
    "iss",
    "nbf",
    "exp",
    "cnf",
    "vct",
    "vct#integrity",
    "aka_vcts",
    "status",
];

/// Optional caller-owned evidence applied during profile validation.
#[derive(Default)]
pub struct ProfileValidationInputs<'a> {
    pub require_key_binding: bool,
    pub issuer_evidence: Option<&'a IssuerEvidence>,
    pub type_metadata: Option<&'a TypeMetadataChain>,
    pub status_evidence: Option<&'a StatusListEvidence>,
}

impl fmt::Debug for ProfileValidationInputs<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProfileValidationInputs")
            .field("require_key_binding", &self.require_key_binding)
            .field("issuer_evidence", &self.issuer_evidence.is_some())
            .field("type_metadata", &self.type_metadata.is_some())
            .field("status_evidence", &self.status_evidence.is_some())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Draft22VerificationReceipt {
    pub issuer_signature: bool,
    pub disclosures_processed: bool,
    pub profile_validated: bool,
    pub issuer_evidence: Option<IssuerEvidenceKind>,
    pub type_metadata_checked: bool,
    pub status_evidence_checked: bool,
    pub additional_type_count: usize,
}

/// Canonical draft-22 issuer output.
pub struct IssuedDraft22SdJwtVc(identus_sd_jwt::IssuedSdJwt);

impl IssuedDraft22SdJwtVc {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_envelope(self) -> Result<CredentialEnvelope, SdJwtVcError> {
        let format = CredentialFormat::parse(ENVELOPE_FORMAT)
            .map_err(|_| SdJwtVcError::InternalInvariant)?;
        let payload = CredentialPayload::new(self.0.as_str().as_bytes().to_vec())
            .map_err(|_| SdJwtVcError::ResourceLimit)?;
        Ok(CredentialEnvelope::new(format, payload))
    }
}

impl fmt::Debug for IssuedDraft22SdJwtVc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuedDraft22SdJwtVc")
            .field("bytes", &self.0.as_str().len())
            .finish()
    }
}

/// Issue exact `dc+sd-jwt` bytes after validating profile claims and placement.
pub fn issue_draft22(
    claims: Value,
    plan: &IssuancePlan,
    random: &mut impl SecureRandom,
    signer: &dyn JwsSigner,
    base_limits: SdJwtLimits,
    profile_limits: SdJwtVcLimits,
) -> Result<IssuedDraft22SdJwtVc, SdJwtVcError> {
    profile_limits.validate()?;
    let object = claims.as_object().ok_or(SdJwtVcError::ProfileClaims)?;
    let status = object
        .get("status")
        .map(|status| parse_status(status, profile_limits))
        .transpose()?;
    let (vct, additional) = validate_claims(object, profile_limits, true)?;
    ensure_retained_output(&vct, &additional, status.as_ref(), profile_limits)?;
    for path in &plan.disclose {
        if let Some(PathSegment::Member(root)) = path.segments().first()
            && NON_SELECTIVE.contains(&root.as_str())
        {
            return Err(SdJwtVcError::ClaimPlacement);
        }
    }
    issue_with_options(
        claims,
        plan,
        IssuanceOptions {
            protected_type: Some(PROTECTED_TYPE),
        },
        random,
        signer,
        base_limits,
    )
    .map(IssuedDraft22SdJwtVc)
    .map_err(|_| SdJwtVcError::Base)
}

/// Parse the exact canonical generic envelope without assigning trust.
pub fn parse_draft22_envelope(
    envelope: &CredentialEnvelope,
    limits: SdJwtLimits,
) -> Result<UnverifiedSdJwt, SdJwtVcError> {
    if envelope.format().as_str() != ENVELOPE_FORMAT {
        return Err(SdJwtVcError::Envelope);
    }
    let compact =
        std::str::from_utf8(envelope.payload().as_bytes()).map_err(|_| SdJwtVcError::Envelope)?;
    UnverifiedSdJwt::parse(compact, limits).map_err(|_| SdJwtVcError::Base)
}

/// Disclosure-verified base credential with draft-22 semantics established.
pub struct Draft22SdJwtVc {
    base: DisclosureVerifiedSdJwt,
    vct: String,
    additional_types: Vec<String>,
    status: Option<StatusListBinding>,
    receipt: Draft22VerificationReceipt,
}

#[derive(Clone, Copy)]
pub(crate) enum ProfileFailureStage {
    Structural,
    IssuerKey,
    Schema,
    Status,
}

impl Draft22SdJwtVc {
    pub fn validate(
        base: DisclosureVerifiedSdJwt,
        inputs: ProfileValidationInputs<'_>,
        limits: SdJwtVcLimits,
    ) -> Result<Self, SdJwtVcError> {
        Self::validate_with_stage(base, inputs, limits).map_err(|(error, _)| error)
    }

    pub(crate) fn validate_with_stage(
        base: DisclosureVerifiedSdJwt,
        inputs: ProfileValidationInputs<'_>,
        limits: SdJwtVcLimits,
    ) -> Result<Self, (SdJwtVcError, ProfileFailureStage)> {
        map_failure(limits.validate(), ProfileFailureStage::Structural)?;
        if base.issuer_protected_header().type_() != Some(PROTECTED_TYPE) {
            return Err((SdJwtVcError::ProtectedType, ProfileFailureStage::Structural));
        }
        let claims = base
            .claims()
            .as_object()
            .ok_or((SdJwtVcError::ProfileClaims, ProfileFailureStage::Structural))?;
        let (vct, additional_types) = map_failure(
            validate_claims(claims, limits, true),
            ProfileFailureStage::Structural,
        )?;
        map_failure(
            validate_non_selective_placement(&base),
            ProfileFailureStage::Structural,
        )?;
        if inputs.require_key_binding && !valid_confirmation(claims.get("cnf"), limits) {
            return Err((
                SdJwtVcError::KeyBindingRequired,
                ProfileFailureStage::Structural,
            ));
        }
        let issuer = claims
            .get("iss")
            .and_then(Value::as_str)
            .ok_or((SdJwtVcError::ProfileClaims, ProfileFailureStage::Structural))?;
        let status = claims
            .get("status")
            .map(|value| parse_status(value, limits))
            .transpose()
            .map_err(|error| (error, ProfileFailureStage::Structural))?;
        map_failure(
            ensure_retained_output(&vct, &additional_types, status.as_ref(), limits),
            ProfileFailureStage::Structural,
        )?;
        let issuer_kind = inputs
            .issuer_evidence
            .map(|evidence| {
                evidence.correlate(
                    base.issuer_protected_header(),
                    base.issuer_verification_key(),
                    issuer,
                    limits,
                )
            })
            .transpose()
            .map_err(|error| (error, ProfileFailureStage::IssuerKey))?;
        if let Some(metadata) = inputs.type_metadata {
            metadata
                .validate_for(&vct, &base, limits)
                .map_err(|error| (error, ProfileFailureStage::Schema))?;
        }
        if let Some(evidence) = inputs.status_evidence {
            let binding = status
                .as_ref()
                .ok_or((SdJwtVcError::EvidenceMismatch, ProfileFailureStage::Status))?;
            correlate_status_evidence(binding, evidence)
                .map_err(|error| (error, ProfileFailureStage::Status))?;
        }
        let receipt = Draft22VerificationReceipt {
            issuer_signature: true,
            disclosures_processed: true,
            profile_validated: true,
            issuer_evidence: issuer_kind,
            type_metadata_checked: inputs.type_metadata.is_some(),
            status_evidence_checked: inputs.status_evidence.is_some(),
            additional_type_count: additional_types.len(),
        };
        Ok(Self {
            base,
            vct,
            additional_types,
            status,
            receipt,
        })
    }

    pub fn matches_type(&self, candidate: &str) -> bool {
        self.vct == candidate || self.additional_types.iter().any(|value| value == candidate)
    }

    pub const fn status_binding(&self) -> Option<&StatusListBinding> {
        self.status.as_ref()
    }

    pub const fn receipt(&self) -> Draft22VerificationReceipt {
        self.receipt
    }

    pub const fn base(&self) -> &DisclosureVerifiedSdJwt {
        &self.base
    }

    pub fn into_base(self) -> DisclosureVerifiedSdJwt {
        self.base
    }
}

fn map_failure<T>(
    result: Result<T, SdJwtVcError>,
    stage: ProfileFailureStage,
) -> Result<T, (SdJwtVcError, ProfileFailureStage)> {
    result.map_err(|error| (error, stage))
}

impl fmt::Debug for Draft22SdJwtVc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Draft22SdJwtVc")
            .field("additional_types", &self.additional_types.len())
            .field("status", &self.status.is_some())
            .field("receipt", &self.receipt)
            .finish_non_exhaustive()
    }
}

pub(crate) fn validate_claims(
    claims: &Map<String, Value>,
    limits: SdJwtVcLimits,
    require_issuer: bool,
) -> Result<(String, Vec<String>), SdJwtVcError> {
    if require_issuer && !valid_text(claims.get("iss"), limits.issuer_identifier_bytes) {
        return Err(SdJwtVcError::ProfileClaims);
    }
    let vct = claims
        .get("vct")
        .and_then(Value::as_str)
        .ok_or(SdJwtVcError::CredentialType)?;
    validate_type(vct, limits)?;
    let additional = match claims.get("aka_vcts") {
        None => Vec::new(),
        Some(Value::Array(values))
            if !values.is_empty() && values.len() <= limits.additional_types =>
        {
            let mut seen = BTreeSet::new();
            let mut output = Vec::with_capacity(values.len());
            for value in values {
                let candidate = value.as_str().ok_or(SdJwtVcError::AdditionalTypes)?;
                validate_type(candidate, limits)?;
                if candidate == vct || !seen.insert(candidate) {
                    return Err(SdJwtVcError::AdditionalTypes);
                }
                output.push(candidate.to_owned());
            }
            output
        }
        Some(_) => return Err(SdJwtVcError::AdditionalTypes),
    };
    ensure_retained_output(vct, &additional, None, limits)?;
    let nbf = numeric_date(claims.get("nbf"))?;
    let exp = numeric_date(claims.get("exp"))?;
    let _ = numeric_date(claims.get("iat"))?;
    if matches!((nbf, exp), (Some(start), Some(end)) if start.is_after(end)) {
        return Err(SdJwtVcError::NumericDate);
    }
    if let Some(cnf) = claims.get("cnf")
        && !valid_confirmation(Some(cnf), limits)
    {
        return Err(SdJwtVcError::ProfileClaims);
    }
    if let Some(subject) = claims.get("sub")
        && !valid_text(Some(subject), limits.issuer_identifier_bytes)
    {
        return Err(SdJwtVcError::ProfileClaims);
    }
    if let Some(integrity) = claims.get("vct#integrity")
        && !valid_integrity_metadata(integrity, limits.credential_type_bytes)
    {
        return Err(SdJwtVcError::ProfileClaims);
    }
    Ok((vct.to_owned(), additional))
}

pub(crate) fn validate_type(value: &str, limits: SdJwtVcLimits) -> Result<(), SdJwtVcError> {
    if value.is_empty()
        || value.len() > limits.credential_type_bytes
        || value.trim() != value
        || value.chars().any(char::is_control)
        || !collision_resistant_name_shape(value)
    {
        return Err(SdJwtVcError::CredentialType);
    }
    Ok(())
}

fn valid_text(value: Option<&Value>, maximum: usize) -> bool {
    value.and_then(Value::as_str).is_some_and(|value| {
        !value.is_empty() && value.len() <= maximum && !value.chars().any(char::is_control)
    })
}

fn valid_integrity_metadata(value: &Value, maximum: usize) -> bool {
    let Some(value) = value.as_str() else {
        return false;
    };
    if value.is_empty() || value.len() > maximum || value.trim() != value {
        return false;
    }
    let mut expressions = value.split_ascii_whitespace().peekable();
    if expressions.peek().is_none() {
        return false;
    }
    expressions.all(|expression| {
        let Some((algorithm, digest_and_options)) = expression.split_once('-') else {
            return false;
        };
        let digest = digest_and_options
            .split_once('?')
            .map_or(digest_and_options, |(digest, _)| digest);
        let expected = match algorithm {
            "sha256" => 32,
            "sha384" => 48,
            "sha512" => 64,
            _ => return false,
        };
        STANDARD
            .decode(digest)
            .is_ok_and(|bytes| bytes.len() == expected)
    })
}

#[derive(Clone, Copy)]
enum NumericDate {
    Integer(i128),
    Fraction(f64),
}

impl NumericDate {
    fn is_after(self, other: Self) -> bool {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => left > right,
            (Self::Fraction(left), Self::Fraction(right)) => left > right,
            (Self::Integer(integer), Self::Fraction(fraction)) => {
                compare_integer_fraction(integer, fraction).is_gt()
            }
            (Self::Fraction(fraction), Self::Integer(integer)) => {
                compare_integer_fraction(integer, fraction).is_lt()
            }
        }
    }
}

fn compare_integer_fraction(integer: i128, fraction: f64) -> std::cmp::Ordering {
    if fraction >= i128::MAX as f64 {
        return std::cmp::Ordering::Less;
    }
    if fraction <= i128::MIN as f64 {
        return std::cmp::Ordering::Greater;
    }
    let truncated = fraction.trunc() as i128;
    integer.cmp(&truncated).then_with(|| {
        if fraction.fract().is_sign_positive() && fraction.fract() != 0.0 {
            std::cmp::Ordering::Less
        } else if fraction.fract().is_sign_negative() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

fn numeric_date(value: Option<&Value>) -> Result<Option<NumericDate>, SdJwtVcError> {
    let Some(value) = value else { return Ok(None) };
    let number = value.as_number().ok_or(SdJwtVcError::NumericDate)?;
    if let Some(value) = number.as_i64() {
        return Ok(Some(NumericDate::Integer(i128::from(value))));
    }
    if let Some(value) = number.as_u64() {
        return Ok(Some(NumericDate::Integer(i128::from(value))));
    }
    number
        .as_f64()
        .filter(|value| value.is_finite())
        .map(NumericDate::Fraction)
        .map(Some)
        .ok_or(SdJwtVcError::NumericDate)
}

pub(crate) fn valid_confirmation(value: Option<&Value>, limits: SdJwtVcLimits) -> bool {
    value.and_then(Value::as_object).is_some_and(|object| {
        if object.len() != 1 {
            return false;
        }
        match (object.get("jwk"), object.get("kid")) {
            (Some(jwk @ Value::Object(_)), None) => bounded_json_bytes(
                jwk,
                limits
                    .issuer_metadata_bytes
                    .min(limits.retained_output_bytes),
            )
            .and_then(|bytes| {
                serde_json::from_slice::<PublicKeyJwk>(&bytes)
                    .map_err(|_| SdJwtVcError::ProfileClaims)
            })
            .is_ok(),
            (None, Some(Value::String(kid))) => {
                !kid.is_empty()
                    && kid.len() <= limits.issuer_identifier_bytes
                    && !kid.chars().any(char::is_control)
            }
            _ => false,
        }
    })
}

fn collision_resistant_name_shape(value: &str) -> bool {
    let Ok(reference) = URIReference::try_from(value) else {
        return false;
    };
    reference.scheme().is_some()
        || value
            .split('.')
            .count()
            .checked_sub(1)
            .is_some_and(|separators| {
                separators > 0
                    && value.split('.').all(|part| {
                        !part.is_empty()
                            && part.bytes().all(|byte| {
                                byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')
                            })
                    })
            })
        || uuid_shape(value)
}

fn uuid_shape(value: &str) -> bool {
    let groups = [8, 4, 4, 4, 12];
    let mut parts = value.split('-');
    groups.iter().all(|length| {
        parts.next().is_some_and(|part| {
            part.len() == *length && part.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
    }) && parts.next().is_none()
}

pub(crate) fn validate_non_selective_placement(
    base: &DisclosureVerifiedSdJwt,
) -> Result<(), SdJwtVcError> {
    for name in NON_SELECTIVE {
        let path = ClaimPath::new(
            vec![PathSegment::Member(name.to_owned())],
            SdJwtLimits::default(),
        )
        .map_err(|_| SdJwtVcError::InternalInvariant)?;
        if base.was_disclosed_at_or_below(&path) {
            return Err(SdJwtVcError::ClaimPlacement);
        }
    }
    Ok(())
}

fn parse_status(value: &Value, limits: SdJwtVcLimits) -> Result<StatusListBinding, SdJwtVcError> {
    bounded_json_size(value, limits.status_bytes)?;
    let object = value.as_object().ok_or(SdJwtVcError::Status)?;
    if object.len() != 1 {
        return Err(SdJwtVcError::UnsupportedStatus);
    }
    let status = object
        .get("status_list")
        .ok_or(SdJwtVcError::UnsupportedStatus)?
        .as_object()
        .ok_or(SdJwtVcError::Status)?;
    if status.len() != 2 {
        return Err(SdJwtVcError::Status);
    }
    let index = status
        .get("idx")
        .and_then(Value::as_u64)
        .ok_or(SdJwtVcError::Status)?;
    let uri = status
        .get("uri")
        .and_then(Value::as_str)
        .ok_or(SdJwtVcError::Status)?;
    StatusListBinding::new(index, uri, limits)
}

pub(crate) fn ensure_retained_output(
    vct: &str,
    additional: &[String],
    status: Option<&StatusListBinding>,
    limits: SdJwtVcLimits,
) -> Result<(), SdJwtVcError> {
    let retained = additional
        .iter()
        .try_fold(vct.len(), |total, value| total.checked_add(value.len()));
    let retained = retained
        .and_then(|total| total.checked_add(status.map_or(0, StatusListBinding::retained_bytes)));
    if retained.is_none_or(|bytes| bytes > limits.retained_output_bytes) {
        return Err(SdJwtVcError::ResourceLimit);
    }
    Ok(())
}

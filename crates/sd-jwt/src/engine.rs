use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use identus_core::UnixTimestampMillis;
use identus_credentials::{CredentialEnvelope, CredentialFormat, CredentialPayload};
use identus_crypto::{PublicKeyJwk, SecureRandom};
use identus_jose::{
    JoseError, JwsLimits, JwsSigner, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry, UnverifiedCompactJws,
};
use serde_json::{Map, Value};

use crate::{
    ClaimPath, DecoyLocation, DisclosureHasher, IssuanceOptions, IssuancePlan, PathSegment,
    SdJwtError, SdJwtLimits, SelectionPlan, Sha256DisclosureHasher, VerificationReceipt, json,
    limits::Work,
};

fn jws_limits(limits: SdJwtLimits, compact: usize) -> Result<JwsLimits, SdJwtError> {
    JwsLimits::new(
        compact,
        4_096,
        limits.issuer_json_bytes.max(4_096),
        1_024,
        2_048,
    )
    .map_err(|_| SdJwtError::InvalidLimits)
}

fn signing_error(error: JoseError) -> SdJwtError {
    match error {
        JoseError::SignerUnavailable => SdJwtError::EffectUnavailable,
        _ => SdJwtError::EffectRejected,
    }
}

fn digest(
    encoded: &str,
    work: &mut Work,
    hasher: &dyn DisclosureHasher,
) -> Result<String, SdJwtError> {
    work.charge(encoded.len())?;
    Ok(URL_SAFE_NO_PAD.encode(hasher.hash(encoded.as_bytes())))
}

fn decode(segment: &str, maximum: usize, work: &mut Work) -> Result<Vec<u8>, SdJwtError> {
    if segment.is_empty()
        || segment.len()
            > maximum
                .saturating_mul(4)
                .saturating_div(3)
                .saturating_add(4)
    {
        return Err(SdJwtError::ResourceLimit);
    }
    let bytes = URL_SAFE_NO_PAD
        .decode(segment)
        .map_err(|_| SdJwtError::Encoding)?;
    if bytes.len() > maximum || URL_SAFE_NO_PAD.encode(&bytes) != segment {
        return Err(SdJwtError::Encoding);
    }
    work.charge(bytes.len())?;
    Ok(bytes)
}

fn base64url_encoded_len(bytes: usize) -> Result<usize, SdJwtError> {
    bytes
        .checked_mul(4)
        .and_then(|value| value.checked_add(2))
        .map(|value| value / 3)
        .ok_or(SdJwtError::InvalidLimits)
}

fn valid_digest(value: &str) -> bool {
    value.len() == 43
        && URL_SAFE_NO_PAD
            .decode(value)
            .is_ok_and(|bytes| bytes.len() == 32 && URL_SAFE_NO_PAD.encode(bytes) == value)
}

/// A structurally parsed compact value without cryptographic evidence.
pub struct UnverifiedSdJwt {
    issuer: UnverifiedCompactJws,
    disclosures: Vec<String>,
    key_binding: Option<String>,
    limits: SdJwtLimits,
}

impl fmt::Debug for UnverifiedSdJwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnverifiedSdJwt")
            .field("disclosures", &self.disclosures.len())
            .field("issuer_bytes", &self.issuer.compact().len())
            .finish()
    }
}

impl UnverifiedSdJwt {
    /// Split only the compact RFC 9901 form and retain exact JWS/disclosure bytes.
    pub fn parse(compact: &str, limits: SdJwtLimits) -> Result<Self, SdJwtError> {
        let limits = limits.validate()?;
        if compact.len() > limits.compact_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        let (issuer_text, tail) = compact
            .split_once('~')
            .ok_or(SdJwtError::CompactStructure)?;
        if issuer_text.is_empty() || issuer_text.len() > limits.issuer_jws_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        let mut segments: Vec<&str> = tail.split('~').collect();
        if segments.len() > limits.disclosures + 2 {
            return Err(SdJwtError::ResourceLimit);
        }
        let final_segment = segments.pop().ok_or(SdJwtError::CompactStructure)?;
        let key_binding = if final_segment.is_empty() {
            None
        } else {
            if final_segment.len() > limits.key_binding_jws_bytes {
                return Err(SdJwtError::ResourceLimit);
            }
            Some(final_segment.to_owned())
        };
        if segments.len() > limits.disclosures
            || segments
                .iter()
                .any(|s| s.is_empty() || s.len() > limits.disclosure_encoded_bytes)
        {
            return Err(SdJwtError::ResourceLimit);
        }
        let mut work = Work::new(limits);
        work.charge(compact.len())?;
        let issuer =
            UnverifiedCompactJws::parse(issuer_text, jws_limits(limits, limits.issuer_jws_bytes)?)
                .map_err(|_| SdJwtError::CompactStructure)?;
        if issuer.payload().len() > limits.issuer_json_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        if let Some(ref kb) = key_binding {
            UnverifiedCompactJws::parse(kb, jws_limits(limits, limits.key_binding_jws_bytes)?)
                .map_err(|_| SdJwtError::KeyBindingStructure)?;
        }
        Ok(Self {
            issuer,
            disclosures: segments.into_iter().map(str::to_owned).collect(),
            key_binding,
            limits,
        })
    }

    /// Verify only the issuer signature with a caller-selected algorithm-bound key.
    pub fn verify_issuer(
        self,
        suites: &SignatureSuiteRegistry,
        key: &JwsVerificationKey<'_>,
    ) -> Result<IssuerVerifiedSdJwt, SdJwtError> {
        suites
            .verify(&self.issuer, key)
            .map_err(|_| SdJwtError::IssuerSignature)?;
        let verification_key_bytes =
            json::encoded_len(key.public_key(), self.limits.retained_output_bytes)?;
        Ok(IssuerVerifiedSdJwt {
            inner: self,
            algorithm: key.algorithm(),
            verification_key: key.public_key().clone(),
            verification_key_bytes,
        })
    }
}

/// An issuer-authenticated value whose disclosure graph has not been processed.
pub struct IssuerVerifiedSdJwt {
    inner: UnverifiedSdJwt,
    algorithm: identus_jose::JwsAlgorithm,
    verification_key: PublicKeyJwk,
    verification_key_bytes: usize,
}
impl fmt::Debug for IssuerVerifiedSdJwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssuerVerifiedSdJwt")
            .field("algorithm", &self.algorithm)
            .finish()
    }
}

struct Disclosure {
    name: Option<String>,
    value: Value,
    encoded: String,
}

fn parse_disclosure(
    encoded: &str,
    limits: SdJwtLimits,
    work: &mut Work,
) -> Result<Disclosure, SdJwtError> {
    let bytes = decode(encoded, limits.disclosure_decoded_bytes, work)?;
    let value = json::parse(&bytes, limits, work)?;
    let Value::Array(mut parts) = value else {
        return Err(SdJwtError::DisclosureShape);
    };
    if parts.len() != 2 && parts.len() != 3 {
        return Err(SdJwtError::DisclosureShape);
    }
    let value = parts.pop().ok_or(SdJwtError::DisclosureShape)?;
    let name = if parts.len() == 2 {
        Some(
            parts
                .pop()
                .and_then(|v| v.as_str().map(str::to_owned))
                .ok_or(SdJwtError::DisclosureShape)?,
        )
    } else {
        None
    };
    let salt = parts
        .pop()
        .and_then(|v| v.as_str().map(str::to_owned))
        .ok_or(SdJwtError::DisclosureShape)?;
    // RFC 9901 defines the salt as an arbitrary JSON string. Base64url is a
    // recommended issuer construction, not verifier syntax, so verification
    // applies only the SDK's UTF-8 byte bounds.
    if salt.is_empty()
        || salt.len() > base64url_encoded_len(limits.salt_max_bytes)?
        || name.as_ref().is_some_and(|n| {
            n.is_empty() || n.len() > 1_024 || matches!(n.as_str(), "_sd" | "_sd_alg" | "...")
        })
    {
        return Err(SdJwtError::DisclosureShape);
    }
    Ok(Disclosure {
        name,
        value,
        encoded: encoded.to_owned(),
    })
}

struct Graph<'a> {
    index: BTreeMap<String, Disclosure>,
    visited: BTreeSet<String>,
    inventory: BTreeMap<ClaimPath, String>,
    limits: SdJwtLimits,
    work: &'a mut Work,
    nodes: usize,
    members: usize,
}

impl Graph<'_> {
    fn take(
        &mut self,
        reference: &str,
        path: &[PathSegment],
    ) -> Result<Option<Disclosure>, SdJwtError> {
        if !valid_digest(reference) {
            return Err(SdJwtError::DisclosureShape);
        }
        self.work.charge(1)?;
        if !self.visited.insert(reference.to_owned()) {
            return Err(SdJwtError::AmbiguousReference);
        }
        let item = self.index.remove(reference);
        if let Some(ref found) = item {
            let path = ClaimPath::new(path.to_vec(), self.limits)?;
            if self.inventory.insert(path, found.encoded.clone()).is_some() {
                return Err(SdJwtError::AmbiguousReference);
            }
        }
        Ok(item)
    }

    fn resolve(
        &mut self,
        value: Value,
        path: &mut Vec<PathSegment>,
        depth: usize,
    ) -> Result<Value, SdJwtError> {
        if depth > self.limits.json_depth {
            return Err(SdJwtError::ResourceLimit);
        }
        self.nodes = self.nodes.checked_add(1).ok_or(SdJwtError::ResourceLimit)?;
        if self.nodes > self.limits.json_nodes {
            return Err(SdJwtError::ResourceLimit);
        }
        self.work.charge(1)?;
        match value {
            Value::Object(mut object) => {
                if object.contains_key("_sd_alg") || object.contains_key("...") {
                    return Err(SdJwtError::ReservedClaim);
                }
                let refs = object.remove("_sd");
                self.members = self
                    .members
                    .checked_add(object.len())
                    .ok_or(SdJwtError::ResourceLimit)?;
                if self.members > self.limits.json_members {
                    return Err(SdJwtError::ResourceLimit);
                }
                for (name, value) in &mut object {
                    path.push(PathSegment::Member(name.clone()));
                    *value = self.resolve(std::mem::take(value), path, depth + 1)?;
                    path.pop();
                }
                if let Some(refs) = refs {
                    let Value::Array(refs) = refs else {
                        return Err(SdJwtError::DisclosureShape);
                    };
                    for reference in refs {
                        let reference = reference.as_str().ok_or(SdJwtError::DisclosureShape)?;
                        let candidate = self.index.get(reference).and_then(|d| d.name.clone());
                        let mut child = path.clone();
                        if let Some(name) = candidate {
                            child.push(PathSegment::Member(name));
                        }
                        if let Some(disclosure) = self.take(reference, &child)? {
                            let name = disclosure.name.ok_or(SdJwtError::DisclosureShape)?;
                            if object.contains_key(&name) {
                                return Err(SdJwtError::AmbiguousReference);
                            }
                            self.members = self
                                .members
                                .checked_add(1)
                                .ok_or(SdJwtError::ResourceLimit)?;
                            if self.members > self.limits.json_members {
                                return Err(SdJwtError::ResourceLimit);
                            }
                            path.push(PathSegment::Member(name.clone()));
                            let value = self.resolve(disclosure.value, path, depth + 1)?;
                            path.pop();
                            object.insert(name, value);
                        }
                    }
                }
                Ok(Value::Object(object))
            }
            Value::Array(array) => {
                let mut output = Vec::new();
                for value in array {
                    path.push(PathSegment::Index(output.len()));
                    if let Value::Object(ref map) = value
                        && let Some(reference) = map.get("...")
                    {
                        if map.len() != 1 {
                            return Err(SdJwtError::DisclosureShape);
                        }
                        let reference = reference.as_str().ok_or(SdJwtError::DisclosureShape)?;
                        if let Some(disclosure) = self.take(reference, path)? {
                            if disclosure.name.is_some() {
                                return Err(SdJwtError::DisclosureShape);
                            }
                            output.push(self.resolve(disclosure.value, path, depth + 1)?);
                        }
                        path.pop();
                        continue;
                    }
                    output.push(self.resolve(value, path, depth + 1)?);
                    path.pop();
                }
                Ok(Value::Array(output))
            }
            scalar => Ok(scalar),
        }
    }
}

/// Authenticated disclosures and bounded materialized claims.
pub struct DisclosureVerifiedSdJwt {
    issuer: String,
    inventory: BTreeMap<ClaimPath, String>,
    claims: Value,
    full_compact: Option<String>,
    key_binding: Option<String>,
    limits: SdJwtLimits,
    algorithm: identus_jose::JwsAlgorithm,
    protected_header: ProtectedHeader,
    verification_key: PublicKeyJwk,
}
impl fmt::Debug for DisclosureVerifiedSdJwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DisclosureVerifiedSdJwt")
            .field("disclosures", &self.inventory.len())
            .finish()
    }
}

impl IssuerVerifiedSdJwt {
    /// Resolve exactly one finite graph; every supplied disclosure must be used.
    pub fn process_disclosures(self) -> Result<DisclosureVerifiedSdJwt, SdJwtError> {
        self.process_disclosures_with_hasher(&Sha256DisclosureHasher)
    }

    /// Resolve with an injected digest capability for deterministic fault testing.
    pub fn process_disclosures_with_hasher(
        self,
        hasher: &dyn DisclosureHasher,
    ) -> Result<DisclosureVerifiedSdJwt, SdJwtError> {
        let limits = self.inner.limits;
        let mut work = Work::new(limits);
        let mut payload = json::parse(self.inner.issuer.payload(), limits, &mut work)?;
        let object = payload.as_object_mut().ok_or(SdJwtError::Json)?;
        if let Some(algorithm) = object.remove("_sd_alg")
            && algorithm.as_str() != Some("sha-256")
        {
            return Err(SdJwtError::UnsupportedHashAlgorithm);
        }
        let mut index = BTreeMap::new();
        let mut encoded_seen = BTreeSet::new();
        for encoded in &self.inner.disclosures {
            if !encoded_seen.insert(encoded) {
                return Err(SdJwtError::DuplicateDisclosure);
            }
            let disclosure = parse_disclosure(encoded, limits, &mut work)?;
            let key = digest(encoded, &mut work, hasher)?;
            if index.insert(key, disclosure).is_some() {
                return Err(SdJwtError::DigestCollision);
            }
        }
        let mut graph = Graph {
            index,
            visited: BTreeSet::new(),
            inventory: BTreeMap::new(),
            limits,
            work: &mut work,
            nodes: 0,
            members: 0,
        };
        let claims = graph.resolve(payload, &mut Vec::new(), 0)?;
        if !graph.index.is_empty() {
            return Err(SdJwtError::UnusedDisclosure);
        }
        let inventory = std::mem::take(&mut graph.inventory);
        drop(graph);
        let claims_bytes = json::encode(&claims, limits.retained_output_bytes, &mut work)?;
        let protected_header_bytes = json::encoded_len(
            self.inner.issuer.protected_header(),
            limits.retained_output_bytes,
        )?;
        let mut full_compact = self.inner.issuer.compact().to_owned();
        full_compact.push('~');
        for encoded in &self.inner.disclosures {
            full_compact.push_str(encoded);
            full_compact.push('~');
        }
        if let Some(ref kb) = self.inner.key_binding {
            full_compact.push_str(kb);
        }
        let retained = claims_bytes
            .len()
            .checked_add(full_compact.len())
            .and_then(|n| n.checked_add(self.inner.issuer.compact().len()))
            .and_then(|n| n.checked_add(self.inner.key_binding.as_ref().map_or(0, String::len)))
            .and_then(|n| n.checked_add(inventory.values().map(String::len).sum::<usize>()))
            .and_then(|n| n.checked_add(self.verification_key_bytes))
            .and_then(|n| n.checked_add(protected_header_bytes))
            .ok_or(SdJwtError::ResourceLimit)?;
        if retained > limits.retained_output_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        work.charge(retained)?;
        Ok(DisclosureVerifiedSdJwt {
            issuer: self.inner.issuer.compact().to_owned(),
            inventory,
            claims,
            full_compact: Some(full_compact),
            key_binding: self.inner.key_binding,
            limits,
            algorithm: self.algorithm,
            protected_header: self.inner.issuer.protected_header().clone(),
            verification_key: self.verification_key,
        })
    }
}

impl DisclosureVerifiedSdJwt {
    /// Borrow materialized claims only after issuer and disclosure verification.
    pub fn claims(&self) -> &Value {
        &self.claims
    }

    /// Borrow the exact bounded protected header authenticated by the issuer
    /// signature. Profile and trust semantics remain caller-owned.
    pub fn issuer_protected_header(&self) -> &ProtectedHeader {
        &self.protected_header
    }

    /// Borrow the exact public key used to verify the issuer signature.
    pub fn issuer_verification_key(&self) -> &PublicKeyJwk {
        &self.verification_key
    }

    /// Return whether one validated path originated in a Disclosure.
    pub fn was_disclosed(&self, path: &ClaimPath) -> bool {
        self.inventory.contains_key(path)
    }

    /// Return whether a Disclosure supplied the path itself or any descendant.
    ///
    /// Profile layers use this bounded provenance query when an entire claim
    /// subtree must remain non-selective. The disclosure inventory is already
    /// limited by [`SdJwtLimits::disclosures`].
    pub fn was_disclosed_at_or_below(&self, path: &ClaimPath) -> bool {
        self.inventory
            .keys()
            .any(|candidate| candidate.segments().starts_with(path.segments()))
    }

    /// Produce a deterministic minimal closure for the chosen claim paths.
    pub fn select(&self, plan: &SelectionPlan) -> Result<SdJwtPresentation, SdJwtError> {
        if plan.paths.len() > self.limits.selected_paths {
            return Err(SdJwtError::ResourceLimit);
        }
        let mut work = Work::new(self.limits);
        let mut selected = BTreeSet::new();
        for path in &plan.paths {
            if !json::path_exists(&self.claims, path.segments()) {
                return Err(SdJwtError::InvalidSelection);
            }
            for candidate in self.inventory.keys() {
                work.charge(1)?;
                if path.segments().starts_with(candidate.segments()) {
                    selected.insert(candidate.clone());
                }
            }
        }
        let disclosure_count = selected.len();
        let mut compact = self.issuer.clone();
        compact.push('~');
        for path in selected {
            let encoded = self
                .inventory
                .get(&path)
                .ok_or(SdJwtError::InternalInvariant)?;
            let new_len = compact
                .len()
                .checked_add(encoded.len() + 1)
                .ok_or(SdJwtError::ResourceLimit)?;
            if new_len > self.limits.compact_bytes || new_len > self.limits.retained_output_bytes {
                return Err(SdJwtError::ResourceLimit);
            }
            work.charge(encoded.len() + 1)?;
            compact.push_str(encoded);
            compact.push('~');
        }
        Ok(SdJwtPresentation {
            compact,
            limits: self.limits,
            algorithm: self.algorithm,
            disclosure_count,
        })
    }
}

/// Exact presentation bytes ending in the no-key-binding separator.
pub struct SdJwtPresentation {
    compact: String,
    limits: SdJwtLimits,
    algorithm: identus_jose::JwsAlgorithm,
    disclosure_count: usize,
}
impl fmt::Debug for SdJwtPresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdJwtPresentation")
            .field("bytes", &self.compact.len())
            .field("disclosures", &self.disclosure_count)
            .finish()
    }
}

impl SdJwtPresentation {
    pub fn as_str(&self) -> &str {
        &self.compact
    }

    /// Bind exact presentation bytes to caller-owned audience, nonce, and time.
    pub fn bind(
        &self,
        audience: &str,
        nonce: &str,
        issued_at: UnixTimestampMillis,
        signer: &dyn JwsSigner,
    ) -> Result<String, SdJwtError> {
        self.bind_with_hasher(audience, nonce, issued_at, signer, &Sha256DisclosureHasher)
    }

    /// Construct holder binding with an injected digest capability for testing.
    pub fn bind_with_hasher(
        &self,
        audience: &str,
        nonce: &str,
        issued_at: UnixTimestampMillis,
        signer: &dyn JwsSigner,
        hasher: &dyn DisclosureHasher,
    ) -> Result<String, SdJwtError> {
        if audience.is_empty()
            || nonce.is_empty()
            || audience.len() > self.limits.audience_nonce_bytes
            || nonce.len() > self.limits.audience_nonce_bytes
        {
            return Err(SdJwtError::ResourceLimit);
        }
        let mut work = Work::new(self.limits);
        let hash = digest(&self.compact, &mut work, hasher)?;
        let payload = serde_json::json!({"aud": audience, "nonce": nonce,
            "iat": issued_at.whole_seconds(), "sd_hash": hash});
        let bytes = json::encode(&payload, self.limits.key_binding_jws_bytes, &mut work)?;
        let jws_limits = jws_limits(self.limits, self.limits.key_binding_jws_bytes)?;
        let header = ProtectedHeader::new(
            signer.algorithm().as_str(),
            Some("kb+jwt"),
            None,
            jws_limits,
        )
        .map_err(|_| SdJwtError::KeyBindingStructure)?;
        let signing = JwsSigningInput::new(header, bytes, jws_limits)
            .map_err(|_| SdJwtError::ResourceLimit)?;
        let total = self
            .compact
            .len()
            .checked_add(signing.as_bytes().len())
            .and_then(|n| n.checked_add(87))
            .ok_or(SdJwtError::ResourceLimit)?;
        if total > self.limits.compact_bytes || total > self.limits.retained_output_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        work.charge(total)?;
        let signed = signing.sign_with(signer).map_err(signing_error)?;
        Ok(format!("{}{}", self.compact, signed.compact()))
    }

    pub fn receipt(&self) -> VerificationReceipt {
        VerificationReceipt {
            issuer_signature: true,
            disclosures_processed: true,
            holder_binding: false,
            disclosure_count: self.disclosure_count,
            algorithm: self.algorithm,
        }
    }
}

/// Caller-owned key-binding comparison inputs.
pub struct KeyBindingExpectations<'a> {
    pub audience: &'a str,
    pub nonce: &'a str,
    pub issued_at: UnixTimestampMillis,
}

/// All cryptographic stages passed; no trust or replay policy is inferred.
pub struct VerifiedSdJwt {
    receipt: VerificationReceipt,
}
impl VerifiedSdJwt {
    pub fn receipt(&self) -> VerificationReceipt {
        self.receipt
    }
}
impl fmt::Debug for VerifiedSdJwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedSdJwt")
            .field("receipt", &self.receipt)
            .finish()
    }
}

impl DisclosureVerifiedSdJwt {
    /// Verify holder binding over exact received pre-KB bytes.
    pub fn verify_key_binding(
        self,
        suites: &SignatureSuiteRegistry,
        key: &JwsVerificationKey<'_>,
        expected: KeyBindingExpectations<'_>,
    ) -> Result<VerifiedSdJwt, SdJwtError> {
        self.verify_key_binding_with_hasher(suites, key, expected, &Sha256DisclosureHasher)
    }

    /// Verify holder binding with an injected digest capability for testing.
    pub fn verify_key_binding_with_hasher(
        self,
        suites: &SignatureSuiteRegistry,
        key: &JwsVerificationKey<'_>,
        expected: KeyBindingExpectations<'_>,
        hasher: &dyn DisclosureHasher,
    ) -> Result<VerifiedSdJwt, SdJwtError> {
        if expected.audience.is_empty()
            || expected.nonce.is_empty()
            || expected.audience.len() > self.limits.audience_nonce_bytes
            || expected.nonce.len() > self.limits.audience_nonce_bytes
        {
            return Err(SdJwtError::ResourceLimit);
        }
        let kb = self.inner_key_binding()?;
        let mut work = Work::new(self.limits);
        let base = self.base_compact()?;
        let expected_hash = digest(&base, &mut work, hasher)?;
        let parsed = UnverifiedCompactJws::parse(
            &kb,
            jws_limits(self.limits, self.limits.key_binding_jws_bytes)?,
        )
        .map_err(|_| SdJwtError::KeyBindingStructure)?;
        if parsed.protected_header().type_() != Some("kb+jwt") {
            return Err(SdJwtError::KeyBindingType);
        }
        suites
            .verify(&parsed, key)
            .map_err(|_| SdJwtError::KeyBindingSignature)?;
        let body = json::parse(parsed.payload(), self.limits, &mut work)?;
        let claims = body.as_object().ok_or(SdJwtError::KeyBindingStructure)?;
        if claims.get("sd_hash").and_then(Value::as_str) != Some(&expected_hash) {
            return Err(SdJwtError::KeyBindingHash);
        }
        if claims.get("aud").and_then(Value::as_str) != Some(expected.audience) {
            return Err(SdJwtError::KeyBindingAudience);
        }
        if claims.get("nonce").and_then(Value::as_str) != Some(expected.nonce) {
            return Err(SdJwtError::KeyBindingNonce);
        }
        if claims.get("iat").and_then(Value::as_u64) != Some(expected.issued_at.whole_seconds()) {
            return Err(SdJwtError::KeyBindingTime);
        }
        Ok(VerifiedSdJwt {
            receipt: VerificationReceipt {
                issuer_signature: true,
                disclosures_processed: true,
                holder_binding: true,
                disclosure_count: self.inventory.len(),
                algorithm: self.algorithm,
            },
        })
    }

    fn inner_key_binding(&self) -> Result<String, SdJwtError> {
        self.key_binding
            .clone()
            .ok_or(SdJwtError::KeyBindingStructure)
    }
    fn base_compact(&self) -> Result<String, SdJwtError> {
        let full = self
            .full_compact
            .as_ref()
            .ok_or(SdJwtError::KeyBindingStructure)?;
        let (base, _) = full
            .rsplit_once('~')
            .ok_or(SdJwtError::KeyBindingStructure)?;
        Ok(format!("{base}~"))
    }
}

/// One bounded encoded disclosure. Debug reveals only its byte length.
pub struct EncodedDisclosure(String);

impl EncodedDisclosure {
    /// Validate one exact encoded disclosure under the selected limits.
    pub fn parse(encoded: &str, limits: SdJwtLimits) -> Result<Self, SdJwtError> {
        let limits = limits.validate()?;
        if encoded.len() > limits.disclosure_encoded_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        let mut work = Work::new(limits);
        parse_disclosure(encoded, limits, &mut work)?;
        Ok(Self(encoded.to_owned()))
    }

    /// Borrow the exact encoded disclosure through an explicit accessor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for EncodedDisclosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EncodedDisclosure")
            .field("bytes", &self.0.len())
            .finish()
    }
}

/// Issuer output with exact compact bytes and a private inventory.
pub struct IssuedSdJwt {
    compact: String,
    inventory: Vec<EncodedDisclosure>,
}
impl IssuedSdJwt {
    pub fn as_str(&self) -> &str {
        &self.compact
    }
    pub fn disclosures(&self) -> &[EncodedDisclosure] {
        &self.inventory
    }
    pub fn into_envelope(self) -> Result<CredentialEnvelope, SdJwtError> {
        let format =
            CredentialFormat::parse("sd-jwt").map_err(|_| SdJwtError::InternalInvariant)?;
        let payload = CredentialPayload::new(self.compact.into_bytes())
            .map_err(|_| SdJwtError::ResourceLimit)?;
        Ok(CredentialEnvelope::new(format, payload))
    }
}
impl fmt::Debug for IssuedSdJwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssuedSdJwt")
            .field("bytes", &self.compact.len())
            .finish()
    }
}

/// Re-enter verification from an opaque format-neutral envelope.
pub fn parse_envelope(
    envelope: &CredentialEnvelope,
    limits: SdJwtLimits,
) -> Result<UnverifiedSdJwt, SdJwtError> {
    if envelope.format().as_str() != "sd-jwt" {
        return Err(SdJwtError::CompactStructure);
    }
    let text =
        std::str::from_utf8(envelope.payload().as_bytes()).map_err(|_| SdJwtError::Encoding)?;
    UnverifiedSdJwt::parse(text, limits)
}

fn locate_mut<'a>(
    root: &'a mut Value,
    segments: &[PathSegment],
) -> Result<&'a mut Value, SdJwtError> {
    let mut current = root;
    for segment in segments {
        current = match (current, segment) {
            (Value::Object(map), PathSegment::Member(name)) => map.get_mut(name),
            (Value::Array(array), PathSegment::Index(index)) => array.get_mut(*index),
            _ => None,
        }
        .ok_or(SdJwtError::InvalidSelection)?;
    }
    Ok(current)
}

fn add_reference(object: &mut Map<String, Value>, digest: String) -> Result<(), SdJwtError> {
    let refs = object
        .entry("_sd")
        .or_insert_with(|| Value::Array(Vec::new()));
    refs.as_array_mut()
        .ok_or(SdJwtError::ReservedClaim)?
        .push(Value::String(digest));
    Ok(())
}

/// Issue RFC 9901 object and array disclosures through injected entropy and signing.
pub fn issue(
    claims: Value,
    plan: &IssuancePlan,
    random: &mut impl SecureRandom,
    signer: &dyn JwsSigner,
    limits: SdJwtLimits,
) -> Result<IssuedSdJwt, SdJwtError> {
    issue_with_options_and_hasher(
        claims,
        plan,
        IssuanceOptions::default(),
        random,
        signer,
        limits,
        &Sha256DisclosureHasher,
    )
}

/// Issue with an injected digest capability for deterministic collision testing.
pub fn issue_with_hasher(
    claims: Value,
    plan: &IssuancePlan,
    random: &mut impl SecureRandom,
    signer: &dyn JwsSigner,
    limits: SdJwtLimits,
    hasher: &dyn DisclosureHasher,
) -> Result<IssuedSdJwt, SdJwtError> {
    issue_with_options_and_hasher(
        claims,
        plan,
        IssuanceOptions::default(),
        random,
        signer,
        limits,
        hasher,
    )
}

/// Issue with a caller-validated protected type while retaining RFC mechanics.
pub fn issue_with_options(
    claims: Value,
    plan: &IssuancePlan,
    options: IssuanceOptions<'_>,
    random: &mut impl SecureRandom,
    signer: &dyn JwsSigner,
    limits: SdJwtLimits,
) -> Result<IssuedSdJwt, SdJwtError> {
    issue_with_options_and_hasher(
        claims,
        plan,
        options,
        random,
        signer,
        limits,
        &Sha256DisclosureHasher,
    )
}

/// Issue with profile-neutral header options and an injected digest capability.
pub fn issue_with_options_and_hasher(
    mut claims: Value,
    plan: &IssuancePlan,
    options: IssuanceOptions<'_>,
    random: &mut impl SecureRandom,
    signer: &dyn JwsSigner,
    limits: SdJwtLimits,
    hasher: &dyn DisclosureHasher,
) -> Result<IssuedSdJwt, SdJwtError> {
    let limits = limits.validate()?;
    if plan.disclose.len() > limits.disclosures
        || plan.decoy_locations.len() > limits.decoys
        || plan.salt_bytes < limits.salt_min_bytes
        || plan.salt_bytes > limits.salt_max_bytes
    {
        return Err(SdJwtError::ResourceLimit);
    }
    let mut work = Work::new(limits);
    let encoded = json::encode(&claims, limits.issuer_json_bytes, &mut work)?;
    let reparsed = json::parse(&encoded, limits, &mut work)?;
    if reparsed != claims || !claims.is_object() {
        return Err(SdJwtError::Json);
    }
    reject_reserved(&claims, 0, limits, &mut work)?;
    let mut seen_digests = BTreeSet::new();
    for location in &plan.decoy_locations {
        let container = match location {
            DecoyLocation::Root => &mut claims,
            DecoyLocation::Path(path) => locate_mut(&mut claims, path.segments())?,
        };
        let object = container
            .as_object_mut()
            .ok_or(SdJwtError::InvalidSelection)?;
        let mut entropy = [0_u8; 32];
        random
            .fill_bytes(&mut entropy)
            .map_err(|_| SdJwtError::EffectUnavailable)?;
        work.charge(32)?;
        let decoy = URL_SAFE_NO_PAD.encode(entropy);
        if !seen_digests.insert(decoy.clone()) {
            return Err(SdJwtError::DigestCollision);
        }
        add_reference(object, decoy)?;
    }
    let mut seen = BTreeSet::new();
    let mut seen_salts = BTreeSet::new();
    let mut paths = plan.disclose.clone();
    paths.sort_by_key(|p| std::cmp::Reverse(p.segments().len()));
    let mut inventory = Vec::new();
    for path in paths {
        if !seen.insert(path.clone()) {
            return Err(SdJwtError::InvalidSelection);
        }
        let (last, parent) = path
            .segments()
            .split_last()
            .ok_or(SdJwtError::InvalidSelection)?;
        let parent = locate_mut(&mut claims, parent)?;
        let (name, value) = match (parent, last) {
            (Value::Object(map), PathSegment::Member(name)) => (
                Some(name.clone()),
                map.remove(name).ok_or(SdJwtError::InvalidSelection)?,
            ),
            (Value::Array(array), PathSegment::Index(index)) => (
                None,
                std::mem::replace(
                    array.get_mut(*index).ok_or(SdJwtError::InvalidSelection)?,
                    Value::Null,
                ),
            ),
            _ => return Err(SdJwtError::InvalidSelection),
        };
        let mut salt = vec![0; plan.salt_bytes];
        random
            .fill_bytes(&mut salt)
            .map_err(|_| SdJwtError::EffectUnavailable)?;
        let salt = URL_SAFE_NO_PAD.encode(salt);
        if !seen_salts.insert(salt.clone()) {
            return Err(SdJwtError::DuplicateDisclosure);
        }
        let disclosure = match &name {
            Some(name) => serde_json::json!([salt, name, value]),
            None => serde_json::json!([salt, value]),
        };
        let bytes = json::encode(&disclosure, limits.disclosure_decoded_bytes, &mut work)?;
        let encoded = URL_SAFE_NO_PAD.encode(bytes);
        if encoded.len() > limits.disclosure_encoded_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        let hash = digest(&encoded, &mut work, hasher)?;
        if !seen_digests.insert(hash.clone()) {
            return Err(SdJwtError::DigestCollision);
        }
        let parent = locate_mut(&mut claims, parent_path(&path))?;
        match (parent, last) {
            (Value::Object(map), PathSegment::Member(_)) => add_reference(map, hash)?,
            (Value::Array(array), PathSegment::Index(index)) => {
                array[*index] = serde_json::json!({"...": hash});
            }
            _ => return Err(SdJwtError::InternalInvariant),
        }
        inventory.push(encoded);
    }
    let root = claims.as_object_mut().ok_or(SdJwtError::Json)?;
    root.insert("_sd_alg".to_owned(), Value::String("sha-256".to_owned()));
    let bytes = json::encode(&claims, limits.issuer_json_bytes, &mut work)?;
    let validated = json::parse(&bytes, limits, &mut work)?;
    if validated != claims {
        return Err(SdJwtError::InternalInvariant);
    }
    let jws_limits = jws_limits(limits, limits.issuer_jws_bytes)?;
    let header = ProtectedHeader::new(
        signer.algorithm().as_str(),
        options.protected_type,
        None,
        jws_limits,
    )
    .map_err(signing_error)?;
    let signing =
        JwsSigningInput::new(header, bytes, jws_limits).map_err(|_| SdJwtError::ResourceLimit)?;
    let mut predicted = signing
        .as_bytes()
        .len()
        .checked_add(88)
        .ok_or(SdJwtError::ResourceLimit)?;
    let mut inventory_bytes = 0_usize;
    for disclosure in &inventory {
        predicted = predicted
            .checked_add(disclosure.len() + 1)
            .ok_or(SdJwtError::ResourceLimit)?;
        inventory_bytes = inventory_bytes
            .checked_add(disclosure.len())
            .ok_or(SdJwtError::ResourceLimit)?;
    }
    if predicted > limits.compact_bytes
        || predicted
            .checked_add(inventory_bytes)
            .ok_or(SdJwtError::ResourceLimit)?
            > limits.retained_output_bytes
    {
        return Err(SdJwtError::ResourceLimit);
    }
    work.charge(
        predicted
            .checked_add(inventory_bytes)
            .ok_or(SdJwtError::ResourceLimit)?,
    )?;
    let signed = signing.sign_with(signer).map_err(signing_error)?;
    let mut compact = signed.compact().to_owned();
    compact.push('~');
    for disclosure in &inventory {
        let new_len = compact
            .len()
            .checked_add(disclosure.len() + 1)
            .ok_or(SdJwtError::ResourceLimit)?;
        if new_len > limits.compact_bytes || new_len > limits.retained_output_bytes {
            return Err(SdJwtError::ResourceLimit);
        }
        compact.push_str(disclosure);
        compact.push('~');
    }
    if compact.len() > limits.compact_bytes {
        return Err(SdJwtError::ResourceLimit);
    }
    Ok(IssuedSdJwt {
        compact,
        inventory: inventory.into_iter().map(EncodedDisclosure).collect(),
    })
}

fn parent_path(path: &ClaimPath) -> &[PathSegment] {
    &path.segments()[..path.segments().len() - 1]
}

fn reject_reserved(
    value: &Value,
    depth: usize,
    limits: SdJwtLimits,
    work: &mut Work,
) -> Result<(), SdJwtError> {
    if depth > limits.json_depth {
        return Err(SdJwtError::ResourceLimit);
    }
    work.charge(1)?;
    match value {
        Value::Object(map) => {
            if map
                .keys()
                .any(|k| matches!(k.as_str(), "_sd" | "_sd_alg" | "..."))
            {
                return Err(SdJwtError::ReservedClaim);
            }
            for child in map.values() {
                reject_reserved(child, depth + 1, limits, work)?;
            }
        }
        Value::Array(array) => {
            for child in array {
                reject_reserved(child, depth + 1, limits, work)?;
            }
        }
        _ => {}
    }
    Ok(())
}

//! Client identifier classification and caller-owned verifier trust evidence.

use std::{collections::BTreeSet, fmt, future::Future, pin::Pin};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::PublicKeyJwk;
use identus_did::{Did, DidUrl};
use identus_jose::ProtectedHeader;
use serde_json::{Map, Value};
use zeroize::Zeroizing;

use crate::{Oid4vpError, RequestObjectSecurityEvidence};

/// OpenID4VP Final client identifier scheme supported by the signed-JAR profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClientIdentifierScheme {
    /// OAuth pre-registered client identifier without a colon.
    PreRegistered,
    /// OpenID Federation entity identifier.
    OpenIdFederation,
    /// W3C Decentralized Identifier.
    DecentralizedIdentifier,
    /// Verifier attestation bound to the Request Object key.
    VerifierAttestation,
    /// X.509 leaf certificate with a matching DNS SAN.
    X509SanDns,
    /// X.509 leaf certificate identified by its SHA-256 hash.
    X509Hash,
}

/// Finite policy for verifier metadata and verifier-info parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifierAuthorizationLimits {
    max_metadata_bytes: usize,
    max_verifier_info_entries: usize,
    max_credential_ids: usize,
    max_string_bytes: usize,
    max_trust_chain_entries: usize,
}

impl VerifierAuthorizationLimits {
    /// Construct a positive resource policy.
    pub const fn new(
        max_metadata_bytes: usize,
        max_verifier_info_entries: usize,
        max_credential_ids: usize,
        max_string_bytes: usize,
        max_trust_chain_entries: usize,
    ) -> Result<Self, Oid4vpError> {
        if max_metadata_bytes == 0
            || max_verifier_info_entries == 0
            || max_credential_ids == 0
            || max_string_bytes == 0
            || max_trust_chain_entries == 0
        {
            return Err(Oid4vpError::InvalidLimits);
        }
        Ok(Self {
            max_metadata_bytes,
            max_verifier_info_entries,
            max_credential_ids,
            max_string_bytes,
            max_trust_chain_entries,
        })
    }
}

impl Default for VerifierAuthorizationLimits {
    fn default() -> Self {
        Self {
            max_metadata_bytes: 32_768,
            max_verifier_info_entries: 16,
            max_credential_ids: 64,
            max_string_bytes: 16_384,
            max_trust_chain_entries: 8,
        }
    }
}

/// Bounded opaque verifier metadata serialized as exact JSON bytes.
pub struct VerifierMetadata {
    json: Zeroizing<Vec<u8>>,
    member_count: usize,
}

impl VerifierMetadata {
    /// Explicitly reveal bounded verifier-controlled JSON for an adapter.
    pub fn expose_sensitive_json(&self) -> &[u8] {
        &self.json
    }

    /// Number of top-level metadata members.
    pub const fn member_count(&self) -> usize {
        self.member_count
    }
}

impl fmt::Debug for VerifierMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifierMetadata")
            .field("json_len", &self.json.len())
            .field("member_count", &self.member_count)
            .finish()
    }
}

/// One bounded Final `verifier_info` entry.
pub struct VerifierInfo {
    format: Zeroizing<String>,
    data_json: Zeroizing<Vec<u8>>,
    credential_ids: Vec<Zeroizing<String>>,
}

impl VerifierInfo {
    /// Explicitly reveal the verifier-info format identifier.
    pub fn expose_sensitive_format(&self) -> &str {
        &self.format
    }

    /// Explicitly reveal the bounded object-or-string data as JSON bytes.
    pub fn expose_sensitive_data_json(&self) -> &[u8] {
        &self.data_json
    }

    /// Iterate exact referenced DCQL credential identifiers.
    pub fn expose_sensitive_credential_ids(&self) -> impl ExactSizeIterator<Item = &str> {
        self.credential_ids.iter().map(|value| value.as_str())
    }
}

impl fmt::Debug for VerifierInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifierInfo")
            .field("format_len", &self.format.len())
            .field("data_json_len", &self.data_json.len())
            .field("credential_id_count", &self.credential_ids.len())
            .finish()
    }
}

/// Static failure returned by a caller-owned verifier trust adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VerifierTrustFailure {
    /// Scheme evidence, exact key binding, or caller trust policy was rejected.
    Rejected,
    /// The trust provider could not decide.
    Unavailable,
}

/// Exact signed response destination presented to caller-owned trust policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VerifierResponseDestination<'a> {
    /// Final `response_uri` used by direct-post modes.
    ResponseUri(&'a str),
    /// OAuth `redirect_uri` used by redirect response modes.
    RedirectUri(&'a str),
}

/// Borrowed exact evidence supplied to a verifier trust adapter.
#[derive(Clone, Copy)]
pub struct VerifierTrustInput<'a> {
    scheme: ClientIdentifierScheme,
    full_client_id: &'a str,
    original_client_id: &'a str,
    response_destination: Option<VerifierResponseDestination<'a>>,
    protected_header: &'a ProtectedHeader,
    verification_key: &'a PublicKeyJwk,
    metadata: Option<&'a VerifierMetadata>,
    verifier_info: &'a [VerifierInfo],
    trust_chain: &'a [Zeroizing<String>],
}

impl<'a> VerifierTrustInput<'a> {
    /// Classified Final scheme.
    pub const fn scheme(&self) -> ClientIdentifierScheme {
        self.scheme
    }

    /// Explicitly reveal the full prefixed client identifier.
    pub const fn expose_sensitive_full_client_id(&self) -> &'a str {
        self.full_client_id
    }

    /// Explicitly reveal the identifier inside the scheme namespace.
    pub const fn expose_sensitive_original_client_id(&self) -> &'a str {
        self.original_client_id
    }

    /// Explicitly reveal the exact signed response destination, when textual.
    pub const fn expose_sensitive_response_uri(&self) -> Option<&'a str> {
        match self.response_destination {
            Some(VerifierResponseDestination::ResponseUri(value)) => Some(value),
            _ => None,
        }
    }

    /// Explicitly reveal the exact signed response destination, if present.
    pub const fn expose_sensitive_response_destination(
        &self,
    ) -> Option<VerifierResponseDestination<'a>> {
        self.response_destination
    }

    /// Protected header of the exact verified Request Object.
    pub const fn protected_header(&self) -> &'a ProtectedHeader {
        self.protected_header
    }

    /// Exact public key that verified the correlated Request Object.
    pub const fn verification_key(&self) -> &'a PublicKeyJwk {
        self.verification_key
    }

    /// Inline verifier metadata when Final permits it for this scheme.
    pub const fn metadata(&self) -> Option<&'a VerifierMetadata> {
        self.metadata
    }

    /// Bounded verifier-info entries.
    pub const fn verifier_info(&self) -> &'a [VerifierInfo] {
        self.verifier_info
    }

    /// Bounded Federation trust-chain statements.
    pub fn expose_sensitive_trust_chain(&self) -> impl ExactSizeIterator<Item = &'a str> {
        self.trust_chain.iter().map(|value| value.as_str())
    }
}

impl fmt::Debug for VerifierTrustInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifierTrustInput")
            .field("scheme", &self.scheme)
            .field("full_client_id_len", &self.full_client_id.len())
            .field("original_client_id_len", &self.original_client_id.len())
            .field(
                "response_destination_len",
                &self
                    .response_destination
                    .map(|destination| match destination {
                        VerifierResponseDestination::ResponseUri(value)
                        | VerifierResponseDestination::RedirectUri(value) => value.len(),
                    }),
            )
            .field("has_metadata", &self.metadata.is_some())
            .field("verifier_info_count", &self.verifier_info.len())
            .field("trust_chain_count", &self.trust_chain.len())
            .finish_non_exhaustive()
    }
}

/// Future returned by [`VerifierTrustPolicy`].
pub type VerifierTrustFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), VerifierTrustFailure>> + Send + 'a>>;

/// Caller-owned Final scheme validation and exact verifier-key trust policy.
pub trait VerifierTrustPolicy: Send + Sync {
    /// Validate the selected scheme completely and authorize the exact key.
    fn authorize<'a>(&'a self, input: VerifierTrustInput<'a>) -> VerifierTrustFuture<'a>;
}

/// Request security evidence accepted by an exact verifier trust policy.
pub struct VerifierAuthorizationEvidence {
    security: RequestObjectSecurityEvidence,
    scheme: ClientIdentifierScheme,
}

impl VerifierAuthorizationEvidence {
    /// Accepted Final client identifier scheme.
    pub const fn scheme(&self) -> ClientIdentifierScheme {
        self.scheme
    }

    pub(crate) fn into_parts(self) -> (RequestObjectSecurityEvidence, ClientIdentifierScheme) {
        (self.security, self.scheme)
    }
}

impl fmt::Debug for VerifierAuthorizationEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifierAuthorizationEvidence")
            .field("scheme", &self.scheme)
            .field("security", &self.security)
            .finish()
    }
}

impl RequestObjectSecurityEvidence {
    /// Consume request-security evidence through Final verifier authorization.
    pub async fn authorize_verifier(
        self,
        policy: &dyn VerifierTrustPolicy,
        limits: VerifierAuthorizationLimits,
    ) -> Result<VerifierAuthorizationEvidence, Oid4vpError> {
        let client_id = self.expose_sensitive_client_id();
        let (scheme, original_client_id) = classify_client_id(client_id)?;
        validate_scheme_evidence(scheme, original_client_id, self.protected_header())?;

        let Value::Object(payload) = serde_json::from_slice(self.expose_sensitive_payload())
            .map_err(|_| Oid4vpError::InvalidVerifierMetadata)?
        else {
            return Err(Oid4vpError::InvalidVerifierMetadata);
        };
        let metadata = parse_metadata(payload.get("client_metadata"), scheme, limits)?;
        let trust_chain = parse_trust_chain(
            self.protected_header().trust_chain(),
            payload.get("trust_chain"),
            scheme,
            limits,
        )?;
        let dcql_ids = dcql_credential_ids(&payload)?;
        let verifier_info = parse_verifier_info(payload.get("verifier_info"), &dcql_ids, limits)?;
        let response_uri = payload.get("response_uri").and_then(Value::as_str);
        let redirect_uri = payload.get("redirect_uri").and_then(Value::as_str);
        let response_destination = match (response_uri, redirect_uri) {
            (Some(value), None) => Some(VerifierResponseDestination::ResponseUri(value)),
            (None, Some(value)) => Some(VerifierResponseDestination::RedirectUri(value)),
            _ => None,
        };

        policy
            .authorize(VerifierTrustInput {
                scheme,
                full_client_id: client_id,
                original_client_id,
                response_destination,
                protected_header: self.protected_header(),
                verification_key: self.verification_key(),
                metadata: metadata.as_ref(),
                verifier_info: &verifier_info,
                trust_chain: &trust_chain,
            })
            .await
            .map_err(|failure| match failure {
                VerifierTrustFailure::Rejected => Oid4vpError::VerifierTrustRejected,
                VerifierTrustFailure::Unavailable => Oid4vpError::VerifierTrustUnavailable,
            })?;

        Ok(VerifierAuthorizationEvidence {
            security: self,
            scheme,
        })
    }
}

fn classify_client_id(value: &str) -> Result<(ClientIdentifierScheme, &str), Oid4vpError> {
    let Some((prefix, original)) = value.split_once(':') else {
        return Ok((ClientIdentifierScheme::PreRegistered, value));
    };
    if original.is_empty() {
        return Err(Oid4vpError::InvalidClientIdentifier);
    }
    let scheme = match prefix {
        "openid_federation" => ClientIdentifierScheme::OpenIdFederation,
        "decentralized_identifier" => ClientIdentifierScheme::DecentralizedIdentifier,
        "verifier_attestation" => ClientIdentifierScheme::VerifierAttestation,
        "x509_san_dns" => ClientIdentifierScheme::X509SanDns,
        "x509_hash" => ClientIdentifierScheme::X509Hash,
        "redirect_uri" | "origin" => return Err(Oid4vpError::UnsupportedClientIdentifierScheme),
        _ => return Err(Oid4vpError::UnsupportedClientIdentifierScheme),
    };
    Ok((scheme, original))
}

fn validate_scheme_evidence(
    scheme: ClientIdentifierScheme,
    original: &str,
    header: &ProtectedHeader,
) -> Result<(), Oid4vpError> {
    match scheme {
        ClientIdentifierScheme::PreRegistered => Ok(()),
        ClientIdentifierScheme::OpenIdFederation => valid_text(original)
            .then_some(())
            .ok_or(Oid4vpError::InvalidClientIdentifier),
        ClientIdentifierScheme::DecentralizedIdentifier => {
            let did = Did::parse(original).map_err(|_| Oid4vpError::InvalidClientIdentifier)?;
            let kid = header
                .key_id()
                .ok_or(Oid4vpError::InvalidClientIdentifier)?;
            let did_url = DidUrl::parse(kid).map_err(|_| Oid4vpError::InvalidClientIdentifier)?;
            if did_url.as_did_str() != did.as_str() || did_url.fragment().is_none() {
                return Err(Oid4vpError::InvalidClientIdentifier);
            }
            Ok(())
        }
        ClientIdentifierScheme::VerifierAttestation => header
            .nested_jwt()
            .is_some()
            .then_some(())
            .ok_or(Oid4vpError::InvalidClientIdentifier),
        ClientIdentifierScheme::X509SanDns => {
            if !valid_dns_name(original) || header.certificate_chain().is_none() {
                return Err(Oid4vpError::InvalidClientIdentifier);
            }
            Ok(())
        }
        ClientIdentifierScheme::X509Hash => {
            let valid_hash = URL_SAFE_NO_PAD
                .decode(original)
                .is_ok_and(|bytes| bytes.len() == 32 && URL_SAFE_NO_PAD.encode(bytes) == original);
            if !valid_hash || header.certificate_chain().is_none() {
                return Err(Oid4vpError::InvalidClientIdentifier);
            }
            Ok(())
        }
    }
}

fn parse_metadata(
    value: Option<&Value>,
    scheme: ClientIdentifierScheme,
    limits: VerifierAuthorizationLimits,
) -> Result<Option<VerifierMetadata>, Oid4vpError> {
    if matches!(
        scheme,
        ClientIdentifierScheme::OpenIdFederation | ClientIdentifierScheme::PreRegistered
    ) {
        return Ok(None);
    }
    let Some(Value::Object(object)) = value else {
        return if value.is_none() {
            Ok(None)
        } else {
            Err(Oid4vpError::InvalidVerifierMetadata)
        };
    };
    let json = serde_json::to_vec(object).map_err(|_| Oid4vpError::InvalidVerifierMetadata)?;
    if json.len() > limits.max_metadata_bytes {
        return Err(Oid4vpError::VerifierMetadataTooLarge);
    }
    Ok(Some(VerifierMetadata {
        json: Zeroizing::new(json),
        member_count: object.len(),
    }))
}

fn parse_trust_chain(
    protected_value: Option<&[String]>,
    payload_value: Option<&Value>,
    scheme: ClientIdentifierScheme,
    limits: VerifierAuthorizationLimits,
) -> Result<Vec<Zeroizing<String>>, Oid4vpError> {
    if payload_value.is_some() {
        return Err(Oid4vpError::InvalidVerifierMetadata);
    }
    if scheme != ClientIdentifierScheme::OpenIdFederation {
        return if protected_value.is_none() {
            Ok(Vec::new())
        } else {
            Err(Oid4vpError::InvalidVerifierMetadata)
        };
    }
    let Some(values) = protected_value else {
        return Ok(Vec::new());
    };
    if values.is_empty() || values.len() > limits.max_trust_chain_entries {
        return Err(Oid4vpError::InvalidVerifierMetadata);
    }
    values
        .iter()
        .map(|value| {
            if value.len() > limits.max_string_bytes || !valid_compact(value) {
                return Err(Oid4vpError::InvalidVerifierMetadata);
            }
            Ok(Zeroizing::new(value.to_owned()))
        })
        .collect()
}

fn dcql_credential_ids(payload: &Map<String, Value>) -> Result<BTreeSet<&str>, Oid4vpError> {
    let Some(credentials) = payload
        .get("dcql_query")
        .and_then(Value::as_object)
        .and_then(|query| query.get("credentials"))
        .and_then(Value::as_array)
    else {
        return Ok(BTreeSet::new());
    };
    let mut ids = BTreeSet::new();
    for credential in credentials {
        let id = credential
            .as_object()
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str)
            .ok_or(Oid4vpError::InvalidVerifierMetadata)?;
        if !ids.insert(id) {
            return Err(Oid4vpError::InvalidVerifierMetadata);
        }
    }
    Ok(ids)
}

fn parse_verifier_info(
    value: Option<&Value>,
    dcql_ids: &BTreeSet<&str>,
    limits: VerifierAuthorizationLimits,
) -> Result<Vec<VerifierInfo>, Oid4vpError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let values = value
        .as_array()
        .ok_or(Oid4vpError::InvalidVerifierMetadata)?;
    if values.is_empty() || values.len() > limits.max_verifier_info_entries {
        return Err(Oid4vpError::InvalidVerifierMetadata);
    }
    values
        .iter()
        .map(|value| {
            let object = value
                .as_object()
                .ok_or(Oid4vpError::InvalidVerifierMetadata)?;
            let format = object
                .get("format")
                .and_then(Value::as_str)
                .filter(|value| valid_text(value) && value.len() <= limits.max_string_bytes)
                .ok_or(Oid4vpError::InvalidVerifierMetadata)?;
            let data = object
                .get("data")
                .ok_or(Oid4vpError::InvalidVerifierMetadata)?;
            if !matches!(data, Value::Object(_) | Value::String(_)) {
                return Err(Oid4vpError::InvalidVerifierMetadata);
            }
            if data
                .as_str()
                .is_some_and(|value| value.len() > limits.max_string_bytes)
            {
                return Err(Oid4vpError::InvalidVerifierMetadata);
            }
            let data_json =
                serde_json::to_vec(data).map_err(|_| Oid4vpError::InvalidVerifierMetadata)?;
            if data_json.len() > limits.max_metadata_bytes {
                return Err(Oid4vpError::VerifierMetadataTooLarge);
            }
            let credential_ids = match object.get("credential_ids") {
                None => Vec::new(),
                Some(Value::Array(ids))
                    if !ids.is_empty() && ids.len() <= limits.max_credential_ids =>
                {
                    let mut seen = BTreeSet::new();
                    ids.iter()
                        .map(|id| {
                            let id = id
                                .as_str()
                                .filter(|id| {
                                    id.len() <= limits.max_string_bytes
                                        && dcql_ids.contains(id)
                                        && seen.insert(*id)
                                })
                                .ok_or(Oid4vpError::InvalidVerifierMetadata)?;
                            Ok(Zeroizing::new(id.to_owned()))
                        })
                        .collect::<Result<Vec<_>, _>>()?
                }
                Some(_) => return Err(Oid4vpError::InvalidVerifierMetadata),
            };
            Ok(VerifierInfo {
                format: Zeroizing::new(format.to_owned()),
                data_json: Zeroizing::new(data_json),
                credential_ids,
            })
        })
        .collect()
}

fn valid_text(value: &str) -> bool {
    !value.is_empty() && !value.chars().any(char::is_control)
}

fn valid_dns_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 253
        && !value.ends_with('.')
        && value.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

fn valid_compact(value: &str) -> bool {
    let mut segments = value.split('.');
    let valid = (0..3).all(|_| {
        segments.next().is_some_and(|segment| {
            !segment.is_empty()
                && URL_SAFE_NO_PAD
                    .decode(segment)
                    .is_ok_and(|bytes| URL_SAFE_NO_PAD.encode(bytes) == segment)
        })
    });
    valid && segments.next().is_none()
}

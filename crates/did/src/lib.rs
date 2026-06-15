//! Type-safe `DID` and `DID URL` primitives.
//!
//! This crate owns the parsing boundary for decentralized identifiers used by
//! Identus products. Method-specific creation and resolution remain behind
//! later adapters, but all crates can already depend on validated `DID` values
//! instead of unchecked strings.

use identus_core::{
    CapabilityId, ErrorCode, ErrorEnvelope, ErrorKind, IdentusError, IdentusResult,
};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-did",
    summary: "DID Core, PRISM DID, peer DID, and resolver contracts.",
};

/// Supported DID method classification.
///
/// `Prism` and `Peer` are first-class Identus product methods today. `Web`,
/// `Key`, `Jwk`, and `Pkh` are parsed as extension-compatible methods because
/// they appear in SDK fixtures, `DIDComm`/`AnonCreds` interop inputs, or common
/// SSI ecosystems. `Example` is accepted for conformance vectors only.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DidMethodKind {
    /// `did:prism`, used by Cloud Agent, SDKs, neoprism, and PRISM VDR.
    Prism,
    /// `did:peer`, primarily `did:peer:2`, used for `DIDComm` and Mediator.
    Peer,
    /// `did:web`, observed in `AnonCreds` interop fixtures and common SSI usage.
    Web,
    /// `did:key`, observed in `DIDComm` and VC ecosystem fixtures.
    Key,
    /// `did:jwk`, common key-material DID method candidate.
    Jwk,
    /// `did:pkh`, common account-bound DID method candidate.
    Pkh,
    /// `did:example`, allowed only for tests and conformance examples.
    Example,
    /// Any syntactically valid method not yet assigned product semantics.
    Other,
}

impl DidMethodKind {
    /// Whether this method is a first-class Identus product method today.
    #[must_use]
    pub const fn is_identus_product_method(self) -> bool {
        matches!(self, Self::Prism | Self::Peer)
    }

    /// Whether this method can be accepted by generic DID syntax boundaries.
    #[must_use]
    pub const fn is_extension_method(self) -> bool {
        matches!(self, Self::Web | Self::Key | Self::Jwk | Self::Pkh)
    }
}

/// Current support tier for a DID method.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DidMethodSupport {
    /// First-class product method that must have owned creation, parsing, and
    /// resolver behavior.
    Product,
    /// Accepted extension method that must be represented safely and can gain
    /// resolver adapters without changing the core type.
    Extension,
    /// Method accepted for conformance vectors and protocol examples only.
    FixtureOnly,
    /// Syntactically valid DID method with no product commitment yet.
    Unknown,
}

/// DID method support profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DidMethodProfile {
    /// Method name without the `did:` prefix.
    pub method: &'static str,
    /// Method classification.
    pub kind: DidMethodKind,
    /// Current support tier.
    pub support: DidMethodSupport,
    /// Where this method was observed or why it is tracked.
    pub source: &'static str,
    /// Owner or likely owner for method-specific behavior.
    pub owner: &'static str,
    /// Candidate Rust crates discovered for later adapter evaluation.
    pub candidate_crates: &'static [&'static str],
}

/// DID methods currently tracked by `sdk-rust`.
pub const DID_METHOD_PROFILES: &[DidMethodProfile] = &[
    DidMethodProfile {
        method: "prism",
        kind: DidMethodKind::Prism,
        support: DidMethodSupport::Product,
        source: "Cloud Agent, SDKs, neoprism, PRISM VDR",
        owner: "identus-did with neoprism convergence",
        candidate_crates: &[],
    },
    DidMethodProfile {
        method: "peer",
        kind: DidMethodKind::Peer,
        support: DidMethodSupport::Product,
        source: "Mediator, DIDComm, SDK peer connection flows",
        owner: "identus-did and identus-messaging",
        candidate_crates: &["did-peer", "ssi-dids"],
    },
    DidMethodProfile {
        method: "web",
        kind: DidMethodKind::Web,
        support: DidMethodSupport::Extension,
        source: "AnonCreds interop fixtures and common SSI deployments",
        owner: "identus-did resolver adapter",
        candidate_crates: &["ssi-dids", "affinidi-did-web"],
    },
    DidMethodProfile {
        method: "key",
        kind: DidMethodKind::Key,
        support: DidMethodSupport::Extension,
        source: "DIDComm and VC ecosystem fixtures",
        owner: "identus-did resolver adapter",
        candidate_crates: &["did-key", "did-method-key", "ssi-dids", "affinidi-did-key"],
    },
    DidMethodProfile {
        method: "jwk",
        kind: DidMethodKind::Jwk,
        support: DidMethodSupport::Extension,
        source: "Common key-material DID ecosystem",
        owner: "identus-did resolver adapter",
        candidate_crates: &["did-jwk", "ssi-dids"],
    },
    DidMethodProfile {
        method: "pkh",
        kind: DidMethodKind::Pkh,
        support: DidMethodSupport::Extension,
        source: "Common account-bound DID ecosystem and ssi method support",
        owner: "identus-did resolver adapter",
        candidate_crates: &["ssi-dids"],
    },
    DidMethodProfile {
        method: "example",
        kind: DidMethodKind::Example,
        support: DidMethodSupport::FixtureOnly,
        source: "DIDComm test vectors and W3C examples",
        owner: "identus-conformance",
        candidate_crates: &["ssi-dids"],
    },
];

/// Validated DID method name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DidMethod(String);

impl DidMethod {
    /// Parse and validate a DID method name.
    ///
    /// # Errors
    ///
    /// Returns [`DidParseError::InvalidMethod`] when the method name is empty
    /// or contains characters outside the W3C DID method grammar.
    pub fn parse(input: &str) -> Result<Self, DidParseError> {
        if input.is_empty()
            || !input
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        {
            return Err(DidParseError::InvalidMethod);
        }

        Ok(Self(input.to_owned()))
    }

    /// Parse and validate a DID method name with the shared core error surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the method name is empty or contains
    /// characters outside the W3C DID method grammar.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the method as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Classify the method against the current Identus support policy.
    #[must_use]
    pub fn kind(&self) -> DidMethodKind {
        match self.as_str() {
            "prism" => DidMethodKind::Prism,
            "peer" => DidMethodKind::Peer,
            "web" => DidMethodKind::Web,
            "key" => DidMethodKind::Key,
            "jwk" => DidMethodKind::Jwk,
            "pkh" => DidMethodKind::Pkh,
            "example" => DidMethodKind::Example,
            _ => DidMethodKind::Other,
        }
    }

    /// Return the tracked support profile for this method, if known.
    #[must_use]
    pub fn profile(&self) -> Option<&'static DidMethodProfile> {
        DID_METHOD_PROFILES
            .iter()
            .find(|profile| profile.method == self.as_str())
    }

    /// Return the current support tier for this method.
    #[must_use]
    pub fn support(&self) -> DidMethodSupport {
        self.profile()
            .map_or(DidMethodSupport::Unknown, |profile| profile.support)
    }
}

impl Display for DidMethod {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DidMethod {
    type Err = DidParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// Validated method-specific identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MethodSpecificId(String);

impl MethodSpecificId {
    /// Parse and validate a method-specific identifier.
    ///
    /// # Errors
    ///
    /// Returns [`DidParseError::InvalidMethodSpecificId`] when the identifier
    /// is empty or contains characters outside the DID Core grammar.
    pub fn parse(input: &str) -> Result<Self, DidParseError> {
        if input.is_empty() || !is_valid_did_component(input, is_method_specific_id_byte) {
            return Err(DidParseError::InvalidMethodSpecificId);
        }

        Ok(Self(input.to_owned()))
    }

    /// Parse and validate a method-specific identifier with the shared core
    /// error surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the identifier is empty or contains
    /// characters outside the DID Core grammar.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the method-specific identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for MethodSpecificId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validated decentralized identifier without path, query, or fragment.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Did {
    value: String,
    method: DidMethod,
    method_specific_id: MethodSpecificId,
}

impl Did {
    /// Parse a DID string.
    ///
    /// # Errors
    ///
    /// Returns [`DidParseError`] when the input is not a DID, contains DID URL
    /// components, or violates the method/method-specific-id grammar.
    pub fn parse(input: &str) -> Result<Self, DidParseError> {
        let parsed = parse_did_prefix(input)?;

        if parsed.remainder.bytes().any(is_did_url_separator) {
            return Err(DidParseError::DidUrlComponentsNotAllowed);
        }

        let method_specific_id = MethodSpecificId::parse(&parsed.remainder)?;

        Ok(Self {
            value: input.to_owned(),
            method: parsed.method,
            method_specific_id,
        })
    }

    /// Parse a DID string with the shared core error surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the input is not a DID, contains DID URL
    /// components, or violates the method/method-specific-id grammar.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the canonical DID string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Borrow the validated DID method.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Borrow the validated method-specific identifier.
    #[must_use]
    pub const fn method_specific_id(&self) -> &MethodSpecificId {
        &self.method_specific_id
    }

    /// Return the current support classification for the DID method.
    #[must_use]
    pub fn method_kind(&self) -> DidMethodKind {
        self.method.kind()
    }

    /// Whether this DID uses a first-class Identus product method.
    #[must_use]
    pub fn is_identus_product_method(&self) -> bool {
        self.method_kind().is_identus_product_method()
    }

    /// Return the current support tier for this DID's method.
    #[must_use]
    pub fn method_support(&self) -> DidMethodSupport {
        self.method.support()
    }
}

impl Display for Did {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Did {
    type Err = DidParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// Validated DID URL with optional path, query, and fragment.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DidUrl {
    value: String,
    did: Did,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

impl DidUrl {
    /// Parse a DID URL string.
    ///
    /// # Errors
    ///
    /// Returns [`DidParseError`] when the DID prefix is invalid, DID Core
    /// method syntax is invalid, or any DID URL component contains whitespace.
    pub fn parse(input: &str) -> Result<Self, DidParseError> {
        let component_start = input.find(is_did_url_separator_char).unwrap_or(input.len());
        let did = Did::parse(&input[..component_start])?;
        let components = parse_url_components(&input[component_start..])?;

        Ok(Self {
            value: input.to_owned(),
            did,
            path: components.path,
            query: components.query,
            fragment: components.fragment,
        })
    }

    /// Parse a DID URL string with the shared core error surface.
    ///
    /// # Errors
    ///
    /// Returns [`IdentusError`] when the DID prefix is invalid, DID Core
    /// method syntax is invalid, or any DID URL component contains whitespace.
    pub fn parse_with_core_error(input: &str) -> IdentusResult<Self> {
        Self::parse(input).map_err(Into::into)
    }

    /// Borrow the canonical DID URL string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Borrow the DID part.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the optional path, without leading slash normalization.
    #[must_use]
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Borrow the optional query, without the leading `?`.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// Borrow the optional fragment, without the leading `#`.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }
}

impl Display for DidUrl {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DidUrl {
    type Err = DidParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// DID parsing error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DidParseError {
    /// The input does not start with `did:`.
    MissingDidScheme,
    /// The DID method is missing or invalid.
    InvalidMethod,
    /// The method-specific identifier is missing or invalid.
    InvalidMethodSpecificId,
    /// A plain DID cannot contain path, query, or fragment components.
    DidUrlComponentsNotAllowed,
    /// A DID URL component is malformed.
    InvalidDidUrlComponent,
}

impl DidParseError {
    /// Stable core error code for this parser failure.
    #[must_use]
    pub const fn code(self) -> ErrorCode {
        match self {
            Self::MissingDidScheme => ErrorCode::new("missing_did_scheme"),
            Self::InvalidMethod => ErrorCode::new("invalid_did_method"),
            Self::InvalidMethodSpecificId => ErrorCode::new("invalid_did_method_specific_id"),
            Self::DidUrlComponentsNotAllowed => ErrorCode::new("did_url_components_not_allowed"),
            Self::InvalidDidUrlComponent => ErrorCode::new("invalid_did_url_component"),
        }
    }

    /// Shared core error family for this parser failure.
    #[must_use]
    pub const fn kind(self) -> ErrorKind {
        ErrorKind::InvalidInput
    }

    /// Redaction-safe public error message.
    #[must_use]
    pub const fn public_message(self) -> &'static str {
        match self {
            Self::MissingDidScheme => "DID must start with 'did:'",
            Self::InvalidMethod => "DID method is invalid",
            Self::InvalidMethodSpecificId => "DID method-specific identifier is invalid",
            Self::DidUrlComponentsNotAllowed => "DID URL components are not allowed in a DID",
            Self::InvalidDidUrlComponent => "DID URL component is invalid",
        }
    }

    /// Convert to the shared core error surface.
    #[must_use]
    pub const fn to_identus_error(self) -> IdentusError {
        IdentusError::public(
            self.code(),
            self.kind(),
            CapabilityId::new("did"),
            self.public_message(),
        )
    }

    /// Convert to a binding-safe error envelope.
    #[must_use]
    pub fn to_error_envelope(self) -> ErrorEnvelope {
        ErrorEnvelope::from_error(&self.to_identus_error())
    }
}

impl Display for DidParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.public_message())
    }
}

impl Error for DidParseError {}

impl From<DidParseError> for IdentusError {
    fn from(error: DidParseError) -> Self {
        error.to_identus_error()
    }
}

struct ParsedDidPrefix {
    method: DidMethod,
    remainder: String,
}

struct DidUrlComponents {
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

fn parse_did_prefix(input: &str) -> Result<ParsedDidPrefix, DidParseError> {
    let without_scheme = input
        .strip_prefix("did:")
        .ok_or(DidParseError::MissingDidScheme)?;
    let (method, remainder) = without_scheme
        .split_once(':')
        .ok_or(DidParseError::InvalidMethodSpecificId)?;

    Ok(ParsedDidPrefix {
        method: DidMethod::parse(method)?,
        remainder: remainder.to_owned(),
    })
}

fn parse_url_components(components: &str) -> Result<DidUrlComponents, DidParseError> {
    if components.is_empty() {
        return Ok(DidUrlComponents {
            path: None,
            query: None,
            fragment: None,
        });
    }

    if components.bytes().any(|byte| byte.is_ascii_whitespace())
        || !is_valid_did_component(components, is_did_url_component_byte)
    {
        return Err(DidParseError::InvalidDidUrlComponent);
    }

    let fragment_split = components.split_once('#');
    let (before_fragment, fragment) = match fragment_split {
        Some((before_fragment, fragment)) => (before_fragment, Some(fragment.to_owned())),
        None => (components, None),
    };

    if fragment.as_deref().is_some_and(|value| value.contains('#')) {
        return Err(DidParseError::InvalidDidUrlComponent);
    }

    let query_split = before_fragment.split_once('?');
    let (path, query) = match query_split {
        Some((path, query)) => (optional_non_empty(path), Some(query.to_owned())),
        None => (optional_non_empty(before_fragment), None),
    };

    if query.as_deref().is_some_and(|value| value.contains('?')) {
        return Err(DidParseError::InvalidDidUrlComponent);
    }

    Ok(DidUrlComponents {
        path,
        query,
        fragment,
    })
}

fn optional_non_empty(input: &str) -> Option<String> {
    if input.is_empty() {
        None
    } else {
        Some(input.to_owned())
    }
}

fn is_method_specific_id_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':' | b'%')
}

fn is_did_url_component_byte(byte: u8) -> bool {
    is_method_specific_id_byte(byte)
        || matches!(
            byte,
            b'/' | b'?' | b'#' | b'&' | b'=' | b'+' | b',' | b';' | b'@' | b'~'
        )
}

fn is_valid_did_component(input: &str, allowed: fn(u8) -> bool) -> bool {
    let bytes = input.as_bytes();
    let mut index = 0;

    while let Some(byte) = bytes.get(index).copied() {
        if !allowed(byte) {
            return false;
        }

        if byte == b'%' {
            let Some(first) = bytes.get(index + 1).copied() else {
                return false;
            };
            let Some(second) = bytes.get(index + 2).copied() else {
                return false;
            };

            if !first.is_ascii_hexdigit() || !second.is_ascii_hexdigit() {
                return false;
            }

            index += 3;
        } else {
            index += 1;
        }
    }

    true
}

fn is_did_url_separator(byte: u8) -> bool {
    matches!(byte, b'/' | b'?' | b'#')
}

fn is_did_url_separator_char(character: char) -> bool {
    matches!(character, '/' | '?' | '#')
}

#[cfg(test)]
mod tests {
    use super::{
        DID_METHOD_PROFILES, Did, DidMethod, DidMethodKind, DidMethodSupport, DidParseError, DidUrl,
    };
    use identus_core::{CapabilityId, ErrorKind};

    #[test]
    fn parses_product_did_methods() {
        let prism: Did =
            "did:prism:9bf36a6dd4090ad66e359a0c041e25662c3f84c00467e9a61eeba68477c8a595"
                .parse()
                .expect("PRISM DID should parse");
        let peer: Did = "did:peer:2.Ez6LSghwSE437wnDE1pt3X6hVDUQzSjsHzinpX3XFvMjRAm7y"
            .parse()
            .expect("peer DID should parse");

        assert_eq!(prism.method().as_str(), "prism");
        assert_eq!(prism.method_kind(), DidMethodKind::Prism);
        assert_eq!(prism.method_support(), DidMethodSupport::Product);
        assert!(prism.is_identus_product_method());
        assert_eq!(peer.method_kind(), DidMethodKind::Peer);
        assert!(peer.is_identus_product_method());
    }

    #[test]
    fn classifies_extension_and_fixture_methods() {
        for (method, expected_kind, expected_support) in [
            ("web", DidMethodKind::Web, DidMethodSupport::Extension),
            ("key", DidMethodKind::Key, DidMethodSupport::Extension),
            ("jwk", DidMethodKind::Jwk, DidMethodSupport::Extension),
            ("pkh", DidMethodKind::Pkh, DidMethodSupport::Extension),
            (
                "example",
                DidMethodKind::Example,
                DidMethodSupport::FixtureOnly,
            ),
            ("cheqd", DidMethodKind::Other, DidMethodSupport::Unknown),
        ] {
            let parsed = DidMethod::parse(method).expect("method should parse");

            assert_eq!(parsed.kind(), expected_kind);
            assert_eq!(parsed.support(), expected_support);
            assert!(!parsed.kind().is_identus_product_method());
        }
    }

    #[test]
    fn did_method_profiles_cover_inventory() {
        let methods = DID_METHOD_PROFILES
            .iter()
            .map(|profile| profile.method)
            .collect::<std::collections::BTreeSet<_>>();

        for expected_method in ["prism", "peer", "web", "key", "jwk", "pkh", "example"] {
            assert!(methods.contains(expected_method), "{expected_method}");
        }
    }

    #[test]
    fn parses_did_url_components_without_erasing_the_did() {
        let did_url = DidUrl::parse("did:web:issuer.example:tenant/path/to/key?version=1#keys-1")
            .expect("DID URL should parse");

        assert_eq!(did_url.did().method_kind(), DidMethodKind::Web);
        assert_eq!(did_url.did().as_str(), "did:web:issuer.example:tenant");
        assert_eq!(did_url.path(), Some("/path/to/key"));
        assert_eq!(did_url.query(), Some("version=1"));
        assert_eq!(did_url.fragment(), Some("keys-1"));
        assert_eq!(
            did_url.as_str(),
            "did:web:issuer.example:tenant/path/to/key?version=1#keys-1"
        );
    }

    #[test]
    fn rejects_malformed_dids() {
        for (input, expected_error) in [
            ("prism:abc", DidParseError::MissingDidScheme),
            ("did:PRISM:abc", DidParseError::InvalidMethod),
            ("did:prism:", DidParseError::InvalidMethodSpecificId),
            ("did:prism:abc def", DidParseError::InvalidMethodSpecificId),
            ("did:prism:abc%zz", DidParseError::InvalidMethodSpecificId),
            (
                "did:prism:abc#key-1",
                DidParseError::DidUrlComponentsNotAllowed,
            ),
        ] {
            assert_eq!(Did::parse(input), Err(expected_error), "{input}");
        }
    }

    #[test]
    fn rejects_malformed_did_urls() {
        for input in [
            "did:example:abc/path with spaces",
            "did:example:abc?one?two",
            "did:example:abc#one#two",
        ] {
            assert_eq!(
                DidUrl::parse(input),
                Err(DidParseError::InvalidDidUrlComponent),
                "{input}"
            );
        }
    }

    #[test]
    fn did_parse_errors_map_to_core_error_surface() {
        let parse_error = DidParseError::InvalidMethod;
        let core_error = parse_error.to_identus_error();

        assert_eq!(core_error.code().as_str(), "invalid_did_method");
        assert_eq!(core_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            core_error.capability().map(CapabilityId::as_str),
            Some("did")
        );
        assert_eq!(core_error.public_message(), "DID method is invalid");
        assert_eq!(
            core_error.to_string(),
            "invalid_did_method: DID method is invalid"
        );

        let envelope = parse_error.to_error_envelope();
        assert_eq!(envelope.code, "invalid_did_method");
        assert_eq!(envelope.kind, ErrorKind::InvalidInput);
        assert_eq!(envelope.capability, Some("did"));
        assert_eq!(envelope.message, "DID method is invalid");
    }

    #[test]
    fn did_parse_with_core_error_preserves_typed_codes() {
        let error = Did::parse_with_core_error("did:PRISM:abc").expect_err("DID must fail");
        assert_eq!(error.code().as_str(), "invalid_did_method");

        let url_error = DidUrl::parse_with_core_error("did:example:abc/path with spaces")
            .expect_err("DID URL must fail");
        assert_eq!(url_error.code().as_str(), "invalid_did_url_component");
    }
}

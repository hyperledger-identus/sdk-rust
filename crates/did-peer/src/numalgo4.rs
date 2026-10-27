//! Current peer DID numalgo-4 short/long-form profile.

use bs58::{decode, encode};
use identus_did::{
    Did, DidDocument, JsonWireError, JsonWireLimits, MAX_DID_DOCUMENT_BYTES,
    validate_unique_object_names,
};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::fmt;

use crate::Error;

const PREFIX: &str = "did:peer:4";
const JSON_MULTICODEC: [u8; 2] = [0x80, 0x04];
const SHA2_256_MULTIHASH: [u8; 2] = [0x12, 0x20];
const MULTIHASH_BYTES: usize = 34;
const MAX_ENCODED_MULTIHASH_BYTES: usize = 48;
const PRIVATE_JWK_MEMBERS: &[&str] = &["d", "p", "q", "dp", "dq", "qi", "oth", "k"];

/// Maximum decoded JSON input-document bytes accepted by this profile.
///
/// This intentionally stays well below the generic DID ceiling because
/// base58btc conversion is super-linear for large inputs.
pub const MAX_INPUT_DOCUMENT_BYTES: usize = 16 * 1_024;
/// Maximum encoded input-document characters accepted before base decoding.
pub const MAX_ENCODED_DOCUMENT_BYTES: usize = 24 * 1_024;
/// Maximum total characters accepted by the long-form parser.
pub const MAX_LONG_FORM_BYTES: usize =
    PREFIX.len() + MAX_ENCODED_MULTIHASH_BYTES + 1 + MAX_ENCODED_DOCUMENT_BYTES;
/// Maximum total characters accepted by the short-form parser.
pub const MAX_SHORT_FORM_BYTES: usize = PREFIX.len() + MAX_ENCODED_MULTIHASH_BYTES;
/// Maximum JSON nesting depth accepted by the input-document parser.
pub const MAX_INPUT_DOCUMENT_DEPTH: usize = 32;
/// Maximum aggregate JSON values accepted by the input-document parser.
pub const MAX_INPUT_DOCUMENT_NODES: usize = 4_096;
/// Maximum members accepted in any one input-document object.
pub const MAX_INPUT_OBJECT_MEMBERS: usize = 128;
/// Maximum aggregate live object-key bytes retained while scanning JSON.
pub const MAX_INPUT_LIVE_KEY_BYTES: usize = 64 * 1_024;

/// A validated numalgo-4 construction input without a root `id`.
#[derive(Clone, PartialEq, Eq)]
pub struct InputDocument {
    value: Value,
    compact: Vec<u8>,
}

impl InputDocument {
    /// Parse, bound and validate a construction input document.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        if input.len() > MAX_INPUT_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        validate_unique_object_names(
            input,
            JsonWireLimits {
                max_depth: MAX_INPUT_DOCUMENT_DEPTH,
                max_nodes: MAX_INPUT_DOCUMENT_NODES,
                max_object_members: MAX_INPUT_OBJECT_MEMBERS,
                max_live_key_bytes: MAX_INPUT_LIVE_KEY_BYTES,
            },
        )
        .map_err(map_json_wire_error)?;
        let value: Value =
            serde_json::from_slice(input).map_err(|_| Error::InvalidInputDocument)?;
        let object = value.as_object().ok_or(Error::InvalidInputDocument)?;
        if object.contains_key("id") || object.len() > 128 {
            return Err(Error::InvalidInputDocument);
        }
        reject_secret_members(&value)?;
        let compact = serde_json::to_vec(&value).map_err(|_| Error::InvalidInputDocument)?;
        if compact.len() > MAX_INPUT_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        // Validate the final generic document shape before retaining input.
        let placeholder = Did::parse("did:peer:4zQmYwAPJzv5CZsnAzt8auVZRnGiHZQ6ZMHTQm14iFe9QqF")
            .map_err(|_| Error::Projection)?;
        contextualize(&value, &placeholder, None)?;
        Ok(Self { value, compact })
    }

    /// Parse a JSON string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the exact validated representation hashed during creation.
    ///
    /// Native construction canonicalizes JSON to compact form. Parsing a long
    /// form retains its original decoded bytes so recreation preserves the DID.
    #[must_use]
    pub fn compact_json(&self) -> &[u8] {
        &self.compact
    }
}

impl fmt::Debug for InputDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputDocument")
            .field("compact_bytes", &self.compact.len())
            .finish_non_exhaustive()
    }
}

const fn map_json_wire_error(error: JsonWireError) -> Error {
    match error {
        JsonWireError::DuplicateName | JsonWireError::Malformed => Error::InvalidInputDocument,
        JsonWireError::TooDeep
        | JsonWireError::TooManyNodes
        | JsonWireError::TooManyMembers
        | JsonWireError::TooManyLiveKeyBytes => Error::ResourceLimit,
        _ => Error::InvalidInputDocument,
    }
}

/// A validated numalgo-4 long form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongForm {
    did: Did,
    short: Did,
    input: InputDocument,
}

impl LongForm {
    /// Create a long form from a validated input document.
    pub fn create(input: InputDocument) -> Result<Self, Error> {
        let mut document_bytes = Vec::with_capacity(JSON_MULTICODEC.len() + input.compact.len());
        document_bytes.extend_from_slice(&JSON_MULTICODEC);
        document_bytes.extend_from_slice(&input.compact);
        let encoded_document = format!("z{}", encode(document_bytes).into_string());
        if encoded_document.len() > MAX_ENCODED_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }

        let digest = Sha256::digest(encoded_document.as_bytes());
        let mut multihash = Vec::with_capacity(MULTIHASH_BYTES);
        multihash.extend_from_slice(&SHA2_256_MULTIHASH);
        multihash.extend_from_slice(&digest);
        let hash = format!("z{}", encode(multihash).into_string());
        let short = Did::parse(&format!("{PREFIX}{hash}")).map_err(|_| Error::Projection)?;
        let did = Did::parse(&format!("{PREFIX}{hash}:{encoded_document}"))
            .map_err(|_| Error::ResourceLimit)?;
        let long = Self { did, short, input };
        long.resolve()?;
        ShortForm(long.short.clone()).resolve_with(&long)?;
        Ok(long)
    }

    /// Parse and verify a numalgo-4 long form.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if value.len() > MAX_LONG_FORM_BYTES {
            return Err(Error::ResourceLimit);
        }
        if !value.starts_with(PREFIX) {
            return classify_wrong_profile(value);
        }
        let remainder = &value[PREFIX.len()..];
        let (hash, encoded_document) = remainder.split_once(':').ok_or(Error::WrongProfile)?;
        if hash.contains(':') || encoded_document.contains(':') {
            return Err(Error::MalformedEncoding);
        }
        if encoded_document.len() > MAX_ENCODED_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        validate_hash(hash, encoded_document)?;
        let decoded = decode_multibase(encoded_document, MAX_INPUT_DOCUMENT_BYTES + 2)?;
        if !decoded.starts_with(&JSON_MULTICODEC) {
            return Err(Error::UnsupportedCodec);
        }
        let document_bytes = &decoded[JSON_MULTICODEC.len()..];
        let mut input = InputDocument::from_json_slice(document_bytes)?;
        input.compact = document_bytes.to_vec();
        let did = Did::parse(value).map_err(|_| Error::MalformedEncoding)?;
        let short = Did::parse(&format!("{PREFIX}{hash}")).map_err(|_| Error::MalformedEncoding)?;
        let long = Self { did, short, input };
        long.resolve()?;
        ShortForm(long.short.clone()).resolve_with(&long)?;
        Ok(long)
    }

    /// Borrow the long-form DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the derived short-form DID.
    #[must_use]
    pub const fn short_form(&self) -> &Did {
        &self.short
    }

    /// Borrow the decoded construction input.
    #[must_use]
    pub const fn input_document(&self) -> &InputDocument {
        &self.input
    }

    /// Resolve the long form locally.
    pub fn resolve(&self) -> Result<DidDocument, Error> {
        contextualize(&self.input.value, &self.did, Some(&self.short))
    }
}

/// A validated numalgo-4 short form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortForm(Did);

impl ShortForm {
    /// Parse a short form without performing resolution I/O.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if value.len() > MAX_SHORT_FORM_BYTES {
            return Err(Error::ResourceLimit);
        }
        if !value.starts_with(PREFIX) {
            return classify_wrong_profile(value);
        }
        let hash = &value[PREFIX.len()..];
        if hash.is_empty() || hash.contains(':') {
            return Err(Error::WrongProfile);
        }
        validate_multihash(hash)?;
        let did = Did::parse(value).map_err(|_| Error::MalformedEncoding)?;
        Ok(Self(did))
    }

    /// Borrow the short-form DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.0
    }

    /// Reject effect-free resolution without matching long-form material.
    pub const fn resolve(&self) -> Result<DidDocument, Error> {
        Err(Error::LongFormRequired)
    }

    /// Resolve using caller-supplied matching long-form material.
    pub fn resolve_with(&self, long: &LongForm) -> Result<DidDocument, Error> {
        if self.0 != long.short {
            return Err(Error::HashMismatch);
        }
        contextualize(&long.input.value, &self.0, Some(&long.did))
    }
}

fn validate_hash(hash: &str, encoded_document: &str) -> Result<(), Error> {
    let claimed = validate_multihash(hash)?;
    let actual = Sha256::digest(encoded_document.as_bytes());
    let mut difference = 0_u8;
    for (left, right) in claimed.iter().zip(actual.iter()) {
        difference |= left ^ right;
    }
    if difference == 0 {
        Ok(())
    } else {
        Err(Error::HashMismatch)
    }
}

fn validate_multihash(hash: &str) -> Result<[u8; 32], Error> {
    if hash.len() > MAX_ENCODED_MULTIHASH_BYTES {
        return Err(Error::ResourceLimit);
    }
    let decoded = decode_multibase(hash, MULTIHASH_BYTES)?;
    if decoded.len() != MULTIHASH_BYTES || !decoded.starts_with(&SHA2_256_MULTIHASH) {
        return Err(Error::UnsupportedCodec);
    }
    let mut digest = [0_u8; 32];
    digest.copy_from_slice(&decoded[2..]);
    Ok(digest)
}

fn decode_multibase(value: &str, max_decoded: usize) -> Result<Vec<u8>, Error> {
    let payload = value.strip_prefix('z').ok_or(Error::MalformedEncoding)?;
    if payload.is_empty() || payload.len() > MAX_ENCODED_DOCUMENT_BYTES {
        return Err(Error::ResourceLimit);
    }
    let decoded = decode(payload)
        .into_vec()
        .map_err(|_| Error::MalformedEncoding)?;
    if decoded.len() > max_decoded {
        return Err(Error::ResourceLimit);
    }
    Ok(decoded)
}

fn contextualize(
    input: &Value,
    subject: &Did,
    counterpart: Option<&Did>,
) -> Result<DidDocument, Error> {
    preflight_projection(input, subject, counterpart)?;
    let mut object = input
        .as_object()
        .cloned()
        .ok_or(Error::InvalidInputDocument)?;
    object.insert("id".into(), Value::String(subject.to_string()));
    contextualize_methods(&mut object, subject)?;
    contextualize_services(&mut object, subject)?;
    if let Some(counterpart) = counterpart {
        let aliases = object
            .entry("alsoKnownAs")
            .or_insert_with(|| Value::Array(Vec::new()))
            .as_array_mut()
            .ok_or(Error::InvalidInputDocument)?;
        if aliases.len() >= 128 {
            return Err(Error::ResourceLimit);
        }
        aliases.push(Value::String(counterpart.to_string()));
    }
    let bytes = serde_json::to_vec(&Value::Object(object)).map_err(|_| Error::Projection)?;
    DidDocument::from_json_slice(&bytes).map_err(|_| Error::Projection)
}

fn preflight_projection(
    input: &Value,
    subject: &Did,
    counterpart: Option<&Did>,
) -> Result<(), Error> {
    let mut projected_bytes = serde_json::to_vec(input)
        .map_err(|_| Error::Projection)?
        .len()
        .checked_add(subject.as_str().len() + 8)
        .ok_or(Error::ResourceLimit)?;

    if let Some(methods) = input.get("verificationMethod") {
        projected_bytes = projected_bytes
            .checked_add(projection_method_growth(methods, subject, false)?)
            .ok_or(Error::ResourceLimit)?;
    }
    for name in [
        "authentication",
        "assertionMethod",
        "keyAgreement",
        "capabilityInvocation",
        "capabilityDelegation",
    ] {
        if let Some(values) = input.get(name) {
            projected_bytes = projected_bytes
                .checked_add(projection_method_growth(values, subject, true)?)
                .ok_or(Error::ResourceLimit)?;
        }
    }
    if let Some(services) = input.get("service") {
        let services = services.as_array().ok_or(Error::InvalidInputDocument)?;
        if services.len() > 128 {
            return Err(Error::ResourceLimit);
        }
        for service in services {
            let fragment = service
                .as_object()
                .and_then(|service| service.get("id"))
                .and_then(Value::as_str)
                .ok_or(Error::InvalidInputDocument)?;
            validate_projection_fragment(fragment)?;
            projected_bytes = projected_bytes
                .checked_add(subject.as_str().len())
                .ok_or(Error::ResourceLimit)?;
        }
    }
    if let Some(counterpart) = counterpart {
        let alias_growth = match input.get("alsoKnownAs") {
            Some(Value::Array(values)) if values.len() >= 128 => return Err(Error::ResourceLimit),
            Some(Value::Array(values)) if values.is_empty() => counterpart.as_str().len() + 2,
            Some(Value::Array(_)) => counterpart.as_str().len() + 3,
            Some(_) => return Err(Error::InvalidInputDocument),
            None => counterpart.as_str().len() + 19,
        };
        projected_bytes = projected_bytes
            .checked_add(alias_growth)
            .ok_or(Error::ResourceLimit)?;
    }
    if projected_bytes > MAX_DID_DOCUMENT_BYTES {
        return Err(Error::ResourceLimit);
    }
    Ok(())
}

fn projection_method_growth(
    value: &Value,
    subject: &Did,
    allow_references: bool,
) -> Result<usize, Error> {
    let values = value.as_array().ok_or(Error::InvalidInputDocument)?;
    if values.len() > 128 {
        return Err(Error::ResourceLimit);
    }
    let mut growth = 0_usize;
    for value in values {
        if allow_references && let Some(fragment) = value.as_str() {
            validate_projection_fragment(fragment)?;
            growth = growth
                .checked_add(subject.as_str().len())
                .ok_or(Error::ResourceLimit)?;
            continue;
        }
        let method = value.as_object().ok_or(Error::InvalidInputDocument)?;
        let fragment = method
            .get("id")
            .and_then(Value::as_str)
            .ok_or(Error::InvalidInputDocument)?;
        validate_projection_fragment(fragment)?;
        growth = growth
            .checked_add(subject.as_str().len())
            .ok_or(Error::ResourceLimit)?;
        match method.get("controller") {
            None => {
                growth = growth
                    .checked_add(subject.as_str().len() + 16)
                    .ok_or(Error::ResourceLimit)?;
            }
            Some(Value::String(controller)) if controller.starts_with('#') => {
                validate_projection_fragment(controller)?;
                growth = growth
                    .checked_add(subject.as_str().len())
                    .ok_or(Error::ResourceLimit)?;
            }
            Some(_) => {}
        }
    }
    Ok(growth)
}

fn validate_projection_fragment(fragment: &str) -> Result<(), Error> {
    if fragment.starts_with('#') && fragment.len() >= 2 {
        Ok(())
    } else {
        Err(Error::InvalidInputDocument)
    }
}

fn reject_secret_members(value: &Value) -> Result<(), Error> {
    reject_secret_members_in_context(value, false)
}

fn reject_secret_members_in_context(value: &Value, jwk_context: bool) -> Result<(), Error> {
    match value {
        Value::Object(object) => {
            if (jwk_context || object.contains_key("kty"))
                && PRIVATE_JWK_MEMBERS
                    .iter()
                    .any(|member| object.contains_key(*member))
            {
                return Err(Error::InvalidInputDocument);
            }
            for (name, value) in object {
                let normalized: String = name
                    .chars()
                    .filter(|character| character.is_ascii_alphanumeric())
                    .flat_map(char::to_lowercase)
                    .collect();
                if normalized.contains("privatekey")
                    || normalized.contains("secretkey")
                    || normalized == "secret"
                    || normalized.contains("seed")
                    || normalized.contains("mnemonic")
                {
                    return Err(Error::InvalidInputDocument);
                }
                reject_secret_members_in_context(value, normalized.ends_with("jwk"))?;
            }
        }
        Value::Array(values) => {
            for value in values {
                reject_secret_members_in_context(value, false)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}

fn contextualize_methods(object: &mut Map<String, Value>, subject: &Did) -> Result<(), Error> {
    if let Some(methods) = object.get_mut("verificationMethod") {
        contextualize_method_array(methods, subject, false)?;
    }
    for name in [
        "authentication",
        "assertionMethod",
        "keyAgreement",
        "capabilityInvocation",
        "capabilityDelegation",
    ] {
        if let Some(values) = object.get_mut(name) {
            contextualize_method_array(values, subject, true)?;
        }
    }
    Ok(())
}

fn contextualize_method_array(
    value: &mut Value,
    subject: &Did,
    allow_references: bool,
) -> Result<(), Error> {
    let values = value.as_array_mut().ok_or(Error::InvalidInputDocument)?;
    if values.len() > 128 {
        return Err(Error::ResourceLimit);
    }
    for value in values {
        if allow_references && value.is_string() {
            contextualize_fragment(value, subject)?;
            continue;
        }
        let method = value.as_object_mut().ok_or(Error::InvalidInputDocument)?;
        let id = method.get_mut("id").ok_or(Error::InvalidInputDocument)?;
        contextualize_fragment(id, subject)?;
        match method.get_mut("controller") {
            None => {
                method.insert("controller".into(), Value::String(subject.to_string()));
            }
            Some(controller) => {
                if controller
                    .as_str()
                    .is_some_and(|value| value.starts_with('#'))
                {
                    contextualize_fragment(controller, subject)?;
                }
            }
        }
    }
    Ok(())
}

fn contextualize_services(object: &mut Map<String, Value>, subject: &Did) -> Result<(), Error> {
    let Some(services) = object.get_mut("service") else {
        return Ok(());
    };
    let services = services.as_array_mut().ok_or(Error::InvalidInputDocument)?;
    if services.len() > 128 {
        return Err(Error::ResourceLimit);
    }
    for service in services {
        let service = service.as_object_mut().ok_or(Error::InvalidInputDocument)?;
        contextualize_fragment(
            service.get_mut("id").ok_or(Error::InvalidInputDocument)?,
            subject,
        )?;
    }
    Ok(())
}

fn contextualize_fragment(value: &mut Value, subject: &Did) -> Result<(), Error> {
    let fragment = value.as_str().ok_or(Error::InvalidInputDocument)?;
    if !fragment.starts_with('#') || fragment.len() < 2 {
        return Err(Error::InvalidInputDocument);
    }
    *value = Value::String(format!("{subject}{fragment}"));
    Ok(())
}

fn classify_wrong_profile<T>(value: &str) -> Result<T, Error> {
    if value.starts_with("did:peer:") {
        Err(Error::UnsupportedAlgorithm)
    } else {
        Err(Error::WrongProfile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"{"verificationMethod":[{"id":"#key-1","type":"Multikey","publicKeyMultibase":"z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv"}],"authentication":["#key-1"],"service":[{"id":"#messages","type":"DIDCommMessaging","serviceEndpoint":"https://example.com/didcomm"}]}"##;

    #[test]
    fn creates_parses_and_resolves_long_and_short_forms() {
        let input = InputDocument::from_json_str(INPUT).expect("valid input");
        let long = LongForm::create(input).expect("long form");
        let reparsed = LongForm::parse(long.did().as_str()).expect("parse long");
        assert_eq!(reparsed, long);
        assert_eq!(long.resolve().expect("resolve long").id(), long.did());

        let short = ShortForm::parse(long.short_form().as_str()).expect("short");
        assert_eq!(
            short.resolve_with(&long).expect("resolve short").id(),
            short.did()
        );
    }

    #[test]
    fn matches_reference_implementation_minimal_vector() {
        let input = InputDocument::from_json_str(r#"{"hello":"world"}"#).expect("valid input");
        let long = LongForm::create(input).expect("long form");
        assert_eq!(
            long.did().as_str(),
            "did:peer:4zQmb7xLdVY9TXx8oov5XgpGUmGELgqiAV2699s43i6Qdm3M:zQSJgiFTYiCHjQ9MktwNThRXM7a"
        );
    }

    #[test]
    fn parsed_noncanonical_json_recreates_the_same_long_form() {
        let document_bytes = br#"{ "second" : 2, "first" : 1 }"#;
        let mut encoded_bytes = JSON_MULTICODEC.to_vec();
        encoded_bytes.extend_from_slice(document_bytes);
        let encoded_document = format!("z{}", encode(encoded_bytes).into_string());
        let digest = Sha256::digest(encoded_document.as_bytes());
        let mut multihash = SHA2_256_MULTIHASH.to_vec();
        multihash.extend_from_slice(&digest);
        let hash = format!("z{}", encode(multihash).into_string());
        let original = format!("{PREFIX}{hash}:{encoded_document}");

        let parsed = LongForm::parse(&original).expect("parse noncanonical input");
        assert_eq!(parsed.input_document().compact_json(), document_bytes);
        assert_eq!(
            LongForm::create(parsed.input_document().clone())
                .expect("recreate")
                .did()
                .as_str(),
            original
        );
    }

    #[test]
    fn rejects_tampered_document_and_cross_profile_inputs() {
        let input = InputDocument::from_json_str(INPUT).expect("valid input");
        let long = LongForm::create(input).expect("long form");
        let mut tampered = long.did().to_string();
        let last = tampered.pop().expect("last character");
        tampered.push(if last == '1' { '2' } else { '1' });
        assert_eq!(LongForm::parse(&tampered), Err(Error::HashMismatch));
        assert_eq!(
            ShortForm::parse("did:peer:2.Vz6Mkabc"),
            Err(Error::UnsupportedAlgorithm)
        );
    }

    #[test]
    fn rejects_oversized_hash_before_base58_decoding() {
        let oversized = format!("{PREFIX}z{}", "1".repeat(MAX_ENCODED_MULTIHASH_BYTES));
        assert_eq!(ShortForm::parse(&oversized), Err(Error::ResourceLimit));
    }

    #[test]
    fn rejects_oversized_raw_forms_before_delimiter_scans() {
        let oversized_long = format!("{PREFIX}{}", "1".repeat(MAX_LONG_FORM_BYTES));
        assert_eq!(LongForm::parse(&oversized_long), Err(Error::ResourceLimit));
        let oversized_short = format!("{PREFIX}{}", "1".repeat(MAX_SHORT_FORM_BYTES));
        assert_eq!(
            ShortForm::parse(&oversized_short),
            Err(Error::ResourceLimit)
        );
    }

    #[test]
    fn rejects_root_id_and_oversized_input() {
        assert_eq!(
            InputDocument::from_json_str(r#"{"id":"did:example:123"}"#),
            Err(Error::InvalidInputDocument)
        );
        assert_eq!(
            InputDocument::from_json_slice(&vec![b' '; MAX_INPUT_DOCUMENT_BYTES + 1]),
            Err(Error::ResourceLimit)
        );
        assert_eq!(
            InputDocument::from_json_str(r#"{"claim":1,"claim":2}"#),
            Err(Error::InvalidInputDocument)
        );
    }

    #[test]
    fn rejects_secret_shaped_members_at_any_depth() {
        for input in [
            r#"{"privateKey":"secret"}"#,
            r#"{"extension":{"secret_key":"secret"}}"#,
            r#"{"extension":[{"mnemonic":"words"}]}"#,
            r#"{"extension":{"jwk":{"kty":"oct","k":"c2VjcmV0"}}}"#,
            r#"{"extension":{"jwk":{"d":"private-scalar"}}}"#,
            r#"{"extension":{"publicKeyJwk":{"d":"private-scalar"}}}"#,
        ] {
            assert_eq!(
                InputDocument::from_json_str(input),
                Err(Error::InvalidInputDocument)
            );
        }
    }

    #[test]
    fn rejects_input_that_only_overflows_after_counterpart_alias() {
        let aliases = (0..128)
            .map(|index| format!(r#""https://example.com/{index}""#))
            .collect::<Vec<_>>()
            .join(",");
        let input = format!(r#"{{"alsoKnownAs":[{aliases}]}}"#);
        let input = InputDocument::from_json_str(&input).expect("input is valid before projection");
        assert_eq!(LongForm::create(input), Err(Error::ResourceLimit));
    }

    #[test]
    fn rejects_projection_growth_before_expanding_fragments() {
        let references = (0..16)
            .map(|index| format!("\"#key-{index}\""))
            .collect::<Vec<_>>()
            .join(",");
        let input =
            serde_json::from_str::<Value>(&format!(r#"{{"authentication":[{references}]}}"#))
                .expect("input");
        let oversized_subject = Did::parse(&format!("did:peer:4{}", "a".repeat(20_000)))
            .expect("syntactically valid DID");
        assert_eq!(
            contextualize(&input, &oversized_subject, None),
            Err(Error::ResourceLimit)
        );
    }

    #[test]
    fn property_generated_input_sizes_preserve_short_long_linkage() {
        for size in 0..=512 {
            let input = format!(r#"{{"claim":"{}"}}"#, "a".repeat(size));
            let long = LongForm::create(InputDocument::from_json_str(&input).expect("input"))
                .expect("create");
            let parsed = LongForm::parse(long.did().as_str()).expect("parse");
            assert_eq!(parsed.short_form(), long.short_form());
            assert_eq!(parsed.resolve().expect("resolve").id(), parsed.did());
        }
    }

    #[test]
    fn exact_maximum_input_round_trips_through_long_form() {
        let overhead = r#"{"claim":["",""]}"#.len();
        let content = MAX_INPUT_DOCUMENT_BYTES - overhead;
        let source = format!(
            r#"{{"claim":["{}","{}"]}}"#,
            "a".repeat(content / 2),
            "b".repeat(content - content / 2)
        );
        assert_eq!(source.len(), MAX_INPUT_DOCUMENT_BYTES);
        let long = LongForm::create(InputDocument::from_json_str(&source).expect("input"))
            .expect("create exact maximum");
        assert_eq!(
            LongForm::parse(long.did().as_str()).expect("parse exact maximum"),
            long
        );
    }
}

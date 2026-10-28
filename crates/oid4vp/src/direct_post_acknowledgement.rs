//! Request-bound parsing of direct-post endpoint acknowledgements.

use std::{collections::BTreeSet, fmt};

use fluent_uri::Uri as ParsedUri;
use identus_openid4vc::http_field::{has_bare_no_store, is_application_json};
use serde::de::{DeserializeSeed, IgnoredAny, MapAccess, Visitor};
use zeroize::Zeroizing;

use crate::{
    DirectPostAcknowledgementLimits, DirectPostResponse, EncryptedDirectPostResponse, Oid4vpError,
};

/// Parsed result of one successful direct-post endpoint acknowledgement.
#[derive(Debug)]
#[non_exhaustive]
pub enum DirectPostAcknowledgement {
    /// The verifier returned no post-submission redirect.
    Complete,
    /// The verifier returned inert redirect material for caller policy.
    Redirect(DirectPostNavigationCandidate),
}

/// Bounded, inert redirect candidate returned by a direct-post endpoint.
pub struct DirectPostNavigationCandidate {
    redirect_uri: Zeroizing<String>,
    response_code: Option<Zeroizing<String>>,
}

impl DirectPostNavigationCandidate {
    /// Explicitly reveal the untrusted absolute redirect URI.
    ///
    /// Syntax and request lineage do not authorize navigation.
    pub fn expose_sensitive_redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Explicitly reveal the unique opaque response code, when present.
    ///
    /// The SDK makes no entropy, redemption, or replay claim about this value.
    pub fn expose_sensitive_response_code(&self) -> Option<&str> {
        self.response_code.as_ref().map(|value| value.as_str())
    }
}

impl fmt::Debug for DirectPostNavigationCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostNavigationCandidate")
            .field("redirect_uri_bytes", &self.redirect_uri.len())
            .field(
                "response_code_bytes",
                &self.response_code.as_ref().map(|value| value.len()),
            )
            .finish_non_exhaustive()
    }
}

impl DirectPostResponse {
    /// Consume sent response material and bind its endpoint acknowledgement.
    ///
    /// Callers must invoke this only for the HTTP response to the exact request
    /// represented by `self`. This method performs no HTTP or navigation.
    pub fn bind_endpoint_acknowledgement(
        self,
        status: u16,
        content_type: Option<&str>,
        cache_control: Option<&str>,
        body: &[u8],
        limits: DirectPostAcknowledgementLimits,
    ) -> Result<DirectPostAcknowledgement, Oid4vpError> {
        parse_acknowledgement(status, content_type, cache_control, body, limits)
    }
}

impl EncryptedDirectPostResponse {
    /// Consume sent encrypted response material and bind its acknowledgement.
    pub fn bind_endpoint_acknowledgement(
        self,
        status: u16,
        content_type: Option<&str>,
        cache_control: Option<&str>,
        body: &[u8],
        limits: DirectPostAcknowledgementLimits,
    ) -> Result<DirectPostAcknowledgement, Oid4vpError> {
        parse_acknowledgement(status, content_type, cache_control, body, limits)
    }
}

fn parse_acknowledgement(
    status: u16,
    content_type: Option<&str>,
    cache_control: Option<&str>,
    body: &[u8],
    limits: DirectPostAcknowledgementLimits,
) -> Result<DirectPostAcknowledgement, Oid4vpError> {
    if status != 200 {
        return Err(Oid4vpError::InvalidDirectPostAcknowledgement);
    }
    let content_type = content_type.ok_or(Oid4vpError::InvalidDirectPostAcknowledgement)?;
    if content_type.len() > limits.max_content_type_bytes()
        || !is_application_json(content_type.as_bytes())
    {
        return Err(Oid4vpError::InvalidDirectPostAcknowledgement);
    }
    let cache_control = cache_control.ok_or(Oid4vpError::InvalidDirectPostAcknowledgement)?;
    if cache_control.len() > limits.max_cache_control_bytes()
        || !has_bare_no_store(cache_control.as_bytes())
    {
        return Err(Oid4vpError::InvalidDirectPostAcknowledgement);
    }
    if body.len() > limits.max_body_bytes() {
        return Err(Oid4vpError::ResponseBodyTooLarge);
    }
    let mut deserializer = serde_json::Deserializer::from_slice(body);
    let redirect_uri = AckSeed { limits }
        .deserialize(&mut deserializer)
        .map_err(|_| Oid4vpError::InvalidDirectPostAcknowledgement)?;
    deserializer
        .end()
        .map_err(|_| Oid4vpError::InvalidDirectPostAcknowledgement)?;
    let Some(redirect_uri) = redirect_uri else {
        return Ok(DirectPostAcknowledgement::Complete);
    };
    let response_code = validate_redirect_and_extract_code(&redirect_uri, limits)?;
    Ok(DirectPostAcknowledgement::Redirect(
        DirectPostNavigationCandidate {
            redirect_uri,
            response_code,
        },
    ))
}

struct AckSeed {
    limits: DirectPostAcknowledgementLimits,
}

impl<'de> DeserializeSeed<'de> for AckSeed {
    type Value = Option<Zeroizing<String>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AckVisitor {
            limits: self.limits,
        })
    }
}

struct AckVisitor {
    limits: DirectPostAcknowledgementLimits,
}

impl<'de> Visitor<'de> for AckVisitor {
    type Value = Option<Zeroizing<String>>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded direct-post acknowledgement object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut seen = BTreeSet::new();
        let mut members = 0usize;
        let mut redirect_uri = None;
        while let Some(name) = map.next_key::<String>()? {
            members += 1;
            if members > self.limits.max_members()
                || name.len() > self.limits.max_member_name_bytes()
                || !seen.insert(name.clone())
            {
                return Err(serde::de::Error::custom("bounded acknowledgement member"));
            }
            if name == "redirect_uri" {
                let value = map.next_value::<String>()?;
                if value.is_empty() || value.len() > self.limits.max_redirect_uri_bytes() {
                    return Err(serde::de::Error::custom("bounded redirect URI"));
                }
                redirect_uri = Some(Zeroizing::new(value));
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(redirect_uri)
    }
}

fn validate_redirect_and_extract_code(
    redirect_uri: &str,
    limits: DirectPostAcknowledgementLimits,
) -> Result<Option<Zeroizing<String>>, Oid4vpError> {
    let parsed = ParsedUri::parse(redirect_uri)
        .map_err(|_| Oid4vpError::InvalidDirectPostAcknowledgement)?;
    if parsed.scheme().as_str().is_empty()
        || parsed
            .authority()
            .is_some_and(|authority| authority.userinfo().is_some())
    {
        return Err(Oid4vpError::InvalidDirectPostAcknowledgement);
    }
    let mut response_code = None;
    for component in [
        parsed.query().map(|value| value.as_str()),
        parsed.fragment().map(|value| value.as_str()),
    ] {
        let Some(component) = component else { continue };
        for pair in component.split('&') {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            let name = percent_decode(name)?;
            if name != "response_code" {
                continue;
            }
            if response_code.is_some() {
                return Err(Oid4vpError::AmbiguousResponseCode);
            }
            let value = percent_decode(value)?;
            if value.is_empty() || value.len() > limits.max_response_code_bytes() {
                return Err(Oid4vpError::InvalidDirectPostAcknowledgement);
            }
            response_code = Some(Zeroizing::new(value));
        }
    }
    Ok(response_code)
}

fn percent_decode(value: &str) -> Result<String, Oid4vpError> {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                let high = hex(bytes[index + 1])?;
                let low = hex(bytes[index + 2])?;
                output.push((high << 4) | low);
                index += 3;
            }
            b'%' => return Err(Oid4vpError::InvalidDirectPostAcknowledgement),
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            byte => {
                output.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(output).map_err(|_| Oid4vpError::InvalidDirectPostAcknowledgement)
}

const fn hex(byte: u8) -> Result<u8, Oid4vpError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(Oid4vpError::InvalidDirectPostAcknowledgement),
    }
}

use std::{fmt, str};

use fluent_uri::Uri as ParsedUri;
use zeroize::Zeroizing;

use crate::{AuthorizationRequestInvocationLimits, Oid4vpError};

/// Retrieval method requested for a referenced Request Object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestUriMethod {
    /// Retrieve the Request Object with HTTP GET.
    Get,
    /// Retrieve the Request Object with HTTP POST.
    Post,
}

/// A validated Authorization Request invocation transport.
#[derive(Debug)]
pub enum AuthorizationRequestInvocation {
    /// The invocation identifies a Request Object by HTTPS URI.
    Referenced(ReferencedAuthorizationRequest),
}

impl AuthorizationRequestInvocation {
    /// Parse one complete, bounded Authorization Request invocation.
    pub fn parse(
        input: &str,
        limits: AuthorizationRequestInvocationLimits,
    ) -> Result<Self, Oid4vpError> {
        if input.len() > limits.max_invocation_bytes() {
            return Err(Oid4vpError::InvocationTooLarge);
        }
        let (base, query) = input
            .split_once('?')
            .ok_or(Oid4vpError::InvalidInvocation)?;
        if query.is_empty() || query.contains('#') {
            return Err(Oid4vpError::InvalidInvocation);
        }
        let parsed = ParsedUri::parse(base).map_err(|_| Oid4vpError::InvalidInvocation)?;
        if !parsed.scheme().as_str().eq_ignore_ascii_case("openid4vp")
            || parsed.authority().is_some()
            || !parsed.path().as_str().is_empty()
            || parsed.fragment().is_some()
            || parsed.query().is_some()
        {
            return Err(Oid4vpError::InvalidInvocation);
        }

        parse_reference(query, limits).map(Self::Referenced)
    }
}

/// Least-authority description of a Request Object reference.
pub struct ReferencedAuthorizationRequest {
    client_id: Zeroizing<String>,
    request_uri: Zeroizing<String>,
    method: RequestUriMethod,
}

impl ReferencedAuthorizationRequest {
    /// Explicitly reveal the untrusted, verifier-controlled client identifier.
    pub fn expose_sensitive_client_id(&self) -> &str {
        &self.client_id
    }

    /// Explicitly reveal the untrusted, syntactically validated HTTPS Request URI.
    pub fn expose_sensitive_request_uri(&self) -> &str {
        &self.request_uri
    }

    /// Decoded byte length of the retained client identifier.
    pub fn client_id_len(&self) -> usize {
        self.client_id.len()
    }

    /// Decoded byte length of the retained Request URI.
    pub fn request_uri_len(&self) -> usize {
        self.request_uri.len()
    }

    /// Requested retrieval method; this does not perform transport.
    pub const fn request_uri_method(&self) -> RequestUriMethod {
        self.method
    }
}

impl fmt::Debug for ReferencedAuthorizationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReferencedAuthorizationRequest")
            .field("client_id_len", &self.client_id.len())
            .field("request_uri_len", &self.request_uri.len())
            .field("method", &self.method)
            .finish()
    }
}

fn parse_reference(
    query: &str,
    limits: AuthorizationRequestInvocationLimits,
) -> Result<ReferencedAuthorizationRequest, Oid4vpError> {
    let mut names = Vec::<String>::new();
    let mut client_id = None;
    let mut request_uri = None;
    let mut method = RequestUriMethod::Get;
    let mut inline_transport = false;

    for (index, pair) in query.split('&').enumerate() {
        if index == limits.max_parameter_pairs() {
            return Err(Oid4vpError::TooManyParameters);
        }
        if pair.is_empty() {
            return Err(Oid4vpError::InvalidInvocation);
        }
        let (encoded_name, encoded_value) =
            pair.split_once('=').ok_or(Oid4vpError::InvalidInvocation)?;
        let name = decode_component(
            encoded_name,
            limits.max_parameter_name_bytes(),
            Oid4vpError::ParameterNameTooLarge,
        )?;
        let value = decode_component(
            encoded_value,
            limits.max_parameter_value_bytes(),
            Oid4vpError::ParameterValueTooLarge,
        )?;
        if name.is_empty() || names.iter().any(|previous| previous == name.as_str()) {
            return if name.is_empty() {
                Err(Oid4vpError::InvalidInvocation)
            } else {
                Err(Oid4vpError::DuplicateParameter)
            };
        }
        names.push(name.to_string());

        match name.as_str() {
            "client_id" => {
                if value.is_empty() {
                    return Err(Oid4vpError::MissingRequiredParameter);
                }
                if value.len() > limits.max_client_id_bytes() {
                    return Err(Oid4vpError::ClientIdTooLarge);
                }
                client_id = Some(value);
            }
            "request_uri" => {
                if value.is_empty() {
                    return Err(Oid4vpError::MissingRequiredParameter);
                }
                if value.len() > limits.max_request_uri_bytes() {
                    return Err(Oid4vpError::RequestUriTooLarge);
                }
                validate_request_uri(&value)?;
                request_uri = Some(value);
            }
            "request_uri_method" => {
                method = if value.as_str() == "post" {
                    RequestUriMethod::Post
                } else {
                    return Err(Oid4vpError::UnsupportedRequestUriMethod);
                };
            }
            "request" => return Err(Oid4vpError::UnsupportedTransport),
            "transaction_data" => return Err(Oid4vpError::UnsupportedParameter),
            "response_type" | "dcql_query" | "scope" | "nonce" | "response_mode" => {
                inline_transport = true;
            }
            _ => {}
        }
    }

    let client_id = client_id.ok_or(Oid4vpError::MissingRequiredParameter)?;
    let Some(request_uri) = request_uri else {
        return if inline_transport {
            Err(Oid4vpError::UnsupportedTransport)
        } else {
            Err(Oid4vpError::MissingRequiredParameter)
        };
    };
    Ok(ReferencedAuthorizationRequest {
        client_id,
        request_uri,
        method,
    })
}

fn validate_request_uri(value: &str) -> Result<(), Oid4vpError> {
    let parsed = ParsedUri::parse(value).map_err(|_| Oid4vpError::UnsafeRequestUri)?;
    let authority = parsed.authority().ok_or(Oid4vpError::UnsafeRequestUri)?;
    if !parsed.scheme().as_str().eq_ignore_ascii_case("https")
        || authority.host().is_empty()
        || authority.userinfo().is_some()
        || parsed.fragment().is_some()
    {
        return Err(Oid4vpError::UnsafeRequestUri);
    }
    Ok(())
}

fn decode_component(
    encoded: &str,
    max_decoded_bytes: usize,
    too_large: Oid4vpError,
) -> Result<Zeroizing<String>, Oid4vpError> {
    let input = encoded.as_bytes();
    let mut decoded = Zeroizing::new(Vec::with_capacity(input.len().min(max_decoded_bytes)));
    let mut cursor = 0;

    while cursor < input.len() {
        let byte = match input[cursor] {
            b'+' => {
                cursor += 1;
                b' '
            }
            b'%' => {
                let high = input
                    .get(cursor + 1)
                    .and_then(|value| hex_nibble(*value))
                    .ok_or(Oid4vpError::InvalidFormEncoding)?;
                let low = input
                    .get(cursor + 2)
                    .and_then(|value| hex_nibble(*value))
                    .ok_or(Oid4vpError::InvalidFormEncoding)?;
                cursor += 3;
                (high << 4) | low
            }
            byte if byte.is_ascii() => {
                cursor += 1;
                byte
            }
            _ => return Err(Oid4vpError::InvalidFormEncoding),
        };
        if byte == 0 {
            return Err(Oid4vpError::InvalidFormEncoding);
        }
        if decoded.len() == max_decoded_bytes {
            return Err(too_large);
        }
        decoded.push(byte);
    }

    let decoded = str::from_utf8(&decoded).map_err(|_| Oid4vpError::InvalidFormEncoding)?;
    Ok(Zeroizing::new(decoded.to_owned()))
}

const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

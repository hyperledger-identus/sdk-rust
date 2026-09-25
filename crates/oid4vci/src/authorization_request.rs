use std::{
    collections::BTreeSet,
    fmt,
    io::{self, Write},
};

use fluent_uri::Uri as ParsedUri;
use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, CredentialOfferWithAuthorizationRequestInput,
    form::{append_pair, decode_component, pair_len},
};

/// HTTP method used when dispatching the constructed Authorization Request.
pub const AUTHORIZATION_REQUEST_HTTP_METHOD: &str = "GET";

const RESERVED_QUERY_NAMES: [&str; 12] = [
    "response_type",
    "client_id",
    "redirect_uri",
    "state",
    "code_challenge",
    "code_challenge_method",
    "authorization_details",
    "issuer_state",
    "scope",
    "resource",
    "request",
    "request_uri",
];

/// Positive resource limits for deterministic Authorization Request
/// construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationRequestLimits {
    max_authorization_details_bytes: usize,
    max_endpoint_query_parameters: usize,
    max_endpoint_query_name_bytes: usize,
    max_endpoint_query_value_bytes: usize,
    max_request_uri_bytes: usize,
}

impl AuthorizationRequestLimits {
    /// Construct positive limits for Authorization Details, retained endpoint
    /// query fields and the complete request URI.
    pub const fn new(
        max_authorization_details_bytes: usize,
        max_endpoint_query_parameters: usize,
        max_endpoint_query_name_bytes: usize,
        max_endpoint_query_value_bytes: usize,
        max_request_uri_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_authorization_details_bytes == 0
            || max_endpoint_query_parameters == 0
            || max_endpoint_query_name_bytes == 0
            || max_endpoint_query_value_bytes == 0
            || max_request_uri_bytes == 0
        {
            return Err(CredentialOfferError::InvalidAuthorizationRequestLimits);
        }
        Ok(Self {
            max_authorization_details_bytes,
            max_endpoint_query_parameters,
            max_endpoint_query_name_bytes,
            max_endpoint_query_value_bytes,
            max_request_uri_bytes,
        })
    }

    /// Maximum minified Authorization Details JSON bytes.
    pub const fn max_authorization_details_bytes(self) -> usize {
        self.max_authorization_details_bytes
    }

    /// Maximum existing Authorization Endpoint query parameter count.
    pub const fn max_endpoint_query_parameters(self) -> usize {
        self.max_endpoint_query_parameters
    }

    /// Maximum decoded bytes in one existing endpoint query name.
    pub const fn max_endpoint_query_name_bytes(self) -> usize {
        self.max_endpoint_query_name_bytes
    }

    /// Maximum decoded bytes in one existing endpoint query value.
    pub const fn max_endpoint_query_value_bytes(self) -> usize {
        self.max_endpoint_query_value_bytes
    }

    /// Maximum complete Authorization Request URI bytes.
    pub const fn max_request_uri_bytes(self) -> usize {
        self.max_request_uri_bytes
    }
}

impl Default for AuthorizationRequestLimits {
    fn default() -> Self {
        Self {
            max_authorization_details_bytes: 4_096,
            max_endpoint_query_parameters: 32,
            max_endpoint_query_name_bytes: 256,
            max_endpoint_query_value_bytes: 4_096,
            max_request_uri_bytes: 16_384,
        }
    }
}

/// An owned, deterministic OID4VCI Authorization Request description.
///
/// The exact URI contains caller state and may contain issuer state. It must
/// not be logged, sent to telemetry, cached, or retained beyond the
/// authorization transaction. This value does not launch a browser, execute
/// PAR or HTTP, validate a callback, correlate returned state or issuer,
/// exchange a code, or establish authorization or server trust.
pub struct AuthorizationRequest {
    input: CredentialOfferWithAuthorizationRequestInput,
    request_uri: Zeroizing<String>,
    issuer_state_present: bool,
}

impl AuthorizationRequest {
    /// Borrow the complete validated input lineage needed by later response
    /// correlation and code exchange.
    pub const fn authorization_request_input(
        &self,
    ) -> &CredentialOfferWithAuthorizationRequestInput {
        &self.input
    }

    /// Return the HTTP method used by a direct Authorization Endpoint request.
    pub const fn http_method(&self) -> &'static str {
        AUTHORIZATION_REQUEST_HTTP_METHOD
    }

    /// Return the exact request URI byte count.
    pub fn request_uri_len(&self) -> usize {
        self.request_uri.len()
    }

    /// Report whether the exact request carries offered issuer state.
    pub const fn issuer_state_present(&self) -> bool {
        self.issuer_state_present
    }

    /// Borrow the exact sensitive request URI for immediate adapter use.
    ///
    /// The returned URI contains the CSRF state and may contain issuer state.
    /// Keep it out of logs, browser-independent storage, telemetry, referrer
    /// surfaces and generic serializers.
    pub fn expose_sensitive_request_uri(&self) -> &str {
        &self.request_uri
    }
}

impl fmt::Debug for AuthorizationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizationRequest")
            .field("request_uri_bytes", &self.request_uri.len())
            .field("issuer_state_present", &self.issuer_state_present)
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithAuthorizationRequestInput {
    /// Consume validated request inputs and construct one bounded deterministic
    /// Authorization-Details-based request URI.
    pub fn try_into_authorization_request(
        self,
        limits: AuthorizationRequestLimits,
    ) -> Result<AuthorizationRequest, CredentialOfferError> {
        let server = self.credential_offer_with_authorization_code_server();
        let matched = server.credential_offer_with_metadata();
        let metadata = matched.credential_issuer_metadata();
        let endpoint = server
            .authorization_server_metadata()
            .authorization_endpoint()
            .ok_or(CredentialOfferError::AuthorizationEndpointRequired)?
            .as_str();
        let parsed = ParsedUri::parse(endpoint)
            .map_err(|_| CredentialOfferError::InvalidAuthorizationEndpointQuery)?;
        let existing_query = parsed.query().map(|query| query.as_str());
        if let Some(query) = existing_query {
            validate_existing_query(query, limits)?;
        }

        let include_locations = metadata.advertised_authorization_servers().is_some();
        let authorization_details = build_authorization_details(
            self.selected_credential_configuration().as_str(),
            include_locations.then(|| metadata.credential_issuer().as_str()),
            limits.max_authorization_details_bytes(),
        )?;
        let issuer_state = matched
            .credential_offer()
            .authorization_code()
            .and_then(|grant| grant.issuer_state())
            .map(|state| state.as_str());
        let issuer_state_present = issuer_state.is_some();
        let parameters = request_parameters(&self, &authorization_details, issuer_state);
        let query_len =
            query_len(&parameters).ok_or(CredentialOfferError::AuthorizationRequestUriTooLarge)?;
        let request_uri_len = endpoint
            .len()
            .checked_add(1)
            .and_then(|length| length.checked_add(query_len))
            .ok_or(CredentialOfferError::AuthorizationRequestUriTooLarge)?;
        if request_uri_len > limits.max_request_uri_bytes() {
            return Err(CredentialOfferError::AuthorizationRequestUriTooLarge);
        }

        let mut request_uri = Zeroizing::new(String::with_capacity(request_uri_len));
        request_uri.push_str(endpoint);
        request_uri.push(if existing_query.is_some() { '&' } else { '?' });
        for (index, (name, value)) in parameters.iter().enumerate() {
            if index != 0 {
                request_uri.push('&');
            }
            append_pair(&mut request_uri, name, value);
        }
        debug_assert_eq!(request_uri.len(), request_uri_len);

        Ok(AuthorizationRequest {
            input: self,
            request_uri,
            issuer_state_present,
        })
    }
}

fn request_parameters<'a>(
    input: &'a CredentialOfferWithAuthorizationRequestInput,
    authorization_details: &'a str,
    issuer_state: Option<&'a str>,
) -> Vec<(&'static str, &'a str)> {
    let mut parameters = Vec::with_capacity(8);
    parameters.push(("response_type", "code"));
    parameters.push(("client_id", input.client_id().as_str()));
    parameters.push(("redirect_uri", input.redirect_uri().as_str()));
    parameters.push(("state", input.state().as_str()));
    parameters.push(("code_challenge", input.code_challenge().as_str()));
    parameters.push(("code_challenge_method", input.code_challenge_method()));
    parameters.push(("authorization_details", authorization_details));
    if let Some(value) = issuer_state {
        parameters.push(("issuer_state", value));
    }
    parameters
}

fn query_len(parameters: &[(&str, &str)]) -> Option<usize> {
    parameters
        .iter()
        .enumerate()
        .try_fold(0usize, |length, (index, (name, value))| {
            length
                .checked_add(usize::from(index != 0))?
                .checked_add(pair_len(name, value)?)
        })
}

fn validate_existing_query(
    query: &str,
    limits: AuthorizationRequestLimits,
) -> Result<(), CredentialOfferError> {
    if query.is_empty() {
        return Err(CredentialOfferError::InvalidAuthorizationEndpointQuery);
    }
    let mut names = BTreeSet::new();
    for (index, field) in query.split('&').enumerate() {
        if index == limits.max_endpoint_query_parameters() {
            return Err(CredentialOfferError::TooManyAuthorizationEndpointQueryParameters);
        }
        if field.is_empty() {
            return Err(CredentialOfferError::InvalidAuthorizationEndpointQuery);
        }
        let (encoded_name, encoded_value) = field.split_once('=').unwrap_or((field, ""));
        let name = decode_component(
            encoded_name,
            limits.max_endpoint_query_name_bytes(),
            CredentialOfferError::InvalidAuthorizationEndpointQuery,
            CredentialOfferError::AuthorizationEndpointQueryComponentTooLarge,
        )?;
        let _value = decode_component(
            encoded_value,
            limits.max_endpoint_query_value_bytes(),
            CredentialOfferError::InvalidAuthorizationEndpointQuery,
            CredentialOfferError::AuthorizationEndpointQueryComponentTooLarge,
        )?;
        if name.is_empty() {
            return Err(CredentialOfferError::InvalidAuthorizationEndpointQuery);
        }
        if RESERVED_QUERY_NAMES.contains(&name.as_str()) || !names.insert(name.to_string()) {
            return Err(CredentialOfferError::AuthorizationEndpointQueryParameterCollision);
        }
    }
    Ok(())
}

fn build_authorization_details(
    configuration_id: &str,
    location: Option<&str>,
    max_bytes: usize,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    let mut writer = BoundedJson::new(max_bytes);
    writer.push_static(b"[{\"type\":\"openid_credential\"")?;
    if let Some(issuer) = location {
        writer.push_static(b",\"locations\":[")?;
        writer.push_string(issuer)?;
        writer.push_static(b"]")?;
    }
    writer.push_static(b",\"credential_configuration_id\":")?;
    writer.push_string(configuration_id)?;
    writer.push_static(b"}]")?;
    writer.into_string()
}

struct BoundedJson {
    bytes: Zeroizing<Vec<u8>>,
    max_bytes: usize,
}

impl BoundedJson {
    fn new(max_bytes: usize) -> Self {
        Self {
            bytes: Zeroizing::new(Vec::with_capacity(max_bytes.min(4_096))),
            max_bytes,
        }
    }

    fn push_static(&mut self, value: &[u8]) -> Result<(), CredentialOfferError> {
        self.write_all(value)
            .map_err(|_| CredentialOfferError::AuthorizationDetailsTooLarge)
    }

    fn push_string(&mut self, value: &str) -> Result<(), CredentialOfferError> {
        serde_json::to_writer(&mut *self, value)
            .map_err(|_| CredentialOfferError::AuthorizationDetailsTooLarge)
    }

    fn into_string(self) -> Result<Zeroizing<String>, CredentialOfferError> {
        String::from_utf8(self.bytes.to_vec())
            .map(Zeroizing::new)
            .map_err(|_| CredentialOfferError::AuthorizationDetailsTooLarge)
    }
}

impl Write for BoundedJson {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let Some(next_len) = self.bytes.len().checked_add(buffer.len()) else {
            return Err(io::Error::other("Authorization Details limit exceeded"));
        };
        if next_len > self.max_bytes {
            return Err(io::Error::other("Authorization Details limit exceeded"));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationRequestLimits, validate_existing_query};
    use crate::CredentialOfferError;

    #[test]
    fn rejects_malformed_existing_query_without_disclosure() {
        for query in ["", "x=%", "x=%FF", "=value", "x=ok&&y=bad", "x=%00"] {
            assert!(matches!(
                validate_existing_query(query, AuthorizationRequestLimits::default()),
                Err(CredentialOfferError::InvalidAuthorizationEndpointQuery)
            ));
        }
    }
}

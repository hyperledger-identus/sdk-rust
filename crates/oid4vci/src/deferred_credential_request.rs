use std::{fmt, io, io::Write};

use zeroize::Zeroizing;

use crate::{
    CredentialIssuerIdentifier, CredentialIssuerMetadata, CredentialOfferError,
    DeferredCredentialEndpoint, DeferredCredentialRequestLimits, DeferredCredentialResponseCore,
    jwt_credential_request::DeferredCredentialContinuationAuthority,
};

/// HTTP method required for a Final Deferred Credential Request.
pub const DEFERRED_CREDENTIAL_REQUEST_HTTP_METHOD: &str = "POST";

/// Media type required for an unencrypted Final Deferred Credential Request.
pub const DEFERRED_CREDENTIAL_REQUEST_MEDIA_TYPE: &str = "application/json";

/// A bounded unencrypted OpenID4VCI Final Deferred Credential Request.
///
/// This value contains a bearer-adjacent transaction identifier. It does not
/// own an OAuth access token, execute HTTP, enforce an interval, or establish
/// endpoint provenance, transaction validity, replay safety, or TLS policy.
pub struct DeferredCredentialRequest {
    deferred_credential_endpoint: DeferredCredentialEndpoint,
    json_body: Zeroizing<Vec<u8>>,
    transaction_id: Zeroizing<String>,
}

/// A Deferred Credential Request carrying the exact authority of its initial request.
///
/// This value owns a bearer Authorization field and a transaction identifier.
/// Keep its sensitive accessors out of logs, URLs, telemetry, caches and
/// long-lived storage. It performs no HTTP or polling behavior.
pub struct RequestBoundDeferredCredentialRequest {
    request: DeferredCredentialRequest,
    credential_issuer: CredentialIssuerIdentifier,
    authorization: Zeroizing<String>,
    request_proof_count: usize,
}

impl RequestBoundDeferredCredentialRequest {
    pub(crate) fn try_from_parts(
        authority: DeferredCredentialContinuationAuthority,
        response: DeferredCredentialResponseCore,
        limits: DeferredCredentialRequestLimits,
    ) -> Result<Self, CredentialOfferError> {
        let DeferredCredentialContinuationAuthority {
            credential_issuer,
            deferred_credential_endpoint,
            authorization,
            request_proof_count,
        } = authority;
        let endpoint = deferred_credential_endpoint
            .ok_or(CredentialOfferError::DeferredCredentialEndpointRequired)?;
        let request = create_deferred_credential_request(&response, &endpoint, limits)?;
        Ok(Self {
            request,
            credential_issuer,
            authorization,
            request_proof_count,
        })
    }

    /// Borrow the exact validated Credential Issuer Identifier.
    pub const fn credential_issuer(&self) -> &CredentialIssuerIdentifier {
        &self.credential_issuer
    }

    /// Borrow the exact advertised Deferred Credential Endpoint.
    pub const fn deferred_credential_endpoint(&self) -> &DeferredCredentialEndpoint {
        self.request.deferred_credential_endpoint()
    }

    /// Return the originating Credential Request JWT proof count.
    pub const fn request_proof_count(&self) -> usize {
        self.request_proof_count
    }

    /// Return the required HTTP method.
    pub const fn http_method(&self) -> &'static str {
        self.request.http_method()
    }

    /// Return the required media type for this unencrypted request.
    pub const fn media_type(&self) -> &'static str {
        self.request.media_type()
    }

    /// Return the complete Authorization field-value byte count.
    pub fn authorization_len(&self) -> usize {
        self.authorization.len()
    }

    /// Return the exact complete JSON body byte count.
    pub fn json_body_len(&self) -> usize {
        self.request.json_body_len()
    }

    /// Borrow the exact bearer Authorization field for immediate transport.
    pub fn expose_sensitive_authorization(&self) -> &str {
        &self.authorization
    }

    /// Borrow the exact transaction request body for immediate transport.
    pub fn expose_sensitive_json_body(&self) -> &[u8] {
        self.request.expose_sensitive_json_body()
    }
}

impl fmt::Debug for RequestBoundDeferredCredentialRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestBoundDeferredCredentialRequest")
            .field("authorization_bytes", &self.authorization.len())
            .field("json_body_bytes", &self.request.json_body_len())
            .field("request_proof_count", &self.request_proof_count)
            .finish_non_exhaustive()
    }
}

impl DeferredCredentialRequest {
    /// Borrow the exact validated Deferred Credential Endpoint.
    pub const fn deferred_credential_endpoint(&self) -> &DeferredCredentialEndpoint {
        &self.deferred_credential_endpoint
    }

    /// Return the required HTTP method.
    pub const fn http_method(&self) -> &'static str {
        DEFERRED_CREDENTIAL_REQUEST_HTTP_METHOD
    }

    /// Return the required media type for this unencrypted request.
    pub const fn media_type(&self) -> &'static str {
        DEFERRED_CREDENTIAL_REQUEST_MEDIA_TYPE
    }

    /// Return the exact complete JSON body byte count.
    pub fn json_body_len(&self) -> usize {
        self.json_body.len()
    }

    /// Report that execution requires a valid OAuth access token.
    pub const fn access_token_required(&self) -> bool {
        true
    }

    /// Borrow the exact sensitive JSON body for immediate transport.
    ///
    /// The returned bytes contain a correlating transaction identifier. Keep
    /// them out of logs, URLs, telemetry, caches and unrelated storage.
    pub fn expose_sensitive_json_body(&self) -> &[u8] {
        &self.json_body
    }

    pub(crate) fn transaction_id(&self) -> &str {
        &self.transaction_id
    }
}

impl fmt::Debug for DeferredCredentialRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeferredCredentialRequest")
            .field("json_body_bytes", &self.json_body.len())
            .field("access_token_required", &true)
            .finish_non_exhaustive()
    }
}

impl DeferredCredentialResponseCore {
    /// Construct a bounded unencrypted Final request from this response and
    /// the issuer's advertised Deferred Credential Endpoint.
    ///
    /// This operation may be repeated for a transaction that remains deferred;
    /// callers retain responsibility for interval, retry, replay, invalidation,
    /// token, transport and response-correlation policy.
    pub fn try_deferred_credential_request(
        &self,
        metadata: &CredentialIssuerMetadata,
        limits: DeferredCredentialRequestLimits,
    ) -> Result<DeferredCredentialRequest, CredentialOfferError> {
        let endpoint = metadata
            .deferred_credential_endpoint()
            .ok_or(CredentialOfferError::DeferredCredentialEndpointRequired)?;
        create_deferred_credential_request(self, endpoint, limits)
    }
}

fn create_deferred_credential_request(
    response: &DeferredCredentialResponseCore,
    endpoint: &DeferredCredentialEndpoint,
    limits: DeferredCredentialRequestLimits,
) -> Result<DeferredCredentialRequest, CredentialOfferError> {
    let transaction_id = response.transaction_id().expose_sensitive_transaction_id();
    let json_body = serialize_request_body(transaction_id, limits.max_json_body_bytes())?;
    Ok(DeferredCredentialRequest {
        deferred_credential_endpoint: endpoint.duplicate(),
        json_body,
        transaction_id: Zeroizing::new(transaction_id.to_owned()),
    })
}

struct BoundedJsonWriter {
    bytes: Zeroizing<Vec<u8>>,
    maximum: usize,
}

impl BoundedJsonWriter {
    fn new(maximum: usize) -> Self {
        Self {
            bytes: Zeroizing::new(Vec::new()),
            maximum,
        }
    }

    fn into_bytes(self) -> Zeroizing<Vec<u8>> {
        self.bytes
    }
}

impl Write for BoundedJsonWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let Some(next_len) = self.bytes.len().checked_add(buffer.len()) else {
            return Err(io::Error::other("bounded JSON request is too large"));
        };
        if next_len > self.maximum {
            return Err(io::Error::other("bounded JSON request is too large"));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn serialize_request_body(
    transaction_id: &str,
    maximum: usize,
) -> Result<Zeroizing<Vec<u8>>, CredentialOfferError> {
    let mut writer = BoundedJsonWriter::new(maximum);
    writer
        .write_all(b"{\"transaction_id\":")
        .map_err(|_| CredentialOfferError::DeferredCredentialRequestTooLarge)?;
    serde_json::to_writer(&mut writer, transaction_id)
        .map_err(|_| CredentialOfferError::DeferredCredentialRequestTooLarge)?;
    writer
        .write_all(b"}")
        .map_err(|_| CredentialOfferError::DeferredCredentialRequestTooLarge)?;
    Ok(writer.into_bytes())
}

use std::fmt;

use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, ImmediateCredentialResponseLimits,
    json::parse_immediate_credential_response_fields,
};

/// JSON representation used by one opaque issued credential value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialValueKind {
    /// A JSON string, such as a JWT, SD-JWT or base64url-encoded binary value.
    String,
    /// A JSON object whose interpretation belongs to its credential format.
    Object,
}

/// One opaque credential from an immediate OID4VCI Credential Response.
pub struct IssuedCredential {
    exact_json: Zeroizing<String>,
    decoded_string: Option<Zeroizing<String>>,
}

impl IssuedCredential {
    /// Return the credential value's JSON representation kind.
    pub const fn kind(&self) -> CredentialValueKind {
        if self.decoded_string.is_some() {
            CredentialValueKind::String
        } else {
            CredentialValueKind::Object
        }
    }

    /// Return the exact byte length of the credential JSON value.
    pub fn json_len(&self) -> usize {
        self.exact_json.len()
    }

    /// Borrow the exact validated credential JSON value.
    ///
    /// Keep the value out of logs, telemetry, URLs, caches, and unrelated or
    /// unprotected storage. Parsing this value does not establish format
    /// validity, issuer trust, or cryptographic verification.
    pub fn expose_sensitive_json(&self) -> &str {
        &self.exact_json
    }

    /// Borrow the decoded credential when its JSON representation is a string.
    ///
    /// Object credentials return `None`. Keep the value out of logs,
    /// telemetry, URLs, caches, and unrelated or unprotected storage.
    pub fn expose_sensitive_string(&self) -> Option<&str> {
        self.decoded_string.as_ref().map(|value| value.as_str())
    }
}

impl fmt::Debug for IssuedCredential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuedCredential")
            .field("kind", &self.kind())
            .field("json_bytes", &self.exact_json.len())
            .finish_non_exhaustive()
    }
}

/// A bounded unencrypted immediate OID4VCI Credential Response body.
///
/// This state proves only JSON body syntax. It does not prove HTTP semantics,
/// transport or issuer provenance, request correlation, credential format or
/// signature validity, trust, status, storage authorization, or notification
/// delivery.
pub struct ImmediateCredentialResponseCore {
    response_len: usize,
    credentials: Vec<IssuedCredential>,
    notification_id: Option<Zeroizing<String>>,
}

impl ImmediateCredentialResponseCore {
    /// Parse an immediate Credential Response under explicit resource limits.
    ///
    /// The caller retains responsibility for erasing its input allocation.
    pub fn parse(
        json: &str,
        limits: ImmediateCredentialResponseLimits,
    ) -> Result<Self, CredentialOfferError> {
        if json.len() > limits.max_json_bytes() {
            return Err(CredentialOfferError::ImmediateCredentialResponseTooLarge);
        }
        let fields = parse_immediate_credential_response_fields(json.as_bytes(), limits)?;
        let credentials = fields
            .credentials
            .into_iter()
            .map(|credential| IssuedCredential {
                exact_json: credential.exact_json,
                decoded_string: credential.decoded_string,
            })
            .collect();
        Ok(Self {
            response_len: json.len(),
            credentials,
            notification_id: fields.notification_id,
        })
    }

    /// Return the exact response byte count.
    pub const fn response_len(&self) -> usize {
        self.response_len
    }

    /// Borrow the ordered opaque credentials.
    pub fn credentials(&self) -> &[IssuedCredential] {
        &self.credentials
    }

    /// Report whether the issuer supplied a notification identifier.
    pub const fn notification_id_present(&self) -> bool {
        self.notification_id.is_some()
    }

    /// Borrow the optional opaque notification identifier.
    ///
    /// Keep the value out of logs, telemetry, URLs, caches, and unrelated or
    /// long-lived storage. This does not authorize Notification Endpoint use.
    pub fn expose_sensitive_notification_id(&self) -> Option<&str> {
        self.notification_id
            .as_ref()
            .map(|notification_id| notification_id.as_str())
    }
}

impl fmt::Debug for ImmediateCredentialResponseCore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmediateCredentialResponseCore")
            .field("response_bytes", &self.response_len)
            .field("credential_count", &self.credentials.len())
            .field("notification_id_present", &self.notification_id.is_some())
            .finish_non_exhaustive()
    }
}

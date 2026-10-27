use std::fmt;

use identus_did::{Did, DidUrl};
use identus_jose::{JwsAlgorithm, JwsSigner, JwsSigningInput, ProtectedHeader};
use serde::Serialize;
use zeroize::Zeroizing;

use crate::{Siopv2Error, ValidatedSiopRequest};

/// Protected-header type for the self-issued ID Token.
pub const ID_TOKEN_JWT_TYPE: &str = "JWT";
/// Media type for the exact direct-post form body.
pub const DIRECT_POST_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

#[derive(Serialize)]
struct IdTokenClaims<'a> {
    iss: &'a str,
    sub: &'a str,
    aud: &'a str,
    nonce: &'a str,
    iat: i64,
    exp: i64,
}

/// Unsigned response evidence ready for an injected signer.
pub struct SiopResponsePlan {
    signing_input: JwsSigningInput,
    redirect_uri: Zeroizing<String>,
    state: Option<Zeroizing<String>>,
    limits: crate::Siopv2Limits,
}

impl ValidatedSiopRequest {
    /// Plan a response for one caller-authorized DID authentication method.
    pub fn plan_response(
        &self,
        subject: Did,
        key_id: DidUrl,
        algorithm: JwsAlgorithm,
        issued_at: i64,
        expires_at: i64,
    ) -> Result<SiopResponsePlan, Siopv2Error> {
        if key_id.to_did() != subject || key_id.fragment().is_none() {
            return Err(Siopv2Error::InvalidKeyId);
        }
        if issued_at > self.expires_at
            || expires_at <= issued_at
            || expires_at > self.expires_at
            || expires_at.saturating_sub(issued_at) > self.limits.max_request_lifetime_seconds()
        {
            return Err(Siopv2Error::InvalidTimeWindow);
        }
        let claims = IdTokenClaims {
            iss: subject.as_str(),
            sub: subject.as_str(),
            aud: &self.client_id,
            nonce: &self.nonce,
            iat: issued_at,
            exp: expires_at,
        };
        let payload = serde_json::to_vec(&claims).map_err(|_| Siopv2Error::ResponseTooLarge)?;
        if payload.len() > self.limits.jws().max_payload_bytes() {
            return Err(Siopv2Error::ResponseTooLarge);
        }
        let header = ProtectedHeader::new(
            algorithm.as_str(),
            Some(ID_TOKEN_JWT_TYPE),
            Some(key_id.as_str()),
            self.limits.jws(),
        )
        .map_err(|_| Siopv2Error::InvalidKeyId)?;
        let signing_input = JwsSigningInput::new(header, payload, self.limits.jws())
            .map_err(|_| Siopv2Error::ResponseTooLarge)?;
        Ok(SiopResponsePlan {
            signing_input,
            redirect_uri: self.redirect_uri.clone(),
            state: self.state.clone(),
            limits: self.limits,
        })
    }
}

impl SiopResponsePlan {
    /// Exact public signing input passed to the injected signer.
    pub fn signing_input(&self) -> &[u8] {
        self.signing_input.as_bytes()
    }
    /// Sign and encode the exact bounded direct-post response.
    pub fn sign(self, signer: &dyn JwsSigner) -> Result<DirectPostResponse, Siopv2Error> {
        let compact = self
            .signing_input
            .sign_with(signer)
            .map_err(|error| match error {
                identus_jose::JoseError::SigningRejected => Siopv2Error::SigningRejected,
                identus_jose::JoseError::SignerUnavailable => Siopv2Error::SignerUnavailable,
                identus_jose::JoseError::AlgorithmMismatch => Siopv2Error::AlgorithmMismatch,
                _ => Siopv2Error::ResponseTooLarge,
            })?;
        let mut body = Zeroizing::new(Vec::new());
        append_pair(
            &mut body,
            b"id_token",
            compact.compact().as_bytes(),
            self.limits.max_response_body_bytes(),
        )?;
        if let Some(state) = self.state.as_deref() {
            append_pair(
                &mut body,
                b"state",
                state.as_bytes(),
                self.limits.max_response_body_bytes(),
            )?;
        }
        Ok(DirectPostResponse {
            redirect_uri: self.redirect_uri,
            body,
        })
    }
}

/// Exact caller-deliverable draft-13 direct-post response.
pub struct DirectPostResponse {
    redirect_uri: Zeroizing<String>,
    body: Zeroizing<Vec<u8>>,
}

impl DirectPostResponse {
    /// Required HTTP method.
    pub const fn method(&self) -> &'static str {
        "POST"
    }
    /// Required static content type.
    pub const fn content_type(&self) -> &'static str {
        DIRECT_POST_MEDIA_TYPE
    }
    /// Explicitly reveal the verified destination to a transport adapter.
    pub fn expose_sensitive_endpoint(&self) -> &str {
        &self.redirect_uri
    }
    /// Explicitly reveal the signed response body to a transport adapter.
    pub fn expose_sensitive_body(&self) -> &[u8] {
        &self.body
    }
    /// Encoded body size in bytes.
    pub fn body_len(&self) -> usize {
        self.body.len()
    }
}

impl fmt::Debug for DirectPostResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DirectPostResponse")
            .field("endpoint_len", &self.redirect_uri.len())
            .field("body_len", &self.body.len())
            .finish()
    }
}

fn append_pair(
    output: &mut Vec<u8>,
    name: &[u8],
    value: &[u8],
    maximum: usize,
) -> Result<(), Siopv2Error> {
    if !output.is_empty() {
        push(output, b'&', maximum)?;
    }
    for byte in name {
        push(output, *byte, maximum)?;
    }
    push(output, b'=', maximum)?;
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in value {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            push(output, *byte, maximum)?;
        } else {
            push(output, b'%', maximum)?;
            push(output, HEX[(byte >> 4) as usize], maximum)?;
            push(output, HEX[(byte & 0x0f) as usize], maximum)?;
        }
    }
    Ok(())
}

fn push(output: &mut Vec<u8>, byte: u8, maximum: usize) -> Result<(), Siopv2Error> {
    if output.len() >= maximum {
        return Err(Siopv2Error::ResponseTooLarge);
    }
    output.push(byte);
    Ok(())
}

use std::fmt;

use fluent_uri::Uri as ParsedUri;
use identus_crypto::{Base64UrlStrNoPad, sha256};
use zeroize::Zeroizing;

use crate::{
    CredentialConfigurationId, CredentialOfferError, CredentialOfferWithAuthorizationCodeServer,
};

const MIN_PKCE_CODE_VERIFIER_BYTES: usize = 43;
const MAX_PKCE_CODE_VERIFIER_BYTES: usize = 128;

/// The only PKCE challenge method produced by this state.
pub const PKCE_S256_METHOD: &str = "S256";

/// Positive resource limits for retained Authorization Request inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationRequestInputLimits {
    max_client_id_bytes: usize,
    max_redirect_uri_bytes: usize,
    max_state_bytes: usize,
}

impl AuthorizationRequestInputLimits {
    /// Construct positive limits for client identifier, redirect URI and state.
    pub const fn new(
        max_client_id_bytes: usize,
        max_redirect_uri_bytes: usize,
        max_state_bytes: usize,
    ) -> Result<Self, CredentialOfferError> {
        if max_client_id_bytes == 0 || max_redirect_uri_bytes == 0 || max_state_bytes == 0 {
            return Err(CredentialOfferError::InvalidAuthorizationRequestInputLimits);
        }
        Ok(Self {
            max_client_id_bytes,
            max_redirect_uri_bytes,
            max_state_bytes,
        })
    }

    /// Maximum retained OAuth client identifier bytes.
    pub const fn max_client_id_bytes(self) -> usize {
        self.max_client_id_bytes
    }

    /// Maximum retained redirect URI bytes.
    pub const fn max_redirect_uri_bytes(self) -> usize {
        self.max_redirect_uri_bytes
    }

    /// Maximum retained CSRF state bytes.
    pub const fn max_state_bytes(self) -> usize {
        self.max_state_bytes
    }
}

impl Default for AuthorizationRequestInputLimits {
    fn default() -> Self {
        Self {
            max_client_id_bytes: 2_048,
            max_redirect_uri_bytes: 2_048,
            max_state_bytes: 1_024,
        }
    }
}

macro_rules! redacted_string_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        pub struct $name {
            value: Zeroizing<String>,
        }

        impl $name {
            /// Borrow the exact validated value.
            pub fn as_str(&self) -> &str {
                &self.value
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .finish_non_exhaustive()
            }
        }
    };
}

redacted_string_type!(
    AuthorizationRequestClientId,
    "A bounded RFC 6749 OAuth client identifier."
);
redacted_string_type!(
    AuthorizationRequestRedirectUri,
    "A bounded absolute fragment-free OAuth redirect URI."
);
redacted_string_type!(
    AuthorizationRequestState,
    "A bounded caller-generated OAuth CSRF state value."
);
redacted_string_type!(
    PkceCodeVerifier,
    "A validated caller-generated RFC 7636 PKCE code verifier."
);
redacted_string_type!(
    PkceS256Challenge,
    "The canonical RFC 7636 S256 challenge derived from a code verifier."
);

/// A server-bound Credential Offer with bounded inputs for a later
/// Authorization Request serializer.
///
/// This state validates inputs and the PKCE S256 relationship. It does not
/// serialize or send a request, choose scope or `authorization_details`,
/// validate a callback, compare returned state, exchange a code, or establish
/// authorization, redirect registration, server trust, or mix-up protection.
pub struct CredentialOfferWithAuthorizationRequestInput {
    server: CredentialOfferWithAuthorizationCodeServer,
    selected_configuration_index: usize,
    client_id: AuthorizationRequestClientId,
    redirect_uri: AuthorizationRequestRedirectUri,
    state: AuthorizationRequestState,
    code_verifier: PkceCodeVerifier,
    code_challenge: PkceS256Challenge,
}

impl CredentialOfferWithAuthorizationRequestInput {
    /// Borrow the capable server-bound offer.
    pub const fn credential_offer_with_authorization_code_server(
        &self,
    ) -> &CredentialOfferWithAuthorizationCodeServer {
        &self.server
    }

    /// Borrow the selected existing offered Credential Configuration ID.
    pub fn selected_credential_configuration(&self) -> &CredentialConfigurationId {
        self.server
            .credential_offer_with_metadata()
            .credential_offer()
            .credential_offer()
            .credential_configuration_ids()
            .get(self.selected_configuration_index)
            .expect("the constructor validates the retained configuration index")
    }

    /// Borrow the validated OAuth client identifier.
    pub const fn client_id(&self) -> &AuthorizationRequestClientId {
        &self.client_id
    }

    /// Borrow the validated redirect URI.
    pub const fn redirect_uri(&self) -> &AuthorizationRequestRedirectUri {
        &self.redirect_uri
    }

    /// Borrow the caller-generated CSRF state.
    pub const fn state(&self) -> &AuthorizationRequestState {
        &self.state
    }

    /// Borrow the sensitive PKCE verifier needed by a later code exchange.
    pub const fn code_verifier(&self) -> &PkceCodeVerifier {
        &self.code_verifier
    }

    /// Borrow the public S256 challenge for a later Authorization Request.
    pub const fn code_challenge(&self) -> &PkceS256Challenge {
        &self.code_challenge
    }

    /// Return the PKCE challenge method paired with [`Self::code_challenge`].
    pub const fn code_challenge_method(&self) -> &'static str {
        PKCE_S256_METHOD
    }
}

impl fmt::Debug for CredentialOfferWithAuthorizationRequestInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialOfferWithAuthorizationRequestInput")
            .finish_non_exhaustive()
    }
}

impl CredentialOfferWithAuthorizationCodeServer {
    /// Consume this capable server-bound offer after validating the inputs for
    /// a later Authorization Request serializer.
    pub fn try_with_authorization_request_input(
        self,
        selected_configuration_index: usize,
        client_id: &str,
        redirect_uri: &str,
        state: &str,
        code_verifier: &str,
        limits: AuthorizationRequestInputLimits,
    ) -> Result<CredentialOfferWithAuthorizationRequestInput, CredentialOfferError> {
        if self
            .credential_offer_with_metadata()
            .credential_offer()
            .credential_offer()
            .credential_configuration_ids()
            .get(selected_configuration_index)
            .is_none()
        {
            return Err(CredentialOfferError::AuthorizationRequestConfigurationMissing);
        }

        let client_id = validate_visible_ascii(
            client_id,
            limits.max_client_id_bytes(),
            CredentialOfferError::AuthorizationRequestClientIdTooLarge,
            CredentialOfferError::InvalidAuthorizationRequestClientId,
        )?;
        let redirect_uri = validate_redirect_uri(redirect_uri, limits.max_redirect_uri_bytes())?;
        let state = validate_visible_ascii(
            state,
            limits.max_state_bytes(),
            CredentialOfferError::AuthorizationRequestStateTooLarge,
            CredentialOfferError::InvalidAuthorizationRequestState,
        )?;
        let code_verifier = validate_code_verifier(code_verifier)?;
        let digest = sha256(code_verifier.as_bytes());
        let code_challenge = Base64UrlStrNoPad::try_from_bytes(digest.as_array())
            .map_err(|_| CredentialOfferError::InvalidPkceCodeVerifier)?;

        Ok(CredentialOfferWithAuthorizationRequestInput {
            server: self,
            selected_configuration_index,
            client_id: AuthorizationRequestClientId { value: client_id },
            redirect_uri: AuthorizationRequestRedirectUri {
                value: redirect_uri,
            },
            state: AuthorizationRequestState { value: state },
            code_verifier: PkceCodeVerifier {
                value: code_verifier,
            },
            code_challenge: PkceS256Challenge {
                value: Zeroizing::new(code_challenge.as_str().to_owned()),
            },
        })
    }
}

fn validate_visible_ascii(
    value: &str,
    max_bytes: usize,
    too_large: CredentialOfferError,
    invalid: CredentialOfferError,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    if value.len() > max_bytes {
        return Err(too_large);
    }
    if value.is_empty() || !value.bytes().all(|byte| (0x20..=0x7e).contains(&byte)) {
        return Err(invalid);
    }
    Ok(Zeroizing::new(value.to_owned()))
}

fn validate_redirect_uri(
    value: &str,
    max_bytes: usize,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    if value.len() > max_bytes {
        return Err(CredentialOfferError::AuthorizationRequestRedirectUriTooLarge);
    }
    let parsed = ParsedUri::parse(value)
        .map_err(|_| CredentialOfferError::InvalidAuthorizationRequestRedirectUri)?;
    if value.is_empty()
        || parsed.scheme().as_str().is_empty()
        || parsed.fragment().is_some()
        || parsed
            .authority()
            .is_some_and(|authority| authority.userinfo().is_some() || authority.host().is_empty())
    {
        return Err(CredentialOfferError::InvalidAuthorizationRequestRedirectUri);
    }
    Ok(Zeroizing::new(value.to_owned()))
}

fn validate_code_verifier(value: &str) -> Result<Zeroizing<String>, CredentialOfferError> {
    if value.len() > MAX_PKCE_CODE_VERIFIER_BYTES {
        return Err(CredentialOfferError::PkceCodeVerifierTooLarge);
    }
    if value.len() < MIN_PKCE_CODE_VERIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~'))
    {
        return Err(CredentialOfferError::InvalidPkceCodeVerifier);
    }
    Ok(Zeroizing::new(value.to_owned()))
}

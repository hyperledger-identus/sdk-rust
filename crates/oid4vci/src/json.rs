use std::ops::Range;

use zeroize::Zeroizing;

use crate::{
    AuthorizationServerMetadataLimits, CredentialIssuerMetadataLimits,
    CredentialNonceResponseLimits, CredentialOfferError, CredentialOfferGrantLimits,
    CredentialOfferLimits, CredentialOfferSemanticLimits, TokenErrorResponseLimits,
    TokenResponseLimits,
};

const AUTHORIZATION_CODE_GRANT: &str = "authorization_code";
const PRE_AUTHORIZED_CODE_GRANT: &str = "urn:ietf:params:oauth:grant-type:pre-authorized_code";
const MAX_TRANSACTION_CODE_DESCRIPTION_CHARACTERS: usize = 300;

pub(crate) struct CredentialOfferFields {
    pub(crate) credential_issuer: Zeroizing<String>,
    pub(crate) credential_configuration_ids: Vec<Zeroizing<String>>,
    pub(crate) grants_present: bool,
}

pub(crate) struct CredentialOfferGrantFields {
    pub(crate) authorization_code: Option<AuthorizationCodeGrantFields>,
    pub(crate) pre_authorized_code: Option<PreAuthorizedCodeGrantFields>,
}

pub(crate) struct AuthorizationCodeGrantFields {
    pub(crate) issuer_state: Option<Zeroizing<String>>,
    pub(crate) authorization_server: Option<Zeroizing<String>>,
}

pub(crate) struct PreAuthorizedCodeGrantFields {
    pub(crate) pre_authorized_code: Zeroizing<String>,
    pub(crate) transaction_code: Option<TransactionCodeFields>,
    pub(crate) authorization_server: Option<Zeroizing<String>>,
}

pub(crate) struct TransactionCodeFields {
    pub(crate) input_mode: Option<Zeroizing<String>>,
    pub(crate) length: Option<usize>,
    pub(crate) description: Option<Zeroizing<String>>,
}

pub(crate) struct CredentialIssuerMetadataFields {
    pub(crate) credential_issuer: Zeroizing<String>,
    pub(crate) authorization_servers: Option<Vec<Zeroizing<String>>>,
    pub(crate) credential_endpoint: Zeroizing<String>,
    pub(crate) credential_configurations: Vec<CredentialConfigurationFields>,
}

pub(crate) struct CredentialConfigurationFields {
    pub(crate) id: Zeroizing<String>,
    pub(crate) format: Zeroizing<String>,
}

pub(crate) struct AuthorizationServerMetadataFields {
    pub(crate) issuer: Zeroizing<String>,
    pub(crate) authorization_endpoint: Option<Zeroizing<String>>,
    pub(crate) token_endpoint: Option<Zeroizing<String>>,
    pub(crate) grant_types_supported: Option<Vec<Zeroizing<String>>>,
    pub(crate) anonymous_pre_authorized_access: Option<bool>,
}

pub(crate) struct TokenResponseFields {
    pub(crate) access_token: Zeroizing<String>,
    pub(crate) token_type: Zeroizing<String>,
    pub(crate) expires_in: Option<u64>,
    pub(crate) refresh_token: Option<Zeroizing<String>>,
    pub(crate) scope: Option<Zeroizing<String>>,
    pub(crate) authorization_details_present: bool,
}

pub(crate) struct TokenErrorResponseFields {
    pub(crate) error: Zeroizing<String>,
    pub(crate) error_description: Option<Zeroizing<String>>,
    pub(crate) error_uri: Option<Zeroizing<String>>,
}

pub(crate) struct CredentialNonceResponseFields {
    pub(crate) nonce: Zeroizing<String>,
}

pub(crate) fn validate_json(
    input: &[u8],
    max_depth: usize,
    max_nodes: usize,
) -> Result<(), CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth,
        max_nodes,
        nodes: 0,
    };
    scanner.skip_whitespace();
    if scanner.peek() != Some(b'{') {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    scanner.parse_value(0)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    Ok(())
}

pub(crate) fn parse_credential_offer_fields(
    input: &[u8],
    transport_limits: CredentialOfferLimits,
    semantic_limits: CredentialOfferSemanticLimits,
) -> Result<CredentialOfferFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: transport_limits.max_json_depth(),
        max_nodes: transport_limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    let fields = scanner.parse_credential_offer_object(depth, semantic_limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    Ok(fields)
}

pub(crate) fn parse_credential_offer_grant_fields(
    input: &[u8],
    transport_limits: CredentialOfferLimits,
    grant_limits: CredentialOfferGrantLimits,
) -> Result<CredentialOfferGrantFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: transport_limits.max_json_depth(),
        max_nodes: transport_limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    let fields = scanner.parse_offer_for_grants(depth, grant_limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    Ok(fields)
}

pub(crate) fn parse_credential_issuer_metadata_fields(
    input: &[u8],
    limits: CredentialIssuerMetadataLimits,
) -> Result<CredentialIssuerMetadataFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: limits.max_json_depth(),
        max_nodes: limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidMetadata);
    }
    let fields = scanner.parse_credential_issuer_metadata_object(depth, limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidMetadata);
    }
    Ok(fields)
}

pub(crate) fn parse_authorization_server_metadata_fields(
    input: &[u8],
    limits: AuthorizationServerMetadataLimits,
) -> Result<AuthorizationServerMetadataFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: limits.max_json_depth(),
        max_nodes: limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidAuthorizationServerMetadata);
    }
    let fields = scanner.parse_authorization_server_metadata_object(depth, limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidAuthorizationServerMetadata);
    }
    Ok(fields)
}

pub(crate) fn parse_token_response_fields(
    input: &[u8],
    limits: TokenResponseLimits,
) -> Result<TokenResponseFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: limits.max_json_depth(),
        max_nodes: limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidTokenResponse);
    }
    let fields = scanner.parse_token_response_object(depth, limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidTokenResponse);
    }
    Ok(fields)
}

pub(crate) fn parse_token_error_response_fields(
    input: &[u8],
    limits: TokenErrorResponseLimits,
) -> Result<TokenErrorResponseFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: limits.max_json_depth(),
        max_nodes: limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidTokenErrorResponse);
    }
    let fields = scanner.parse_token_error_response_object(depth, limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidTokenErrorResponse);
    }
    Ok(fields)
}

pub(crate) fn parse_credential_nonce_response_fields(
    input: &[u8],
    limits: CredentialNonceResponseLimits,
) -> Result<CredentialNonceResponseFields, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: limits.max_json_depth(),
        max_nodes: limits.max_json_nodes(),
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(CredentialOfferError::InvalidCredentialNonceResponse);
    }
    let fields = scanner.parse_credential_nonce_response_object(depth, limits)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidCredentialNonceResponse);
    }
    Ok(fields)
}

struct Scanner<'a> {
    input: &'a [u8],
    cursor: usize,
    max_depth: usize,
    max_nodes: usize,
    nodes: usize,
}

impl Scanner<'_> {
    fn parse_value(&mut self, depth: usize) -> Result<(), CredentialOfferError> {
        self.visit_node()?;

        self.skip_whitespace();
        match self.peek() {
            Some(b'{') => {
                let depth = self.enter_container(depth)?;
                self.cursor += 1;
                self.parse_object(depth)
            }
            Some(b'[') => {
                let depth = self.enter_container(depth)?;
                self.cursor += 1;
                self.parse_array(depth)
            }
            Some(b'"') => {
                let token = self.scan_string()?;
                let _decoded = Zeroizing::new(
                    serde_json::from_slice::<String>(&self.input[token])
                        .map_err(|_| CredentialOfferError::InvalidEmbeddedJson)?,
                );
                Ok(())
            }
            Some(b't') => self.consume_literal(b"true"),
            Some(b'f') => self.consume_literal(b"false"),
            Some(b'n') => self.consume_literal(b"null"),
            Some(b'-' | b'0'..=b'9') => self.scan_number(),
            _ => Err(CredentialOfferError::InvalidEmbeddedJson),
        }
    }

    fn parse_credential_offer_object(
        &mut self,
        depth: usize,
        limits: CredentialOfferSemanticLimits,
    ) -> Result<CredentialOfferFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidOfferFields);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut credential_issuer = None;
        let mut credential_configuration_ids = None;
        let mut grants_present = false;

        loop {
            self.skip_whitespace();
            if self.peek() != Some(b'"') {
                return Err(CredentialOfferError::InvalidEmbeddedJson);
            }
            let token = self.scan_string()?;
            let name = Zeroizing::new(
                serde_json::from_slice::<String>(&self.input[token])
                    .map_err(|_| CredentialOfferError::InvalidEmbeddedJson)?,
            );
            if names
                .iter()
                .any(|existing| existing.as_str() == name.as_str())
            {
                return Err(CredentialOfferError::DuplicateJsonProperty);
            }
            names.push(name);

            self.skip_whitespace();
            if !self.consume_if(b':') {
                return Err(CredentialOfferError::InvalidEmbeddedJson);
            }
            match names.last().map(|name| name.as_str()) {
                Some("credential_issuer") => {
                    if credential_issuer.is_some() {
                        return Err(CredentialOfferError::InvalidOfferFields);
                    }
                    credential_issuer = Some(self.parse_bounded_string(
                        limits.max_credential_issuer_bytes(),
                        CredentialOfferError::InvalidOfferFields,
                        CredentialOfferError::IssuerTooLarge,
                    )?);
                }
                Some("credential_configuration_ids") => {
                    if credential_configuration_ids.is_some() {
                        return Err(CredentialOfferError::InvalidOfferFields);
                    }
                    credential_configuration_ids =
                        Some(self.parse_configuration_ids(depth, limits)?);
                }
                Some("grants") => {
                    self.skip_whitespace();
                    if self.peek() != Some(b'{') {
                        return Err(CredentialOfferError::InvalidGrants);
                    }
                    grants_present = true;
                    self.parse_value(depth)?;
                }
                Some(_) => self.parse_value(depth)?,
                None => return Err(CredentialOfferError::InvalidEmbeddedJson),
            }

            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b'}') => {
                    self.cursor += 1;
                    break;
                }
                _ => return Err(CredentialOfferError::InvalidEmbeddedJson),
            }
        }

        Ok(CredentialOfferFields {
            credential_issuer: credential_issuer.ok_or(CredentialOfferError::InvalidOfferFields)?,
            credential_configuration_ids: credential_configuration_ids
                .ok_or(CredentialOfferError::InvalidConfigurationIds)?,
            grants_present,
        })
    }

    fn parse_offer_for_grants(
        &mut self,
        depth: usize,
        limits: CredentialOfferGrantLimits,
    ) -> Result<CredentialOfferGrantFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Ok(CredentialOfferGrantFields {
                authorization_code: None,
                pre_authorized_code: None,
            });
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut grant_fields = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            if name.as_str() == "grants" {
                if grant_fields.is_some() {
                    return Err(CredentialOfferError::InvalidGrants);
                }
                grant_fields = Some(self.parse_known_grants(depth, limits)?);
            } else {
                self.parse_value(depth)?;
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(grant_fields.unwrap_or(CredentialOfferGrantFields {
            authorization_code: None,
            pre_authorized_code: None,
        }))
    }

    fn parse_credential_issuer_metadata_object(
        &mut self,
        depth: usize,
        limits: CredentialIssuerMetadataLimits,
    ) -> Result<CredentialIssuerMetadataFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidMetadata);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut credential_issuer = None;
        let mut authorization_servers = None;
        let mut authorization_servers_present = false;
        let mut credential_endpoint = None;
        let mut credential_configurations = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "credential_issuer" => {
                    credential_issuer = Some(self.parse_nonempty_bounded_string(
                        limits.max_credential_issuer_bytes(),
                        CredentialOfferError::InvalidMetadata,
                        CredentialOfferError::IssuerTooLarge,
                    )?);
                }
                "authorization_servers" => {
                    authorization_servers_present = true;
                    authorization_servers = Some(self.parse_authorization_servers(depth, limits)?);
                }
                "credential_endpoint" => {
                    credential_endpoint = Some(self.parse_nonempty_bounded_string(
                        limits.max_credential_endpoint_bytes(),
                        CredentialOfferError::InvalidMetadata,
                        CredentialOfferError::CredentialEndpointTooLarge,
                    )?);
                }
                "credential_configurations_supported" => {
                    credential_configurations =
                        Some(self.parse_credential_configurations(depth, limits)?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(CredentialIssuerMetadataFields {
            credential_issuer: credential_issuer.ok_or(CredentialOfferError::InvalidMetadata)?,
            authorization_servers: if authorization_servers_present {
                authorization_servers
            } else {
                None
            },
            credential_endpoint: credential_endpoint
                .ok_or(CredentialOfferError::InvalidMetadata)?,
            credential_configurations: credential_configurations
                .ok_or(CredentialOfferError::InvalidCredentialConfigurations)?,
        })
    }

    fn parse_authorization_server_metadata_object(
        &mut self,
        depth: usize,
        limits: AuthorizationServerMetadataLimits,
    ) -> Result<AuthorizationServerMetadataFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidAuthorizationServerMetadata);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut issuer = None;
        let mut authorization_endpoint = None;
        let mut token_endpoint = None;
        let mut grant_types_supported = None;
        let mut grant_types_present = false;
        let mut anonymous_pre_authorized_access = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "issuer" => {
                    issuer = Some(self.parse_nonempty_bounded_string(
                        limits.max_issuer_bytes(),
                        CredentialOfferError::InvalidAuthorizationServerMetadata,
                        CredentialOfferError::AuthorizationServerTooLarge,
                    )?);
                }
                "authorization_endpoint" => {
                    authorization_endpoint = Some(self.parse_nonempty_bounded_string(
                        limits.max_endpoint_bytes(),
                        CredentialOfferError::InvalidAuthorizationServerMetadata,
                        CredentialOfferError::AuthorizationEndpointTooLarge,
                    )?);
                }
                "token_endpoint" => {
                    token_endpoint = Some(self.parse_nonempty_bounded_string(
                        limits.max_endpoint_bytes(),
                        CredentialOfferError::InvalidAuthorizationServerMetadata,
                        CredentialOfferError::TokenEndpointTooLarge,
                    )?);
                }
                "grant_types_supported" => {
                    grant_types_present = true;
                    grant_types_supported = Some(self.parse_grant_types(depth, limits)?);
                }
                "pre-authorized_grant_anonymous_access_supported" => {
                    anonymous_pre_authorized_access = Some(self.parse_boolean(
                        CredentialOfferError::InvalidAnonymousPreAuthorizedAccess,
                    )?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(AuthorizationServerMetadataFields {
            issuer: issuer.ok_or(CredentialOfferError::InvalidAuthorizationServerMetadata)?,
            authorization_endpoint,
            token_endpoint,
            grant_types_supported: if grant_types_present {
                grant_types_supported
            } else {
                None
            },
            anonymous_pre_authorized_access,
        })
    }

    fn parse_token_response_object(
        &mut self,
        depth: usize,
        limits: TokenResponseLimits,
    ) -> Result<TokenResponseFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidTokenResponse);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut access_token = None;
        let mut token_type = None;
        let mut expires_in = None;
        let mut refresh_token = None;
        let mut scope = None;
        let mut authorization_details_present = false;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "access_token" => {
                    access_token = Some(self.parse_nonempty_bounded_string(
                        limits.max_access_token_bytes(),
                        CredentialOfferError::InvalidAccessToken,
                        CredentialOfferError::AccessTokenTooLarge,
                    )?);
                }
                "token_type" => {
                    token_type = Some(self.parse_nonempty_bounded_string(
                        limits.max_token_type_bytes(),
                        CredentialOfferError::InvalidTokenType,
                        CredentialOfferError::TokenTypeTooLarge,
                    )?);
                }
                "expires_in" => {
                    expires_in = Some(
                        self.parse_non_negative_u64(CredentialOfferError::InvalidTokenExpiresIn)?,
                    );
                }
                "refresh_token" => {
                    refresh_token = Some(self.parse_nonempty_bounded_string(
                        limits.max_refresh_token_bytes(),
                        CredentialOfferError::InvalidRefreshToken,
                        CredentialOfferError::RefreshTokenTooLarge,
                    )?);
                }
                "scope" => {
                    scope = Some(self.parse_nonempty_bounded_string(
                        limits.max_scope_bytes(),
                        CredentialOfferError::InvalidTokenScope,
                        CredentialOfferError::TokenScopeTooLarge,
                    )?);
                }
                "authorization_details" => {
                    authorization_details_present = true;
                    self.parse_value(depth)?;
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(TokenResponseFields {
            access_token: access_token.ok_or(CredentialOfferError::InvalidTokenResponse)?,
            token_type: token_type.ok_or(CredentialOfferError::InvalidTokenResponse)?,
            expires_in,
            refresh_token,
            scope,
            authorization_details_present,
        })
    }

    fn parse_token_error_response_object(
        &mut self,
        depth: usize,
        limits: TokenErrorResponseLimits,
    ) -> Result<TokenErrorResponseFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidTokenErrorResponse);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut error = None;
        let mut error_description = None;
        let mut error_uri = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "error" => {
                    error = Some(self.parse_nonempty_bounded_string(
                        limits.max_error_code_bytes(),
                        CredentialOfferError::InvalidTokenEndpointErrorCode,
                        CredentialOfferError::TokenEndpointErrorCodeTooLarge,
                    )?);
                }
                "error_description" => {
                    error_description = Some(self.parse_nonempty_bounded_string(
                        limits.max_error_description_bytes(),
                        CredentialOfferError::InvalidTokenErrorDescription,
                        CredentialOfferError::TokenErrorDescriptionTooLarge,
                    )?);
                }
                "error_uri" => {
                    error_uri = Some(self.parse_nonempty_bounded_string(
                        limits.max_error_uri_bytes(),
                        CredentialOfferError::InvalidTokenErrorUri,
                        CredentialOfferError::TokenErrorUriTooLarge,
                    )?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(TokenErrorResponseFields {
            error: error.ok_or(CredentialOfferError::InvalidTokenErrorResponse)?,
            error_description,
            error_uri,
        })
    }

    fn parse_credential_nonce_response_object(
        &mut self,
        depth: usize,
        limits: CredentialNonceResponseLimits,
    ) -> Result<CredentialNonceResponseFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidCredentialNonceResponse);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut nonce = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            if name.as_str() == "c_nonce" {
                nonce = Some(self.parse_nonempty_bounded_string(
                    limits.max_nonce_bytes(),
                    CredentialOfferError::InvalidCredentialNonce,
                    CredentialOfferError::CredentialNonceTooLarge,
                )?);
            } else {
                self.parse_value(depth)?;
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(CredentialNonceResponseFields {
            nonce: nonce.ok_or(CredentialOfferError::InvalidCredentialNonceResponse)?,
        })
    }

    fn parse_grant_types(
        &mut self,
        depth: usize,
        limits: AuthorizationServerMetadataLimits,
    ) -> Result<Vec<Zeroizing<String>>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'[') {
            return Err(CredentialOfferError::InvalidGrantTypes);
        }
        self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Err(CredentialOfferError::InvalidGrantTypes);
        }

        let mut grant_types = Vec::new();
        loop {
            if grant_types.len() == limits.max_grant_types() {
                return Err(CredentialOfferError::TooManyGrantTypes);
            }
            let grant_type = self.parse_nonempty_bounded_string(
                limits.max_grant_type_bytes(),
                CredentialOfferError::InvalidGrantTypes,
                CredentialOfferError::GrantTypeTooLarge,
            )?;
            if grant_types
                .iter()
                .any(|existing: &Zeroizing<String>| existing.as_str() == grant_type.as_str())
            {
                return Err(CredentialOfferError::DuplicateGrantType);
            }
            grant_types.push(grant_type);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(grant_types);
                }
                _ => return Err(CredentialOfferError::InvalidGrantTypes),
            }
        }
    }

    fn parse_authorization_servers(
        &mut self,
        depth: usize,
        limits: CredentialIssuerMetadataLimits,
    ) -> Result<Vec<Zeroizing<String>>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'[') {
            return Err(CredentialOfferError::InvalidAuthorizationServers);
        }
        self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Err(CredentialOfferError::InvalidAuthorizationServers);
        }

        let mut servers = Vec::new();
        loop {
            if servers.len() == limits.max_authorization_servers() {
                return Err(CredentialOfferError::TooManyAuthorizationServers);
            }
            let server = self.parse_nonempty_bounded_string(
                limits.max_authorization_server_bytes(),
                CredentialOfferError::InvalidAuthorizationServers,
                CredentialOfferError::AuthorizationServerTooLarge,
            )?;
            if servers
                .iter()
                .any(|existing: &Zeroizing<String>| existing.as_str() == server.as_str())
            {
                return Err(CredentialOfferError::DuplicateAuthorizationServer);
            }
            servers.push(server);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(servers);
                }
                _ => return Err(CredentialOfferError::InvalidMetadata),
            }
        }
    }

    fn parse_credential_configurations(
        &mut self,
        depth: usize,
        limits: CredentialIssuerMetadataLimits,
    ) -> Result<Vec<CredentialConfigurationFields>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidCredentialConfigurations);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidCredentialConfigurations);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut configurations = Vec::new();
        loop {
            if configurations.len() == limits.max_credential_configurations() {
                return Err(CredentialOfferError::TooManyCredentialConfigurations);
            }
            let id = self.parse_unique_member_name(&mut names)?;
            if id.len() > limits.max_credential_configuration_id_bytes() {
                return Err(CredentialOfferError::ConfigurationIdTooLarge);
            }
            self.require_member_separator()?;
            let format = self.parse_credential_configuration(depth, limits)?;
            configurations.push(CredentialConfigurationFields { id, format });
            if self.finish_or_continue_object()? {
                return Ok(configurations);
            }
        }
    }

    fn parse_credential_configuration(
        &mut self,
        depth: usize,
        limits: CredentialIssuerMetadataLimits,
    ) -> Result<Zeroizing<String>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidCredentialConfigurations);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidCredentialFormat);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut format = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            if name.as_str() == "format" {
                format = Some(self.parse_nonempty_bounded_string(
                    limits.max_credential_format_bytes(),
                    CredentialOfferError::InvalidCredentialFormat,
                    CredentialOfferError::CredentialFormatTooLarge,
                )?);
            } else {
                self.parse_value(depth)?;
            }
            if self.finish_or_continue_object()? {
                return format.ok_or(CredentialOfferError::InvalidCredentialFormat);
            }
        }
    }

    fn parse_known_grants(
        &mut self,
        depth: usize,
        limits: CredentialOfferGrantLimits,
    ) -> Result<CredentialOfferGrantFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidGrants);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Ok(CredentialOfferGrantFields {
                authorization_code: None,
                pre_authorized_code: None,
            });
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut authorization_code = None;
        let mut pre_authorized_code = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                AUTHORIZATION_CODE_GRANT => {
                    authorization_code = Some(self.parse_authorization_code_grant(depth, limits)?);
                }
                PRE_AUTHORIZED_CODE_GRANT => {
                    pre_authorized_code =
                        Some(self.parse_pre_authorized_code_grant(depth, limits)?);
                }
                _ => {
                    self.skip_whitespace();
                    if self.peek() != Some(b'{') {
                        return Err(CredentialOfferError::InvalidGrants);
                    }
                    self.parse_value(depth)?;
                }
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }
        Ok(CredentialOfferGrantFields {
            authorization_code,
            pre_authorized_code,
        })
    }

    fn parse_authorization_code_grant(
        &mut self,
        depth: usize,
        limits: CredentialOfferGrantLimits,
    ) -> Result<AuthorizationCodeGrantFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidAuthorizationCodeGrant);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Ok(AuthorizationCodeGrantFields {
                issuer_state: None,
                authorization_server: None,
            });
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut issuer_state = None;
        let mut authorization_server = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "issuer_state" => {
                    issuer_state = Some(self.parse_nonempty_bounded_string(
                        limits.max_issuer_state_bytes(),
                        CredentialOfferError::InvalidAuthorizationCodeGrant,
                        CredentialOfferError::IssuerStateTooLarge,
                    )?);
                }
                "authorization_server" => {
                    authorization_server = Some(self.parse_nonempty_bounded_string(
                        limits.max_authorization_server_bytes(),
                        CredentialOfferError::InvalidAuthorizationCodeGrant,
                        CredentialOfferError::AuthorizationServerTooLarge,
                    )?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }
        Ok(AuthorizationCodeGrantFields {
            issuer_state,
            authorization_server,
        })
    }

    fn parse_pre_authorized_code_grant(
        &mut self,
        depth: usize,
        limits: CredentialOfferGrantLimits,
    ) -> Result<PreAuthorizedCodeGrantFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidPreAuthorizedCodeGrant);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidPreAuthorizedCodeGrant);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut pre_authorized_code = None;
        let mut transaction_code = None;
        let mut authorization_server = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "pre-authorized_code" => {
                    pre_authorized_code = Some(self.parse_nonempty_bounded_string(
                        limits.max_pre_authorized_code_bytes(),
                        CredentialOfferError::InvalidPreAuthorizedCodeGrant,
                        CredentialOfferError::PreAuthorizedCodeTooLarge,
                    )?);
                }
                "tx_code" => {
                    transaction_code = Some(self.parse_transaction_code(depth, limits)?);
                }
                "authorization_server" => {
                    authorization_server = Some(self.parse_nonempty_bounded_string(
                        limits.max_authorization_server_bytes(),
                        CredentialOfferError::InvalidPreAuthorizedCodeGrant,
                        CredentialOfferError::AuthorizationServerTooLarge,
                    )?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(PreAuthorizedCodeGrantFields {
            pre_authorized_code: pre_authorized_code
                .ok_or(CredentialOfferError::InvalidPreAuthorizedCodeGrant)?,
            transaction_code,
            authorization_server,
        })
    }

    fn parse_transaction_code(
        &mut self,
        depth: usize,
        limits: CredentialOfferGrantLimits,
    ) -> Result<TransactionCodeFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidTransactionCode);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Ok(TransactionCodeFields {
                input_mode: None,
                length: None,
                description: None,
            });
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut input_mode = None;
        let mut length = None;
        let mut description = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "input_mode" => {
                    let value = self.parse_nonempty_bounded_string(
                        "numeric".len(),
                        CredentialOfferError::InvalidTransactionCodeMode,
                        CredentialOfferError::InvalidTransactionCodeMode,
                    )?;
                    if !matches!(value.as_str(), "numeric" | "text") {
                        return Err(CredentialOfferError::InvalidTransactionCodeMode);
                    }
                    input_mode = Some(value);
                }
                "length" => {
                    length =
                        Some(self.parse_positive_integer(limits.max_transaction_code_length())?);
                }
                "description" => {
                    let value = self.parse_nonempty_bounded_string(
                        limits.max_transaction_code_description_bytes(),
                        CredentialOfferError::InvalidTransactionCode,
                        CredentialOfferError::TransactionCodeDescriptionTooLarge,
                    )?;
                    if value.chars().count() > MAX_TRANSACTION_CODE_DESCRIPTION_CHARACTERS {
                        return Err(CredentialOfferError::TransactionCodeDescriptionTooLarge);
                    }
                    description = Some(value);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }
        Ok(TransactionCodeFields {
            input_mode,
            length,
            description,
        })
    }

    fn parse_unique_member_name(
        &mut self,
        names: &mut Vec<Zeroizing<String>>,
    ) -> Result<Zeroizing<String>, CredentialOfferError> {
        self.skip_whitespace();
        if self.peek() != Some(b'"') {
            return Err(CredentialOfferError::InvalidEmbeddedJson);
        }
        let token = self.scan_string()?;
        let name = Zeroizing::new(
            serde_json::from_slice::<String>(&self.input[token])
                .map_err(|_| CredentialOfferError::InvalidEmbeddedJson)?,
        );
        if names
            .iter()
            .any(|existing| existing.as_str() == name.as_str())
        {
            return Err(CredentialOfferError::DuplicateJsonProperty);
        }
        names.push(Zeroizing::new(name.to_string()));
        Ok(name)
    }

    fn require_member_separator(&mut self) -> Result<(), CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b':') {
            Ok(())
        } else {
            Err(CredentialOfferError::InvalidEmbeddedJson)
        }
    }

    fn finish_or_continue_object(&mut self) -> Result<bool, CredentialOfferError> {
        self.skip_whitespace();
        match self.peek() {
            Some(b',') => {
                self.cursor += 1;
                Ok(false)
            }
            Some(b'}') => {
                self.cursor += 1;
                Ok(true)
            }
            _ => Err(CredentialOfferError::InvalidEmbeddedJson),
        }
    }

    fn parse_nonempty_bounded_string(
        &mut self,
        max_bytes: usize,
        invalid: CredentialOfferError,
        too_large: CredentialOfferError,
    ) -> Result<Zeroizing<String>, CredentialOfferError> {
        let value = self.parse_bounded_string(max_bytes, invalid, too_large)?;
        if value.is_empty() {
            return Err(invalid);
        }
        Ok(value)
    }

    fn parse_boolean(
        &mut self,
        invalid_error: CredentialOfferError,
    ) -> Result<bool, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if self.input[self.cursor..].starts_with(b"true") {
            self.cursor += 4;
            Ok(true)
        } else if self.input[self.cursor..].starts_with(b"false") {
            self.cursor += 5;
            Ok(false)
        } else {
            Err(invalid_error)
        }
    }

    fn parse_positive_integer(&mut self, max: usize) -> Result<usize, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.peek().is_some_and(|byte| matches!(byte, b'1'..=b'9')) {
            return Err(CredentialOfferError::InvalidTransactionCodeLength);
        }
        let mut value = 0usize;
        while let Some(digit @ b'0'..=b'9') = self.peek() {
            value = value
                .checked_mul(10)
                .and_then(|current| current.checked_add(usize::from(digit - b'0')))
                .ok_or(CredentialOfferError::TransactionCodeLengthTooLarge)?;
            if value > max {
                return Err(CredentialOfferError::TransactionCodeLengthTooLarge);
            }
            self.cursor += 1;
        }
        if self
            .peek()
            .is_some_and(|byte| matches!(byte, b'.' | b'e' | b'E'))
        {
            return Err(CredentialOfferError::InvalidTransactionCodeLength);
        }
        Ok(value)
    }

    fn parse_non_negative_u64(
        &mut self,
        invalid: CredentialOfferError,
    ) -> Result<u64, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        let start = self.cursor;
        match self.peek() {
            Some(b'0') => {
                self.cursor += 1;
                if self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                    return Err(invalid);
                }
            }
            Some(b'1'..=b'9') => self.consume_digits(),
            _ => return Err(invalid),
        }
        if self
            .peek()
            .is_some_and(|byte| matches!(byte, b'.' | b'e' | b'E'))
        {
            return Err(invalid);
        }
        std::str::from_utf8(&self.input[start..self.cursor])
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .ok_or(invalid)
    }

    fn parse_configuration_ids(
        &mut self,
        depth: usize,
        limits: CredentialOfferSemanticLimits,
    ) -> Result<Vec<Zeroizing<String>>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'[') {
            return Err(CredentialOfferError::InvalidConfigurationIds);
        }
        self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Err(CredentialOfferError::InvalidConfigurationIds);
        }

        let mut ids = Vec::new();
        loop {
            if ids.len() == limits.max_credential_configuration_ids() {
                return Err(CredentialOfferError::TooManyConfigurationIds);
            }
            let id = self.parse_bounded_string(
                limits.max_credential_configuration_id_bytes(),
                CredentialOfferError::InvalidConfigurationIds,
                CredentialOfferError::ConfigurationIdTooLarge,
            )?;
            if ids
                .iter()
                .any(|existing: &Zeroizing<String>| existing.as_str() == id.as_str())
            {
                return Err(CredentialOfferError::DuplicateConfigurationId);
            }
            ids.push(id);

            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(ids);
                }
                _ => return Err(CredentialOfferError::InvalidEmbeddedJson),
            }
        }
    }

    fn parse_bounded_string(
        &mut self,
        max_bytes: usize,
        invalid: CredentialOfferError,
        too_large: CredentialOfferError,
    ) -> Result<Zeroizing<String>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if self.peek() != Some(b'"') {
            return Err(invalid);
        }
        let token = self.scan_string()?;
        let value = Zeroizing::new(
            serde_json::from_slice::<String>(&self.input[token])
                .map_err(|_| CredentialOfferError::InvalidEmbeddedJson)?,
        );
        if value.len() > max_bytes {
            return Err(too_large);
        }
        Ok(value)
    }

    fn visit_node(&mut self) -> Result<(), CredentialOfferError> {
        self.nodes = self.nodes.saturating_add(1);
        if self.nodes > self.max_nodes {
            return Err(CredentialOfferError::JsonTooManyNodes);
        }
        Ok(())
    }

    fn parse_object(&mut self, depth: usize) -> Result<(), CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Ok(());
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        loop {
            self.skip_whitespace();
            if self.peek() != Some(b'"') {
                return Err(CredentialOfferError::InvalidEmbeddedJson);
            }
            let token = self.scan_string()?;
            let name = Zeroizing::new(
                serde_json::from_slice::<String>(&self.input[token])
                    .map_err(|_| CredentialOfferError::InvalidEmbeddedJson)?,
            );
            if names
                .iter()
                .any(|existing| existing.as_str() == name.as_str())
            {
                return Err(CredentialOfferError::DuplicateJsonProperty);
            }
            names.push(name);

            self.skip_whitespace();
            if !self.consume_if(b':') {
                return Err(CredentialOfferError::InvalidEmbeddedJson);
            }
            self.parse_value(depth)?;
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b'}') => {
                    self.cursor += 1;
                    return Ok(());
                }
                _ => return Err(CredentialOfferError::InvalidEmbeddedJson),
            }
        }
    }

    fn parse_array(&mut self, depth: usize) -> Result<(), CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Ok(());
        }

        loop {
            self.parse_value(depth)?;
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(());
                }
                _ => return Err(CredentialOfferError::InvalidEmbeddedJson),
            }
        }
    }

    fn enter_container(&self, depth: usize) -> Result<usize, CredentialOfferError> {
        let child_depth = depth.saturating_add(1);
        if child_depth > self.max_depth {
            return Err(CredentialOfferError::JsonTooDeep);
        }
        Ok(child_depth)
    }

    fn scan_string(&mut self) -> Result<Range<usize>, CredentialOfferError> {
        let start = self.cursor;
        self.cursor += 1;
        while let Some(byte) = self.peek() {
            match byte {
                b'"' => {
                    self.cursor += 1;
                    return Ok(start..self.cursor);
                }
                b'\\' => {
                    self.cursor += 1;
                    if self.peek().is_none() {
                        return Err(CredentialOfferError::InvalidEmbeddedJson);
                    }
                    self.cursor += 1;
                }
                _ => self.cursor += 1,
            }
        }
        Err(CredentialOfferError::InvalidEmbeddedJson)
    }

    fn scan_number(&mut self) -> Result<(), CredentialOfferError> {
        self.consume_if(b'-');
        match self.peek() {
            Some(b'0') => {
                self.cursor += 1;
                if self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                    return Err(CredentialOfferError::InvalidEmbeddedJson);
                }
            }
            Some(b'1'..=b'9') => self.consume_digits(),
            _ => return Err(CredentialOfferError::InvalidEmbeddedJson),
        }

        if self.consume_if(b'.') {
            if !self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                return Err(CredentialOfferError::InvalidEmbeddedJson);
            }
            self.consume_digits();
        }

        if self.peek().is_some_and(|byte| matches!(byte, b'e' | b'E')) {
            self.cursor += 1;
            if self.peek().is_some_and(|byte| matches!(byte, b'+' | b'-')) {
                self.cursor += 1;
            }
            if !self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                return Err(CredentialOfferError::InvalidEmbeddedJson);
            }
            self.consume_digits();
        }
        Ok(())
    }

    fn consume_digits(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.cursor += 1;
        }
    }

    fn consume_literal(&mut self, literal: &[u8]) -> Result<(), CredentialOfferError> {
        if self.input.get(self.cursor..self.cursor + literal.len()) == Some(literal) {
            self.cursor += literal.len();
            Ok(())
        } else {
            Err(CredentialOfferError::InvalidEmbeddedJson)
        }
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek()
            .is_some_and(|byte| matches!(byte, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.cursor += 1;
        }
    }

    fn consume_if(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.cursor).copied()
    }
}

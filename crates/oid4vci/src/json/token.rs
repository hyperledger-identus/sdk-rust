use zeroize::Zeroizing;

use crate::{
    CredentialNonceResponseLimits, CredentialOfferError, TokenAuthorizationDetailsLimits,
    TokenErrorResponseLimits, TokenResponseLimits,
};

use super::{Scanner, parse_root_object};

pub(crate) struct TokenResponseFields {
    pub(crate) access_token: Zeroizing<String>,
    pub(crate) token_type: Zeroizing<String>,
    pub(crate) expires_in: Option<u64>,
    pub(crate) refresh_token: Option<Zeroizing<String>>,
    pub(crate) scope: Option<Zeroizing<String>>,
    pub(crate) authorization_details_present: bool,
}

pub(crate) struct TokenAuthorizationDetailsFields {
    pub(crate) credential_details: Vec<CredentialAuthorizationDetailFields>,
    pub(crate) unknown_type_count: usize,
}

pub(crate) struct CredentialAuthorizationDetailFields {
    pub(crate) credential_configuration_id: Zeroizing<String>,
    pub(crate) credential_identifiers: Vec<Zeroizing<String>>,
}

pub(crate) struct TokenErrorResponseFields {
    pub(crate) error: Zeroizing<String>,
    pub(crate) error_description: Option<Zeroizing<String>>,
    pub(crate) error_uri: Option<Zeroizing<String>>,
}

pub(crate) struct CredentialNonceResponseFields {
    pub(crate) nonce: Zeroizing<String>,
}

pub(crate) fn parse_token_response_fields(
    input: &[u8],
    limits: TokenResponseLimits,
) -> Result<TokenResponseFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidTokenResponse,
        |scanner, depth| scanner.parse_token_response_object(depth, limits),
    )
}

pub(crate) fn parse_token_authorization_details_fields(
    input: &[u8],
    token_limits: TokenResponseLimits,
    limits: TokenAuthorizationDetailsLimits,
) -> Result<TokenAuthorizationDetailsFields, CredentialOfferError> {
    parse_root_object(
        input,
        token_limits.max_json_depth(),
        token_limits.max_json_nodes(),
        CredentialOfferError::InvalidTokenAuthorizationDetails,
        |scanner, depth| scanner.parse_token_authorization_details_object(depth, limits),
    )
}

pub(crate) fn parse_token_error_response_fields(
    input: &[u8],
    limits: TokenErrorResponseLimits,
) -> Result<TokenErrorResponseFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidTokenErrorResponse,
        |scanner, depth| scanner.parse_token_error_response_object(depth, limits),
    )
}

pub(crate) fn parse_credential_nonce_response_fields(
    input: &[u8],
    limits: CredentialNonceResponseLimits,
) -> Result<CredentialNonceResponseFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidCredentialNonceResponse,
        |scanner, depth| scanner.parse_credential_nonce_response_object(depth, limits),
    )
}

impl Scanner<'_> {
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

    fn parse_token_authorization_details_object(
        &mut self,
        depth: usize,
        limits: TokenAuthorizationDetailsLimits,
    ) -> Result<TokenAuthorizationDetailsFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut authorization_details = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            if name.as_str() == "authorization_details" {
                authorization_details =
                    Some(self.parse_token_authorization_details_array(depth, limits)?);
            } else {
                self.parse_value(depth)?;
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        authorization_details.ok_or(CredentialOfferError::InvalidTokenAuthorizationDetails)
    }

    fn parse_token_authorization_details_array(
        &mut self,
        depth: usize,
        limits: TokenAuthorizationDetailsLimits,
    ) -> Result<TokenAuthorizationDetailsFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'[') {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        }

        let mut total = 0usize;
        let mut unknown_type_count = 0usize;
        let mut credential_details: Vec<CredentialAuthorizationDetailFields> = Vec::new();
        loop {
            if total == limits.max_authorization_details() {
                return Err(CredentialOfferError::TooManyTokenAuthorizationDetails);
            }
            total += 1;
            match self.parse_token_authorization_detail(depth, limits)? {
                Some(detail) => {
                    for identifier in &detail.credential_identifiers {
                        if credential_details
                            .iter()
                            .flat_map(|existing| existing.credential_identifiers.iter())
                            .any(|existing| existing.as_str() == identifier.as_str())
                        {
                            return Err(CredentialOfferError::DuplicateCredentialIdentifier);
                        }
                    }
                    credential_details.push(detail);
                }
                None => unknown_type_count += 1,
            }
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    break;
                }
                _ => return Err(CredentialOfferError::InvalidTokenAuthorizationDetails),
            }
        }

        if credential_details.is_empty() {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        }
        Ok(TokenAuthorizationDetailsFields {
            credential_details,
            unknown_type_count,
        })
    }

    fn parse_token_authorization_detail(
        &mut self,
        depth: usize,
        limits: TokenAuthorizationDetailsLimits,
    ) -> Result<Option<CredentialAuthorizationDetailFields>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut detail_type = None;
        let mut configuration_range = None;
        let mut identifiers_range = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "type" => {
                    detail_type = Some(self.parse_nonempty_bounded_string(
                        limits.max_type_bytes(),
                        CredentialOfferError::InvalidTokenAuthorizationDetails,
                        CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
                    )?);
                }
                "credential_configuration_id" => {
                    configuration_range = Some(self.capture_value_range(depth)?);
                }
                "credential_identifiers" => {
                    identifiers_range = Some(self.capture_value_range(depth)?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        let detail_type =
            detail_type.ok_or(CredentialOfferError::InvalidTokenAuthorizationDetails)?;
        if detail_type.as_str() != "openid_credential" {
            return Ok(None);
        }
        let configuration_range =
            configuration_range.ok_or(CredentialOfferError::InvalidTokenAuthorizationDetails)?;
        let identifiers_range =
            identifiers_range.ok_or(CredentialOfferError::InvalidTokenAuthorizationDetails)?;
        let credential_configuration_id = parse_authorization_detail_string(
            &self.input[configuration_range],
            limits.max_credential_configuration_id_bytes(),
        )?;
        let credential_identifiers =
            parse_credential_identifiers(&self.input[identifiers_range], limits)?;
        Ok(Some(CredentialAuthorizationDetailFields {
            credential_configuration_id,
            credential_identifiers,
        }))
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
}

fn parse_authorization_detail_string(
    input: &[u8],
    max_bytes: usize,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: 1,
        max_nodes: 1,
        nodes: 0,
    };
    let value = scanner.parse_nonempty_bounded_string(
        max_bytes,
        CredentialOfferError::InvalidTokenAuthorizationDetails,
        CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
    )?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
    }
    Ok(value)
}

fn parse_credential_identifiers(
    input: &[u8],
    limits: TokenAuthorizationDetailsLimits,
) -> Result<Vec<Zeroizing<String>>, CredentialOfferError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        max_depth: 1,
        max_nodes: limits
            .max_credential_identifiers_per_detail()
            .saturating_add(1),
        nodes: 0,
    };
    scanner.visit_node()?;
    scanner.skip_whitespace();
    if !scanner.consume_if(b'[') {
        return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
    }
    scanner.enter_container(0)?;
    scanner.skip_whitespace();
    if scanner.consume_if(b']') {
        return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
    }

    let mut identifiers = Vec::new();
    loop {
        if identifiers.len() == limits.max_credential_identifiers_per_detail() {
            return Err(CredentialOfferError::TooManyCredentialIdentifiers);
        }
        let identifier = scanner.parse_nonempty_bounded_string(
            limits.max_credential_identifier_bytes(),
            CredentialOfferError::InvalidTokenAuthorizationDetails,
            CredentialOfferError::TokenAuthorizationDetailValueTooLarge,
        )?;
        if identifiers
            .iter()
            .any(|existing: &Zeroizing<String>| existing.as_str() == identifier.as_str())
        {
            return Err(CredentialOfferError::DuplicateCredentialIdentifier);
        }
        identifiers.push(identifier);
        scanner.skip_whitespace();
        match scanner.peek() {
            Some(b',') => scanner.cursor += 1,
            Some(b']') => {
                scanner.cursor += 1;
                break;
            }
            _ => return Err(CredentialOfferError::InvalidTokenAuthorizationDetails),
        }
    }
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(CredentialOfferError::InvalidTokenAuthorizationDetails);
    }
    Ok(identifiers)
}

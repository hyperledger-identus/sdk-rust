use zeroize::Zeroizing;

use crate::{
    AuthorizationServerMetadataLimits, CredentialIssuerMetadataLimits, CredentialOfferError,
};

use super::{Scanner, parse_root_object};

pub(crate) struct CredentialIssuerMetadataFields {
    pub(crate) credential_issuer: Zeroizing<String>,
    pub(crate) authorization_servers: Option<Vec<Zeroizing<String>>>,
    pub(crate) credential_endpoint: Zeroizing<String>,
    pub(crate) nonce_endpoint: Option<Zeroizing<String>>,
    pub(crate) deferred_credential_endpoint: Option<Zeroizing<String>>,
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
    pub(crate) authorization_response_iss_parameter_supported: Option<bool>,
}

pub(crate) fn parse_credential_issuer_metadata_fields(
    input: &[u8],
    limits: CredentialIssuerMetadataLimits,
) -> Result<CredentialIssuerMetadataFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidMetadata,
        |scanner, depth| scanner.parse_credential_issuer_metadata_object(depth, limits),
    )
}

pub(crate) fn parse_authorization_server_metadata_fields(
    input: &[u8],
    limits: AuthorizationServerMetadataLimits,
) -> Result<AuthorizationServerMetadataFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidAuthorizationServerMetadata,
        |scanner, depth| scanner.parse_authorization_server_metadata_object(depth, limits),
    )
}

impl Scanner<'_> {
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
        let mut nonce_endpoint = None;
        let mut deferred_credential_endpoint = None;
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
                "nonce_endpoint" => {
                    nonce_endpoint = Some(self.parse_nonempty_bounded_string(
                        limits.max_credential_endpoint_bytes(),
                        CredentialOfferError::InvalidMetadata,
                        CredentialOfferError::NonceEndpointTooLarge,
                    )?);
                }
                "deferred_credential_endpoint" => {
                    deferred_credential_endpoint = Some(self.parse_nonempty_bounded_string(
                        limits.max_credential_endpoint_bytes(),
                        CredentialOfferError::InvalidMetadata,
                        CredentialOfferError::DeferredCredentialEndpointTooLarge,
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
            nonce_endpoint,
            deferred_credential_endpoint,
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
        let mut authorization_response_iss_parameter_supported = None;
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
                "authorization_response_iss_parameter_supported" => {
                    authorization_response_iss_parameter_supported =
                        Some(self.parse_boolean(
                            CredentialOfferError::InvalidAuthorizationServerMetadata,
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
            authorization_response_iss_parameter_supported,
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
}

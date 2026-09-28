use zeroize::Zeroizing;

use crate::{
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits,
};

use super::{Scanner, parse_root_object};

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

pub(crate) fn parse_credential_offer_fields(
    input: &[u8],
    transport_limits: CredentialOfferLimits,
    semantic_limits: CredentialOfferSemanticLimits,
) -> Result<CredentialOfferFields, CredentialOfferError> {
    parse_root_object(
        input,
        transport_limits.max_json_depth(),
        transport_limits.max_json_nodes(),
        CredentialOfferError::InvalidEmbeddedJson,
        |scanner, depth| scanner.parse_credential_offer_object(depth, semantic_limits),
    )
}

pub(crate) fn parse_credential_offer_grant_fields(
    input: &[u8],
    transport_limits: CredentialOfferLimits,
    grant_limits: CredentialOfferGrantLimits,
) -> Result<CredentialOfferGrantFields, CredentialOfferError> {
    parse_root_object(
        input,
        transport_limits.max_json_depth(),
        transport_limits.max_json_nodes(),
        CredentialOfferError::InvalidEmbeddedJson,
        |scanner, depth| scanner.parse_offer_for_grants(depth, grant_limits),
    )
}

impl Scanner<'_> {
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
}

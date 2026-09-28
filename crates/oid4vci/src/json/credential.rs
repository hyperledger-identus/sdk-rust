use zeroize::Zeroizing;

use crate::{
    CredentialErrorResponseLimits, CredentialOfferError, DeferredCredentialResponseLimits,
    ImmediateCredentialResponseLimits,
};

use super::{Scanner, parse_root_object};

pub(crate) struct CredentialErrorResponseFields {
    pub(crate) error: Zeroizing<String>,
    pub(crate) error_description: Option<Zeroizing<String>>,
}

pub(crate) struct ImmediateCredentialResponseFields {
    pub(crate) credentials: Vec<IssuedCredentialFields>,
    pub(crate) notification_id: Option<Zeroizing<String>>,
}

pub(crate) struct DeferredCredentialResponseFields {
    pub(crate) transaction_id: Zeroizing<String>,
    pub(crate) interval: Zeroizing<String>,
}

pub(crate) struct IssuedCredentialFields {
    pub(crate) exact_json: Zeroizing<String>,
    pub(crate) decoded_string: Option<Zeroizing<String>>,
}

pub(crate) fn parse_credential_error_response_fields(
    input: &[u8],
    limits: CredentialErrorResponseLimits,
) -> Result<CredentialErrorResponseFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidCredentialErrorResponse,
        |scanner, depth| scanner.parse_credential_error_response_object(depth, limits),
    )
}

pub(crate) fn parse_immediate_credential_response_fields(
    input: &[u8],
    limits: ImmediateCredentialResponseLimits,
) -> Result<ImmediateCredentialResponseFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidImmediateCredentialResponse,
        |scanner, depth| scanner.parse_immediate_credential_response_object(depth, limits),
    )
}

pub(crate) fn parse_deferred_credential_response_fields(
    input: &[u8],
    limits: DeferredCredentialResponseLimits,
) -> Result<DeferredCredentialResponseFields, CredentialOfferError> {
    parse_root_object(
        input,
        limits.max_json_depth(),
        limits.max_json_nodes(),
        CredentialOfferError::InvalidDeferredCredentialResponse,
        |scanner, depth| scanner.parse_deferred_credential_response_object(depth, limits),
    )
}

impl Scanner<'_> {
    fn parse_credential_error_response_object(
        &mut self,
        depth: usize,
        limits: CredentialErrorResponseLimits,
    ) -> Result<CredentialErrorResponseFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidCredentialErrorResponse);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut error = None;
        let mut error_description = None;
        loop {
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "error" => {
                    error = Some(self.parse_nonempty_bounded_string(
                        limits.max_error_code_bytes(),
                        CredentialOfferError::InvalidCredentialEndpointErrorCode,
                        CredentialOfferError::CredentialEndpointErrorCodeTooLarge,
                    )?);
                }
                "error_description" => {
                    error_description = Some(self.parse_nonempty_bounded_string(
                        limits.max_error_description_bytes(),
                        CredentialOfferError::InvalidCredentialErrorDescription,
                        CredentialOfferError::CredentialErrorDescriptionTooLarge,
                    )?);
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        Ok(CredentialErrorResponseFields {
            error: error.ok_or(CredentialOfferError::InvalidCredentialErrorResponse)?,
            error_description,
        })
    }

    fn parse_immediate_credential_response_object(
        &mut self,
        depth: usize,
        limits: ImmediateCredentialResponseLimits,
    ) -> Result<ImmediateCredentialResponseFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidImmediateCredentialResponse);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut credentials = None;
        let mut notification_id = None;
        let mut deferred = false;
        let mut interval = false;
        loop {
            if names.len() == limits.max_response_members() {
                return Err(CredentialOfferError::TooManyCredentialResponseMembers);
            }
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "credentials" => {
                    credentials = Some(self.parse_issued_credentials(depth, limits)?);
                }
                "notification_id" => {
                    notification_id = Some(self.parse_nonempty_bounded_string(
                        limits.max_notification_id_bytes(),
                        CredentialOfferError::InvalidCredentialNotificationId,
                        CredentialOfferError::CredentialNotificationIdTooLarge,
                    )?);
                }
                "transaction_id" => {
                    deferred = true;
                    self.parse_value(depth)?;
                }
                "interval" => {
                    interval = true;
                    self.parse_value(depth)?;
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        if deferred {
            return Err(CredentialOfferError::DeferredCredentialResponseUnsupported);
        }
        if interval {
            return Err(CredentialOfferError::InvalidImmediateCredentialResponse);
        }
        Ok(ImmediateCredentialResponseFields {
            credentials: credentials
                .ok_or(CredentialOfferError::InvalidImmediateCredentialResponse)?,
            notification_id,
        })
    }

    fn parse_deferred_credential_response_object(
        &mut self,
        depth: usize,
        limits: DeferredCredentialResponseLimits,
    ) -> Result<DeferredCredentialResponseFields, CredentialOfferError> {
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidDeferredCredentialResponse);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut transaction_id = None;
        let mut interval = None;
        let mut branch_conflict = false;
        loop {
            if names.len() == limits.max_response_members() {
                return Err(CredentialOfferError::TooManyDeferredCredentialResponseMembers);
            }
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            match name.as_str() {
                "transaction_id" => {
                    transaction_id = Some(self.parse_nonempty_bounded_string(
                        limits.max_transaction_id_bytes(),
                        CredentialOfferError::InvalidDeferredTransactionId,
                        CredentialOfferError::DeferredTransactionIdTooLarge,
                    )?);
                }
                "interval" => {
                    interval =
                        Some(self.parse_positive_bounded_json_number(limits.max_interval_bytes())?);
                }
                "credentials" | "notification_id" => {
                    branch_conflict = true;
                    self.parse_value(depth)?;
                }
                _ => self.parse_value(depth)?,
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        if branch_conflict {
            return Err(CredentialOfferError::DeferredCredentialResponseBranchConflict);
        }
        Ok(DeferredCredentialResponseFields {
            transaction_id: transaction_id
                .ok_or(CredentialOfferError::InvalidDeferredCredentialResponse)?,
            interval: interval.ok_or(CredentialOfferError::InvalidDeferredCredentialResponse)?,
        })
    }

    fn parse_issued_credentials(
        &mut self,
        depth: usize,
        limits: ImmediateCredentialResponseLimits,
    ) -> Result<Vec<IssuedCredentialFields>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'[') {
            return Err(CredentialOfferError::InvalidImmediateCredentialResponse);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Err(CredentialOfferError::InvalidImmediateCredentialResponse);
        }

        let mut credentials = Vec::new();
        let mut total_bytes = 0usize;
        loop {
            if credentials.len() == limits.max_credentials() {
                return Err(CredentialOfferError::TooManyIssuedCredentials);
            }
            let credential = self.parse_issued_credential(depth, limits)?;
            total_bytes = total_bytes
                .checked_add(credential.exact_json.len())
                .ok_or(CredentialOfferError::IssuedCredentialsTooLarge)?;
            if total_bytes > limits.max_total_credential_bytes() {
                return Err(CredentialOfferError::IssuedCredentialsTooLarge);
            }
            credentials.push(credential);

            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(credentials);
                }
                _ => return Err(CredentialOfferError::InvalidImmediateCredentialResponse),
            }
        }
    }

    fn parse_issued_credential(
        &mut self,
        depth: usize,
        limits: ImmediateCredentialResponseLimits,
    ) -> Result<IssuedCredentialFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        if !self.consume_if(b'{') {
            return Err(CredentialOfferError::InvalidIssuedCredential);
        }
        let depth = self.enter_container(depth)?;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Err(CredentialOfferError::InvalidIssuedCredential);
        }

        let mut names: Vec<Zeroizing<String>> = Vec::new();
        let mut credential = None;
        loop {
            if names.len() == limits.max_credential_members() {
                return Err(CredentialOfferError::TooManyIssuedCredentialMembers);
            }
            let name = self.parse_unique_member_name(&mut names)?;
            self.require_member_separator()?;
            if name.as_str() == "credential" {
                credential = Some(self.parse_issued_credential_value(depth, limits)?);
            } else {
                self.parse_value(depth)?;
            }
            if self.finish_or_continue_object()? {
                break;
            }
        }

        credential.ok_or(CredentialOfferError::InvalidIssuedCredential)
    }

    fn parse_issued_credential_value(
        &mut self,
        depth: usize,
        limits: ImmediateCredentialResponseLimits,
    ) -> Result<IssuedCredentialFields, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        let start = self.cursor;
        let decoded_string = match self.peek() {
            Some(b'"') => {
                let token = self.scan_string()?;
                Some(Zeroizing::new(
                    serde_json::from_slice::<String>(&self.input[token])
                        .map_err(|_| CredentialOfferError::InvalidIssuedCredential)?,
                ))
            }
            Some(b'{') => {
                let depth = self.enter_container(depth)?;
                self.cursor += 1;
                self.parse_object(depth)?;
                None
            }
            _ => return Err(CredentialOfferError::InvalidIssuedCredential),
        };
        let exact = self
            .input
            .get(start..self.cursor)
            .ok_or(CredentialOfferError::InvalidIssuedCredential)?;
        if exact.len() > limits.max_credential_bytes() {
            return Err(CredentialOfferError::IssuedCredentialTooLarge);
        }
        let exact_json = Zeroizing::new(
            std::str::from_utf8(exact)
                .map_err(|_| CredentialOfferError::InvalidIssuedCredential)?
                .to_owned(),
        );
        Ok(IssuedCredentialFields {
            exact_json,
            decoded_string,
        })
    }
}

use std::ops::Range;

use zeroize::Zeroizing;

use crate::{CredentialOfferError, CredentialOfferLimits, CredentialOfferSemanticLimits};

pub(crate) struct CredentialOfferFields {
    pub(crate) credential_issuer: Zeroizing<String>,
    pub(crate) credential_configuration_ids: Vec<Zeroizing<String>>,
    pub(crate) grants_present: bool,
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

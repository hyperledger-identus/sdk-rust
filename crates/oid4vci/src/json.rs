use std::ops::Range;

use zeroize::Zeroizing;

use crate::CredentialOfferError;

pub(crate) mod credential;
pub(crate) mod metadata;
pub(crate) mod offer;
pub(crate) mod token;

pub(crate) use credential::{
    parse_credential_error_response_fields, parse_deferred_credential_response_fields,
    parse_immediate_credential_response_fields,
};
pub(crate) use metadata::{
    parse_authorization_server_metadata_fields, parse_credential_issuer_metadata_fields,
};
pub(crate) use offer::{
    AuthorizationCodeGrantFields, PreAuthorizedCodeGrantFields, TransactionCodeFields,
    parse_credential_offer_fields, parse_credential_offer_grant_fields,
};
pub(crate) use token::{
    parse_credential_nonce_response_fields, parse_token_authorization_details_fields,
    parse_token_error_response_fields, parse_token_response_fields,
};

pub(crate) fn validate_json(
    input: &[u8],
    max_depth: usize,
    max_nodes: usize,
) -> Result<(), CredentialOfferError> {
    let mut scanner = Scanner::new(input, max_depth, max_nodes);
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

fn parse_root_object<T>(
    input: &[u8],
    max_depth: usize,
    max_nodes: usize,
    invalid: CredentialOfferError,
    parse: impl FnOnce(&mut Scanner<'_>, usize) -> Result<T, CredentialOfferError>,
) -> Result<T, CredentialOfferError> {
    let mut scanner = Scanner::new(input, max_depth, max_nodes);
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let depth = scanner.enter_container(0)?;
    if !scanner.consume_if(b'{') {
        return Err(invalid);
    }
    let value = parse(&mut scanner, depth)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(invalid);
    }
    Ok(value)
}

struct Scanner<'a> {
    input: &'a [u8],
    cursor: usize,
    max_depth: usize,
    max_nodes: usize,
    nodes: usize,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a [u8], max_depth: usize, max_nodes: usize) -> Self {
        Self {
            input,
            cursor: 0,
            max_depth,
            max_nodes,
            nodes: 0,
        }
    }

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

    fn capture_value_range(&mut self, depth: usize) -> Result<Range<usize>, CredentialOfferError> {
        self.skip_whitespace();
        let start = self.cursor;
        self.parse_value(depth)?;
        Ok(start..self.cursor)
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

    fn parse_positive_bounded_json_number(
        &mut self,
        max_bytes: usize,
    ) -> Result<Zeroizing<String>, CredentialOfferError> {
        self.visit_node()?;
        self.skip_whitespace();
        let start = self.cursor;
        if self.peek() == Some(b'-') {
            return Err(CredentialOfferError::InvalidDeferredCredentialInterval);
        }
        self.scan_number()
            .map_err(|_| CredentialOfferError::InvalidDeferredCredentialInterval)?;
        if !self.input[self.cursor..]
            .iter()
            .copied()
            .find(|byte| !matches!(byte, b' ' | b'\n' | b'\r' | b'\t'))
            .is_some_and(|byte| matches!(byte, b',' | b'}'))
        {
            return Err(CredentialOfferError::InvalidDeferredCredentialInterval);
        }
        let value = &self.input[start..self.cursor];
        if value.len() > max_bytes {
            return Err(CredentialOfferError::DeferredCredentialIntervalTooLarge);
        }
        let mantissa = value
            .split(|byte| matches!(byte, b'e' | b'E'))
            .next()
            .unwrap_or(value);
        if !mantissa.iter().any(|byte| matches!(byte, b'1'..=b'9')) {
            return Err(CredentialOfferError::InvalidDeferredCredentialInterval);
        }
        let value = std::str::from_utf8(value)
            .map_err(|_| CredentialOfferError::InvalidDeferredCredentialInterval)?;
        Ok(Zeroizing::new(value.to_owned()))
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

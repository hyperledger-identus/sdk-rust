use std::ops::Range;

use zeroize::Zeroizing;

use crate::CredentialOfferError;

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

struct Scanner<'a> {
    input: &'a [u8],
    cursor: usize,
    max_depth: usize,
    max_nodes: usize,
    nodes: usize,
}

impl Scanner<'_> {
    fn parse_value(&mut self, depth: usize) -> Result<(), CredentialOfferError> {
        self.nodes = self.nodes.saturating_add(1);
        if self.nodes > self.max_nodes {
            return Err(CredentialOfferError::JsonTooManyNodes);
        }

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

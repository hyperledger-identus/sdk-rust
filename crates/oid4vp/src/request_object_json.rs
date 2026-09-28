use zeroize::Zeroizing;

use crate::{Oid4vpError, RequestObjectValidationLimits};

#[derive(Default)]
pub(crate) struct RequestObjectClaims {
    pub(crate) client_id: Option<Zeroizing<String>>,
    pub(crate) wallet_nonce: Option<Zeroizing<String>>,
}

pub(crate) fn parse_request_object_claims(
    input: &[u8],
    limits: RequestObjectValidationLimits,
) -> Result<RequestObjectClaims, Oid4vpError> {
    let mut scanner = Scanner {
        input,
        cursor: 0,
        limits,
        nodes: 0,
    };
    scanner.skip_whitespace();
    scanner.visit_node()?;
    let claims = scanner.parse_object(1, true)?;
    scanner.skip_whitespace();
    if scanner.cursor != input.len() {
        return Err(Oid4vpError::InvalidRequestObjectPayload);
    }
    claims.ok_or(Oid4vpError::InvalidRequestObjectPayload)
}

struct Scanner<'input> {
    input: &'input [u8],
    cursor: usize,
    limits: RequestObjectValidationLimits,
    nodes: usize,
}

impl Scanner<'_> {
    fn parse_object(
        &mut self,
        depth: usize,
        capture_claims: bool,
    ) -> Result<Option<RequestObjectClaims>, Oid4vpError> {
        self.ensure_depth(depth)?;
        self.expect(b'{')?;
        self.skip_whitespace();

        let mut names = Vec::<Zeroizing<String>>::new();
        let mut members = 0_usize;
        let mut claims = RequestObjectClaims::default();
        if self.consume_if(b'}') {
            return Ok(capture_claims.then_some(claims));
        }

        loop {
            members = members
                .checked_add(1)
                .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
            if members > self.limits.max_object_members() {
                return Err(Oid4vpError::InvalidRequestObjectPayload);
            }
            self.parse_object_member(depth, capture_claims, &mut names, &mut claims)?;
            self.skip_whitespace();
            if self.consume_if(b'}') {
                break;
            }
            self.expect(b',')?;
            self.skip_whitespace();
        }

        Ok(capture_claims.then_some(claims))
    }

    fn parse_object_member(
        &mut self,
        depth: usize,
        capture_claims: bool,
        names: &mut Vec<Zeroizing<String>>,
        claims: &mut RequestObjectClaims,
    ) -> Result<(), Oid4vpError> {
        self.visit_node()?;
        let name = self.parse_string()?;
        if names
            .iter()
            .any(|existing| existing.as_str() == name.as_str())
        {
            return Err(claim_error(&name));
        }
        self.skip_whitespace();
        self.expect(b':')?;
        self.skip_whitespace();

        if capture_claims && matches!(name.as_str(), "client_id" | "wallet_nonce") {
            let value = self.parse_claim_string(&name)?;
            if name.as_str() == "client_id" {
                claims.client_id = Some(value);
            } else {
                claims.wallet_nonce = Some(value);
            }
        } else {
            self.parse_value(depth)?;
        }
        names.push(name);
        Ok(())
    }

    fn parse_claim_string(
        &mut self,
        name: &Zeroizing<String>,
    ) -> Result<Zeroizing<String>, Oid4vpError> {
        self.visit_node()?;
        let value = self.parse_string().map_err(|_| claim_error(name))?;
        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(claim_error(name));
        }
        Ok(value)
    }

    fn parse_array(&mut self, depth: usize) -> Result<(), Oid4vpError> {
        self.ensure_depth(depth)?;
        self.expect(b'[')?;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Ok(());
        }
        loop {
            self.parse_value(depth)?;
            self.skip_whitespace();
            if self.consume_if(b']') {
                return Ok(());
            }
            self.expect(b',')?;
            self.skip_whitespace();
        }
    }

    fn parse_value(&mut self, parent_depth: usize) -> Result<(), Oid4vpError> {
        self.visit_node()?;
        match self.peek() {
            Some(b'{') => {
                self.parse_object(parent_depth + 1, false)?;
                Ok(())
            }
            Some(b'[') => self.parse_array(parent_depth + 1),
            Some(b'"') => self.parse_string().map(drop),
            Some(b't') => self.consume_literal(b"true"),
            Some(b'f') => self.consume_literal(b"false"),
            Some(b'n') => self.consume_literal(b"null"),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            _ => Err(Oid4vpError::InvalidRequestObjectPayload),
        }
    }

    fn parse_string(&mut self) -> Result<Zeroizing<String>, Oid4vpError> {
        if self.peek() != Some(b'"') {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        }
        let start = self.cursor;
        self.cursor += 1;
        let mut escaped = false;
        while let Some(byte) = self.peek() {
            self.cursor += 1;
            if escaped {
                escaped = false;
                continue;
            }
            match byte {
                b'\\' => escaped = true,
                b'"' => {
                    let value: String = serde_json::from_slice(&self.input[start..self.cursor])
                        .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)?;
                    if value.len() > self.limits.max_string_bytes() {
                        return Err(Oid4vpError::InvalidRequestObjectPayload);
                    }
                    return Ok(Zeroizing::new(value));
                }
                0x00..=0x1f => return Err(Oid4vpError::InvalidRequestObjectPayload),
                _ => {}
            }
        }
        Err(Oid4vpError::InvalidRequestObjectPayload)
    }

    fn parse_number(&mut self) -> Result<(), Oid4vpError> {
        let start = self.cursor;
        while let Some(byte) = self.peek() {
            if byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}') {
                break;
            }
            self.cursor += 1;
        }
        if start == self.cursor {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        }
        serde_json::from_slice::<serde_json::Number>(&self.input[start..self.cursor])
            .map(|_| ())
            .map_err(|_| Oid4vpError::InvalidRequestObjectPayload)
    }

    fn consume_literal(&mut self, literal: &[u8]) -> Result<(), Oid4vpError> {
        if self.input.get(self.cursor..self.cursor + literal.len()) == Some(literal) {
            self.cursor += literal.len();
            Ok(())
        } else {
            Err(Oid4vpError::InvalidRequestObjectPayload)
        }
    }

    fn visit_node(&mut self) -> Result<(), Oid4vpError> {
        self.nodes = self
            .nodes
            .checked_add(1)
            .ok_or(Oid4vpError::InvalidRequestObjectPayload)?;
        if self.nodes > self.limits.max_json_nodes() {
            return Err(Oid4vpError::InvalidRequestObjectPayload);
        }
        Ok(())
    }

    fn ensure_depth(&self, depth: usize) -> Result<(), Oid4vpError> {
        if depth > self.limits.max_json_depth() {
            Err(Oid4vpError::InvalidRequestObjectPayload)
        } else {
            Ok(())
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.cursor += 1;
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), Oid4vpError> {
        if self.consume_if(expected) {
            Ok(())
        } else {
            Err(Oid4vpError::InvalidRequestObjectPayload)
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

fn claim_error(name: &str) -> Oid4vpError {
    match name {
        "client_id" => Oid4vpError::RequestObjectClientIdMismatch,
        "wallet_nonce" => Oid4vpError::RequestObjectWalletNonceMismatch,
        _ => Oid4vpError::InvalidRequestObjectPayload,
    }
}

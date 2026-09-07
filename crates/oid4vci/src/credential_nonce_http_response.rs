use crate::{
    CredentialNonceHttpResponseLimits, CredentialNonceRequest, CredentialNonceResponseCore,
    CredentialOfferError,
};

impl CredentialNonceRequest {
    /// Validate caller-supplied Final Credential Nonce HTTP response metadata.
    ///
    /// The caller remains responsible for HTTP execution, response origin,
    /// network policy, DPoP handling, and nonce lifecycle policy.
    pub fn validate_response(
        &self,
        status_code: u16,
        content_type: &str,
        cache_control: &str,
        body: &str,
        limits: CredentialNonceHttpResponseLimits,
    ) -> Result<CredentialNonceResponseCore, CredentialOfferError> {
        if !(200..=299).contains(&status_code) {
            return Err(CredentialOfferError::InvalidCredentialNonceHttpStatus);
        }
        if content_type.len() > limits.max_content_type_bytes() {
            return Err(CredentialOfferError::CredentialNonceContentTypeTooLarge);
        }
        if !is_application_json(content_type.as_bytes()) {
            return Err(CredentialOfferError::InvalidCredentialNonceContentType);
        }
        if cache_control.len() > limits.max_cache_control_bytes() {
            return Err(CredentialOfferError::CredentialNonceCacheControlTooLarge);
        }
        if !has_bare_no_store(cache_control.as_bytes()) {
            return Err(CredentialOfferError::InvalidCredentialNonceCacheControl);
        }
        CredentialNonceResponseCore::parse(body, limits.response_limits())
    }
}

fn is_application_json(bytes: &[u8]) -> bool {
    let mut cursor = Cursor::new(bytes);
    cursor.ows();
    let Some(kind) = cursor.token() else {
        return false;
    };
    if !cursor.take(b'/') {
        return false;
    }
    let Some(subtype) = cursor.token() else {
        return false;
    };
    if !kind.eq_ignore_ascii_case(b"application") || !subtype.eq_ignore_ascii_case(b"json") {
        return false;
    }

    loop {
        cursor.ows();
        if cursor.done() {
            return true;
        }
        if !cursor.take(b';') {
            return false;
        }
        cursor.ows();
        if cursor.token().is_none() || !cursor.take(b'=') || !cursor.token_or_quoted() {
            return false;
        }
    }
}

fn has_bare_no_store(bytes: &[u8]) -> bool {
    let mut cursor = Cursor::new(bytes);
    let mut found = false;

    loop {
        cursor.ows();
        while cursor.take(b',') {
            cursor.ows();
        }
        if cursor.done() {
            return found;
        }

        let Some(name) = cursor.token() else {
            return false;
        };
        let argument_boundary = cursor.index;
        cursor.ows();
        if cursor.peek() == Some(b'=') && cursor.index != argument_boundary {
            return false;
        }
        let has_argument = if cursor.take(b'=') {
            if !cursor.token_or_quoted() {
                return false;
            }
            true
        } else {
            false
        };
        if name.eq_ignore_ascii_case(b"no-store") && !has_argument {
            found = true;
        }

        cursor.ows();
        if cursor.done() {
            return found;
        }
        if !cursor.take(b',') {
            return false;
        }
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    fn done(&self) -> bool {
        self.index == self.bytes.len()
    }

    fn ows(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t')) {
            self.index += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    fn take(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&mut self) -> Option<&'a [u8]> {
        let start = self.index;
        while self.peek().is_some_and(is_tchar) {
            self.index += 1;
        }
        (self.index > start).then(|| &self.bytes[start..self.index])
    }

    fn token_or_quoted(&mut self) -> bool {
        if self.peek() == Some(b'"') {
            self.quoted()
        } else {
            self.token().is_some()
        }
    }

    fn quoted(&mut self) -> bool {
        if !self.take(b'"') {
            return false;
        }
        loop {
            let Some(byte) = self.peek() else {
                return false;
            };
            match byte {
                b'"' => {
                    self.index += 1;
                    return true;
                }
                b'\\' => {
                    self.index += 1;
                    let Some(escaped) = self.peek() else {
                        return false;
                    };
                    if !is_quoted_pair_byte(escaped) {
                        return false;
                    }
                    self.index += 1;
                }
                value if is_qdtext(value) => self.index += 1,
                _ => return false,
            }
        }
    }
}

fn is_tchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

fn is_qdtext(byte: u8) -> bool {
    matches!(byte, b'\t' | b' ' | b'!' | 0x23..=0x5b | 0x5d..=0x7e | 0x80..=0xff)
}

fn is_quoted_pair_byte(byte: u8) -> bool {
    matches!(byte, b'\t' | b' ' | 0x21..=0x7e | 0x80..=0xff)
}

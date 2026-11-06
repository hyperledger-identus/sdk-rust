//! Strict, allocation-bounded HTTP field-value validation shared by OpenID protocols.

/// Return whether a field value is a syntactically valid `application/json` media type.
///
/// Optional parameters are accepted only when each has a unique token name and
/// a valid token or quoted-string value.
pub fn is_application_json(bytes: &[u8]) -> bool {
    let mut cursor = Cursor::new(bytes);
    let mut parameter_names: Vec<&[u8]> = Vec::new();
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
        if cursor.done() || cursor.peek() == Some(b';') {
            continue;
        }
        let Some(parameter_name) = cursor.token() else {
            return false;
        };
        if parameter_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(parameter_name))
        {
            return false;
        }
        parameter_names.push(parameter_name);
        if !cursor.take(b'=') || !cursor.token_or_quoted() {
            return false;
        }
    }
}

/// Return whether a cache-control field contains one bare `no-store` directive.
pub fn has_bare_no_store(bytes: &[u8]) -> bool {
    has_bare_directive(bytes, b"no-store")
}

/// Return whether a cache-control field contains one bare `no-cache` directive.
pub fn has_bare_no_cache(bytes: &[u8]) -> bool {
    has_bare_directive(bytes, b"no-cache")
}

fn has_bare_directive(bytes: &[u8], required: &[u8]) -> bool {
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
        if name.eq_ignore_ascii_case(required) && !has_argument {
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

#[cfg(test)]
mod tests {
    use super::{has_bare_no_cache, has_bare_no_store, is_application_json};

    #[test]
    fn json_media_type_requires_valid_complete_parameters() {
        for valid in [
            "application/json",
            " Application/JSON ; charset=utf-8 ",
            "application/json;charset=\"utf-8\";profile=wallet",
        ] {
            assert!(is_application_json(valid.as_bytes()), "{valid}");
        }
        for invalid in [
            "text/json",
            "application/json; charset",
            "application/json;charset= utf-8",
            "application/json;charset=\"unterminated",
            "application/json;charset=utf-8;CHARSET=iso-8859-1",
        ] {
            assert!(!is_application_json(invalid.as_bytes()), "{invalid}");
        }
    }

    #[test]
    fn cache_directive_requires_a_bare_complete_member() {
        assert!(has_bare_no_store(b"private=\"field,a\", no-store"));
        assert!(has_bare_no_cache(b"max-age=0, NO-CACHE"));
        for invalid in [
            b"no-store=value".as_slice(),
            b"no-store=\"value\"",
            b"private=\"a,no-store,b\"",
            b"private=\"unterminated, no-store",
        ] {
            assert!(!has_bare_no_store(invalid));
        }
    }
}

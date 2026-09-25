use std::str;

use zeroize::Zeroizing;

use crate::CredentialOfferError;

pub(crate) fn pair_len(name: &str, value: &str) -> Option<usize> {
    encoded_len(name)?
        .checked_add(1)?
        .checked_add(encoded_len(value)?)
}

pub(crate) fn encoded_len(value: &str) -> Option<usize> {
    value.as_bytes().iter().try_fold(0usize, |length, byte| {
        length.checked_add(if is_form_literal(*byte) || *byte == b' ' {
            1
        } else {
            3
        })
    })
}

pub(crate) fn append_pair(output: &mut String, name: &str, value: &str) {
    append_encoded(output, name);
    output.push('=');
    append_encoded(output, value);
}

pub(crate) fn decode_component(
    encoded: &str,
    max_decoded_bytes: usize,
    invalid: CredentialOfferError,
    too_large: CredentialOfferError,
) -> Result<Zeroizing<String>, CredentialOfferError> {
    let input = encoded.as_bytes();
    let mut decoded = Zeroizing::new(Vec::with_capacity(input.len().min(max_decoded_bytes)));
    let mut cursor = 0;

    while cursor < input.len() {
        let byte = match input[cursor] {
            b'+' => {
                cursor += 1;
                b' '
            }
            b'%' => {
                let high = input
                    .get(cursor + 1)
                    .and_then(|value| hex_nibble(*value))
                    .ok_or(invalid)?;
                let low = input
                    .get(cursor + 2)
                    .and_then(|value| hex_nibble(*value))
                    .ok_or(invalid)?;
                cursor += 3;
                (high << 4) | low
            }
            byte if byte.is_ascii() => {
                cursor += 1;
                byte
            }
            _ => return Err(invalid),
        };
        if byte == 0 {
            return Err(invalid);
        }
        if decoded.len() == max_decoded_bytes {
            return Err(too_large);
        }
        decoded.push(byte);
    }

    let decoded_str = str::from_utf8(&decoded).map_err(|_| invalid)?;
    Ok(Zeroizing::new(decoded_str.to_owned()))
}

fn append_encoded(output: &mut String, value: &str) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    for byte in value.bytes() {
        if is_form_literal(byte) {
            output.push(char::from(byte));
        } else if byte == b' ' {
            output.push('+');
        } else {
            output.push('%');
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
}

const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

const fn is_form_literal(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'*' | b'-' | b'.' | b'_')
}

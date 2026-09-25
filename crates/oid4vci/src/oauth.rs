pub(crate) fn is_nqschar(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, 0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e))
}

pub(crate) fn is_uri_reference_chars(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, 0x21 | 0x23..=0x5b | 0x5d..=0x7e))
}

pub(crate) fn is_vschar(value: &str) -> bool {
    value.bytes().all(|byte| matches!(byte, 0x20..=0x7e))
}

use crate::http_field::{has_bare_no_cache, has_bare_no_store, is_application_json};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenHttpHeaderError {
    ContentTypeTooLarge,
    InvalidContentType,
    CacheControlTooLarge,
    InvalidCacheControl,
    PragmaTooLarge,
    InvalidPragma,
}

pub(crate) fn validate_token_http_headers(
    content_type: &str,
    cache_control: &str,
    pragma: &str,
    max_content_type_bytes: usize,
    max_cache_control_bytes: usize,
    max_pragma_bytes: usize,
) -> Result<(), TokenHttpHeaderError> {
    if content_type.len() > max_content_type_bytes {
        return Err(TokenHttpHeaderError::ContentTypeTooLarge);
    }
    if !is_application_json(content_type.as_bytes()) {
        return Err(TokenHttpHeaderError::InvalidContentType);
    }
    if cache_control.len() > max_cache_control_bytes {
        return Err(TokenHttpHeaderError::CacheControlTooLarge);
    }
    if !has_bare_no_store(cache_control.as_bytes()) {
        return Err(TokenHttpHeaderError::InvalidCacheControl);
    }
    if pragma.len() > max_pragma_bytes {
        return Err(TokenHttpHeaderError::PragmaTooLarge);
    }
    if !has_bare_no_cache(pragma.as_bytes()) {
        return Err(TokenHttpHeaderError::InvalidPragma);
    }
    Ok(())
}

//! Attribute parsing for `#[derive(Newtype)]`.
//!
//! Parses the `#[newtype(...)]` helper attributes into a typed `Attrs`:
//! - `display` / `display = "hex"` / `display = "base64url"` — opt into
//!   `Display` (bytes default to hex).
//! - `serde` — opt into `Serialize`/`Deserialize`.
//! - `validate_fn = <path>` + `validate_err = <type>` — invoke a caller-supplied
//!   validation function over the inner type. The macro calls `<path>(&inner)`
//!   where `inner: Inner`; the function's first parameter may be `&Inner` or
//!   any type `&Inner` derefs to (e.g. `&str` for `String`). `validate_fn` and
//!   `validate_err` must be specified together.

/// The bytes `Display`/serde encoding to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Encoding {
    /// Lowercase hexadecimal.
    Hex,
    /// Base64url without padding.
    Base64Url,
}

/// Parsed `#[newtype(...)]` configuration for a single derive invocation.
#[derive(Default)]
pub(crate) struct Attrs {
    pub display: Option<Encoding>,
    pub serde: bool,
    pub validate_fn: Option<syn::Path>,
    pub validate_err: Option<syn::Path>,
}

impl Attrs {
    /// Collect every `#[newtype(...)]` attribute on `attrs` into one `Attrs`.
    pub(crate) fn from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut out = Attrs::default();
        for attr in attrs {
            if !attr.path().is_ident("newtype") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("display") {
                    out.display = Some(match meta.value() {
                        Ok(val) => {
                            let lit: syn::LitStr = val.parse()?;
                            match lit.value().as_str() {
                                "hex" => Encoding::Hex,
                                "base64url" => Encoding::Base64Url,
                                other => {
                                    return Err(syn::Error::new(
                                        lit.span(),
                                        format!(
                                            "unknown `display` encoding `{other}`; \
                                            expected `hex` or `base64url`"
                                        ),
                                    ));
                                }
                            }
                        }
                        Err(_) => Encoding::Hex,
                    });
                    return Ok(());
                }
                if meta.path.is_ident("serde") {
                    out.serde = true;
                    return Ok(());
                }
                if meta.path.is_ident("validate_fn") {
                    out.validate_fn = Some(meta.value()?.parse()?);
                    return Ok(());
                }
                if meta.path.is_ident("validate_err") {
                    out.validate_err = Some(meta.value()?.parse()?);
                    return Ok(());
                }
                Err(meta.error(
                    "unknown `#[newtype(...)]` attribute; expected \
                    `display`, `display = \"hex\"|\"base64url\"`, `serde`, \
                    `validate_fn = <fn>`, or `validate_err = <type>`",
                ))
            })?;
        }
        match (out.validate_fn.is_some(), out.validate_err.is_some()) {
            (true, true) => {}
            (true, false) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "`validate_fn = <fn>` requires a companion `validate_err = <type>`",
                ));
            }
            (false, true) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "`validate_err = <type>` requires a companion `validate_fn = <fn>`",
                ));
            }
            (false, false) => {}
        }
        Ok(out)
    }

    /// The effective bytes encoding, defaulting to hex when `display` is unset
    /// (so bytes `serde` without `display` still serializes as a hex string).
    pub(crate) fn encoding_or_hex(&self) -> Encoding {
        self.display.unwrap_or(Encoding::Hex)
    }
}

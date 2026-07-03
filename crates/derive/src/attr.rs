//! Attribute parsing for `#[derive(Newtype)]`.
//!
//! Parses the `#[newtype(...)]` helper attributes into a typed `Attrs`:
//! - `display` / `display = "hex"` / `display = "base64url"` — opt into
//!   `Display` (bytes default to hex).
//! - `serde` — opt into transparent `Serialize`/`Deserialize`.
//! - `parse = <path>` + `err = <type>` — fallible `FromStr`/`parse` via a
//!   caller-supplied validation function returning `Result<(), Err>`.

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
    pub parse: Option<syn::Path>,
    pub err: Option<syn::Path>,
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
                if meta.path.is_ident("parse") {
                    out.parse = Some(meta.value()?.parse()?);
                    return Ok(());
                }
                if meta.path.is_ident("err") {
                    out.err = Some(meta.value()?.parse()?);
                    return Ok(());
                }
                Err(meta.error(
                    "unknown `#[newtype(...)]` attribute; expected \
                    `display`, `display = \"hex\"|\"base64url\"`, `serde`, \
                    `parse = <fn>`, or `err = <type>`",
                ))
            })?;
        }
        if out.parse.is_some() && out.err.is_none() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`parse = <fn>` requires a companion `err = <type>`",
            ));
        }
        Ok(out)
    }

    /// The effective bytes encoding, defaulting to hex when `display` is unset
    /// (so bytes `serde` without `display` still serializes as a hex string).
    pub(crate) fn encoding_or_hex(&self) -> Encoding {
        self.display.unwrap_or(Encoding::Hex)
    }
}

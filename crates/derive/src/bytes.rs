//! Expansion for the bytes (`Vec<u8>`) category.

use crate::Ctx;
use crate::attr::Encoding;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub(crate) fn expand(ctx: &Ctx) -> TokenStream2 {
    let name = &ctx.name;
    let attrs = &ctx.attrs;
    let enc = attrs.encoding_or_hex();

    let mut ts = quote! {
        #[automatically_derived]
        impl #name {
            pub fn new(inner: ::std::vec::Vec<u8>) -> Self {
                Self(inner)
            }
            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }
            pub fn into_bytes(self) -> ::std::vec::Vec<u8> {
                self.0
            }
        }
        #[automatically_derived]
        impl ::core::convert::AsRef<[u8]> for #name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        #[automatically_derived]
        impl ::core::convert::From<::std::vec::Vec<u8>> for #name {
            fn from(inner: ::std::vec::Vec<u8>) -> Self {
                Self(inner)
            }
        }
        #[automatically_derived]
        impl ::core::convert::From<&[u8]> for #name {
            fn from(inner: &[u8]) -> Self {
                Self(inner.to_vec())
            }
        }
    };

    let need_encode = attrs.display.is_some() || attrs.serde;
    let need_decode = attrs.serde || attrs.parse.is_some();
    ts.extend(helpers(name, enc, need_encode, need_decode));

    if attrs.display.is_some() {
        ts.extend(display_impl(name));
    }
    if attrs.serde {
        ts.extend(serde_impl(name));
    }
    if let (Some(parse_fn), Some(err)) = (&attrs.parse, &attrs.err) {
        ts.extend(parse_impl(name, parse_fn, err));
    }

    ts
}

/// The private associated encode/decode helpers, emitted once per type so the
/// `Display`, serde, and `parse` impls can share them without colliding with
/// another derived bytes newtype in the same crate.
fn helpers(name: &syn::Ident, enc: Encoding, need_encode: bool, need_decode: bool) -> TokenStream2 {
    let encode = need_encode.then(|| encode_fn(enc));
    let decode = need_decode.then(|| decode_fn(enc));
    if encode.is_none() && decode.is_none() {
        return quote! {};
    }
    quote! {
        #[automatically_derived]
        impl #name {
            #encode
            #decode
        }
    }
}

fn display_impl(name: &syn::Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&Self::nt_encode_(&self.0), f)
            }
        }
    }
}

fn serde_impl(name: &syn::Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(&Self::nt_encode_(&self.0))
            }
        }
        #[automatically_derived]
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let s = <::std::string::String as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                let bytes = Self::nt_decode_(&s)
                    .map_err(|e| <D::Error as ::serde::de::Error>::custom(e))?;
                ::core::result::Result::Ok(Self(bytes))
            }
        }
    }
}

fn parse_impl(name: &syn::Ident, parse_fn: &syn::Path, err: &syn::Path) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl #name {
            pub fn parse(s: &str) -> ::core::result::Result<Self, #err> {
                #parse_fn(s)?;
                let inner = Self::nt_decode_(s).expect(
                    "validation function must guarantee the string decodes in the display encoding"
                );
                ::core::result::Result::Ok(Self(inner))
            }
        }
        #[automatically_derived]
        impl ::core::str::FromStr for #name {
            type Err = #err;
            fn from_str(s: &str) -> ::core::result::Result<Self, #err> {
                #parse_fn(s)?;
                let inner = Self::nt_decode_(s).expect(
                    "validation function must guarantee the string decodes in the display encoding"
                );
                ::core::result::Result::Ok(Self(inner))
            }
        }
    }
}

fn encode_fn(enc: Encoding) -> TokenStream2 {
    match enc {
        Encoding::Hex => quote! {
            fn nt_encode_(data: &[u8]) -> ::std::string::String {
                let mut out = ::std::string::String::with_capacity(data.len() * 2);
                for &b in data {
                    use ::core::fmt::Write as _;
                    let _ = ::core::write!(out, "{:02x}", b);
                }
                out
            }
        },
        Encoding::Base64Url => quote! {
            fn nt_encode_(data: &[u8]) -> ::std::string::String {
                const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
                let mut out = ::std::string::String::with_capacity(data.len().div_ceil(3) * 4);
                for chunk in data.chunks(3) {
                    let b0 = ::core::primitive::u32::from(chunk[0]);
                    let b1 = ::core::primitive::u32::from(chunk.get(1).copied().unwrap_or(0));
                    let b2 = ::core::primitive::u32::from(chunk.get(2).copied().unwrap_or(0));
                    let n = (b0 << 16) | (b1 << 8) | b2;
                    out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
                    out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
                    if chunk.len() > 1 {
                        out.push(TABLE[((n >> 6) & 0x3f) as usize] as char);
                    }
                    if chunk.len() > 2 {
                        out.push(TABLE[(n & 0x3f) as usize] as char);
                    }
                }
                out
            }
        },
    }
}

fn decode_fn(enc: Encoding) -> TokenStream2 {
    match enc {
        Encoding::Hex => quote! {
            fn nt_decode_(s: &str) -> ::core::result::Result<::std::vec::Vec<u8>, &'static str> {
                fn val(b: u8) -> ::core::option::Option<u8> {
                    match b {
                        b'0'..=b'9' => ::core::option::Option::Some(b - b'0'),
                        b'a'..=b'f' => ::core::option::Option::Some(b - b'a' + 10),
                        b'A'..=b'F' => ::core::option::Option::Some(b - b'A' + 10),
                        _ => ::core::option::Option::None,
                    }
                }
                let bytes = s.as_bytes();
                if bytes.len() % 2 != 0 {
                    return ::core::result::Result::Err("hex string has odd length");
                }
                let mut out = ::std::vec::Vec::with_capacity(bytes.len() / 2);
                for pair in bytes.chunks_exact(2) {
                    let hi = val(pair[0]).ok_or("invalid hex digit")?;
                    let lo = val(pair[1]).ok_or("invalid hex digit")?;
                    out.push((hi << 4) | lo);
                }
                ::core::result::Result::Ok(out)
            }
        },
        Encoding::Base64Url => quote! {
            fn nt_decode_(s: &str) -> ::core::result::Result<::std::vec::Vec<u8>, &'static str> {
                fn val(b: u8) -> ::core::option::Option<u8> {
                    match b {
                        b'A'..=b'Z' => ::core::option::Option::Some(b - b'A'),
                        b'a'..=b'z' => ::core::option::Option::Some(b - b'a' + 26),
                        b'0'..=b'9' => ::core::option::Option::Some(b - b'0' + 52),
                        b'-' => ::core::option::Option::Some(62),
                        b'_' => ::core::option::Option::Some(63),
                        _ => ::core::option::Option::None,
                    }
                }
                let bytes = s.as_bytes();
                if bytes.is_empty() {
                    return ::core::result::Result::Ok(::std::vec::Vec::new());
                }
                if bytes.len() % 4 == 1 {
                    return ::core::result::Result::Err("invalid base64url length");
                }
                let mut out = ::std::vec::Vec::with_capacity(bytes.len() * 3 / 4);
                for chunk in bytes.chunks(4) {
                    let v0 = ::core::primitive::u32::from(val(chunk[0]).ok_or("invalid base64url character")?);
                    let v1 = ::core::primitive::u32::from(val(chunk[1]).ok_or("invalid base64url character")?);
                    let mut n = (v0 << 18) | (v1 << 12);
                    out.push(((n >> 16) & 0xff) as u8);
                    if chunk.len() > 2 {
                        let v2 = ::core::primitive::u32::from(val(chunk[2]).ok_or("invalid base64url character")?);
                        n |= v2 << 6;
                        out.push(((n >> 8) & 0xff) as u8);
                        if chunk.len() > 3 {
                            let v3 = ::core::primitive::u32::from(val(chunk[3]).ok_or("invalid base64url character")?);
                            n |= v3;
                            out.push((n & 0xff) as u8);
                        }
                    }
                }
                ::core::result::Result::Ok(out)
            }
        },
    }
}

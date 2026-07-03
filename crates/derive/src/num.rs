//! Expansion for the numeric category.

use crate::Ctx;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub(crate) fn expand(ctx: &Ctx) -> TokenStream2 {
    let name = &ctx.name;
    let inner = &ctx.inner;
    let attrs = &ctx.attrs;

    let mut ts = quote! {
        #[automatically_derived]
        impl #name {
            pub fn new(inner: #inner) -> Self {
                Self(inner)
            }
            pub fn get(&self) -> #inner {
                self.0
            }
        }
        #[automatically_derived]
        impl ::core::convert::From<#inner> for #name {
            fn from(inner: #inner) -> Self {
                Self(inner)
            }
        }
    };

    if attrs.display.is_some() {
        ts.extend(quote! {
            #[automatically_derived]
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Display::fmt(&self.0, f)
                }
            }
        });
    }

    if attrs.serde {
        ts.extend(quote! {
            #[automatically_derived]
            impl ::serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    ::serde::Serialize::serialize(&self.0, serializer)
                }
            }
            #[automatically_derived]
            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    let inner = <#inner as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                    ::core::result::Result::Ok(Self(inner))
                }
            }
        });
    }

    if let (Some(parse_fn), Some(err)) = (&attrs.parse, &attrs.err) {
        ts.extend(quote! {
            #[automatically_derived]
            impl #name {
                pub fn parse(s: &str) -> ::core::result::Result<Self, #err> {
                    #parse_fn(s)?;
                    let inner: #inner = s.parse().expect(
                        "validation function must guarantee the string parses to the inner numeric type"
                    );
                    ::core::result::Result::Ok(Self(inner))
                }
            }
            #[automatically_derived]
            impl ::core::str::FromStr for #name {
                type Err = #err;
                fn from_str(s: &str) -> ::core::result::Result<Self, #err> {
                    #parse_fn(s)?;
                    let inner: #inner = s.parse().expect(
                        "validation function must guarantee the string parses to the inner numeric type"
                    );
                    ::core::result::Result::Ok(Self(inner))
                }
            }
        });
    }

    ts
}

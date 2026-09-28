//! Shared serde expansion for string- and numeric-backed newtypes.
//!
//! Category-specific construction, parsing, conversion, and validation entry
//! points stay in their owning modules. Bytes use an encoded-string wire form
//! and deliberately retain their separate serde implementation.

use crate::Ctx;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub(crate) fn expand(ctx: &Ctx) -> TokenStream2 {
    let name = &ctx.name;
    let inner = &ctx.inner;

    let deserialize = if let (Some(validate_fn), Some(_validate_err)) =
        (&ctx.attrs.validate_fn, &ctx.attrs.validate_err)
    {
        quote! {
            #[automatically_derived]
            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    let inner = <#inner as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                    #validate_fn(&inner).map_err(|e| <D::Error as ::serde::de::Error>::custom(e))?;
                    ::core::result::Result::Ok(Self(inner))
                }
            }
        }
    } else {
        quote! {
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
        }
    };

    quote! {
        #[automatically_derived]
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&self.0, serializer)
            }
        }
        #deserialize
    }
}

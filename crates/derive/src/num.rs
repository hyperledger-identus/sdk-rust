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
            pub fn get(&self) -> #inner {
                self.0
            }
        }
    };

    if let (Some(validate_fn), Some(validate_err)) = (&attrs.validate_fn, &attrs.validate_err) {
        // validate_fn-configured: replace the infallible `new`/`From` bypasses
        // with a validating `TryFrom<Inner>`, a uniform `try_new`, and a
        // `pub(crate)` `new_unchecked` hatch. No `parse`/`FromStr` for numeric.
        ts.extend(quote! {
            #[automatically_derived]
            impl #name {
                pub fn try_new(inner: #inner) -> ::core::result::Result<Self, #validate_err> {
                    <Self as ::core::convert::TryFrom<#inner>>::try_from(inner)
                }
                pub(crate) fn new_unchecked(inner: #inner) -> Self {
                    Self(inner)
                }
            }
            #[automatically_derived]
            impl ::core::convert::TryFrom<#inner> for #name {
                type Error = #validate_err;
                fn try_from(inner: #inner) -> ::core::result::Result<Self, #validate_err> {
                    #validate_fn(&inner)?;
                    ::core::result::Result::Ok(Self(inner))
                }
            }
        });
    } else {
        // No validator: infallible construction stays (forbidding it would make
        // the type unconstructable).
        ts.extend(quote! {
            #[automatically_derived]
            impl #name {
                pub fn new(inner: #inner) -> Self {
                    Self(inner)
                }
            }
            #[automatically_derived]
            impl ::core::convert::From<#inner> for #name {
                fn from(inner: #inner) -> Self {
                    Self(inner)
                }
            }
        });
    }

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
        ts.extend(crate::scalar_serde::expand(ctx));
    }

    ts
}

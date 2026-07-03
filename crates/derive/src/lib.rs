//! `#[derive(Newtype)]` — packages the recurring domain-newtype pattern.
//!
//! Dispatches on the single unnamed field's type to one of three categories
//! (string, bytes, numeric) and emits the category-appropriate constructors,
//! accessors, conversions, `Display`, optional serde, and fallible `parse`.
//! Proc-macro crates are build-time tooling and carry no `pub const COMPONENT`
//! (exempt from the `core-error-conventions` "Component metadata" obligation).

mod attr;
mod bytes;
mod category;
mod num;
mod str;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

use attr::Attrs;
use category::{Category, classify};

/// Shared, parsed context for a single derive invocation.
struct Ctx {
    name: syn::Ident,
    inner: syn::Type,
    attrs: Attrs,
}

/// `#[identus::port]` — inert port-ness marker for hexagonal-ring port traits.
///
/// Carries no runtime semantics and does not propagate to implementers (it is
/// attached to the trait *item*, not a `Self:` bound). It is the
/// self-documenting declaration that a trait is a port and the single source
/// of truth from which the `identus-conformance` naming guard derives the port
/// set from source. Enforces the no-`Port`-suffix port-naming convention at
/// compile time: emits `compile_error!` if the annotated trait's name ends in
/// `Port`, otherwise emits the trait unchanged.
#[proc_macro_attribute]
pub fn port(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemTrait);
    if input.ident.to_string().ends_with("Port") {
        return syn::Error::new_spanned(
            &input.ident,
            "port traits are bare capability nouns; a `Port` suffix is forbidden",
        )
        .to_compile_error()
        .into();
    }
    quote!(#input).into()
}

#[proc_macro_derive(Newtype, attributes(newtype))]
pub fn derive_newtype(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match expand(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand(input: syn::DeriveInput) -> syn::Result<TokenStream2> {
    let syn::DeriveInput {
        ident, attrs, data, ..
    } = input;

    let fields = match data {
        syn::Data::Struct(s) => s.fields,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &ident,
                format!("Newtype only supports structs; `{ident}` is not a struct"),
            ));
        }
    };

    let field = match &fields {
        syn::Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.len() != 1 {
                return Err(syn::Error::new_spanned(
                    unnamed,
                    "Newtype requires a tuple struct with exactly one unnamed field",
                ));
            }
            &unnamed.unnamed[0]
        }
        syn::Fields::Named(named) => {
            return Err(syn::Error::new_spanned(
                named,
                "Newtype requires a tuple struct with exactly one unnamed field; \
                named fields are not supported",
            ));
        }
        syn::Fields::Unit => {
            return Err(syn::Error::new_spanned(
                &fields,
                "Newtype requires a tuple struct with exactly one unnamed field",
            ));
        }
    };

    let parsed_attrs = Attrs::from_attributes(&attrs)?;
    let category = classify(&field.ty)?;
    let ctx = Ctx {
        name: ident,
        inner: field.ty.clone(),
        attrs: parsed_attrs,
    };
    let ts = match category {
        Category::Str => str::expand(&ctx),
        Category::Bytes => bytes::expand(&ctx),
        Category::Num => num::expand(&ctx),
    };
    Ok(ts)
}

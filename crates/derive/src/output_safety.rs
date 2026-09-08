//! Fail-closed validation for direct Rust syntax emitted by this crate.

use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Meta, Token};

const UNSAFE_ATTRIBUTES: [&str; 4] = ["export_name", "link_section", "naked", "no_mangle"];

pub(crate) fn validate(tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let file = syn::parse2::<syn::File>(tokens.clone())?;
    let mut visitor = UnsafeSyntax::default();
    visitor.visit_file(&file);

    if let Some(found) = visitor.first {
        Err(syn::Error::new(
            found.span,
            format!(
                "identus-derive refuses to emit unsafe Rust ({}) without a dedicated safety decision",
                found.kind
            ),
        ))
    } else {
        Ok(tokens)
    }
}

#[derive(Default)]
struct UnsafeSyntax {
    first: Option<Found>,
}

struct Found {
    kind: &'static str,
    span: Span,
}

impl UnsafeSyntax {
    fn record(&mut self, kind: &'static str, span: Span) {
        if self.first.is_none() {
            self.first = Some(Found { kind, span });
        }
    }
}

fn prohibited_attribute(meta: &Meta) -> Option<(&'static str, Span)> {
    let path = meta.path();
    if path.is_ident("allow_internal_unsafe") {
        return Some(("allow_internal_unsafe attribute", meta.span()));
    }
    if path.is_ident("unsafe") || UNSAFE_ATTRIBUTES.iter().any(|name| path.is_ident(name)) {
        return Some(("unsafe attribute", meta.span()));
    }

    // `cfg_attr` expands after this direct-output check. Reject a prohibited
    // attribute in any branch rather than letting the active configuration
    // determine whether unsafe syntax escapes validation.
    if path.is_ident("cfg_attr") {
        let Meta::List(list) = meta else {
            return None;
        };
        let nested = list
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .ok()?;
        return nested.iter().skip(1).find_map(prohibited_attribute);
    }

    None
}

impl<'ast> Visit<'ast> for UnsafeSyntax {
    fn visit_expr_unsafe(&mut self, node: &'ast syn::ExprUnsafe) {
        self.record("unsafe block", node.unsafe_token.span);
        visit::visit_expr_unsafe(self, node);
    }

    fn visit_signature(&mut self, node: &'ast syn::Signature) {
        if let Some(token) = node.unsafety {
            self.record("unsafe function or method", token.span);
        }
        visit::visit_signature(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        if let Some(token) = node.unsafety {
            self.record("unsafe trait", token.span);
        }
        visit::visit_item_trait(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if let Some(token) = node.unsafety {
            self.record("unsafe implementation", token.span);
        }
        visit::visit_item_impl(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast syn::ItemForeignMod) {
        if let Some(token) = node.unsafety {
            self.record("unsafe extern block", token.span);
        }
        visit::visit_item_foreign_mod(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "global_asm")
        {
            self.record("global assembly", node.path.span());
        }
        visit::visit_macro(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        if let Some((kind, span)) = prohibited_attribute(&node.meta) {
            self.record(kind, span);
        }
        visit::visit_attribute(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn assert_rejected(source: &str, expected: &str) {
        let tokens = TokenStream2::from_str(source).expect("test fixture must tokenize");
        let error = validate(tokens).expect_err("unsafe fixture must be rejected");
        assert!(
            error.to_string().contains(expected),
            "unexpected validation error: {error}"
        );
    }

    #[test]
    fn rejects_every_pinned_unsafe_construct() {
        let cases = [
            ("fn f() { unsafe {} }", "unsafe block"),
            ("unsafe fn f() {}", "unsafe function or method"),
            ("unsafe trait T {}", "unsafe trait"),
            ("unsafe impl T for S {}", "unsafe implementation"),
            ("unsafe extern \"C\" {}", "unsafe extern block"),
            ("core::arch::global_asm!(\"\");", "global assembly"),
            (
                "#[allow_internal_unsafe] macro_rules! m { () => {} }",
                "allow_internal_unsafe attribute",
            ),
        ];

        for (source, expected) in cases {
            assert_rejected(source, expected);
        }
    }

    #[test]
    fn rejects_wrapped_and_legacy_unsafe_attributes() {
        for attribute in UNSAFE_ATTRIBUTES {
            assert_rejected(&format!("#[{attribute}] fn f() {{}}"), "unsafe attribute");
            assert_rejected(
                &format!("#[unsafe({attribute})] fn f() {{}}"),
                "unsafe attribute",
            );
            assert_rejected(
                &format!("#[cfg_attr(any(), {attribute})] fn f() {{}}"),
                "unsafe attribute",
            );
        }

        assert_rejected(
            "#[cfg_attr(any(), allow_internal_unsafe)] macro_rules! m { () => {} }",
            "allow_internal_unsafe attribute",
        );
        assert_rejected(
            "#[cfg_attr(any(), cfg_attr(any(), unsafe(no_mangle)))] fn f() {}",
            "unsafe attribute",
        );

        let safe_condition = TokenStream2::from_str(
            "#[cfg_attr(no_mangle, inline)] fn condition_name_is_not_an_attribute() {}",
        )
        .unwrap();
        assert!(validate(safe_condition).is_ok());
    }

    #[test]
    fn rejects_unsafe_syntax_nested_in_generated_items() {
        assert_rejected(
            "mod nested { impl S { fn f() { unsafe {} } } }",
            "unsafe block",
        );
    }

    #[test]
    fn accepts_safe_items_and_non_syntax_text() {
        let tokens = TokenStream2::from_str(
            r#"impl Safe { fn message() -> &'static str { "unsafe global_asm no_mangle" } }"#,
        )
        .unwrap();
        assert!(validate(tokens).is_ok());
    }
}

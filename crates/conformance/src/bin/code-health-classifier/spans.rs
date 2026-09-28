//! AST span ownership and test-only span collection.

use std::ops::Range;

use proc_macro2::Span;
use syn::{
    Arm, Attribute, BareFnArg, BareVariadic, Expr, Field, FieldPat, FieldValue, FnArg, ForeignItem,
    GenericParam, ImplItem, Item, Pat, Stmt, TraitItem, Variadic,
    spanned::Spanned,
    visit::{self, Visit},
};

use super::{
    MAX_SPANS_PER_FILE,
    cfg::{Truth, attributes_inclusion},
};

fn range(span: Span, source_len: usize) -> Result<Range<usize>, String> {
    let range = span.byte_range();
    if range.start > range.end || range.end > source_len {
        return Err(format!(
            "span {}..{} is outside source length {source_len}",
            range.start, range.end
        ));
    }
    Ok(range)
}

fn attributed_range<T: Spanned>(
    node: &T,
    attributes: &[Attribute],
    source: &str,
) -> Result<Range<usize>, String> {
    let mut result = range(node.span(), source.len())?;
    for attribute in attributes {
        let attribute = range(attribute.span(), source.len())?;
        result.start = result.start.min(attribute.start);
        result.end = result.end.max(attribute.end);
    }
    let tail = &source[result.end..];
    let whitespace = tail.len() - tail.trim_start_matches(char::is_whitespace).len();
    let punctuation = tail[whitespace..]
        .chars()
        .next()
        .filter(|character| matches!(character, ',' | ';'))
        .map_or(0, char::len_utf8);
    result.end += whitespace + punctuation;
    Ok(result)
}

pub(super) fn item_attributes(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(value) => &value.attrs,
        Item::Enum(value) => &value.attrs,
        Item::ExternCrate(value) => &value.attrs,
        Item::Fn(value) => &value.attrs,
        Item::ForeignMod(value) => &value.attrs,
        Item::Impl(value) => &value.attrs,
        Item::Macro(value) => &value.attrs,
        Item::Mod(value) => &value.attrs,
        Item::Static(value) => &value.attrs,
        Item::Struct(value) => &value.attrs,
        Item::Trait(value) => &value.attrs,
        Item::TraitAlias(value) => &value.attrs,
        Item::Type(value) => &value.attrs,
        Item::Union(value) => &value.attrs,
        Item::Use(value) => &value.attrs,
        Item::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) fn impl_item_attributes(item: &ImplItem) -> &[Attribute] {
    match item {
        ImplItem::Const(value) => &value.attrs,
        ImplItem::Fn(value) => &value.attrs,
        ImplItem::Type(value) => &value.attrs,
        ImplItem::Macro(value) => &value.attrs,
        ImplItem::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) fn trait_item_attributes(item: &TraitItem) -> &[Attribute] {
    match item {
        TraitItem::Const(value) => &value.attrs,
        TraitItem::Fn(value) => &value.attrs,
        TraitItem::Type(value) => &value.attrs,
        TraitItem::Macro(value) => &value.attrs,
        TraitItem::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) fn foreign_item_attributes(item: &ForeignItem) -> &[Attribute] {
    match item {
        ForeignItem::Fn(value) => &value.attrs,
        ForeignItem::Static(value) => &value.attrs,
        ForeignItem::Type(value) => &value.attrs,
        ForeignItem::Macro(value) => &value.attrs,
        ForeignItem::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) fn pat_attributes(pattern: &Pat) -> &[Attribute] {
    match pattern {
        Pat::Const(value) => &value.attrs,
        Pat::Ident(value) => &value.attrs,
        Pat::Lit(value) => &value.attrs,
        Pat::Macro(value) => &value.attrs,
        Pat::Or(value) => &value.attrs,
        Pat::Paren(value) => &value.attrs,
        Pat::Path(value) => &value.attrs,
        Pat::Range(value) => &value.attrs,
        Pat::Reference(value) => &value.attrs,
        Pat::Rest(value) => &value.attrs,
        Pat::Slice(value) => &value.attrs,
        Pat::Struct(value) => &value.attrs,
        Pat::Tuple(value) => &value.attrs,
        Pat::TupleStruct(value) => &value.attrs,
        Pat::Type(value) => &value.attrs,
        Pat::Wild(value) => &value.attrs,
        Pat::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) fn expr_attributes(expr: &Expr) -> &[Attribute] {
    match expr {
        Expr::Array(value) => &value.attrs,
        Expr::Assign(value) => &value.attrs,
        Expr::Async(value) => &value.attrs,
        Expr::Await(value) => &value.attrs,
        Expr::Binary(value) => &value.attrs,
        Expr::Block(value) => &value.attrs,
        Expr::Break(value) => &value.attrs,
        Expr::Call(value) => &value.attrs,
        Expr::Cast(value) => &value.attrs,
        Expr::Closure(value) => &value.attrs,
        Expr::Const(value) => &value.attrs,
        Expr::Continue(value) => &value.attrs,
        Expr::Field(value) => &value.attrs,
        Expr::ForLoop(value) => &value.attrs,
        Expr::Group(value) => &value.attrs,
        Expr::If(value) => &value.attrs,
        Expr::Index(value) => &value.attrs,
        Expr::Infer(value) => &value.attrs,
        Expr::Let(value) => &value.attrs,
        Expr::Lit(value) => &value.attrs,
        Expr::Loop(value) => &value.attrs,
        Expr::Macro(value) => &value.attrs,
        Expr::Match(value) => &value.attrs,
        Expr::MethodCall(value) => &value.attrs,
        Expr::Paren(value) => &value.attrs,
        Expr::Path(value) => &value.attrs,
        Expr::Range(value) => &value.attrs,
        Expr::RawAddr(value) => &value.attrs,
        Expr::Reference(value) => &value.attrs,
        Expr::Repeat(value) => &value.attrs,
        Expr::Return(value) => &value.attrs,
        Expr::Struct(value) => &value.attrs,
        Expr::Try(value) => &value.attrs,
        Expr::TryBlock(value) => &value.attrs,
        Expr::Tuple(value) => &value.attrs,
        Expr::Unary(value) => &value.attrs,
        Expr::Unsafe(value) => &value.attrs,
        Expr::While(value) => &value.attrs,
        Expr::Yield(value) => &value.attrs,
        Expr::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) struct SpanCollector<'a> {
    pub(super) source: &'a str,
    pub(super) spans: Vec<Range<usize>>,
    pub(super) error: Option<String>,
}

impl<'a> SpanCollector<'a> {
    fn classify<T: Spanned>(&mut self, node: &T, attributes: &[Attribute]) -> bool {
        if self.error.is_some() || attributes_inclusion(attributes) != Truth::False {
            return false;
        }
        match attributed_range(node, attributes, self.source) {
            Ok(span) => {
                if self.spans.len() >= MAX_SPANS_PER_FILE {
                    self.error = Some(format!("more than {MAX_SPANS_PER_FILE} test spans"));
                } else {
                    self.spans.push(span);
                }
            }
            Err(error) => self.error = Some(error),
        }
        true
    }
}

impl<'ast> Visit<'ast> for SpanCollector<'_> {
    fn visit_file(&mut self, node: &'ast syn::File) {
        if attributes_inclusion(&node.attrs) == Truth::False {
            self.spans.push(0..self.source.len());
        } else {
            visit::visit_file(self, node);
        }
    }

    fn visit_item(&mut self, node: &'ast Item) {
        if !self.classify(node, item_attributes(node)) {
            visit::visit_item(self, node);
        }
    }

    fn visit_field(&mut self, node: &'ast Field) {
        if !self.classify(node, &node.attrs) {
            visit::visit_field(self, node);
        }
    }

    fn visit_field_value(&mut self, node: &'ast FieldValue) {
        if !self.classify(node, &node.attrs) {
            visit::visit_field_value(self, node);
        }
    }

    fn visit_variant(&mut self, node: &'ast syn::Variant) {
        if !self.classify(node, &node.attrs) {
            visit::visit_variant(self, node);
        }
    }

    fn visit_fn_arg(&mut self, node: &'ast FnArg) {
        let attributes: &[Attribute] = match node {
            FnArg::Receiver(value) => &value.attrs,
            FnArg::Typed(value) => &value.attrs,
        };
        if !self.classify(node, attributes) {
            visit::visit_fn_arg(self, node);
        }
    }

    fn visit_bare_fn_arg(&mut self, node: &'ast BareFnArg) {
        if !self.classify(node, &node.attrs) {
            visit::visit_bare_fn_arg(self, node);
        }
    }

    fn visit_variadic(&mut self, node: &'ast Variadic) {
        if !self.classify(node, &node.attrs) {
            visit::visit_variadic(self, node);
        }
    }

    fn visit_bare_variadic(&mut self, node: &'ast BareVariadic) {
        if !self.classify(node, &node.attrs) {
            visit::visit_bare_variadic(self, node);
        }
    }

    fn visit_field_pat(&mut self, node: &'ast FieldPat) {
        if !self.classify(node, &node.attrs) {
            visit::visit_field_pat(self, node);
        }
    }

    fn visit_generic_param(&mut self, node: &'ast GenericParam) {
        let attributes: &[Attribute] = match node {
            GenericParam::Lifetime(value) => &value.attrs,
            GenericParam::Type(value) => &value.attrs,
            GenericParam::Const(value) => &value.attrs,
        };
        if !self.classify(node, attributes) {
            visit::visit_generic_param(self, node);
        }
    }

    fn visit_pat(&mut self, node: &'ast Pat) {
        if !self.classify(node, pat_attributes(node)) {
            visit::visit_pat(self, node);
        }
    }

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        let attributes: &[Attribute] = match node {
            Stmt::Local(value) => &value.attrs,
            Stmt::Item(value) => item_attributes(value),
            Stmt::Expr(value, _) => expr_attributes(value),
            Stmt::Macro(value) => &value.attrs,
        };
        if !self.classify(node, attributes) {
            visit::visit_stmt(self, node);
        }
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        if !self.classify(node, expr_attributes(node)) {
            visit::visit_expr(self, node);
        }
    }

    fn visit_arm(&mut self, node: &'ast Arm) {
        if !self.classify(node, &node.attrs) {
            visit::visit_arm(self, node);
        }
    }

    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        if !self.classify(node, impl_item_attributes(node)) {
            visit::visit_impl_item(self, node);
        }
    }

    fn visit_trait_item(&mut self, node: &'ast TraitItem) {
        if !self.classify(node, trait_item_attributes(node)) {
            visit::visit_trait_item(self, node);
        }
    }

    fn visit_foreign_item(&mut self, node: &'ast ForeignItem) {
        if !self.classify(node, foreign_item_attributes(node)) {
            visit::visit_foreign_item(self, node);
        }
    }
}

//! Internal AST classifier for deterministic code-health source populations.
//!
//! This binary is repository tooling. It is not an SDK API and is never
//! published. Input and output use a bounded, versioned JSON protocol so the
//! Python report orchestrator does not parse Rust syntax.

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{self, Read, Write},
    ops::Range,
    path::{Component, Path, PathBuf},
};

use proc_macro2::Span;
use serde::{Deserialize, Serialize};
use syn::{
    Arm, Attribute, Expr, Field, FieldValue, FnArg, ForeignItem, GenericParam, ImplItem, Item,
    ItemMod, Meta, Stmt, TraitItem,
    parse::Parser,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    visit::{self, Visit},
};

const PROTOCOL_VERSION: u32 = 1;
const CLASSIFIER_NAME: &str = "syn-ast-v1";
const MAX_REQUEST_BYTES: u64 = 32 * 1024 * 1024;
const MAX_SOURCE_BYTES: usize = 2 * 1024 * 1024;
const MAX_TOTAL_SOURCE_BYTES: usize = 24 * 1024 * 1024;
const MAX_FILES: usize = 4096;
const MAX_PATH_BYTES: usize = 4096;
const MAX_SPANS_PER_FILE: usize = 262_144;
const MAX_MODULE_EDGES: usize = 131_072;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    protocol_version: u32,
    sources: Vec<SourceInput>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceInput {
    path: String,
    source: String,
}

#[derive(Debug, Serialize)]
struct Response {
    classifier: &'static str,
    files: Vec<FileOutput>,
    inherited_inline_paths: Vec<String>,
    protocol_version: u32,
}

#[derive(Debug, Serialize)]
struct FileOutput {
    inline_test_lines: Vec<usize>,
    path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Truth {
    True,
    False,
    Unknown,
}

impl Truth {
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::Unknown,
        }
    }

    fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ModuleReference {
    context: Vec<String>,
    name: String,
    path_override: Option<PathBuf>,
}

#[derive(Debug, Default)]
struct ParsedFile {
    spans: Vec<Range<usize>>,
    test_edges: Vec<ModuleReference>,
    production_edges: Vec<ModuleReference>,
}

fn normalized_ident(ident: &syn::Ident) -> String {
    let rendered = ident.to_string();
    rendered.strip_prefix("r#").unwrap_or(&rendered).to_owned()
}

fn path_ident(meta: &Meta) -> Option<String> {
    let Meta::Path(path) = meta else {
        return None;
    };
    let ident = path.get_ident()?;
    Some(normalized_ident(ident))
}

fn cfg_truth(meta: &Meta) -> Truth {
    match meta {
        Meta::Path(_) => match path_ident(meta).as_deref() {
            Some("test" | "false") => Truth::False,
            Some("true") => Truth::True,
            _ => Truth::Unknown,
        },
        Meta::NameValue(_) => Truth::Unknown,
        Meta::List(list) => {
            let Some(name) = list.path.get_ident().map(normalized_ident) else {
                return Truth::Unknown;
            };
            let parser = Punctuated::<Meta, Comma>::parse_terminated;
            let Ok(values) = parser.parse2(list.tokens.clone()) else {
                return Truth::Unknown;
            };
            match name.as_str() {
                "all" => values
                    .iter()
                    .fold(Truth::True, |result, value| result.and(cfg_truth(value))),
                "any" => {
                    let mut unknown = false;
                    for value in &values {
                        match cfg_truth(value) {
                            Truth::True => return Truth::True,
                            Truth::Unknown => unknown = true,
                            Truth::False => {}
                        }
                    }
                    if unknown {
                        Truth::Unknown
                    } else {
                        Truth::False
                    }
                }
                "not" if values.len() == 1 => cfg_truth(&values[0]).not(),
                _ => Truth::Unknown,
            }
        }
    }
}

fn meta_inclusion(meta: &Meta) -> Truth {
    let Some(name) = meta.path().get_ident().map(normalized_ident) else {
        return Truth::True;
    };
    match (name.as_str(), meta) {
        ("cfg", Meta::List(list)) => syn::parse2::<Meta>(list.tokens.clone())
            .map_or(Truth::Unknown, |value| cfg_truth(&value)),
        ("cfg_attr", Meta::List(list)) => {
            let parser = Punctuated::<Meta, Comma>::parse_terminated;
            let Ok(values) = parser.parse2(list.tokens.clone()) else {
                return Truth::Unknown;
            };
            let mut values = values.iter();
            let Some(predicate) = values.next() else {
                return Truth::Unknown;
            };
            let predicate = cfg_truth(predicate);
            if predicate == Truth::False {
                return Truth::True;
            }
            let applied = values.fold(Truth::True, |result, value| {
                result.and(meta_inclusion(value))
            });
            match predicate {
                Truth::True => applied,
                Truth::False => Truth::True,
                Truth::Unknown if applied == Truth::True => Truth::True,
                Truth::Unknown => Truth::Unknown,
            }
        }
        _ => Truth::True,
    }
}

fn attributes_inclusion(attributes: &[Attribute]) -> Truth {
    attributes.iter().fold(Truth::True, |result, attribute| {
        result.and(meta_inclusion(&attribute.meta))
    })
}

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

fn item_attributes(item: &Item) -> &[Attribute] {
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

fn impl_item_attributes(item: &ImplItem) -> &[Attribute] {
    match item {
        ImplItem::Const(value) => &value.attrs,
        ImplItem::Fn(value) => &value.attrs,
        ImplItem::Type(value) => &value.attrs,
        ImplItem::Macro(value) => &value.attrs,
        ImplItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn trait_item_attributes(item: &TraitItem) -> &[Attribute] {
    match item {
        TraitItem::Const(value) => &value.attrs,
        TraitItem::Fn(value) => &value.attrs,
        TraitItem::Type(value) => &value.attrs,
        TraitItem::Macro(value) => &value.attrs,
        TraitItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn foreign_item_attributes(item: &ForeignItem) -> &[Attribute] {
    match item {
        ForeignItem::Fn(value) => &value.attrs,
        ForeignItem::Static(value) => &value.attrs,
        ForeignItem::Type(value) => &value.attrs,
        ForeignItem::Macro(value) => &value.attrs,
        ForeignItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn expr_attributes(expr: &Expr) -> &[Attribute] {
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

struct SpanCollector<'a> {
    source: &'a str,
    spans: Vec<Range<usize>>,
    error: Option<String>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Reachability {
    Production,
    TestOnly,
}

struct ModuleCollector {
    context: Vec<String>,
    inherited: Reachability,
    test_edges: Vec<ModuleReference>,
    production_edges: Vec<ModuleReference>,
    error: Option<String>,
}

impl ModuleCollector {
    fn local_reachability(&self, attributes: &[Attribute]) -> Reachability {
        if self.inherited == Reachability::TestOnly
            || attributes_inclusion(attributes) == Truth::False
        {
            Reachability::TestOnly
        } else {
            Reachability::Production
        }
    }

    fn visit_items(&mut self, items: &[Item]) {
        for item in items {
            self.visit_item(item);
        }
    }

    fn visit_item(&mut self, item: &Item) {
        let local = self.local_reachability(item_attributes(item));
        if let Item::Mod(module) = item {
            self.visit_module(module, local);
            return;
        }
        let previous = self.inherited;
        self.inherited = local;
        let mut nested = NestedModuleVisitor { collector: self };
        visit::visit_item(&mut nested, item);
        self.inherited = previous;
    }

    fn visit_module(&mut self, module: &ItemMod, reachability: Reachability) {
        let name = normalized_ident(&module.ident);
        if let Some((_, items)) = &module.content {
            let path_override = match module_path_override(&module.attrs) {
                Ok(value) => value,
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
            };
            let previous_reachability = self.inherited;
            let previous_context_len = self.context.len();
            self.inherited = reachability;
            if let Some(path) = path_override {
                self.context
                    .extend(path.components().filter_map(|component| {
                        if let Component::Normal(value) = component {
                            value.to_str().map(str::to_owned)
                        } else {
                            None
                        }
                    }));
            } else {
                self.context.push(name);
            }
            self.visit_items(items);
            self.context.truncate(previous_context_len);
            self.inherited = previous_reachability;
            return;
        }
        let path_override = match module_path_override(&module.attrs) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error);
                return;
            }
        };
        let reference = ModuleReference {
            context: self.context.clone(),
            name,
            path_override,
        };
        let edges = match reachability {
            Reachability::Production => &mut self.production_edges,
            Reachability::TestOnly => &mut self.test_edges,
        };
        if edges.len() >= MAX_MODULE_EDGES {
            self.error = Some(format!("more than {MAX_MODULE_EDGES} module edges"));
        } else {
            edges.push(reference);
        }
    }
}

struct NestedModuleVisitor<'a> {
    collector: &'a mut ModuleCollector,
}

impl<'ast> Visit<'ast> for NestedModuleVisitor<'_> {
    fn visit_item(&mut self, item: &'ast Item) {
        self.collector.visit_item(item);
    }

    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        let local = self.collector.local_reachability(&module.attrs);
        self.collector.visit_module(module, local);
    }

    fn visit_stmt(&mut self, statement: &'ast Stmt) {
        let attributes: &[Attribute] = match statement {
            Stmt::Local(value) => &value.attrs,
            Stmt::Item(value) => item_attributes(value),
            Stmt::Expr(value, _) => expr_attributes(value),
            Stmt::Macro(value) => &value.attrs,
        };
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(attributes);
        visit::visit_stmt(self, statement);
        self.collector.inherited = previous;
    }

    fn visit_expr(&mut self, expression: &'ast Expr) {
        let previous = self.collector.inherited;
        self.collector.inherited = self
            .collector
            .local_reachability(expr_attributes(expression));
        visit::visit_expr(self, expression);
        self.collector.inherited = previous;
    }

    fn visit_arm(&mut self, arm: &'ast Arm) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&arm.attrs);
        visit::visit_arm(self, arm);
        self.collector.inherited = previous;
    }

    fn visit_field_value(&mut self, field: &'ast FieldValue) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&field.attrs);
        visit::visit_field_value(self, field);
        self.collector.inherited = previous;
    }

    fn visit_impl_item(&mut self, item: &'ast ImplItem) {
        let previous = self.collector.inherited;
        self.collector.inherited = self
            .collector
            .local_reachability(impl_item_attributes(item));
        visit::visit_impl_item(self, item);
        self.collector.inherited = previous;
    }

    fn visit_trait_item(&mut self, item: &'ast TraitItem) {
        let previous = self.collector.inherited;
        self.collector.inherited = self
            .collector
            .local_reachability(trait_item_attributes(item));
        visit::visit_trait_item(self, item);
        self.collector.inherited = previous;
    }

    fn visit_foreign_item(&mut self, item: &'ast ForeignItem) {
        let previous = self.collector.inherited;
        self.collector.inherited = self
            .collector
            .local_reachability(foreign_item_attributes(item));
        visit::visit_foreign_item(self, item);
        self.collector.inherited = previous;
    }

    fn visit_macro(&mut self, _node: &'ast syn::Macro) {
        // Macro token streams are authored but opaque; do not parse module-like tokens.
    }
}

fn module_path_override(attributes: &[Attribute]) -> Result<Option<PathBuf>, String> {
    let mut result = None;
    for attribute in attributes {
        if attribute.path().is_ident("cfg_attr")
            && conditional_path_override_may_apply(&attribute.meta)?
        {
            return Err(
                "conditional module path override is unsupported and may affect production"
                    .to_owned(),
            );
        }
        if !attribute.path().is_ident("path") {
            continue;
        }
        let Meta::NameValue(value) = &attribute.meta else {
            return Err("module path attribute must be a string name-value".to_owned());
        };
        let Expr::Lit(value) = &value.value else {
            return Err("module path attribute must contain a literal string".to_owned());
        };
        let syn::Lit::Str(value) = &value.lit else {
            return Err("module path attribute must contain a literal string".to_owned());
        };
        if result.is_some() {
            return Err("module has more than one path override".to_owned());
        }
        let path = PathBuf::from(value.value());
        if path.as_os_str().is_empty()
            || path.is_absolute()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
        {
            return Err("module path override must be a contained relative path".to_owned());
        }
        result = Some(path);
    }
    Ok(result)
}

fn conditional_path_override_may_apply(meta: &Meta) -> Result<bool, String> {
    let Meta::List(list) = meta else {
        return Ok(false);
    };
    let parser = Punctuated::<Meta, Comma>::parse_terminated;
    let values = parser
        .parse2(list.tokens.clone())
        .map_err(|_| "conditional module path attribute is malformed".to_owned())?;
    let mut values = values.iter();
    let Some(predicate) = values.next() else {
        return Err("conditional module path attribute has no predicate".to_owned());
    };
    if cfg_truth(predicate) == Truth::False {
        return Ok(false);
    }
    for applied in values {
        if applied.path().is_ident("path") {
            return Ok(true);
        }
        if applied.path().is_ident("cfg_attr") && conditional_path_override_may_apply(applied)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn merge_ranges(mut spans: Vec<Range<usize>>) -> Vec<Range<usize>> {
    spans.sort_by_key(|span| (span.start, span.end));
    let mut merged: Vec<Range<usize>> = Vec::new();
    for span in spans {
        if let Some(last) = merged.last_mut()
            && span.start <= last.end
        {
            last.end = last.end.max(span.end);
        } else {
            merged.push(span);
        }
    }
    merged
}

fn project_lines(source: &str, spans: Vec<Range<usize>>) -> Vec<usize> {
    let spans = merge_ranges(spans);
    let mut result = Vec::new();
    let mut offset = 0;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        let content: Vec<usize> = line
            .char_indices()
            .filter_map(|(index, character)| (!character.is_whitespace()).then_some(offset + index))
            .collect();
        if !content.is_empty()
            && content
                .iter()
                .all(|position| spans.iter().any(|span| span.contains(position)))
        {
            result.push(index + 1);
        }
        offset += line.len();
    }
    result
}

fn parse_source(path: &str, source: &str) -> Result<ParsedFile, String> {
    let file = syn::parse_file(source).map_err(|error| {
        let start = error.span().start();
        format!(
            "{path}:{}:{}: Rust parse failed: {error}",
            start.line,
            start.column + 1
        )
    })?;
    let mut span_collector = SpanCollector {
        source,
        spans: Vec::new(),
        error: None,
    };
    span_collector.visit_file(&file);
    if let Some(error) = span_collector.error {
        return Err(format!("{path}: {error}"));
    }
    let inherited = if attributes_inclusion(&file.attrs) == Truth::False {
        Reachability::TestOnly
    } else {
        Reachability::Production
    };
    let mut module_collector = ModuleCollector {
        context: Vec::new(),
        inherited,
        test_edges: Vec::new(),
        production_edges: Vec::new(),
        error: None,
    };
    module_collector.visit_items(&file.items);
    if let Some(error) = module_collector.error {
        return Err(format!("{path}: {error}"));
    }
    Ok(ParsedFile {
        spans: span_collector.spans,
        test_edges: module_collector.test_edges,
        production_edges: module_collector.production_edges,
    })
}

fn resolve_module(
    parent: &Path,
    reference: &ModuleReference,
    sources: &BTreeMap<PathBuf, String>,
) -> Result<PathBuf, String> {
    let source_directory = parent
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();
    let standard_bin_root = parent.extension().and_then(|value| value.to_str()) == Some("rs")
        && source_directory
            .file_name()
            .and_then(|value| value.to_str())
            == Some("bin")
        && source_directory
            .parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            == Some("src");
    let ordinary_base = if standard_bin_root
        || matches!(
            parent.file_name().and_then(|value| value.to_str()),
            Some("lib.rs" | "main.rs" | "mod.rs")
        ) {
        source_directory.clone()
    } else {
        source_directory.join(
            parent
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default(),
        )
    };
    let ordinary_contextual = reference
        .context
        .iter()
        .fold(ordinary_base, |path, component| path.join(component));
    if let Some(path_override) = &reference.path_override {
        let override_contextual = if reference.context.is_empty() {
            source_directory
        } else {
            ordinary_contextual.clone()
        };
        let candidate = override_contextual.join(path_override);
        if sources.contains_key(&candidate) {
            return Ok(candidate);
        }
        return Err(format!(
            "{}: path-attributed module {:?} does not resolve to an authored source",
            parent.display(),
            path_override
        ));
    }
    let candidates = [
        ordinary_contextual.join(format!("{}.rs", reference.name)),
        ordinary_contextual.join(&reference.name).join("mod.rs"),
    ];
    let matches: Vec<_> = candidates
        .into_iter()
        .filter(|candidate| sources.contains_key(candidate))
        .collect();
    match matches.as_slice() {
        [candidate] => Ok(candidate.clone()),
        _ => Err(format!(
            "{}: module {:?} must resolve to exactly one authored source",
            parent.display(),
            reference.name
        )),
    }
}

fn inherited_test_paths(
    parsed: &BTreeMap<PathBuf, ParsedFile>,
    sources: &BTreeMap<PathBuf, String>,
) -> Result<BTreeSet<PathBuf>, String> {
    let mut queued = VecDeque::new();
    for (parent, file) in parsed {
        for reference in &file.test_edges {
            queued.push_back(resolve_module(parent, reference, sources)?);
        }
    }
    let mut inherited = BTreeSet::new();
    while let Some(path) = queued.pop_front() {
        if !inherited.insert(path.clone()) {
            continue;
        }
        let file = parsed
            .get(&path)
            .ok_or_else(|| format!("missing parsed module source: {}", path.display()))?;
        for reference in file.test_edges.iter().chain(&file.production_edges) {
            queued.push_back(resolve_module(&path, reference, sources)?);
        }
    }

    loop {
        let mut removed = Vec::new();
        for (parent, file) in parsed {
            if inherited.contains(parent) {
                continue;
            }
            for reference in &file.production_edges {
                let child = resolve_module(parent, reference, sources)?;
                if inherited.contains(&child) {
                    removed.push(child);
                }
            }
        }
        removed.sort();
        removed.dedup();
        if removed.is_empty() {
            break;
        }
        for path in removed {
            inherited.remove(&path);
        }
    }
    Ok(inherited)
}

fn classify(request: Request) -> Result<Response, String> {
    if request.protocol_version != PROTOCOL_VERSION {
        return Err(format!(
            "unsupported protocol version {}; expected {PROTOCOL_VERSION}",
            request.protocol_version
        ));
    }
    if request.sources.len() > MAX_FILES {
        return Err(format!("more than {MAX_FILES} source files"));
    }
    let mut total = 0usize;
    let mut sources = BTreeMap::new();
    for input in request.sources {
        if input.path.len() > MAX_PATH_BYTES || input.path.is_empty() {
            return Err("source path is empty or exceeds the protocol bound".to_owned());
        }
        let path = PathBuf::from(&input.path);
        if path.is_absolute()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
            || path.extension().and_then(|value| value.to_str()) != Some("rs")
        {
            return Err(format!(
                "invalid repository-relative Rust path: {}",
                input.path
            ));
        }
        if input.source.len() > MAX_SOURCE_BYTES {
            return Err(format!(
                "{} exceeds {MAX_SOURCE_BYTES} source bytes",
                input.path
            ));
        }
        total = total
            .checked_add(input.source.len())
            .ok_or_else(|| "total source byte count overflowed".to_owned())?;
        if total > MAX_TOTAL_SOURCE_BYTES {
            return Err(format!(
                "total source bytes exceed {MAX_TOTAL_SOURCE_BYTES}"
            ));
        }
        if sources.insert(path, input.source).is_some() {
            return Err(format!("duplicate source path: {}", input.path));
        }
    }

    let mut parsed = BTreeMap::new();
    for (path, source) in &sources {
        parsed.insert(
            path.clone(),
            parse_source(&path.display().to_string(), source)?,
        );
    }
    let inherited = inherited_test_paths(&parsed, &sources)?;
    let mut files = Vec::with_capacity(sources.len());
    for (path, source) in &sources {
        let inline_test_lines = if inherited.contains(path) {
            source
                .split_inclusive('\n')
                .enumerate()
                .filter_map(|(index, line)| {
                    line.chars()
                        .any(|character| !character.is_whitespace())
                        .then_some(index + 1)
                })
                .collect()
        } else {
            project_lines(
                source,
                parsed.get(path).expect("parsed source").spans.clone(),
            )
        };
        files.push(FileOutput {
            inline_test_lines,
            path: path.display().to_string(),
        });
    }
    Ok(Response {
        classifier: CLASSIFIER_NAME,
        files,
        inherited_inline_paths: inherited
            .into_iter()
            .map(|path| path.display().to_string())
            .collect(),
        protocol_version: PROTOCOL_VERSION,
    })
}

fn run() -> Result<(), String> {
    let mut input = Vec::new();
    io::stdin()
        .take(MAX_REQUEST_BYTES + 1)
        .read_to_end(&mut input)
        .map_err(|error| format!("could not read request: {error}"))?;
    if input.len() as u64 > MAX_REQUEST_BYTES {
        return Err(format!("request exceeds {MAX_REQUEST_BYTES} bytes"));
    }
    let request: Request =
        serde_json::from_slice(&input).map_err(|error| format!("invalid request JSON: {error}"))?;
    let response = classify(request)?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &response)
        .map_err(|error| format!("could not encode response: {error}"))?;
    output
        .write_all(b"\n")
        .map_err(|error| format!("could not write response: {error}"))?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("code-health-classifier: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{PROTOCOL_VERSION, Request, SourceInput, classify};

    fn one(source: &str) -> Vec<usize> {
        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: source.to_owned(),
            }],
        })
        .expect("classification");
        response.files[0].inline_test_lines.clone()
    }

    #[test]
    fn classifies_full_ast_nodes_without_consuming_shipping_siblings() {
        let source = r####"
struct Named {
    #[cfg(test)] hidden: Vec<(u8, u8)>,
    visible: u8,
}
enum Choice {
    #[cfg(test)] Hidden { value: u8 },
    Visible,
}
fn parameters(#[cfg(test)] hidden: u8, visible: u8) {
    #[cfg(test)] 'λ: loop { break 'λ; }
    shipping();
    match visible {
        #[cfg(test)] 1 => if test_only() { 1 << 2 } else { 0 },
        _ => shipping(),
    }
}
#[cfg_attr(not(test), cfg(any()))]
δοκιμή! { r###"#[cfg(test)]"### }
pub fn shipping() {}
"####;
        let lines = one(source);
        for line in [3, 7, 11, 14, 18, 19] {
            assert!(
                lines.contains(&line),
                "expected test-only line {line}: {lines:?}"
            );
        }
        for line in [4, 8, 10, 12, 15, 20] {
            assert!(!lines.contains(&line), "shipping line {line}: {lines:?}");
        }
    }

    #[test]
    fn classifies_struct_literal_fields_by_ast_boundary() {
        let source = r#"
struct Values { test: u8, shipping: u8 }
fn literal() { let _ = Values {
    #[cfg(test)]
    test: 1,
    shipping: 2,
}; }
"#;
        let lines = one(source);
        for line in [4, 5] {
            assert!(
                lines.contains(&line),
                "expected test-only field line {line}: {lines:?}"
            );
        }
        for line in [2, 3, 6, 7] {
            assert!(!lines.contains(&line), "shipping line {line}: {lines:?}");
        }
    }

    #[test]
    fn mixed_line_and_macro_token_attributes_remain_production() {
        let source = r#"
#[cfg(test)] fn hidden() {} pub fn shipping() {}
macro_rules! keep { (#[cfg(test)] $item:item) => { $item } }
keep! { #[cfg(test)] pub fn emitted() {} }
"#;
        assert!(one(source).is_empty());
    }

    #[test]
    fn handles_spaced_attributes_inner_cfg_and_recursive_cfg_attr() {
        let source = r###"
# [cfg(test)]
fn spaced_attribute() {}
mod inner_scope {
    #![cfg(test)]
    fn nested() {}
}
#[cfg_attr(not(test), cfg_attr(not(test), cfg(any())))]
fn recursive_cfg_attr() {}
#[cfg(any(test, feature = r#"diagnostics"#))]
fn unknown_alternative() {}
#[cfg(all(test, /* nested /* comment */ remains */ unix))]
fn nested_comment() {}
"###;
        let lines = one(source);
        for line in [2, 3, 4, 5, 6, 7, 8, 9, 12, 13] {
            assert!(
                lines.contains(&line),
                "expected test-only line {line}: {lines:?}"
            );
        }
        assert!(!lines.contains(&11), "unknown cfg must remain production");
    }

    #[test]
    fn classifies_comma_less_members_and_generic_parameters_by_ast_boundary() {
        let source = r#"
struct Named {
    #[cfg(test)] hidden: u8
}
struct Generic<#[cfg(test)] T> { visible: u8 }
enum Choice {
    #[cfg(test)] Hidden
}
fn arguments(
    #[cfg(test)] hidden: u8
) {}
pub const SHIPPING: u8 = 1;
"#;
        let lines = one(source);
        for line in [3, 7, 10] {
            assert!(
                lines.contains(&line),
                "expected test-only line {line}: {lines:?}"
            );
        }
        assert!(
            !lines.contains(&5),
            "mixed generic line must remain production"
        );
        assert!(!lines.contains(&12));
    }

    #[test]
    fn raw_modules_path_overrides_and_production_reachability_are_deterministic() {
        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/lib.rs".to_owned(),
                    source: concat!(
                        "#[cfg(test)] mod r#helper;\n",
                        "#[cfg(not(test))] mod helper;\n",
                        "#[cfg(test)] #[path = \"fixtures/test_only.rs\"] mod fixture;\n",
                    )
                    .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/helper.rs".to_owned(),
                    source: "mod child;\nfn shared() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/helper/child.rs".to_owned(),
                    source: "fn shipping_child() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/fixtures/test_only.rs".to_owned(),
                    source: "fn fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("classification");
        assert_eq!(
            response.inherited_inline_paths,
            vec!["crates/demo/src/fixtures/test_only.rs"]
        );

        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/foo.rs".to_owned(),
                    source: "#[cfg(test)] #[path = \"bar.rs\"] mod direct;\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/bar.rs".to_owned(),
                    source: "fn direct_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("non-root path override classification");
        assert_eq!(
            response.inherited_inline_paths,
            vec!["crates/demo/src/bar.rs"]
        );

        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/lib.rs".to_owned(),
                    source: concat!(
                        "fn local() { #[cfg(test)] { #[path = \"helper.rs\"] mod helper; } }\n",
                        "#[cfg(test)] mod shared;\n",
                    )
                    .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/helper.rs".to_owned(),
                    source: "fn local_fixture() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/shared.rs".to_owned(),
                    source: "fn shared_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("nested expression reachability");
        assert_eq!(
            response.inherited_inline_paths,
            vec!["crates/demo/src/helper.rs", "crates/demo/src/shared.rs",]
        );

        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/lib.rs".to_owned(),
                    source: concat!(
                        "struct Demo;\n",
                        "impl Demo { #[cfg(test)] fn check() { #[path = \"impl_helper.rs\"] mod helper; } }\n",
                        "trait DemoTrait { #[cfg(test)] fn check() { #[path = \"trait_helper.rs\"] mod helper; } }\n",
                    )
                    .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/impl_helper.rs".to_owned(),
                    source: "fn impl_fixture() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/trait_helper.rs".to_owned(),
                    source: "fn trait_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("associated item reachability");
        assert_eq!(
            response.inherited_inline_paths,
            vec![
                "crates/demo/src/impl_helper.rs",
                "crates/demo/src/trait_helper.rs",
            ]
        );

        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/bin/tool.rs".to_owned(),
                    source: "#[cfg(test)] mod helper;\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/bin/helper.rs".to_owned(),
                    source: "fn bin_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("file-based binary root reachability");
        assert_eq!(
            response.inherited_inline_paths,
            vec!["crates/demo/src/bin/helper.rs"]
        );

        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/foo.rs".to_owned(),
                    source: "#[cfg(test)] mod inner { #[path = \"bar.rs\"] mod helper; }\n"
                        .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/foo/inner/bar.rs".to_owned(),
                    source: "fn nested_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("non-root nested path override reachability");
        assert_eq!(
            response.inherited_inline_paths,
            vec!["crates/demo/src/foo/inner/bar.rs"]
        );

        let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/lib.rs".to_owned(),
                    source: "#[cfg(test)] #[path = \"custom\"] mod inline { mod child; }\n"
                        .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/custom/child.rs".to_owned(),
                    source: "fn inline_path_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("path-adjusted inline module reachability");
        assert_eq!(
            response.inherited_inline_paths,
            vec!["crates/demo/src/custom/child.rs"]
        );

        let error = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/lib.rs".to_owned(),
                    source: concat!(
                        "#[cfg_attr(feature = \"x\", path = \"shared.rs\")] mod product;\n",
                        "#[cfg(test)] #[path = \"shared.rs\"] mod fixture;\n",
                    )
                    .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/product.rs".to_owned(),
                    source: "fn default_product() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/shared.rs".to_owned(),
                    source: "fn feature_product() {}\n".to_owned(),
                },
            ],
        })
        .expect_err("conditional path ambiguity");
        assert!(error.contains("conditional module path override"));
    }

    #[test]
    fn rejects_malformed_or_escaping_inputs() {
        let error = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: "#[path = \"../escape.rs\"] mod escape;".to_owned(),
            }],
        })
        .expect_err("escaping path");
        assert!(error.contains("contained relative path"));

        let error = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            sources: vec![SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: "fn broken( {".to_owned(),
            }],
        })
        .expect_err("malformed Rust");
        assert!(error.contains("Rust parse failed"));

        let error = classify(Request {
            protocol_version: PROTOCOL_VERSION + 1,
            sources: Vec::new(),
        })
        .expect_err("protocol mismatch");
        assert!(error.contains("unsupported protocol version"));
    }
}

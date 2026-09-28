//! Bounded Rust module discovery, path resolution, and production-wins reachability.

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    ops::Range,
    path::{Component, Path, PathBuf},
};

use syn::{
    Arm, Attribute, BareFnArg, BareVariadic, Expr, Field, FieldPat, FieldValue, FnArg, ForeignItem,
    GenericParam, ImplItem, Item, ItemMod, Meta, Pat, Stmt, TraitItem, Variadic,
    visit::{self, Visit},
};

use super::{
    MAX_MODULE_EDGES,
    cfg::{Truth, attributes_inclusion_for_test, cfg_attr_parts, normalized_ident},
    spans::{
        expr_attributes, foreign_item_attributes, impl_item_attributes, item_attributes,
        pat_attributes, trait_item_attributes,
    },
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) struct ModuleReference {
    pub(super) context: Vec<String>,
    pub(super) context_from_source_directory: bool,
    pub(super) name: String,
    pub(super) path_override: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub(super) struct ParsedFile {
    pub(super) spans: Vec<Range<usize>>,
    pub(super) test_edges: Vec<ModuleReference>,
    pub(super) production_edges: Vec<ModuleReference>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Reachability {
    Disabled,
    Production,
    TestOnly,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum ModuleRole {
    Root,
    Nested,
}

pub(super) struct ModuleCollector {
    pub(super) context: Vec<String>,
    pub(super) context_from_source_directory: bool,
    pub(super) inherited: Reachability,
    pub(super) test_edges: Vec<ModuleReference>,
    pub(super) production_edges: Vec<ModuleReference>,
    pub(super) error: Option<String>,
}

impl ModuleCollector {
    pub(super) fn local_reachability(&self, attributes: &[Attribute]) -> Reachability {
        if self.inherited == Reachability::Disabled {
            return Reachability::Disabled;
        }
        let production = attributes_inclusion_for_test(attributes, false);
        let test = attributes_inclusion_for_test(attributes, true);
        match self.inherited {
            Reachability::Disabled => Reachability::Disabled,
            Reachability::Production if production != Truth::False => Reachability::Production,
            Reachability::Production if test == Truth::False => Reachability::Disabled,
            Reachability::Production | Reachability::TestOnly if test != Truth::False => {
                Reachability::TestOnly
            }
            Reachability::Production => Reachability::Disabled,
            Reachability::TestOnly => Reachability::Disabled,
        }
    }

    pub(super) fn visit_items(&mut self, items: &[Item]) {
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
        if reachability == Reachability::Disabled {
            return;
        }
        let name = normalized_ident(&module.ident);
        if let Some((_, items)) = &module.content {
            let path_override = match module_path_override(&module.attrs, reachability) {
                Ok(value) => value,
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
            };
            let previous_reachability = self.inherited;
            let previous_context_len = self.context.len();
            let previous_context_base = self.context_from_source_directory;
            self.inherited = reachability;
            if let Some(path) = path_override {
                if self.context.is_empty() {
                    self.context_from_source_directory = true;
                }
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
            self.context_from_source_directory = previous_context_base;
            self.inherited = previous_reachability;
            return;
        }
        let path_override = match module_path_override(&module.attrs, reachability) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error);
                return;
            }
        };
        let reference = ModuleReference {
            context: self.context.clone(),
            context_from_source_directory: self.context_from_source_directory,
            name,
            path_override,
        };
        let edges = match reachability {
            Reachability::Disabled => return,
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

    fn visit_field(&mut self, field: &'ast Field) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&field.attrs);
        visit::visit_field(self, field);
        self.collector.inherited = previous;
    }

    fn visit_bare_fn_arg(&mut self, argument: &'ast BareFnArg) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&argument.attrs);
        visit::visit_bare_fn_arg(self, argument);
        self.collector.inherited = previous;
    }

    fn visit_variadic(&mut self, variadic: &'ast Variadic) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&variadic.attrs);
        visit::visit_variadic(self, variadic);
        self.collector.inherited = previous;
    }

    fn visit_bare_variadic(&mut self, variadic: &'ast BareVariadic) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&variadic.attrs);
        visit::visit_bare_variadic(self, variadic);
        self.collector.inherited = previous;
    }

    fn visit_field_pat(&mut self, field: &'ast FieldPat) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&field.attrs);
        visit::visit_field_pat(self, field);
        self.collector.inherited = previous;
    }

    fn visit_variant(&mut self, variant: &'ast syn::Variant) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(&variant.attrs);
        visit::visit_variant(self, variant);
        self.collector.inherited = previous;
    }

    fn visit_fn_arg(&mut self, argument: &'ast FnArg) {
        let attributes: &[Attribute] = match argument {
            FnArg::Receiver(value) => &value.attrs,
            FnArg::Typed(value) => &value.attrs,
        };
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(attributes);
        visit::visit_fn_arg(self, argument);
        self.collector.inherited = previous;
    }

    fn visit_generic_param(&mut self, parameter: &'ast GenericParam) {
        let attributes: &[Attribute] = match parameter {
            GenericParam::Lifetime(value) => &value.attrs,
            GenericParam::Type(value) => &value.attrs,
            GenericParam::Const(value) => &value.attrs,
        };
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(attributes);
        visit::visit_generic_param(self, parameter);
        self.collector.inherited = previous;
    }

    fn visit_pat(&mut self, pattern: &'ast Pat) {
        let previous = self.collector.inherited;
        self.collector.inherited = self.collector.local_reachability(pat_attributes(pattern));
        visit::visit_pat(self, pattern);
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

pub(super) fn module_path_override(
    attributes: &[Attribute],
    reachability: Reachability,
) -> Result<Option<PathBuf>, String> {
    let production = module_path_override_for_test(attributes, false)?;
    let test = module_path_override_for_test(attributes, true)?;
    match reachability {
        Reachability::Disabled => Ok(None),
        Reachability::TestOnly => Ok(test),
        Reachability::Production
            if attributes_inclusion_for_test(attributes, true) == Truth::False =>
        {
            Ok(production)
        }
        Reachability::Production if production == test => Ok(production),
        Reachability::Production => {
            Err("conditional module path override differs between production and test".to_owned())
        }
    }
}

fn module_path_override_for_test(
    attributes: &[Attribute],
    test_enabled: bool,
) -> Result<Option<PathBuf>, String> {
    let mut result = None;
    for attribute in attributes {
        apply_path_meta(&attribute.meta, test_enabled, &mut result)?;
    }
    Ok(result)
}

fn apply_path_meta(
    meta: &Meta,
    test_enabled: bool,
    result: &mut Option<PathBuf>,
) -> Result<(), String> {
    if meta.path().is_ident("path") {
        let Meta::NameValue(value) = meta else {
            return Err("module path attribute must be a string name-value".to_owned());
        };
        let Expr::Lit(value) = &value.value else {
            return Err("module path attribute must contain a literal string".to_owned());
        };
        let syn::Lit::Str(value) = &value.lit else {
            return Err("module path attribute must contain a literal string".to_owned());
        };
        if result.is_some() {
            return Err("module has more than one active path override".to_owned());
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
        *result = Some(path);
        return Ok(());
    }
    if !meta.path().is_ident("cfg_attr") {
        return Ok(());
    }
    let Meta::List(list) = meta else {
        return Err("conditional module path attribute is malformed".to_owned());
    };
    let (predicate, values) = cfg_attr_parts(list, test_enabled)
        .map_err(|_| "conditional module path attribute is malformed".to_owned())?;
    match predicate {
        Truth::False => Ok(()),
        Truth::True => {
            for applied in &values {
                apply_path_meta(applied, test_enabled, result)?;
            }
            Ok(())
        }
        Truth::Unknown if values.iter().any(meta_applies_path) => {
            Err("conditional module path override has an unknown predicate".to_owned())
        }
        Truth::Unknown => Ok(()),
    }
}

fn meta_applies_path(meta: &Meta) -> bool {
    if meta.path().is_ident("path") {
        return true;
    }
    if !meta.path().is_ident("cfg_attr") {
        return false;
    }
    let Meta::List(list) = meta else {
        return false;
    };
    cfg_attr_parts(list, false).is_ok_and(|(_, values)| values.iter().any(meta_applies_path))
}

pub(super) fn resolve_module(
    parent: &Path,
    reference: &ModuleReference,
    sources: &BTreeMap<PathBuf, String>,
    role: ModuleRole,
) -> Result<PathBuf, String> {
    let source_directory = parent
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();
    let ordinary_base = match role {
        ModuleRole::Root => source_directory.clone(),
        ModuleRole::Nested
            if parent.file_name().and_then(|value| value.to_str()) == Some("mod.rs") =>
        {
            source_directory.clone()
        }
        ModuleRole::Nested => source_directory.join(
            parent
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default(),
        ),
    };
    let contextual_base = if reference.context_from_source_directory {
        source_directory.clone()
    } else {
        ordinary_base
    };
    let ordinary_contextual = reference
        .context
        .iter()
        .fold(contextual_base, |path, component| path.join(component));
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

#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct TraversalWork {
    pub(super) edge_resolutions: usize,
}

fn inherited_test_paths_with_work(
    parsed: &BTreeMap<PathBuf, ParsedFile>,
    sources: &BTreeMap<PathBuf, String>,
    roots: &BTreeSet<PathBuf>,
) -> Result<(BTreeSet<PathBuf>, TraversalWork), String> {
    let mut work = TraversalWork::default();
    let mut contexts: VecDeque<_> = parsed
        .keys()
        .map(|path| {
            let role = if roots.contains(path) {
                ModuleRole::Root
            } else {
                ModuleRole::Nested
            };
            (path.clone(), role)
        })
        .collect();
    let mut visited_contexts = BTreeSet::new();
    let mut queued = VecDeque::new();
    while let Some((parent, role)) = contexts.pop_front() {
        if !visited_contexts.insert((parent.clone(), role)) {
            continue;
        }
        let file = parsed
            .get(&parent)
            .ok_or_else(|| format!("missing parsed module source: {}", parent.display()))?;
        for reference in &file.test_edges {
            work.edge_resolutions += 1;
            queued.push_back(resolve_module(&parent, reference, sources, role)?);
        }
        for reference in &file.production_edges {
            work.edge_resolutions += 1;
            let child = resolve_module(&parent, reference, sources, role)?;
            contexts.push_back((child, ModuleRole::Nested));
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
            work.edge_resolutions += 1;
            queued.push_back(resolve_module(
                &path,
                reference,
                sources,
                ModuleRole::Nested,
            )?);
        }
    }

    let mut production = roots.clone();
    let mut production_queue: VecDeque<_> = roots
        .iter()
        .cloned()
        .map(|path| (path, ModuleRole::Root))
        .collect();
    for path in parsed.keys() {
        if !inherited.contains(path) && !roots.contains(path) {
            production.insert(path.clone());
            production_queue.push_back((path.clone(), ModuleRole::Nested));
        }
    }
    let mut visited = BTreeSet::new();
    while let Some((path, role)) = production_queue.pop_front() {
        if !visited.insert((path.clone(), role)) {
            continue;
        }
        let file = parsed
            .get(&path)
            .ok_or_else(|| format!("missing parsed module source: {}", path.display()))?;
        for reference in &file.production_edges {
            work.edge_resolutions += 1;
            let child = resolve_module(&path, reference, sources, role)?;
            production.insert(child.clone());
            production_queue.push_back((child, ModuleRole::Nested));
        }
    }
    inherited.retain(|path| !production.contains(path));
    Ok((inherited, work))
}

pub(super) fn inherited_test_paths(
    parsed: &BTreeMap<PathBuf, ParsedFile>,
    sources: &BTreeMap<PathBuf, String>,
    roots: &BTreeSet<PathBuf>,
) -> Result<BTreeSet<PathBuf>, String> {
    inherited_test_paths_with_work(parsed, sources, roots).map(|(paths, _)| paths)
}

#[cfg(test)]
pub(super) fn traversal_work(
    parsed: &BTreeMap<PathBuf, ParsedFile>,
    sources: &BTreeMap<PathBuf, String>,
    roots: &BTreeSet<PathBuf>,
) -> Result<TraversalWork, String> {
    inherited_test_paths_with_work(parsed, sources, roots).map(|(_, work)| work)
}

//! Shared guard helpers for `identus-conformance`.
//!
//! File-walking (workspace source files and crate manifests), root-manifest
//! parsing, the dep-graph layer/direction helpers, and `syn` source parsing
//! are centralized here and reused by every guard under `guard/`. A guard
//! SHALL NOT duplicate file-walking, manifest-parsing, or `syn::parse_file`
//! logic that already exists here (per the `conformance-crate-structure`
//! capability).
//!
//! This module and its submodules are `#[cfg(test)]`: no guard logic compiles
//! into a production build, and `syn` (a dev-dependency) stays out of the
//! production dependency graph.

use crate::{LAYER_RULES, Layer};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use syn::File;

/// Resolve the workspace root from this crate's `CARGO_MANIFEST_DIR`
/// (`crates/conformance` → two levels up).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Resolve a crate name to its layer via `LAYER_RULES` membership.
pub(super) fn layer_of(crate_name: &str) -> Option<Layer> {
    LAYER_RULES
        .iter()
        .find(|rule| rule.members.iter().any(|m| m.name == crate_name))
        .map(|rule| rule.layer)
}

/// Whether a crate is a build-time proc-macro crate, per its `proc_macro`
/// flag in `LAYER_RULES`. Proc-macro targets are exempt from the
/// inward-direction policy: any crate may depend on them regardless of
/// layer.
pub(super) fn is_proc_macro(crate_name: &str) -> bool {
    LAYER_RULES
        .iter()
        .find_map(|rule| rule.members.iter().find(|m| m.name == crate_name))
        .map(|m| m.proc_macro)
        .unwrap_or(false)
}

/// Whether a crate sits in the `OuterBoundary` or `Verification` layer
/// (i.e. outside the production domain), derived from `LAYER_RULES`
/// membership rather than a hand-maintained list. Such crates may be
/// depended on by other outer/verification crates but never by production
/// crates.
pub(super) fn is_outer_or_verification(crate_name: &str) -> bool {
    matches!(
        layer_of(crate_name),
        Some(Layer::OuterBoundary) | Some(Layer::Verification)
    )
}

/// The allowed inward target layers for a source layer.
pub(super) fn allowed_target_layers(layer: Layer) -> &'static [Layer] {
    LAYER_RULES
        .iter()
        .find(|rule| rule.layer == layer)
        .expect("every layer has a rulebook entry")
        .allowed_target_layers
}

/// Validate a single workspace-internal dependency edge `source -> target`
/// against the layer policy. Proc-macro targets are exempt from the
/// inward-direction check (any crate may depend on a `proc_macro = true`
/// build-time crate). Production crates (those outside the outer-boundary
/// and verification layers) SHALL NOT depend on outer/verification crates.
/// Returns `Err(message)` on a policy violation for assertion-based tests.
pub(super) fn check_dep_edge(source: &str, target: &str) -> Result<(), String> {
    let source_layer = layer_of(source)
        .unwrap_or_else(|| panic!("crate {source} has no layer assignment in LAYER_RULES"));
    let target_layer = layer_of(target)
        .unwrap_or_else(|| panic!("crate {target} has no layer assignment in LAYER_RULES"));

    // Proc-macro targets are exempt from the inward-direction policy.
    if !is_proc_macro(target) {
        let allowed = allowed_target_layers(source_layer);
        if !allowed.contains(&target_layer) {
            let allowed_names: Vec<&str> = allowed.iter().map(|l| l.as_str()).collect();
            return Err(format!(
                "{source} ({}) depends on {target} ({}), but {} may only depend inward to {:?}",
                source_layer.as_str(),
                target_layer.as_str(),
                source_layer.as_str(),
                allowed_names
            ));
        }
    }

    // No production crate depends on the outer boundary or verification.
    let is_production = !is_outer_or_verification(source);
    if is_production && is_outer_or_verification(target) {
        return Err(format!(
            "production crate {source} must not depend on outer/verification crate {target}"
        ));
    }
    Ok(())
}

/// The set of all `identus-*` workspace crate names declared in
/// `[workspace.dependencies]`. External (non-path) entries in that table
/// are ignored here; they are enumerated separately by the external-dep
/// guard.
pub(super) fn workspace_crate_names(root_manifest: &toml::Value) -> HashSet<String> {
    let mut names = HashSet::new();
    if let Some(deps) = root_manifest
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.as_table())
    {
        for (key, value) in deps {
            // Workspace-internal crates are declared with a `path` key
            // (e.g. `identus-core = { path = "crates/core" }`). External
            // deps (e.g. `toml = "0.8"`) carry a version and no path.
            let is_internal = value
                .as_table()
                .map(|t| t.contains_key("path"))
                .unwrap_or(false);
            if is_internal {
                names.insert(key.clone());
            }
        }
    }
    // Sanity: the workspace dependency map must list exactly the
    // `LAYER_RULES`-derived workspace crate count.
    assert_eq!(
        names.len(),
        LAYER_RULES.iter().map(|r| r.members.len()).sum::<usize>(),
        "root [workspace.dependencies] must list every workspace crate"
    );
    names
}

/// Parse a `Cargo.toml` file into a `toml::Value`.
pub(super) fn read_manifest(path: &Path) -> toml::Value {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    toml::from_str(&contents).unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()))
}

/// The `[dependencies]` keys of a crate manifest that are workspace members.
pub(super) fn inward_workspace_deps(
    manifest: &toml::Value,
    workspace: &HashSet<String>,
) -> Vec<String> {
    let mut deps = Vec::new();
    if let Some(table) = manifest.get("dependencies").and_then(|d| d.as_table()) {
        for key in table.keys() {
            if workspace.contains(key) {
                deps.push(key.clone());
            }
        }
    }
    deps.sort();
    deps
}

/// The crate's `package.name` from its manifest.
pub(super) fn package_name(manifest: &toml::Value) -> String {
    manifest
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .expect("crate manifest has a [package].name")
        .to_owned()
}

/// Every `crates/*/Cargo.toml` in the workspace, sorted for stable order.
pub(super) fn crate_manifest_paths(workspace_root: &Path) -> Vec<PathBuf> {
    let crates_dir = workspace_root.join("crates");
    let mut manifest_paths: Vec<PathBuf> = fs::read_dir(&crates_dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", crates_dir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .map(|p| p.join("Cargo.toml"))
        .filter(|p| p.exists())
        .collect();
    manifest_paths.sort();
    manifest_paths
}

/// The three dependency-section kinds a manifest may carry, in both their
/// top-level and target-scoped forms.
#[derive(Debug, Clone, Copy)]
pub(super) enum DepSection {
    Dependencies,
    DevDependencies,
    BuildDependencies,
}

impl DepSection {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            DepSection::Dependencies => "dependencies",
            DepSection::DevDependencies => "dev-dependencies",
            DepSection::BuildDependencies => "build-dependencies",
        }
    }
}

pub(super) const DEP_SECTIONS: [DepSection; 3] = [
    DepSection::Dependencies,
    DepSection::DevDependencies,
    DepSection::BuildDependencies,
];

/// An enumerated external dependency entry: the crate and section it was
/// found in, plus the inline fields the guard inspects.
#[derive(Debug, Clone)]
pub(super) struct ExternalDepEntry {
    pub(super) crate_name: String,
    /// Human-readable location, e.g. `dependencies`,
    /// `dev-dependencies`, `target.'cfg(...)'.dependencies`.
    pub(super) section: String,
    pub(super) name: String,
    pub(super) has_version: bool,
    pub(super) has_workspace: bool,
    pub(super) optional: bool,
}

/// Enumerate the external dependency entries across every dependency-section
/// kind in `manifest` (top-level and target-scoped), exposing the inline
/// fields the guard inspects. "External" means non-`path` and not a
/// workspace-internal crate name.
pub(super) fn external_dep_entries(
    crate_name: &str,
    manifest: &toml::Value,
    workspace: &HashSet<String>,
) -> Vec<ExternalDepEntry> {
    let mut entries = Vec::new();
    for section in DEP_SECTIONS {
        collect_external_from_table(
            crate_name,
            section.as_str(),
            manifest.get(section.as_str()),
            workspace,
            &mut entries,
        );
    }
    // Target-scoped variants: [target.'cfg(...)'.{dependencies,dev-dependencies,build-dependencies}]
    if let Some(targets) = manifest.get("target").and_then(|t| t.as_table()) {
        for (target_spec, target_value) in targets {
            let Some(target_table) = target_value.as_table() else {
                continue;
            };
            for section in DEP_SECTIONS {
                let section_label = format!("target.'{}'.{}", target_spec, section.as_str());
                collect_external_from_table(
                    crate_name,
                    &section_label,
                    target_table.get(section.as_str()),
                    workspace,
                    &mut entries,
                );
            }
        }
    }
    entries
}

/// Collect external entries from a single dependency table into `out`.
pub(super) fn collect_external_from_table(
    crate_name: &str,
    section: &str,
    table: Option<&toml::Value>,
    workspace: &HashSet<String>,
    out: &mut Vec<ExternalDepEntry>,
) {
    let Some(table) = table.and_then(|t| t.as_table()) else {
        return;
    };
    for (name, value) in table {
        let (has_version, has_workspace, has_path, optional) = match value {
            // `toml = "0.8"` — the string form is an inline version.
            toml::Value::String(_) => (true, false, false, false),
            toml::Value::Table(t) => (
                t.contains_key("version"),
                t.contains_key("workspace"),
                t.contains_key("path"),
                t.get("optional").and_then(|o| o.as_bool()).unwrap_or(false),
            ),
            _ => continue,
        };
        // Skip workspace-internal and path-style entries; they are out of
        // scope for the external single-source rule.
        if workspace.contains(name) || has_path {
            continue;
        }
        out.push(ExternalDepEntry {
            crate_name: crate_name.to_owned(),
            section: section.to_owned(),
            name: name.clone(),
            has_version,
            has_workspace,
            optional,
        });
    }
}

/// Parse a `.rs` source file into a `syn::File`. Fails closed: a parse error
/// is propagated (the caller surfaces it rather than silently skipping the
/// file). Reused by every source-scanning guard.
pub(super) fn parse_rs(path: &Path) -> File {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    syn::parse_file(&contents).unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()))
}

/// Enumerate every `crates/*/src/**/*.rs` source file in the workspace, sorted
/// for stable order. The shared source-file walker used by source-scanning
/// guards (port discovery, adapter-suffix enforcement).
pub(super) fn walk_src_files() -> Vec<PathBuf> {
    let root = workspace_root();
    let crates_dir = root.join("crates");
    let mut files = Vec::new();
    for entry in fs::read_dir(&crates_dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", crates_dir.display()))
        .flatten()
    {
        let path = entry.path();
        if path.is_dir() {
            let src = path.join("src");
            if src.is_dir() {
                walk_dir_rs(&src, &mut files);
            }
        }
    }
    files.sort();
    files
}

/// Recursively collect `*.rs` files under `dir`.
fn walk_dir_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Descend into every directory under a crate (e.g. `src/`,
            // `src/crypto/`); the walker is rooted at `crates/`.
            walk_dir_rs(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

mod boundary;
mod dep_graph;
mod naming;

//! Chain-neutral dependency-boundary enforcement.
//!
//! These tests inspect both direct Cargo declarations and the resolved lockfile
//! closure. The rule is deliberately offline and data-driven: donor
//! repositories are evidence sources, never SDK dependencies.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct DependencyDeclaration {
    section: String,
    alias: String,
    package: String,
    git: Option<String>,
    path: Option<String>,
}

const DENIED_SOURCE_IDENTITIES: &[(&str, &str)] = &[
    ("apollo repository", "hyperledger-identus/apollo"),
    ("NeoPRISM repository", "hyperledger-identus/neoprism"),
    (
        "midnight-identity repository",
        "medianoxlabs/midnight-identity",
    ),
    (
        "Lace ID Portal repository",
        "input-output-hk/lace-id-portal",
    ),
    ("Oxid repository", "medianoxlabs/oxid"),
];

fn normalize_identity(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('_', "-")
}

fn denied_identity(identity: &str) -> Option<&'static str> {
    let identity = normalize_identity(identity);
    let matches_family =
        |exact: &str| identity == exact || identity.starts_with(&format!("{exact}-"));

    if matches_family("midnight") {
        Some("Midnight")
    } else if matches_family("compact-runtime") {
        Some("Compact runtime")
    } else if matches_family("cardano")
        || matches_family("pallas")
        || matches_family("oura")
        || matches_family("blockfrost")
    {
        Some("Cardano")
    } else if matches_family("prism")
        || matches_family("neoprism")
        || identity == "identus-did-prism"
        || identity.starts_with("identus-did-prism-")
    {
        Some("PRISM")
    } else if matches_family("oxid") {
        Some("Oxid")
    } else if identity == "lace-id"
        || identity.starts_with("lace-id-")
        || identity == "lace-wallet"
        || identity.starts_with("lace-wallet-")
    {
        Some("Lace product")
    } else {
        None
    }
}

fn denied_source(source: &str) -> Option<&'static str> {
    let normalized = source.to_ascii_lowercase();
    let exact_source = DENIED_SOURCE_IDENTITIES
        .iter()
        .find(|(_, needle)| normalized.contains(needle))
        .map(|(label, _)| *label);
    if exact_source.is_some() {
        return exact_source;
    }

    let without_fragment = source.split('#').next().unwrap_or(source);
    let without_query = without_fragment
        .split('?')
        .next()
        .unwrap_or(without_fragment);
    let repository = without_query
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .rsplit(|character| ['/', ':'].contains(&character))
        .next()
        .unwrap_or_default();
    denied_identity(repository)
}

fn collect_dependency_table(
    section: &str,
    value: Option<&toml::Value>,
    declarations: &mut Vec<DependencyDeclaration>,
) {
    let Some(table) = value.and_then(toml::Value::as_table) else {
        return;
    };

    for (alias, declaration) in table {
        let (package, git, path) = match declaration {
            toml::Value::String(_) => (alias.clone(), None, None),
            toml::Value::Table(fields) => (
                fields
                    .get("package")
                    .and_then(toml::Value::as_str)
                    .unwrap_or(alias)
                    .to_owned(),
                fields
                    .get("git")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
                fields
                    .get("path")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            ),
            _ => continue,
        };

        declarations.push(DependencyDeclaration {
            section: section.to_owned(),
            alias: alias.clone(),
            package,
            git,
            path,
        });
    }
}

fn dependency_declarations(manifest: &toml::Value) -> Vec<DependencyDeclaration> {
    let mut declarations = Vec::new();

    for section in DEP_SECTIONS {
        collect_dependency_table(
            section.as_str(),
            manifest.get(section.as_str()),
            &mut declarations,
        );
    }

    if let Some(targets) = manifest.get("target").and_then(toml::Value::as_table) {
        for (target, value) in targets {
            let Some(target_table) = value.as_table() else {
                continue;
            };
            for section in DEP_SECTIONS {
                collect_dependency_table(
                    &format!("target.'{target}'.{}", section.as_str()),
                    target_table.get(section.as_str()),
                    &mut declarations,
                );
            }
        }
    }

    collect_dependency_table(
        "workspace.dependencies",
        manifest
            .get("workspace")
            .and_then(|workspace| workspace.get("dependencies")),
        &mut declarations,
    );

    declarations.sort_by(|left, right| {
        (&left.section, &left.alias, &left.package).cmp(&(
            &right.section,
            &right.alias,
            &right.package,
        ))
    });
    declarations
}

fn direct_dependency_violations(
    manifest: &toml::Value,
    manifest_dir: &Path,
    repository_root: &Path,
) -> Vec<String> {
    let mut violations = Vec::new();
    let canonical_root = repository_root.canonicalize().unwrap_or_else(|error| {
        panic!(
            "failed to canonicalize repository root {}: {error}",
            repository_root.display()
        )
    });
    let canonical_crates = canonical_root
        .join("crates")
        .canonicalize()
        .unwrap_or_else(|error| {
            panic!(
                "failed to canonicalize workspace crates directory {}: {error}",
                canonical_root.join("crates").display()
            )
        });

    for dependency in dependency_declarations(manifest) {
        for (field, identity) in [
            ("alias", dependency.alias.as_str()),
            ("package", dependency.package.as_str()),
        ] {
            if let Some(family) = denied_identity(identity) {
                violations.push(format!(
                    "{}: dependency {} `{}` matches prohibited {family} family",
                    dependency.section, field, identity
                ));
            }
        }

        if let Some(git) = &dependency.git {
            if let Some(source) = denied_source(git) {
                violations.push(format!(
                    "{}: dependency `{}` references prohibited {source}: {git}",
                    dependency.section, dependency.alias
                ));
            }
        }

        let Some(path) = &dependency.path else {
            continue;
        };
        let requested = manifest_dir.join(path);
        match requested.canonicalize() {
            Ok(canonical_path) => {
                if !canonical_path.starts_with(&canonical_root) {
                    violations.push(format!(
                        "{}: dependency `{}` path `{path}` escapes repository root",
                        dependency.section, dependency.alias
                    ));
                } else if dependency.section == "workspace.dependencies"
                    && !canonical_path.starts_with(&canonical_crates)
                {
                    violations.push(format!(
                        "{}: dependency `{}` path `{path}` is not beneath crates/",
                        dependency.section, dependency.alias
                    ));
                }
            }
            Err(error) => violations.push(format!(
                "{}: dependency `{}` path `{path}` cannot be resolved: {error}",
                dependency.section, dependency.alias
            )),
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn lockfile_violations(lockfile: &toml::Value) -> Vec<String> {
    let mut violations = Vec::new();
    let Some(packages) = lockfile.get("package").and_then(toml::Value::as_array) else {
        return vec!["Cargo.lock does not contain a package array".to_owned()];
    };

    for package in packages {
        let Some(fields) = package.as_table() else {
            violations.push("Cargo.lock contains a non-table package entry".to_owned());
            continue;
        };
        let Some(name) = fields.get("name").and_then(toml::Value::as_str) else {
            violations.push("Cargo.lock package entry has no name".to_owned());
            continue;
        };
        if let Some(family) = denied_identity(name) {
            violations.push(format!(
                "Cargo.lock package `{name}` matches prohibited {family} family"
            ));
        }
        if let Some(source) = fields.get("source").and_then(toml::Value::as_str) {
            if let Some(denied) = denied_source(source) {
                violations.push(format!(
                    "Cargo.lock package `{name}` references prohibited {denied}: {source}"
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

#[test]
fn current_workspace_respects_chain_neutral_boundary() {
    let root = workspace_root();
    let mut manifests = vec![root.join("Cargo.toml")];
    manifests.extend(crate_manifest_paths(&root));

    let mut violations = Vec::new();
    for manifest_path in manifests {
        let manifest = read_manifest(&manifest_path);
        let manifest_dir = manifest_path
            .parent()
            .expect("Cargo.toml has a parent directory");
        violations.extend(
            direct_dependency_violations(&manifest, manifest_dir, &root)
                .into_iter()
                .map(|violation| format!("{}: {violation}", manifest_path.display())),
        );
    }

    let lockfile = read_manifest(&root.join("Cargo.lock"));
    violations.extend(lockfile_violations(&lockfile));

    assert!(
        violations.is_empty(),
        "chain-neutral dependency violations:\n{}",
        violations.join("\n")
    );
}

#[test]
fn neutral_dependency_is_accepted() {
    let root = workspace_root();
    let manifest: toml::Value = toml::from_str(
        r#"
            [dependencies]
            serde = "1"
            renamed = { package = "serde_json", version = "1" }
        "#,
    )
    .expect("valid synthetic manifest");
    assert!(direct_dependency_violations(&manifest, &root, &root).is_empty());
}

#[test]
fn all_dependency_section_kinds_are_enforced() {
    let root = workspace_root();
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let top_level: toml::Value = toml::from_str(&format!(
            "[{section}]\nforbidden = {{ package = \"midnight-ledger\", version = \"1\" }}"
        ))
        .expect("valid top-level synthetic manifest");
        assert_eq!(
            direct_dependency_violations(&top_level, &root, &root).len(),
            1,
            "top-level {section} was not enforced"
        );

        let target_scoped: toml::Value = toml::from_str(&format!(
            "[target.'cfg(target_os = \"linux\")'.{section}]\nforbidden = {{ package = \"oxid-runtime\", version = \"1\" }}"
        ))
        .expect("valid target-scoped synthetic manifest");
        assert_eq!(
            direct_dependency_violations(&target_scoped, &root, &root).len(),
            1,
            "target-scoped {section} was not enforced"
        );
    }
}

#[test]
fn renamed_package_and_normalized_identity_are_enforced() {
    let root = workspace_root();
    let manifest: toml::Value = toml::from_str(
        r#"
            [dependencies]
            neutral = { package = "Identus_Did_Prism_Ledger", version = "1" }
        "#,
    )
    .expect("valid synthetic manifest");
    let violations = direct_dependency_violations(&manifest, &root, &root);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].contains("package `Identus_Did_Prism_Ledger`"));
}

#[test]
fn donor_git_sources_are_enforced_even_behind_neutral_aliases() {
    let root = workspace_root();
    for (_, repository) in DENIED_SOURCE_IDENTITIES {
        let manifest: toml::Value = toml::from_str(&format!(
            "[dependencies]\nneutral = {{ git = \"https://github.com/{repository}.git\" }}"
        ))
        .expect("valid synthetic manifest");
        let violations = direct_dependency_violations(&manifest, &root, &root);
        assert_eq!(violations.len(), 1, "source {repository} was not enforced");
        assert!(violations[0].contains("dependency `neutral`"));
    }
}

#[test]
fn prohibited_repository_families_are_enforced_behind_neutral_git_packages() {
    let root = workspace_root();
    for repository in [
        "example/compact-runtime",
        "example/cardano-client",
        "example/pallas-network",
        "example/prism-ledger",
    ] {
        let manifest: toml::Value = toml::from_str(&format!(
            "[dependencies]\nneutral = {{ package = \"neutral\", git = \"ssh://git@github.com/{repository}.git\" }}"
        ))
        .expect("valid synthetic manifest");
        let violations = direct_dependency_violations(&manifest, &root, &root);
        assert_eq!(
            violations.len(),
            1,
            "source family {repository} was not enforced"
        );
        assert!(violations[0].contains("references prohibited"));
    }
}

#[test]
fn dependency_paths_cannot_escape_or_leave_workspace_crates() {
    let root = workspace_root();
    let escaping: toml::Value = toml::from_str(
        r#"
            [dependencies]
            neutral = { path = "../../" }
        "#,
    )
    .expect("valid synthetic manifest");
    let escaping_violations = direct_dependency_violations(&escaping, &root, &root);
    assert_eq!(escaping_violations.len(), 1);
    assert!(escaping_violations[0].contains("escapes repository root"));

    let root_outside_crates: toml::Value = toml::from_str(
        r#"
            [workspace.dependencies]
            neutral = { path = "." }
        "#,
    )
    .expect("valid synthetic manifest");
    let root_violations = direct_dependency_violations(&root_outside_crates, &root, &root);
    assert_eq!(root_violations.len(), 1);
    assert!(root_violations[0].contains("is not beneath crates/"));

    let valid_internal: toml::Value = toml::from_str(
        r#"
            [workspace.dependencies]
            identus-core = { path = "crates/core" }
        "#,
    )
    .expect("valid synthetic manifest");
    assert!(direct_dependency_violations(&valid_internal, &root, &root).is_empty());
}

#[test]
fn resolved_lockfile_identity_and_source_are_enforced() {
    let prohibited_identity: toml::Value = toml::from_str(
        r#"
            version = 4
            [[package]]
            name = "pallas-ledger"
            version = "1.0.0"
        "#,
    )
    .expect("valid synthetic lockfile");
    let identity_violations = lockfile_violations(&prohibited_identity);
    assert_eq!(identity_violations.len(), 1);
    assert!(identity_violations[0].contains("prohibited Cardano family"));

    let prohibited_source: toml::Value = toml::from_str(
        r#"
            version = 4
            [[package]]
            name = "neutral"
            version = "1.0.0"
            source = "git+ssh://git@github.com/MediaNoxLabs/midnight-identity.git#abc"
        "#,
    )
    .expect("valid synthetic lockfile");
    let source_violations = lockfile_violations(&prohibited_source);
    assert_eq!(source_violations.len(), 1);
    assert!(source_violations[0].contains("midnight-identity repository"));
}

//! The dep-graph guard: layer-membership and dependency-direction assertions.
//!
//! Moved verbatim from the prior single-file `lib.rs` `mod guard` block,
//! retargeted to use the shared helpers in `guard/mod.rs` and the rulebook
//! items re-exported from `rulebook.rs`.

use super::*;

#[test]
fn every_crate_appears_in_exactly_one_layer() {
    let mut seen: HashSet<&str> = HashSet::new();
    for rule in LAYER_RULES {
        for member in rule.members {
            assert!(
                seen.insert(member.name),
                "crate {} listed in more than one layer",
                member.name
            );
        }
    }
    let total: usize = LAYER_RULES.iter().map(|r| r.members.len()).sum();
    assert_eq!(seen.len(), total, "duplicate crate names across layers");
}

#[test]
fn layer_rulebook_defines_all_seven_layers() {
    let layers: HashSet<Layer> = LAYER_RULES.iter().map(|r| r.layer).collect();
    assert_eq!(layers.len(), 7, "expected exactly 7 layers");
    for expected in [
        Layer::Foundation,
        Layer::DomainPrimitives,
        Layer::CredentialSemantics,
        Layer::ProtocolSemantics,
        Layer::Orchestration,
        Layer::OuterBoundary,
        Layer::Verification,
    ] {
        assert!(
            layers.contains(&expected),
            "missing layer {}",
            expected.as_str()
        );
    }
}

/// Asserts the `proc_macro` flag is carried per member and that
/// `identus-derive` is a foundation member flagged `proc_macro = true`,
/// with every other member `false` (the crate-ring-layout carve-out).
#[test]
fn layer_rulebook_carries_proc_macro_flags() {
    // `identus-derive` is a foundation member flagged proc_macro = true.
    let derive = LAYER_RULES
        .iter()
        .find(|r| r.layer == Layer::Foundation)
        .and_then(|r| r.members.iter().find(|m| m.name == "identus-derive"))
        .expect("identus-derive is a foundation member");
    assert!(
        derive.proc_macro,
        "identus-derive must be proc_macro = true"
    );
    // Every other member is proc_macro = false.
    for rule in LAYER_RULES {
        for member in rule.members {
            if member.name == "identus-derive" {
                continue;
            }
            assert!(
                !member.proc_macro,
                "{} must be proc_macro = false",
                member.name
            );
        }
    }
}

#[test]
fn manifests_conform_to_layer_rules() {
    let workspace_root = workspace_root();
    let root_manifest_path = workspace_root.join("Cargo.toml");
    let root_manifest = read_manifest(&root_manifest_path);
    let workspace = workspace_crate_names(&root_manifest);
    let inherited = workspace_dependency_packages(&root_manifest);

    let manifest_paths = crate_manifest_paths(&workspace_root);
    assert_eq!(
        manifest_paths.len(),
        LAYER_RULES.iter().map(|r| r.members.len()).sum::<usize>(),
        "expected crate manifest count to equal the LAYER_RULES-derived workspace crate count"
    );

    for path in &manifest_paths {
        let manifest = read_manifest(path);
        let source = package_name(&manifest);
        assert!(
            workspace.contains(&source),
            "{source} is not declared in root [workspace.dependencies]"
        );
        let deps = inward_workspace_deps(&manifest, &workspace, &inherited);

        for target in &deps {
            check_dep_edge(&source, target).unwrap_or_else(|e| panic!("layer violation: {e}"));
        }

        match source.as_str() {
            "identus-conformance" => assert_eq!(
                deps,
                ["identus-core"],
                "repository conformance must retain its core-only runtime edge"
            ),
            "identus-did" => assert_eq!(
                deps,
                ["identus-core", "identus-derive"],
                "DID must retain its exact crypto-free internal dependency cone"
            ),
            "identus-oid4vci" => assert_eq!(
                deps,
                ["identus-core", "identus-jose"],
                "OID4VCI must retain its exact core-and-JOSE internal dependency cone"
            ),
            "identus-wallet-conformance" => assert_eq!(
                deps,
                ["identus-wallet"],
                "wallet conformance must retain its wallet-only runtime edge"
            ),
            _ => {}
        }

        // Foundation is runtime-dependency-free: `identus-core` MAY only
        // depend on workspace crates flagged `proc_macro = true` (e.g.
        // `identus-derive`), never on a runtime `identus-*` crate.
        if source == "identus-core" {
            let runtime_deps: Vec<&String> = deps.iter().filter(|d| !is_proc_macro(d)).collect();
            assert!(
                runtime_deps.is_empty(),
                "identus-core must have no identus-* runtime dependencies \
                (proc-macro build-time deps are exempt), found {:?}",
                runtime_deps
            );
        }
    }
}

/// Asserts the `proc_macro` carve-out scenarios the layer guard must honor:
/// a proc-macro target is permitted from any layer (incl. foundation), a
/// non-proc-macro outward edge from foundation is still rejected, an
/// outward dependency to a non-proc-macro crate is rejected, and a
/// production crate depending on conformance is rejected. The conforming
/// inward edge `identus-did -> identus-core` passes.
#[test]
fn proc_macro_carve_out_scenarios() {
    // Proc-macro target is permitted from foundation (any layer).
    assert!(check_dep_edge("identus-core", "identus-derive").is_ok());
    // A non-proc-macro outward dependency from foundation is still rejected.
    assert!(check_dep_edge("identus-core", "identus-did").is_err());
    // A domain crate depending outward on a non-proc-macro crate is rejected.
    assert!(check_dep_edge("identus-did", "identus-adapters-entropy").is_err());
    // A production crate depending on conformance is rejected.
    assert!(check_dep_edge("identus-crypto", "identus-conformance").is_err());
    // A conforming inward edge passes.
    assert!(check_dep_edge("identus-did", "identus-core").is_ok());
}

#[test]
fn wallet_conformance_is_an_inward_verification_leaf() {
    assert!(check_dep_edge("identus-wallet-conformance", "identus-wallet").is_ok());
    assert!(check_dep_edge("identus-wallet", "identus-wallet-conformance").is_err());
}

#[test]
fn jose_is_reusable_below_protocol_semantics() {
    assert!(check_dep_edge("identus-jose", "identus-core").is_ok());
    assert!(check_dep_edge("identus-jose", "identus-crypto").is_ok());
    assert!(check_dep_edge("identus-jose", "identus-did").is_ok());
    assert!(check_dep_edge("identus-openid4vc", "identus-jose").is_ok());
    assert!(check_dep_edge("identus-crypto", "identus-jose").is_err());
}

#[test]
fn oid4vci_protocol_retains_an_exact_inward_internal_cone() {
    assert!(check_dep_edge("identus-oid4vci", "identus-core").is_ok());
    assert!(check_dep_edge("identus-oid4vci", "identus-jose").is_ok());
    assert!(check_dep_edge("identus-core", "identus-oid4vci").is_err());
}

#[test]
fn did_http_adapter_is_an_outer_boundary_over_did_core() {
    assert!(check_dep_edge("identus-did-resolver-http", "identus-core").is_ok());
    assert!(check_dep_edge("identus-did-resolver-http", "identus-did").is_ok());
    assert!(check_dep_edge("identus-did", "identus-did-resolver-http").is_err());
}

#[test]
fn uniffi_did_adapter_is_an_outer_boundary_over_did_core() {
    assert!(check_dep_edge("identus-uniffi-did", "identus-did").is_ok());
    assert!(check_dep_edge("identus-did", "identus-uniffi-did").is_err());
}

#[test]
fn wasm_did_adapter_is_an_outer_boundary_over_did_core() {
    assert!(check_dep_edge("identus-wasm-did", "identus-did").is_ok());
    assert!(check_dep_edge("identus-did", "identus-wasm-did").is_err());
}

#[test]
fn target_specific_runtime_dependencies_are_inward_edges() {
    let manifest: toml::Value = toml::from_str(
        r#"
            [dependencies]
            identus-wallet.workspace = true
            core-alias = { package = "identus-core", path = "../core" }
            identus-did = { workspace = true, package = "identus-derive" }
            inherited-core = { workspace = true }

            [target.'cfg(unix)'.dependencies]
            wallet-again = { package = "identus-wallet", path = "../wallet" }

            [target.'cfg(windows)'.dependencies]
            unauthorized-leaf-edge = { package = "identus-did", path = "../did" }

            [target.'cfg(unix)'.dev-dependencies]
            dev-only-alias = { package = "identus-crypto", path = "../crypto" }
        "#,
    )
    .expect("test manifest is valid TOML");
    let workspace = HashSet::from([
        "identus-core".to_owned(),
        "identus-crypto".to_owned(),
        "identus-did".to_owned(),
        "identus-derive".to_owned(),
        "identus-wallet".to_owned(),
    ]);
    let inherited = HashMap::from([
        ("identus-wallet".to_owned(), "identus-wallet".to_owned()),
        ("identus-did".to_owned(), "identus-did".to_owned()),
        ("inherited-core".to_owned(), "identus-core".to_owned()),
    ]);

    let deps = inward_workspace_deps(&manifest, &workspace, &inherited);
    assert_eq!(
        deps,
        ["identus-core", "identus-did", "identus-wallet"],
        "renamed runtime dependencies must resolve to canonical packages once, inherited dependencies must use root identity, and dev-dependencies must stay excluded"
    );
    assert!(
        !deps.contains(&"identus-derive".to_owned()),
        "a member-local package override must not alter an inherited dependency"
    );
    assert_ne!(
        deps,
        ["identus-wallet"],
        "the wallet-only verification-leaf assertion must observe an unauthorized renamed edge"
    );
    assert!(
        check_dep_edge("identus-wallet-conformance", "identus-core").is_ok(),
        "the exact leaf assertion must reject extra dependencies even when the broad layer rule permits them"
    );
}

#[test]
fn root_alias_has_distinct_graph_and_source_identities() {
    let root: toml::Value = toml::from_str(
        r#"
            [workspace.dependencies]
            core-alias = { package = "identus-core", path = "crates/core" }
        "#,
    )
    .expect("test root manifest is valid TOML");
    let member: toml::Value = toml::from_str(
        r#"
            [dependencies]
            core-alias.workspace = true
        "#,
    )
    .expect("test member manifest is valid TOML");

    let inherited = workspace_dependency_packages(&root);
    let canonical = HashSet::from(["identus-core".to_owned()]);
    assert_eq!(
        inward_workspace_deps(&member, &canonical, &inherited),
        ["identus-core"],
        "the architecture graph must use the root declaration's canonical package"
    );

    let internal_keys = workspace_dependency_keys(&root);
    assert_eq!(internal_keys, HashSet::from(["core-alias".to_owned()]));
    assert!(
        external_dep_entries("consumer", &member, &internal_keys).is_empty(),
        "the external dependency guard must retain the inherited root alias as internal"
    );
}

#[test]
fn no_inline_external_dependency_versions() {
    let workspace_root = workspace_root();
    let root_manifest_path = workspace_root.join("Cargo.toml");
    let root_manifest = read_manifest(&root_manifest_path);
    let workspace = workspace_dependency_keys(&root_manifest);

    // The external dep names declared at the workspace level (non-path
    // entries in [workspace.dependencies]).
    let workspace_external: HashSet<String> = root_manifest
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.as_table())
        .map(|t| {
            t.iter()
                .filter(|(_, v)| {
                    !v.as_table()
                        .map(|tbl| tbl.contains_key("path"))
                        .unwrap_or(false)
                })
                .map(|(k, _)| k.clone())
                .collect()
        })
        .unwrap_or_default();

    let manifest_paths = crate_manifest_paths(&workspace_root);
    for path in &manifest_paths {
        let manifest = read_manifest(path);
        let source = package_name(&manifest);
        for entry in external_dep_entries(&source, &manifest, &workspace) {
            assert!(
                !entry.has_version,
                "{} declares external dep `{}` in [{}] with an inline `version`; use `<dep>.workspace = true` resolving to a root [workspace.dependencies] entry",
                entry.crate_name, entry.name, entry.section
            );
            assert!(
                entry.has_workspace,
                "{} declares external dep `{}` in [{}] without `workspace = true`; external deps must resolve to a root [workspace.dependencies] entry",
                entry.crate_name, entry.name, entry.section
            );
            // Defense-in-depth: `cargo` already refuses to load a manifest
            // that inherits a missing `workspace.dependencies` entry, so this
            // branch is unreachable via `cargo test`. Kept so the guard
            // stays self-sufficient if ever run standalone against raw TOML.
            assert!(
                workspace_external.contains(&entry.name),
                "{} references external dep `{}` via `workspace = true` but no `{}` entry exists in root [workspace.dependencies]",
                entry.crate_name,
                entry.name,
                entry.name
            );
            // `optional` is orthogonal to the version-source rule and is
            // allowed on top of `workspace = true`.
            let _ = entry.optional;
        }
    }
}

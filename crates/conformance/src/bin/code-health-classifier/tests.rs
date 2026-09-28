use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    process::Command,
};

use super::{
    cfg::{Truth, cfg_truth_for_test},
    engine::{PROTOCOL_VERSION, Request, SourceInput, classify, parse_source},
    graph::traversal_work,
    projection::projection_work,
};

fn one(source: &str) -> Vec<usize> {
    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
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
fn classifies_closure_parameters_by_ast_boundary() {
    let source = r#"
fn closure() { let _ = |
    #[cfg(test)]
    hidden: u8,
    shipping: u8,
| shipping; }
"#;
    let lines = one(source);
    for line in [3, 4] {
        assert!(
            lines.contains(&line),
            "expected test-only closure parameter line {line}: {lines:?}"
        );
    }
    for line in [2, 5, 6] {
        assert!(!lines.contains(&line), "shipping line {line}: {lines:?}");
    }
}

#[test]
fn classifies_bare_function_parameters_by_ast_boundary() {
    let source = r#"
type Callback = fn(
    #[cfg(test)]
    u8,
    u16,
);
"#;
    let lines = one(source);
    for line in [3, 4] {
        assert!(
            lines.contains(&line),
            "expected test-only function-pointer parameter line {line}: {lines:?}"
        );
    }
    for line in [2, 5, 6] {
        assert!(!lines.contains(&line), "shipping line {line}: {lines:?}");
    }
}

#[test]
fn classifies_variadics_and_struct_pattern_fields_by_ast_boundary() {
    let source = r#"
struct Demo { hidden: u8, shipping: u8 }
fn pattern(value: Demo) {
    let Demo {
        #[cfg(test)] hidden,
        shipping,
    } = value;
}
unsafe extern "C" {
    fn foreign(
        shipping: u8,
        #[cfg(test)]
        ...
    );
}
type Callback = unsafe extern "C" fn(
    u8,
    #[cfg(test)]
    ...
);
"#;
    let lines = one(source);
    for line in [5, 12, 13, 18, 19] {
        assert!(
            lines.contains(&line),
            "expected test-only attributed node line {line}: {lines:?}"
        );
    }
    for line in [2, 3, 6, 7, 9, 11, 16, 17, 20] {
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
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: "#[cfg_attr(feature = \"x\", cfg(path))] mod helper;\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/helper.rs".to_owned(),
                source: "fn shipping() {}\n".to_owned(),
            },
        ],
    })
    .expect("unrelated cfg(path) is not a module path override");
    assert!(response.inherited_inline_paths.is_empty());

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![SourceInput {
            path: "crates/demo/src/lib.rs".to_owned(),
            source: concat!(
                "#[cfg(any())] mod absent;\n",
                "#[cfg(all(test, any()))] mod also_absent;\n",
            )
            .to_owned(),
        }],
    })
    .expect("modules disabled in both configurations are not resolved");
    assert!(response.inherited_inline_paths.is_empty());

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
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
        target_roots: Vec::new(),
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
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
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
            target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
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
        target_roots: vec!["crates/demo/src/bin/tool.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/bin/tool.rs".to_owned(),
                source: "#[cfg(test)] mod helper;\nfn main() {}\n".to_owned(),
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
        target_roots: vec!["crates/demo/src/bin/tool.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/bin/tool.rs".to_owned(),
                source: "mod helper;\nfn main() {}\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/bin/helper.rs".to_owned(),
                source: "mod child;\nfn helper() {}\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/bin/helper/child.rs".to_owned(),
                source: "fn nested_bin_module() {}\n".to_owned(),
            },
        ],
    })
    .expect("binary child reached as nested module");
    assert!(response.inherited_inline_paths.is_empty());

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec![
            "crates/demo/src/bin/helper.rs".to_owned(),
            "crates/demo/src/lib.rs".to_owned(),
        ],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: "#[cfg(test)] #[path = \"bin/helper.rs\"] mod helper;\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/bin/helper.rs".to_owned(),
                source: "fn main() {}\n".to_owned(),
            },
        ],
    })
    .expect("Cargo target roots remain production");
    assert!(response.inherited_inline_paths.is_empty());

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec![
            "crates/demo/src/bin/helper.rs".to_owned(),
            "crates/demo/src/bin/tool.rs".to_owned(),
        ],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/bin/tool.rs".to_owned(),
                source: "mod helper;\nfn main() {}\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/bin/helper.rs".to_owned(),
                source: "#[cfg(test)] mod fixture;\nfn main() {}\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/bin/fixture.rs".to_owned(),
                source: "fn root_fixture() {}\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/bin/helper/fixture.rs".to_owned(),
                source: "fn nested_fixture() {}\n".to_owned(),
            },
        ],
    })
    .expect("target reached in root and nested roles");
    assert_eq!(
        response.inherited_inline_paths,
        vec![
            "crates/demo/src/bin/fixture.rs",
            "crates/demo/src/bin/helper/fixture.rs",
        ]
    );

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: concat!(
                    "struct Demo {\n",
                    "    #[cfg(test)]\n",
                    "    field: [(); { #[path = \"helper.rs\"] mod helper; 0 }],\n",
                    "    shipping: u8,\n",
                    "}\n",
                )
                .to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/helper.rs".to_owned(),
                source: "fn field_fixture() {}\n".to_owned(),
            },
        ],
    })
    .expect("declaration field reachability");
    assert_eq!(
        response.inherited_inline_paths,
        vec!["crates/demo/src/helper.rs"]
    );

    let response = classify(Request {
            protocol_version: PROTOCOL_VERSION,
            target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
            sources: vec![
                SourceInput {
                    path: "crates/demo/src/lib.rs".to_owned(),
                    source: concat!(
                        "enum Demo {\n",
                        "    #[cfg(test)] Hidden = { #[path = \"variant.rs\"] mod helper; 1 },\n",
                        "    Shipping = 2,\n",
                        "}\n",
                        "fn ordinary(#[cfg(test)] value: [(); { #[path = \"argument.rs\"] mod helper; 0 }]) {}\n",
                        "struct Generic<#[cfg(test)] const N: usize = { #[path = \"generic.rs\"] mod helper; 0 }>;\n",
                    )
                    .to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/variant.rs".to_owned(),
                    source: "fn variant_fixture() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/argument.rs".to_owned(),
                    source: "fn argument_fixture() {}\n".to_owned(),
                },
                SourceInput {
                    path: "crates/demo/src/generic.rs".to_owned(),
                    source: "fn generic_fixture() {}\n".to_owned(),
                },
            ],
        })
        .expect("variant argument and generic reachability");
    assert_eq!(
        response.inherited_inline_paths,
        vec![
            "crates/demo/src/argument.rs",
            "crates/demo/src/generic.rs",
            "crates/demo/src/variant.rs",
        ]
    );

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: Vec::new(),
        sources: vec![
            SourceInput {
                path: "crates/demo/src/foo.rs".to_owned(),
                source: "#[cfg(test)] mod inner { #[path = \"bar.rs\"] mod helper; }\n".to_owned(),
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
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: "#[cfg(test)] #[path = \"custom\"] mod inline { mod child; }\n".to_owned(),
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

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: Vec::new(),
        sources: vec![
            SourceInput {
                path: "crates/demo/src/foo.rs".to_owned(),
                source: "#[cfg(test)] #[path = \"custom\"] mod inline { mod child; }\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/custom/child.rs".to_owned(),
                source: "fn nested_inline_path_fixture() {}\n".to_owned(),
            },
        ],
    })
    .expect("nested-source path-adjusted inline module reachability");
    assert_eq!(
        response.inherited_inline_paths,
        vec!["crates/demo/src/custom/child.rs"]
    );

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: "mod foo;\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/foo.rs".to_owned(),
                source: "fn main() {}\n#[cfg(test)] mod fixture;\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/foo/fixture.rs".to_owned(),
                source: "fn fixture() {}\n".to_owned(),
            },
        ],
    })
    .expect("Cargo roots do not infer ordinary functions named main");
    assert_eq!(
        response.inherited_inline_paths,
        vec!["crates/demo/src/foo/fixture.rs"]
    );

    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: concat!(
                    "#[cfg(test)]\n",
                    "#[cfg_attr(test, path = \"fixtures/helper.rs\")]\n",
                    "mod helper;\n",
                )
                .to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/fixtures/helper.rs".to_owned(),
                source: "fn cfg_attr_fixture() {}\n".to_owned(),
            },
        ],
    })
    .expect("test-only cfg_attr path override");
    assert_eq!(
        response.inherited_inline_paths,
        vec!["crates/demo/src/fixtures/helper.rs"]
    );

    let error = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
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
        target_roots: Vec::new(),
        sources: vec![SourceInput {
            path: "crates/demo/src/lib.rs".to_owned(),
            source: "#[path = \"../escape.rs\"] mod escape;".to_owned(),
        }],
    })
    .expect_err("escaping path");
    assert!(error.contains("contained relative path"));

    let error = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: Vec::new(),
        sources: vec![SourceInput {
            path: "crates/demo/src/lib.rs".to_owned(),
            source: "fn broken( {".to_owned(),
        }],
    })
    .expect_err("malformed Rust");
    assert!(error.contains("Rust parse failed"));

    let error = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/missing.rs".to_owned()],
        sources: Vec::new(),
    })
    .expect_err("missing target root");
    assert!(error.contains("target root is not an input Rust source"));

    let error = classify(Request {
        protocol_version: PROTOCOL_VERSION + 1,
        target_roots: Vec::new(),
        sources: Vec::new(),
    })
    .expect_err("protocol mismatch");
    assert!(error.contains("unsupported protocol version"));
}

#[test]
fn differential_cfg_paths_match_rustc_reachability() {
    let source = concat!(
        "#![allow(dead_code)]\n",
        "#[cfg(not(test))]\n",
        "#[cfg_attr(not(test), path = \"production.rs\")]\n",
        "#[cfg_attr(test, path = \"unreachable.rs\")]\n",
        "mod selected;\n",
        "#[cfg(test)]\n",
        "#[cfg_attr(all(test), cfg_attr(test, path = \"test.rs\"))]\n",
        "mod test_only;\n",
    );
    let response = classify(Request {
        protocol_version: PROTOCOL_VERSION,
        target_roots: vec!["crates/demo/src/lib.rs".to_owned()],
        sources: vec![
            SourceInput {
                path: "crates/demo/src/lib.rs".to_owned(),
                source: source.to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/production.rs".to_owned(),
                source: "pub fn selected() {}\n".to_owned(),
            },
            SourceInput {
                path: "crates/demo/src/test.rs".to_owned(),
                source: "pub fn selected() {}\n".to_owned(),
            },
        ],
    })
    .expect("configuration-specific paths classify");
    assert_eq!(
        response.inherited_inline_paths,
        vec!["crates/demo/src/test.rs"]
    );

    let fixture = std::env::temp_dir().join(format!(
        "identus-code-health-differential-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&fixture);
    fs::create_dir_all(&fixture).expect("create differential fixture");
    fs::write(fixture.join("lib.rs"), source).expect("write root");
    fs::write(fixture.join("production.rs"), "pub fn selected() {}\n")
        .expect("write production module");
    fs::write(fixture.join("test.rs"), "pub fn selected() {}\n").expect("write test module");
    for arguments in [Vec::<&str>::new(), vec!["--cfg", "test"]] {
        let output = fixture.join(if arguments.is_empty() {
            "production.rlib"
        } else {
            "test.rlib"
        });
        let status = Command::new("rustc")
            .arg("--crate-type=lib")
            .arg("--edition=2024")
            .args(arguments)
            .arg(fixture.join("lib.rs"))
            .arg("-o")
            .arg(output)
            .status()
            .expect("run rustc differential oracle");
        assert!(status.success(), "rustc rejected a supported module graph");
    }
    fs::remove_dir_all(&fixture).expect("remove differential fixture");
}

#[test]
fn deterministic_generated_projection_work_is_linear() {
    let mut source = String::new();
    let mut spans = Vec::new();
    for index in 0..10_000 {
        let start = source.len();
        source.push_str("x\n");
        if index % 2 == 0 {
            spans.push(start..start + 1);
        }
    }
    let work = projection_work(&source, spans.clone());
    assert_eq!(work.authored_positions, 10_000);
    assert!(work.span_advances <= spans.len());
}

#[test]
fn deterministic_generated_graph_work_is_edge_bounded() {
    let root = PathBuf::from("crates/demo/src/lib.rs");
    let mut root_source = String::new();
    let mut sources = BTreeMap::new();
    for index in 0..1_024 {
        root_source.push_str(&format!(
            "#[cfg(test)] #[path = \"fixtures/f{index}.rs\"] mod f{index};\n"
        ));
        sources.insert(
            PathBuf::from(format!("crates/demo/src/fixtures/f{index}.rs")),
            "pub fn fixture() {}\n".to_owned(),
        );
    }
    sources.insert(root.clone(), root_source.clone());
    let parsed = sources
        .iter()
        .map(|(path, source)| {
            Ok((
                path.clone(),
                parse_source(&path.display().to_string(), source)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()
        .expect("parse generated graph");
    let work = traversal_work(&parsed, &sources, &BTreeSet::from([root]))
        .expect("traverse generated graph");
    assert_eq!(work.edge_resolutions, 1_024);
}

#[test]
fn deterministic_seeded_cfg_properties_match_boolean_algebra() {
    let cases = [
        ("test", Truth::False, Truth::True),
        ("not(test)", Truth::True, Truth::False),
        ("all(test, true)", Truth::False, Truth::True),
        ("any(test, false)", Truth::False, Truth::True),
        ("all(not(test), true)", Truth::True, Truth::False),
        ("any(not(test), false)", Truth::True, Truth::False),
        ("all(test, feature = \"x\")", Truth::False, Truth::Unknown),
        ("any(test, feature = \"x\")", Truth::Unknown, Truth::True),
    ];
    let mut state = 0x5eed_c0de_u64;
    for _ in 0..1_024 {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let (source, production, test) = cases[(state as usize) % cases.len()];
        let meta = syn::parse_str(source).expect("parse generated cfg predicate");
        assert_eq!(
            cfg_truth_for_test(&meta, false),
            production,
            "production predicate {source}"
        );
        assert_eq!(
            cfg_truth_for_test(&meta, true),
            test,
            "test predicate {source}"
        );
    }
}

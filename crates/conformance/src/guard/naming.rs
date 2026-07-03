//! The naming guard: port/adapter type-naming convention enforcement.
//!
//! Derives the port set from source by reading `#[identus::port]` off
//! `syn::ItemTrait.attrs` (no hand-maintained registry), then scans the
//! `identus-adapters-*` crates for `impl <Port> for <Struct>` and asserts the
//! implementing struct's name ends in `Adapter`. Fails closed: a `syn` parse
//! error on a scanned file fails the test rather than silently passing. The
//! port-side no-`Port`-suffix rule is enforced at compile time by the
//! `#[identus::port]` attribute macro; this guard redundantly re-asserts it
//! for discovered ports. Reuses the shared file-walking and `syn`-parsing
//! helpers from `guard/mod.rs`.

use super::*;
use std::collections::HashSet;

/// Whether an attribute is `#[identus::port]` (path `identus::port`).
fn is_port_attr(attr: &syn::Attribute) -> bool {
    attr.path().segments.len() == 2
        && attr.path().segments[0].ident == "identus"
        && attr.path().segments[1].ident == "port"
}

/// Port trait names declared via `#[identus::port]` in a parsed file.
fn ports_in_file(file: &syn::File) -> Vec<String> {
    let mut out = Vec::new();
    for item in &file.items {
        if let syn::Item::Trait(t) = item {
            if t.attrs.iter().any(is_port_attr) {
                out.push(t.ident.to_string());
            }
        }
    }
    out
}

/// Adapter impls in a parsed file: `(port_name, struct_name)` for each
/// `impl <Port> for <Struct>` whose trait is a discovered port and whose self
/// type is a path (a struct).
fn adapter_impls_in_file(file: &syn::File, ports: &HashSet<String>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for item in &file.items {
        let syn::Item::Impl(imp) = item else {
            continue;
        };
        let Some((_, trait_path, _)) = &imp.trait_ else {
            continue;
        };
        let Some(trait_seg) = trait_path.segments.last() else {
            continue;
        };
        let trait_name = trait_seg.ident.to_string();
        if !ports.contains(&trait_name) {
            continue;
        }
        let syn::Type::Path(self_ty) = &*imp.self_ty else {
            continue;
        };
        let Some(self_seg) = self_ty.path.segments.last() else {
            continue;
        };
        out.push((trait_name, self_seg.ident.to_string()));
    }
    out
}

/// Assert an adapter struct name ends in `Adapter`. `Ok` if it does, `Err`
/// with a diagnostic message otherwise.
fn check_adapter_suffix(struct_name: &str) -> Result<(), String> {
    if struct_name.ends_with("Adapter") {
        Ok(())
    } else {
        Err(format!(
            "adapter `{struct_name}` must end in `Adapter` (form `<Backend><Capability>Adapter`)"
        ))
    }
}

/// Whether `path` lives under an `identus-adapters-*` crate (`crates/adapters-*`).
fn is_adapter_crate(path: &Path, crates_root: &Path) -> bool {
    path.strip_prefix(crates_root).is_ok_and(|rel| {
        rel.iter()
            .next()
            .is_some_and(|c| c.to_string_lossy().starts_with("adapters-"))
    })
}

/// The full naming guard against the workspace source: discovers the port set
/// from `#[identus::port]` attributes, redundantly re-asserts no port name
/// ends in `Port`, then asserts every adapter struct in `identus-adapters-*`
/// crates ends in `Adapter`. Fails closed on any `syn` parse error (via
/// `parse_rs`, which panics to fail the test).
#[test]
fn naming_guard_passes() {
    let crates_root = workspace_root().join("crates");

    // PASS 1 — derive the port set from `#[identus::port]` on `ItemTrait.attrs`.
    let mut ports: HashSet<String> = HashSet::new();
    for path in walk_src_files() {
        let file = parse_rs(&path);
        for name in ports_in_file(&file) {
            // Redundant re-assert: the compile-time attribute already forbids a
            // `Port` suffix for annotated ports; guard against an attribute-macro
            // regression.
            assert!(
                !name.ends_with("Port"),
                "discovered port `{name}` must not end in `Port`"
            );
            assert!(
                ports.insert(name.clone()),
                "duplicate port declaration `{name}`"
            );
        }
    }
    // SecureRandom is the workspace's port, declared via the attribute.
    assert!(
        ports.contains("SecureRandom"),
        "SecureRandom must be discovered via #[identus::port]"
    );

    // PASS 2 — adapter suffix in `crates/adapters-*/src/**/*.rs`.
    let mut checked = 0;
    for path in walk_src_files() {
        if !is_adapter_crate(&path, &crates_root) {
            continue;
        }
        let file = parse_rs(&path);
        for (port, struct_name) in adapter_impls_in_file(&file, &ports) {
            check_adapter_suffix(&struct_name)
                .unwrap_or_else(|e| panic!("{e} (adapter of `{port}` in {})", path.display()));
            checked += 1;
        }
    }
    // The two entropy adapters (getrandom + deterministic) are textually present
    // behind their feature gates; `syn` sees them regardless of cfg, so both
    // are checked.
    assert!(
        checked >= 2,
        "expected both entropy adapters to be checked, found {checked}"
    );
}

/// Positive: a port trait carrying `#[identus::port]` is discovered, and its
/// name lacks the `Port` suffix.
#[test]
fn port_is_discovered_via_attribute() {
    let src =
        "use identus_derive as identus;\n#[identus::port]\npub trait SecureRandom { fn f(); }";
    let file = syn::parse_file(src).expect("parse");
    let ports = ports_in_file(&file);
    assert!(ports.contains(&"SecureRandom".to_string()));
    assert!(!"SecureRandom".ends_with("Port"));
}

/// Positive: an adapter struct ending in `Adapter` is accepted.
#[test]
fn adapter_with_suffix_is_accepted() {
    let src = "impl identus_crypto::SecureRandom for GetrandomSystemRandomAdapter {}";
    let file = syn::parse_file(src).expect("parse");
    let mut ports = HashSet::new();
    ports.insert("SecureRandom".to_string());
    let impls = adapter_impls_in_file(&file, &ports);
    assert_eq!(impls.len(), 1);
    let (port, struct_name) = &impls[0];
    assert_eq!(port, "SecureRandom");
    assert_eq!(struct_name, "GetrandomSystemRandomAdapter");
    assert!(check_adapter_suffix(struct_name).is_ok());
}

/// Negative: an adapter struct lacking the `Adapter` suffix is rejected.
#[test]
fn adapter_without_suffix_is_rejected() {
    let src = "impl identus_crypto::SecureRandom for BadName {}";
    let file = syn::parse_file(src).expect("parse");
    let mut ports = HashSet::new();
    ports.insert("SecureRandom".to_string());
    let impls = adapter_impls_in_file(&file, &ports);
    assert_eq!(impls.len(), 1);
    let (port, struct_name) = &impls[0];
    assert_eq!(port, "SecureRandom");
    assert_eq!(struct_name, "BadName");
    assert!(check_adapter_suffix(struct_name).is_err());
}

/// Negative: a `syn::parse_file` error on a scanned file fails the guard
/// closed (the guard's `parse_rs` panics, failing the test rather than
/// silently skipping the file).
#[test]
fn parse_error_fails_closed() {
    // `syn::parse_file` surfaces the malformed source as an error.
    let result = syn::parse_file("impl not valid rust {{{");
    assert!(result.is_err());
    // The guard's shared `parse_rs` propagates such errors by panicking.
    let tmp = std::env::temp_dir().join("identus_naming_guard_neg.rs");
    fs::write(&tmp, "impl not valid rust {{{").unwrap();
    let panicked = std::panic::catch_unwind(|| parse_rs(&tmp));
    let _ = fs::remove_file(&tmp);
    assert!(
        panicked.is_err(),
        "parse_rs must fail closed (panic) on a parse error"
    );
}

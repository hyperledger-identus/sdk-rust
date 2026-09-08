//! Uniform first-party unsafe-code policy enforcement.
//!
//! Configuration checks prove both Cargo inheritance edges. Behavioral probes
//! independently prove the pinned compiler receives the forbid for each source
//! target class; their unsafe snippets remain inert string data in this crate.

use super::*;
use std::ffi::OsString;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

const MAX_FAILURE_OUTPUT_BYTES: usize = 8 * 1024;
static NEXT_SCRATCH_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct ScratchWorkspace {
    root: PathBuf,
}

impl ScratchWorkspace {
    fn new(case: &str) -> Self {
        let id = NEXT_SCRATCH_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "identus-unsafe-policy-{}-{id}-{case}",
            std::process::id()
        ));
        fs::create_dir(&root)
            .unwrap_or_else(|error| panic!("failed to create {}: {error}", root.display()));
        Self { root }
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|error| panic!("failed to create {}: {error}", parent.display()));
        }
        fs::write(&path, contents)
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
    }

    fn cargo(&self, arguments: &[&str]) -> Output {
        let cargo = std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
        Command::new(cargo)
            .args(arguments)
            .current_dir(&self.root)
            .env("CARGO_TARGET_DIR", self.root.join("target"))
            .output()
            .unwrap_or_else(|error| {
                panic!("failed to run Cargo for {}: {error}", self.root.display())
            })
    }
}

impl Drop for ScratchWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn root_forbid_violation(root_manifest: &toml::Value) -> Option<String> {
    let value = root_manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("lints"))
        .and_then(|lints| lints.get("rust"))
        .and_then(|rust| rust.get("unsafe_code"))
        .and_then(toml::Value::as_str);

    (value != Some("forbid")).then(|| {
        format!(
            "root [workspace.lints.rust].unsafe_code must be exact string \"forbid\", got {value:?}"
        )
    })
}

fn member_inheritance_violation(manifest_path: &Path, manifest: &toml::Value) -> Option<String> {
    let inherited = manifest
        .get("lints")
        .and_then(|lints| lints.get("workspace"))
        .and_then(toml::Value::as_bool);

    (inherited != Some(true)).then(|| {
        format!(
            "{} ({}) must declare exact [lints] workspace = true, got {inherited:?}",
            package_name(manifest),
            manifest_path.display()
        )
    })
}

#[test]
fn workspace_and_every_member_enable_the_exact_unsafe_forbid() {
    let root = workspace_root();
    let root_manifest = read_manifest(&root.join("Cargo.toml"));
    assert_eq!(root_forbid_violation(&root_manifest), None);

    let manifests = crate_manifest_paths(&root);
    let expected = LAYER_RULES
        .iter()
        .map(|rule| rule.members.len())
        .sum::<usize>();
    assert_eq!(
        manifests.len(),
        expected,
        "workspace crate count differs from the shared layer rulebook"
    );
    for path in manifests {
        let manifest = read_manifest(&path);
        assert_eq!(member_inheritance_violation(&path, &manifest), None);
    }
}

#[test]
fn configuration_checks_fail_closed() {
    let missing_root: toml::Value = toml::from_str("[workspace.lints.rust]\n").unwrap();
    let weaker_root: toml::Value =
        toml::from_str("[workspace.lints.rust]\nunsafe_code = \"deny\"\n").unwrap();
    assert!(root_forbid_violation(&missing_root).is_some());
    assert!(root_forbid_violation(&weaker_root).is_some());

    let missing_member: toml::Value =
        toml::from_str("[package]\nname = \"probe\"\nversion = \"0.0.0\"\n").unwrap();
    let disabled_member: toml::Value = toml::from_str(
        "[package]\nname = \"probe\"\nversion = \"0.0.0\"\n[lints]\nworkspace = false\n",
    )
    .unwrap();
    let path = Path::new("crates/probe/Cargo.toml");
    assert!(member_inheritance_violation(path, &missing_member).is_some());
    assert!(member_inheritance_violation(path, &disabled_member).is_some());
}

struct TargetProbe {
    name: &'static str,
    cargo_arguments: &'static [&'static str],
    package_extra: &'static str,
    files: &'static [(&'static str, &'static str)],
}

fn single_package_workspace(probe: &TargetProbe) -> ScratchWorkspace {
    let scratch = ScratchWorkspace::new(probe.name);
    scratch.write(
        "Cargo.toml",
        r#"[workspace]
members = ["probe"]
resolver = "3"

[workspace.lints.rust]
unsafe_code = "forbid"
"#,
    );
    scratch.write(
        "probe/Cargo.toml",
        &format!(
            r#"[package]
name = "unsafe-policy-probe"
version = "0.0.0"
edition = "2024"
{}

[lints]
workspace = true
"#,
            probe.package_extra
        ),
    );
    scratch.write("probe/src/lib.rs", "pub fn safe_library() {}\n");
    for (path, contents) in probe.files {
        scratch.write(path, contents);
    }
    scratch
}

fn bounded_stderr(output: &Output) -> String {
    let end = output.stderr.len().min(MAX_FAILURE_OUTPUT_BYTES);
    String::from_utf8_lossy(&output.stderr[..end]).into_owned()
}

fn assert_unsafe_rejected(case: &str, output: &Output) {
    let stderr = bounded_stderr(output);
    assert!(
        !output.status.success(),
        "{case} unexpectedly compiled under the workspace unsafe forbid"
    );
    assert!(
        stderr.contains("unsafe-code"),
        "{case} failed without the unsafe-code lint; bounded stderr:\n{stderr}"
    );
}

#[test]
fn workspace_forbid_reaches_every_supported_target_class() {
    const UNSAFE_FUNCTION: &str =
        "pub fn probe() { unsafe { core::ptr::read_volatile(&0_u8); } }\n";
    const UNSAFE_MAIN: &str = "fn main() { unsafe { core::ptr::read_volatile(&0_u8); } }\n";
    const UNSAFE_TEST: &str =
        "#[test]\nfn probe() { unsafe { core::ptr::read_volatile(&0_u8); } }\n";
    const ATTEMPTED_ALLOW: &str =
        "#![allow(unsafe_code)]\npub fn probe() { unsafe { core::ptr::read_volatile(&0_u8); } }\n";

    let probes = [
        TargetProbe {
            name: "library",
            cargo_arguments: &["check", "--offline", "-p", "unsafe-policy-probe", "--lib"],
            package_extra: "",
            files: &[("probe/src/lib.rs", UNSAFE_FUNCTION)],
        },
        TargetProbe {
            name: "binary",
            cargo_arguments: &[
                "check",
                "--offline",
                "-p",
                "unsafe-policy-probe",
                "--bin",
                "unsafe-policy-probe",
            ],
            package_extra: "",
            files: &[("probe/src/main.rs", UNSAFE_MAIN)],
        },
        TargetProbe {
            name: "integration-test",
            cargo_arguments: &[
                "check",
                "--offline",
                "-p",
                "unsafe-policy-probe",
                "--test",
                "unsafe",
            ],
            package_extra: "",
            files: &[("probe/tests/unsafe.rs", UNSAFE_TEST)],
        },
        TargetProbe {
            name: "example",
            cargo_arguments: &[
                "check",
                "--offline",
                "-p",
                "unsafe-policy-probe",
                "--example",
                "unsafe",
            ],
            package_extra: "",
            files: &[("probe/examples/unsafe.rs", UNSAFE_MAIN)],
        },
        TargetProbe {
            name: "benchmark",
            cargo_arguments: &[
                "check",
                "--offline",
                "-p",
                "unsafe-policy-probe",
                "--bench",
                "unsafe",
            ],
            package_extra: "",
            files: &[("probe/benches/unsafe.rs", UNSAFE_MAIN)],
        },
        TargetProbe {
            name: "build-script",
            cargo_arguments: &["check", "--offline", "-p", "unsafe-policy-probe", "--lib"],
            package_extra: "build = \"build.rs\"",
            files: &[("probe/build.rs", UNSAFE_MAIN)],
        },
        TargetProbe {
            name: "proc-macro",
            cargo_arguments: &["check", "--offline", "-p", "unsafe-policy-probe", "--lib"],
            package_extra: "\n[lib]\nproc-macro = true",
            files: &[(
                "probe/src/lib.rs",
                r#"extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn probe(_: TokenStream) -> TokenStream {
    unsafe { core::ptr::read_volatile(&0_u8); }
    TokenStream::new()
}
"#,
            )],
        },
        TargetProbe {
            name: "source-allow-override",
            cargo_arguments: &["check", "--offline", "-p", "unsafe-policy-probe", "--lib"],
            package_extra: "",
            files: &[("probe/src/lib.rs", ATTEMPTED_ALLOW)],
        },
    ];

    for probe in probes {
        let scratch = single_package_workspace(&probe);
        let output = scratch.cargo(probe.cargo_arguments);
        assert_unsafe_rejected(probe.name, &output);
    }
}

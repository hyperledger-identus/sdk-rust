#!/usr/bin/env python3
"""Regression tests for the SDK support-policy validator."""

from __future__ import annotations

import importlib.util
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-support-policy.py"
BENCHMARK = ROOT / "scripts/benchmark-support-policy.py"


def load_benchmark():
    spec = importlib.util.spec_from_file_location("support_policy_benchmark", BENCHMARK)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BENCHMARK_MODULE = load_benchmark()


class SupportPolicyBenchmarkTests(unittest.TestCase):
    @staticmethod
    def measurement(p50: float, p95: float) -> dict[str, float]:
        return {
            "warm_p50_ms": p50,
            "warm_p95_ms": p95,
            "process_cold_p50_ms": p50 * 10,
            "process_cold_p95_ms": p95 * 10,
        }

    def test_isolated_p95_outlier_remains_diagnostic(self) -> None:
        baseline = self.measurement(5.0, 6.0)
        current = self.measurement(6.0, 30.0)
        comparisons, regressions = BENCHMARK_MODULE.compare_measurements(
            current, baseline
        )
        self.assertEqual(comparisons["warm_p95_ms_ratio"], 5.0)
        self.assertEqual(regressions, [])

    def test_sustained_p50_pathology_fails(self) -> None:
        baseline = self.measurement(5.0, 6.0)
        current = self.measurement(16.0, 30.0)
        _, regressions = BENCHMARK_MODULE.compare_measurements(current, baseline)
        self.assertIn("warm_p50_ms", regressions[0])


class SupportPolicyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.fixture = Path(self.temporary.name)
        for relative in [
            "Cargo.toml",
            "flake.nix",
            "flake.lock",
            "docs/architecture/sdk-support-policy.toml",
            "docs/adr/0002-neoprism-toolchain-alignment.md",
            "nix/rust-toolchain.nix",
        ]:
            destination = self.fixture / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        shutil.copytree(ROOT / "nix/checks", self.fixture / "nix/checks")
        for manifest in (ROOT / "crates").glob("*/Cargo.toml"):
            destination = self.fixture / manifest.relative_to(ROOT)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(manifest, destination)

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def run_checker(self) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(CHECKER), str(self.fixture)],
            check=False,
            capture_output=True,
            text=True,
        )

    def replace(self, relative: str, old: str, new: str) -> None:
        path = self.fixture / relative
        contents = path.read_text(encoding="utf-8")
        self.assertIn(old, contents)
        path.write_text(contents.replace(old, new, 1), encoding="utf-8")

    def replace_gate(self, name: str, old: str, new: str) -> None:
        path = self.fixture / "nix/checks/gates.toml"
        contents = path.read_text(encoding="utf-8")
        marker = re.search(rf'^name\s*=\s*"{re.escape(name)}"$', contents, re.MULTILINE)
        self.assertIsNotNone(marker)
        assert marker is not None
        marker_index = marker.start()
        start = contents.rfind("[[gates]]", 0, marker_index)
        end = contents.find("[[gates]]", marker.end())
        if end == -1:
            end = len(contents)
        block = re.sub(r"\s*=\s*", " = ", contents[start:end])
        block = block.replace("[  ]", "[]")
        self.assertIn(old, block)
        path.write_text(
            contents[:start] + block.replace(old, new, 1) + contents[end:],
            encoding="utf-8",
        )

    def assert_fails(self, expected: str) -> None:
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertIn(expected, result.stderr)

    def test_canonical_policy_passes(self) -> None:
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_cargo_msrv_drift_fails(self) -> None:
        self.replace("Cargo.toml", 'rust-version = "1.85.0"', 'rust-version = "1.86.0"')
        self.assert_fails("does not match policy MSRV")

    def test_missing_dimension_fails(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml", "[ffi]", "[removed_ffi]"
        )
        self.assert_fails("policy is missing [ffi]")

    def test_removed_gate_fails(self) -> None:
        self.replace_gate(
            "rust-build-wasm32", "rust-build-wasm32", "removed-rust-build-wasm32"
        )
        self.assert_fails("undefined Nix gate rust-build-wasm32")

    def test_gate_must_use_declared_crane_operation(self) -> None:
        self.replace_gate(
            "rust-test", 'operation = "cargoNextest"', 'operation = "cargoBuild"'
        )
        self.assert_fails(
            "gate rust-test uses Crane operation cargoBuild, expected cargoNextest"
        )

    def test_manifest_generator_must_be_imported(self) -> None:
        self.replace(
            "nix/checks/default.nix",
            "    ./rust-gates.nix\n",
            "    # ./rust-gates.nix\n",
        )
        self.assert_fails("does not import rust-gates.nix")

    def test_live_string_decoy_cannot_replace_generator_import(self) -> None:
        self.replace(
            "nix/checks/default.nix",
            "  imports = [\n    ./rust-gates.nix\n  ];",
            '  imports = [];\n  _module.args.gateDecoy = "./rust-gates.nix";',
        )
        self.assert_fails("does not import rust-gates.nix")

    def test_unused_manifest_mapping_cannot_replace_published_checks(self) -> None:
        self.replace(
            "nix/checks/rust-gates.nix",
            "      generatedChecks = listToAttrs (",
            "      unusedGeneratedChecks = listToAttrs (",
        )
        self.replace(
            "nix/checks/rust-gates.nix",
            "        }) manifest.gates\n      );\n    in",
            "        }) manifest.gates\n      );\n      generatedChecks = { };\n    in",
        )
        self.assert_fails("does not map gate names and values from manifest entries")

    def test_constant_mapped_name_cannot_collapse_gate_graph(self) -> None:
        self.replace(
            "nix/checks/rust-gates.nix",
            "          inherit (gate) name;",
            '          name = "rust-gate";',
        )
        self.assert_fails("does not map gate names and values from manifest entries")

    def test_mapped_value_must_use_current_gate(self) -> None:
        self.replace(
            "nix/checks/rust-gates.nix",
            "          value = makeGate gate;",
            "          value = { };",
        )
        self.assert_fails("does not map gate names and values from manifest entries")

    def test_assertion_decoy_cannot_replace_returned_checks(self) -> None:
        self.replace(
            "nix/checks/rust-gates.nix",
            """    in
    {
      checks = generatedChecks;
    };
}""",
            """    in
    assert builtins.isAttrs { checks = generatedChecks; };
    {
      checks = { };
    };
}""",
        )
        self.assert_fails("does not return generatedChecks as top-level checks")

    def test_shadowed_generated_checks_cannot_replace_mapped_result(self) -> None:
        self.replace(
            "nix/checks/rust-gates.nix",
            """    in
    {
      checks = generatedChecks;
    };
}""",
            """    in
    let
      generatedChecks = { };
    in
    {
      checks = generatedChecks;
    };
}""",
        )
        self.assert_fails(
            "does not directly return its manifest-mapped generatedChecks"
        )

    def test_check_graph_must_be_imported_by_flake(self) -> None:
        self.replace("flake.nix", "        ./nix/checks\n", "")
        self.assert_fails("flake.nix does not import the nix/checks module")

    def test_gate_without_target_evidence_fails(self) -> None:
        self.replace_gate(
            "rust-build-wasm32", 'target = "wasm32-unknown-unknown"', 'target = ""'
        )
        self.assert_fails("does not contain evidence token")

    def test_target_gate_must_bind_structured_target(self) -> None:
        self.replace_gate(
            "rust-build-wasm32",
            'target = "wasm32-unknown-unknown"',
            'target = "aarch64-linux-android"',
        )
        self.assert_fails("uses Cargo target 'aarch64-linux-android'")

    def test_target_evidence_cannot_come_from_neighboring_gate(self) -> None:
        self.replace_gate(
            "rust-build-android-aarch64",
            'target = "aarch64-linux-android"',
            'target = "aarch64-apple-ios"',
        )
        self.replace_gate(
            "rust-build-ios-aarch64",
            'target = "aarch64-apple-ios"',
            'target = "aarch64-linux-android"',
        )
        self.assert_fails("rust-build-android-aarch64 does not contain evidence token")

    def test_target_gate_must_build_every_declared_package(self) -> None:
        self.replace_gate(
            "rust-build-wasm32",
            'packages = [ "identus-core", "identus-crypto", "identus-did", "identus-adapters-entropy" ]',
            'packages = [ "identus-core", "identus-crypto", "identus-did" ]',
        )
        self.assert_fails("rust-build-wasm32 selects packages")

    def test_target_gate_must_activate_declared_features(self) -> None:
        self.replace_gate(
            "rust-build-wasm32",
            'features = [ "identus-adapters-entropy/getrandom" ]',
            "features = []",
        )
        self.assert_fails("rust-build-wasm32 selects packages")

    def test_feature_gate_must_preserve_default_feature_mode(self) -> None:
        self.replace_gate(
            "rust-test-entropy-deterministic",
            "no_default_features = true",
            "no_default_features = false",
        )
        self.assert_fails("rust-test-entropy-deterministic selects packages")

    def test_feature_gate_must_select_declared_package(self) -> None:
        self.replace_gate(
            "rust-test-kmp-compat",
            'packages = [ "identus-crypto" ]',
            'packages = [ "identus-core" ]',
        )
        self.assert_fails("rust-test-kmp-compat selects packages")

    def test_feature_gate_must_select_complete_feature_set(self) -> None:
        self.replace_gate(
            "rust-test-entropy-deterministic",
            'features = [ "deterministic" ]',
            'features = [ "deterministic", "getrandom" ]',
        )
        self.assert_fails("rust-test-entropy-deterministic selects packages")

    def test_feature_gate_rejects_shell_encoded_feature_values(self) -> None:
        self.replace_gate(
            "rust-test-entropy-deterministic",
            'features = [ "deterministic" ]',
            'features = [ "deterministic getrandom" ]',
        )
        self.assert_fails("references missing feature 'deterministic getrandom'")

    def test_workspace_feature_gate_cannot_exclude_a_package(self) -> None:
        self.replace_gate(
            "rust-test",
            "exclude_packages = []",
            'exclude_packages = [ "identus-crypto" ]',
        )
        self.assert_fails("excludes=['identus-crypto']")

    def test_workspace_feature_gate_must_select_workspace_explicitly(self) -> None:
        self.replace_gate("rust-test", "workspace = true", "workspace = false")
        self.assert_fails("workspace=False")

    def test_every_feature_surface_requires_an_msrv_gate(self) -> None:
        self.replace_gate(
            "rust-msrv-crypto-kmp-compat",
            "rust-msrv-crypto-kmp-compat",
            "removed-rust-msrv-crypto-kmp-compat",
        )
        self.assert_fails(
            "feature crypto-kmp-compat MSRV references undefined Nix gate"
        )

    def test_msrv_gate_must_preserve_feature_selection(self) -> None:
        self.replace_gate(
            "rust-msrv-entropy-deterministic",
            "no_default_features = true",
            "no_default_features = false",
        )
        self.assert_fails("rust-msrv-entropy-deterministic selects packages")

    def test_msrv_gate_must_use_stable_toolchain_builder(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            'msrv_gate           = "rust-msrv-crypto-kmp-compat"',
            'msrv_gate           = "rust-test-kmp-compat"',
        )
        self.assert_fails("rust-test-kmp-compat is not built with the MSRV toolchain")

    def test_msrv_crane_library_must_wrap_stable_toolchain(self) -> None:
        self.replace(
            "nix/rust-toolchain.nix",
            "overrideToolchain msrvToolchain",
            "overrideToolchain toolchain",
        )
        self.assert_fails("does not wire msrvCraneLib to msrvToolchain")

    def test_unknown_gate_field_fails(self) -> None:
        self.replace_gate(
            "rust-build-wasm32",
            "extra_args = []",
            'extra_args = []\ncargoBuildCommand = "cargo build --wrong"',
        )
        self.assert_fails("unknown=['cargoBuildCommand']")

    def test_invalid_list_field_fails_without_traceback(self) -> None:
        self.replace_gate(
            "rust-build-wasm32",
            'packages = [ "identus-core", "identus-crypto", "identus-did", "identus-adapters-entropy" ]',
            "packages = 7",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertIn("must be a list of non-empty strings", result.stderr)
        self.assertNotIn("Traceback", result.stderr)

    def test_compound_enum_fields_fail_without_traceback(self) -> None:
        for field, value in (
            ("operation", "cargoBuild"),
            ("toolchain", "etalon"),
            ("source", "rust"),
            ("artifacts", "etalon"),
        ):
            self.replace_gate(
                "rust-build-wasm32",
                f'{field} = "{value}"',
                f'{field} = [ "{value}" ]',
            )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0, result.stdout)
        for expected in (
            "unsupported operation",
            "invalid toolchain",
            "invalid source",
            "invalid artifacts",
        ):
            self.assertIn(expected, result.stderr)
        self.assertNotIn("Traceback", result.stderr)

    def test_extra_args_cannot_smuggle_selection(self) -> None:
        self.replace_gate(
            "rust-build-wasm32",
            "extra_args = []",
            'extra_args = [ "--features", "fake" ]',
        )
        self.assert_fails("cargoBuild requires extra_args=[]")

    def test_duplicate_gate_name_fails(self) -> None:
        path = self.fixture / "nix/checks/gates.toml"
        contents = path.read_text(encoding="utf-8")
        first = contents.index("[[gates]]")
        second = contents.index("[[gates]]", first + 1)
        path.write_text(contents + contents[first:second], encoding="utf-8")
        self.assert_fails("duplicate gate 'rust-fmt'")

    def test_dynamic_nix_and_quote_comment_decoys_are_ignored(self) -> None:
        self.replace(
            "nix/checks/rust-gates.nix",
            "      manifest = builtins.fromTOML",
            """      decoy = ''
        # rust-build-wasm32 --target wrong-target
        ${builtins.toString \"rust-test --features fake,quoted\"}
      '';
      manifest = builtins.fromTOML""",
        )
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_dead_module_args_cannot_replace_manifest_gate(self) -> None:
        self.replace_gate(
            "rust-build-wasm32", "rust-build-wasm32", "removed-rust-build-wasm32"
        )
        self.replace(
            "nix/checks/default.nix",
            "      _module.args = {",
            """      _module.args.gateDecoy = ''rust-build-wasm32'';
      _module.args = {""",
        )
        self.assert_fails("undefined Nix gate rust-build-wasm32")

    def test_non_cargo_operation_rejects_selection(self) -> None:
        self.replace_gate("rust-fmt", "workspace = false", "workspace = true")
        self.assert_fails("cargoFmt cannot carry Cargo selection")

    def test_contradictory_workspace_and_packages_fail(self) -> None:
        self.replace_gate("rust-test", "packages = []", 'packages = [ "identus-core" ]')
        self.assert_fails("cannot select workspace and explicit packages")

    def test_cargo_gate_requires_explicit_package_mode(self) -> None:
        self.replace_gate("rust-test", "workspace = true", "workspace = false")
        self.assert_fails("requires explicit workspace or packages")

    def test_lib_and_all_targets_are_contradictory(self) -> None:
        self.replace_gate(
            "rust-msrv-crypto-minimal", "all_targets = false", "all_targets = true"
        )
        self.assert_fails("cannot select lib and all_targets")

    def test_no_default_and_all_features_are_contradictory(self) -> None:
        self.replace_gate(
            "rust-msrv-entropy-all",
            "no_default_features = false",
            "no_default_features = true",
        )
        self.assert_fails("cannot select no_default_features and all_features")

    def test_unknown_target_package_fails(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            '  "identus-did",',
            '  "unknown-package",',
        )
        self.assert_fails("names unknown packages")

    def test_duplicate_host_system_fails(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            "[[hosts]]",
            """[[hosts]]
nix_system = "x86_64-linux"
rust_target = "contradictory"
tier = "planned"
gates = []
limitation = "Contradictory duplicate."

[[hosts]]""",
        )
        self.assert_fails("duplicate host system 'x86_64-linux'")

    def test_duplicate_target_triple_fails(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            "[[targets]]",
            """[[targets]]
triple = "wasm32-unknown-unknown"
surface = "contradictory"
tier = "planned"
packages = []
limitation = "Contradictory duplicate."

[[targets]]""",
        )
        self.assert_fails("duplicate target triple 'wasm32-unknown-unknown'")

    def test_duplicate_feature_surface_fails(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            "[[features]]",
            """[[features]]
name = "workspace-default"
package = "identus-core"
no_default_features = false
features = []
gates = [ "rust-test" ]
msrv_gate = "rust-msrv"
evidence_token = "contradictory"

[[features]]""",
        )
        self.assert_fails("duplicate feature surface 'workspace-default'")


if __name__ == "__main__":
    unittest.main()

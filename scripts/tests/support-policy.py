#!/usr/bin/env python3
"""Regression tests for the SDK support-policy validator."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-support-policy.py"


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

    def test_canonical_policy_passes(self) -> None:
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_cargo_msrv_drift_fails(self) -> None:
        self.replace("Cargo.toml", 'rust-version = "1.85.0"', 'rust-version = "1.86.0"')
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not match policy MSRV", result.stderr)

    def test_missing_dimension_fails(self) -> None:
        self.replace("docs/architecture/sdk-support-policy.toml", "[ffi]", "[removed_ffi]")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("policy is missing [ffi]", result.stderr)

    def test_removed_gate_fails(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            "checks.rust-build-wasm32",
            "checks.removed-rust-build-wasm32",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("undefined Nix gate rust-build-wasm32", result.stderr)

    def test_gate_must_use_declared_crane_operation(self) -> None:
        self.replace(
            "nix/checks/rust-test.nix",
            "craneLib.cargoNextest",
            "craneLib.cargoBuild",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "gate rust-test uses Crane operation cargoBuild, expected cargoNextest",
            result.stderr,
        )

    def test_unimported_gate_module_is_not_discovered(self) -> None:
        self.replace(
            "nix/checks/default.nix",
            "    ./rust-build-mobile.nix\n",
            "    # ./rust-build-mobile.nix\n",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "target aarch64-linux-android references undefined Nix gate",
            result.stderr,
        )

    def test_check_graph_must_be_imported_by_flake(self) -> None:
        self.replace("flake.nix", "        ./nix/checks\n", "")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "flake.nix does not import the nix/checks module", result.stderr
        )

    def test_gate_without_target_evidence_fails(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            "wasm32-unknown-unknown",
            "removed-wasm-target",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not contain evidence token", result.stderr)

    def test_target_gate_must_bind_triple_to_cargo_target_option(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            "      checks.rust-build-wasm32 = craneLib.cargoBuild {\n",
            """      checks.rust-build-wasm32 = craneLib.cargoBuild {
        pname = "wasm32-unknown-unknown";
""",
        )
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            "--target wasm32-unknown-unknown",
            "--target aarch64-linux-android",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "target wasm32-unknown-unknown gate rust-build-wasm32 uses Cargo targets ['aarch64-linux-android']",
            result.stderr,
        )

    def test_target_evidence_cannot_come_from_neighboring_gate(self) -> None:
        path = self.fixture / "nix/checks/rust-build-mobile.nix"
        contents = path.read_text(encoding="utf-8")
        contents = contents.replace("aarch64-linux-android", "swapped-target", 1)
        contents = contents.replace(
            "aarch64-apple-ios", "aarch64-linux-android", 1
        )
        contents = contents.replace("swapped-target", "aarch64-apple-ios", 1)
        path.write_text(contents, encoding="utf-8")

        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "target aarch64-linux-android gate rust-build-android-aarch64 does not contain evidence token",
            result.stderr,
        )

    def test_target_gate_must_build_every_declared_package(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            " -p identus-adapters-entropy --features",
            " --features",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "target wasm32-unknown-unknown gate rust-build-wasm32 selects packages",
            result.stderr,
        )

    def test_target_gate_must_activate_declared_features(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            " --features identus-adapters-entropy/getrandom",
            "",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "target wasm32-unknown-unknown gate rust-build-wasm32 selects",
            result.stderr,
        )

    def test_feature_gate_must_preserve_default_feature_mode(self) -> None:
        self.replace(
            "nix/checks/rust-feature-matrix.nix",
            "--no-default-features --features deterministic",
            "--features deterministic",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature entropy-deterministic gate rust-test-entropy-deterministic selects",
            result.stderr,
        )

    def test_feature_gate_must_select_declared_package(self) -> None:
        self.replace(
            "nix/checks/rust-test-kmp-compat.nix",
            "-p identus-crypto --features kmp-compat",
            "-p identus-core --features kmp-compat",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature crypto-kmp-compat gate rust-test-kmp-compat selects",
            result.stderr,
        )

    def test_feature_gate_must_select_complete_feature_set(self) -> None:
        self.replace(
            "nix/checks/rust-feature-matrix.nix",
            "--features deterministic --no-fail-fast",
            "--features deterministic,getrandom --no-fail-fast",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature entropy-deterministic gate rust-test-entropy-deterministic selects",
            result.stderr,
        )

    def test_feature_gate_parses_space_separated_feature_values(self) -> None:
        self.replace(
            "nix/checks/rust-feature-matrix.nix",
            "--features deterministic --no-fail-fast",
            "--features deterministic getrandom --no-fail-fast",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature entropy-deterministic gate rust-test-entropy-deterministic selects",
            result.stderr,
        )
        self.assertIn("'deterministic', 'getrandom'", result.stderr)

    def test_workspace_feature_gate_cannot_exclude_a_package(self) -> None:
        self.replace(
            "nix/checks/rust-test.nix",
            "--workspace --no-fail-fast --no-tests=pass",
            "--workspace --exclude identus-crypto --no-fail-fast --no-tests=pass",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature workspace-default gate rust-test selects packages",
            result.stderr,
        )
        self.assertIn("excludes=['identus-crypto']", result.stderr)

    def test_workspace_feature_gate_must_select_workspace_explicitly(self) -> None:
        self.replace(
            "nix/checks/rust-test.nix",
            "--workspace --no-fail-fast --no-tests=pass",
            "--no-fail-fast --no-tests=pass",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature workspace-default gate rust-test selects packages",
            result.stderr,
        )
        self.assertIn("workspace=False", result.stderr)

    def test_every_feature_surface_requires_an_msrv_gate(self) -> None:
        self.replace(
            "nix/checks/rust-msrv.nix",
            "rust-msrv-crypto-kmp-compat",
            "removed-rust-msrv-crypto-kmp-compat",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature crypto-kmp-compat MSRV references undefined Nix gate",
            result.stderr,
        )

    def test_msrv_gate_must_preserve_feature_selection(self) -> None:
        self.replace(
            "nix/checks/rust-msrv.nix",
            "--no-default-features --features deterministic",
            "--features deterministic",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature entropy-deterministic MSRV gate rust-msrv-entropy-deterministic selects",
            result.stderr,
        )

    def test_msrv_gate_must_use_stable_toolchain_builder(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            'msrv_gate           = "rust-msrv-crypto-kmp-compat"',
            'msrv_gate           = "rust-test-kmp-compat"',
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "feature crypto-kmp-compat MSRV gate rust-test-kmp-compat is not built with msrvCraneLib",
            result.stderr,
        )

    def test_msrv_crane_library_must_wrap_stable_toolchain(self) -> None:
        self.replace(
            "nix/rust-toolchain.nix",
            "overrideToolchain msrvToolchain",
            "overrideToolchain toolchain",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "does not wire msrvCraneLib to msrvToolchain", result.stderr
        )

    def test_ignored_crane_build_attribute_fails(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            "cargoExtraArgs",
            "cargoBuildCommand",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("passes an ignored custom command", result.stderr)

    def test_unknown_target_package_fails(self) -> None:
        self.replace(
            "docs/architecture/sdk-support-policy.toml",
            '"identus-did",',
            '"unknown-package",',
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("names unknown packages", result.stderr)

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
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "policy contains duplicate host system 'x86_64-linux'", result.stderr
        )

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
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "policy contains duplicate target triple 'wasm32-unknown-unknown'",
            result.stderr,
        )

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
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "policy contains duplicate feature surface 'workspace-default'",
            result.stderr,
        )


if __name__ == "__main__":
    unittest.main()

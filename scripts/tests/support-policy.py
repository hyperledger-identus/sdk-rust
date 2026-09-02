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

    def test_gate_without_target_evidence_fails(self) -> None:
        self.replace(
            "nix/checks/rust-build-wasm32.nix",
            "wasm32-unknown-unknown",
            "removed-wasm-target",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not contain evidence token", result.stderr)

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


if __name__ == "__main__":
    unittest.main()

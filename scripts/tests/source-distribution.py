#!/usr/bin/env python3
"""Mutation tests for the public-source distribution contract."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-source-distribution.py"
FILES = (
    Path("Cargo.toml"),
    Path("README.md"),
    Path("docs/architecture/source-distribution.md"),
    Path("crates/core/Cargo.toml"),
    Path("crates/derive/Cargo.toml"),
    Path("crates/crypto/Cargo.toml"),
    Path("crates/did/Cargo.toml"),
    Path("crates/did-resolver-http/Cargo.toml"),
)


class SourceDistributionContract(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        for relative in FILES:
            target = self.root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, target)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def run_checker(self) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [str(CHECKER), str(self.root)],
            check=False,
            capture_output=True,
            text=True,
        )

    def replace(self, relative: Path, old: str, new: str) -> None:
        path = self.root / relative
        content = path.read_text(encoding="utf-8")
        self.assertIn(old, content)
        path.write_text(content.replace(old, new, 1), encoding="utf-8")

    def test_canonical_contract_passes(self) -> None:
        self.assertEqual(self.run_checker().returncode, 0)

    def test_short_revision_fails(self) -> None:
        self.replace(
            Path("docs/architecture/source-distribution.md"),
            "04b45b7fceb094ae601e3cf7a3291de0a0248b57",
            "04b45b7",
        )
        self.assertIn("full lowercase Git SHA", self.run_checker().stderr)

    def test_branch_selector_fails(self) -> None:
        self.replace(
            Path("docs/architecture/source-distribution.md"),
            'rev = "SDK_COMMIT"',
            'branch = "develop"',
        )
        self.assertIn("mutable/short selector", self.run_checker().stderr)

    def test_private_source_fails(self) -> None:
        self.replace(
            Path("docs/architecture/source-distribution.md"),
            "https://github.com/hyperledger-identus/sdk-rust",
            "ssh://git@github.com/hyperledger-identus/sdk-rust",
        )
        self.assertIn("canonical public HTTPS", self.run_checker().stderr)

    def test_publishable_workspace_fails(self) -> None:
        self.replace(Path("Cargo.toml"), "publish      = false", "publish      = true")
        self.assertIn("publish must remain false", self.run_checker().stderr)

    def test_package_identity_drift_fails(self) -> None:
        self.replace(
            Path("crates/crypto/Cargo.toml"),
            'name                   = "identus-crypto"',
            'name                   = "identus-apollo"',
        )
        self.assertIn("package identity differs", self.run_checker().stderr)

    def test_missing_lock_contract_fails(self) -> None:
        self.replace(
            Path("docs/architecture/source-distribution.md"),
            "Cargo.lock",
            "dependency lock",
        )
        self.assertIn("Cargo.lock", self.run_checker().stderr)

    def test_missing_readme_link_fails(self) -> None:
        self.replace(
            Path("README.md"),
            "[Alpha source distribution](docs/architecture/source-distribution.md)",
            "Alpha source distribution",
        )
        self.assertIn("README must link", self.run_checker().stderr)


if __name__ == "__main__":
    unittest.main()

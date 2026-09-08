#!/usr/bin/env python3
"""Mutation tests for the Apollo parity manifest contract."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-apollo-parity.py"
MANIFEST = Path("docs/architecture/apollo-crypto-parity.toml")


class ApolloParityContract(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        source = ROOT / MANIFEST
        target = self.root / MANIFEST
        target.parent.mkdir(parents=True)
        shutil.copy2(source, target)
        data = tomllib.loads(source.read_text(encoding="utf-8"))
        for field in ("sdk_harness_path", "sdk_runner_path", "workflow_path"):
            relative = Path(data["performance"][field])
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        for vector in data["vectors"]:
            relative = Path(vector["sdk_test_path"])
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def run_checker(self, *arguments: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [str(CHECKER), str(self.root), *arguments],
            check=False, capture_output=True, text=True,
        )

    def replace(self, old: str, new: str, count: int = 1) -> None:
        path = self.root / MANIFEST
        content = path.read_text(encoding="utf-8")
        self.assertIn(old, content)
        path.write_text(content.replace(old, new, count), encoding="utf-8")

    def test_canonical_manifest_passes(self) -> None:
        self.assertEqual(self.run_checker().returncode, 0)

    def test_unknown_disposition_fails(self) -> None:
        self.replace('disposition       = "parity"', 'disposition       = "maybe"')
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unsupported disposition", result.stderr)

    def test_duplicate_capability_fails(self) -> None:
        self.replace('id                = "encoding-hex"', 'id                = "entropy-csprng"')
        self.assertIn("duplicate ids", self.run_checker().stderr)

    def test_unknown_vector_reference_fails(self) -> None:
        self.replace(
            'vector_ids        = [ "apollo-secure-random" ]',
            'vector_ids        = [ "absent-vector" ]',
        )
        self.assertIn("unknown vector ids", self.run_checker().stderr)

    def test_vector_inventory_drift_fails(self) -> None:
        self.replace(
            'id                 = "sdk-hmac-sha512-rfc4231"',
            'id                 = "new-vector"',
        )
        self.assertIn("vectors: inventory differs", self.run_checker().stderr)

    def test_summary_drift_fails(self) -> None:
        self.replace("total               = 27", "total               = 26")
        self.assertIn("summary: expected", self.run_checker().stderr)

    def test_performance_comparison_claim_fails(self) -> None:
        self.replace('apollo_comparison   = "unavailable"', 'apollo_comparison   = "faster"')
        self.assertIn("comparison must be unavailable", self.run_checker().stderr)

    def test_performance_harness_must_be_commit_pinned(self) -> None:
        self.replace(
            "blob/0f1074b766f3cbf1ffc905b4388220b27b1c4556/crates/crypto/examples/crypto_baseline.rs",
            "blob/develop/crates/crypto/examples/crypto_baseline.rs",
        )
        self.assertIn("not commit-pinned", self.run_checker().stderr)

    def test_stale_apollo_link_fails(self) -> None:
        self.replace("ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c/apollo/src", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/apollo/src")
        self.assertIn("not baseline-pinned", self.run_checker().stderr)

    def test_stale_ci_receipt_fails(self) -> None:
        self.replace(
            'ci_receipt        = "https://github.com/hyperledger-identus/sdk-rust/actions/runs/34262060203"',
            'ci_receipt        = "https://github.com/hyperledger-identus/sdk-rust/actions/runs/1"',
        )
        self.assertIn("CI receipt differs", self.run_checker().stderr)

    def test_missing_selector_fails(self) -> None:
        self.replace('"sha2_empty_message_known_answers"', '"selector_does_not_exist"')
        self.assertIn("selector not found", self.run_checker().stderr)

    def test_unsafe_test_path_fails(self) -> None:
        self.replace(
            'sdk_test_path      = "crates/adapters-entropy/src/lib.rs"',
            'sdk_test_path      = "../outside.rs"',
        )
        self.assertIn("unsafe repository-relative path", self.run_checker().stderr)

    def test_symlink_escape_fails(self) -> None:
        test_path = self.root / "crates/adapters-entropy/src/lib.rs"
        outside = self.root.parent / f"{self.root.name}-outside.rs"
        outside.write_text("apollo_secure_random_contract", encoding="utf-8")
        self.addCleanup(outside.unlink, missing_ok=True)
        test_path.unlink()
        test_path.symlink_to(outside)
        self.assertIn("resolves outside", self.run_checker().stderr)

    def test_render_is_deterministic(self) -> None:
        first = self.run_checker("--render-markdown")
        second = self.run_checker("--render-markdown")
        self.assertEqual(first.returncode, 0)
        self.assertEqual(first.stdout, second.stdout)
        self.assertIn("| 27 | 14 | 6 | 7 | 0 |", first.stdout)
        self.assertIn("`cardano-v2-private`", first.stdout)


if __name__ == "__main__":
    unittest.main()
